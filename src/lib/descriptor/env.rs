use crate::error::CargoMakeError;
use crate::types::{
    EnvFile, EnvValue, EnvValueConditioned, EnvValueDecode, EnvValuePathGlob, EnvValueScript,
};
use indexmap::{IndexMap, IndexSet};
use once_cell::sync::Lazy;
use petgraph::algo::{kosaraju_scc, toposort};
use petgraph::graphmap::{DiGraphMap, GraphMap};
use regex::Regex;

#[cfg(test)]
#[path = "env_test.rs"]
mod env_test;

static RE_VARIABLE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\{.*?}").unwrap());

fn env_unique<'a>(vals: &'a [&'a IndexMap<String, EnvValue>]) -> Vec<(&'a str, &'a EnvValue)> {
    let mut visited = IndexSet::new();
    let mut unique = vec![];

    // iterate through the list in reverse, only taking the first value, then
    // reversing again to make sure that we still adhere to the order.
    // This way we will only ever take the latest value.
    for (key, val) in vals.iter().map(|map| map.iter()).flatten().rev() {
        if visited.contains(&key.as_str()) {
            continue;
        }

        visited.insert(key.as_str());
        unique.push((key.as_str(), val))
    }

    unique.reverse();
    unique
}

static RE_SH_PARAM: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"\$(?:([\w-]+)|\{#?([\w-]+)})").unwrap());

/// The depends_on for a script is a bit more complicated to find,
/// this is because it invokes a shell script (`sh`).
/// This means, that we need to go through the shell script and find all environmental variables
/// that are used, but haven't been declared yet.
///
/// One can also explicitly use a list `depends_on = [...]` to overwrite the existing behavior
/// for finding all dependencies.
///
/// This implementation is very conservative, and regardless of the context, will just capture
/// all environment variables that have been used.
///
/// A future implementation might further extend this, by looking for `'` usage,
/// `declare` statements of `[declare] name=value`, and then exclude those in the
/// subsequent analysis.
///
/// To be completely compliant this implementation is based on:
/// https://pubs.opengroup.org/onlinepubs/9699919799/, the official POSIX standard.
///
/// Chapter 8 Environment Variables:
///
/// > Environment variable names used by the utilities in the Shell and Utilities volume of
/// > POSIX.1-2017 consist solely of uppercase letters, digits, and the <underscore> ( '_' )
/// > from the characters defined in Portable Character Set and do not begin with a digit.
/// > Other characters may be permitted by an implementation;
/// > applications shall tolerate the presence of such names.
/// > Uppercase and lowercase letters shall retain their unique identities and shall not be folded
/// > together.
/// > The name space of environment variable names containing lowercase letters is
/// > reserved for applications.
/// > Applications can define any environment variables with names
/// > from this name space without modifying the behavior of the standard utilities.
///
/// A declaration in a shell script is: `declare var=...` where `declare` is optional.
fn env_depends_on_find_script(val: &EnvValueScript) -> Vec<&str> {
    if let Some(depends_on) = &val.depends_on {
        return depends_on.iter().map(String::as_str).collect();
    }

    let mut depends_on = vec![];
    for line in &val.script {
        for captures in RE_SH_PARAM.captures_iter(line) {
            if let Some(capture) = captures.get(1).or_else(|| captures.get(2)) {
                depends_on.push(capture.as_str());
            }
        }
    }

    depends_on
}

fn env_depends_on_find(val: &str) -> Vec<&str> {
    let mut depends_on = vec![];

    for matched in RE_VARIABLE.find_iter(val) {
        let matched = matched.as_str();
        // remove the first two characters (`${`)
        let (_, matched) = matched.split_at(2);
        // remove the last character (`}`)
        let (matched, _) = matched.split_at(matched.len() - 1);

        depends_on.push(matched.trim());
    }

    depends_on
}

fn env_depends_on(val: &EnvValue) -> Vec<&str> {
    match val {
        EnvValue::Value(value) => env_depends_on_find(value),
        EnvValue::Decode(EnvValueDecode { source, .. }) => env_depends_on_find(source),
        EnvValue::List(values) => values
            .iter()
            .map(|value| env_depends_on_find(value))
            .reduce(|mut acc, mut other| {
                acc.append(&mut other);
                acc
            })
            .unwrap_or_default(),
        EnvValue::Conditional(EnvValueConditioned { value, .. }) => env_depends_on_find(value),
        EnvValue::PathGlob(EnvValuePathGlob { glob, .. }) => env_depends_on_find(glob),
        EnvValue::Script(script) => env_depends_on_find_script(script),
        _ => vec![],
    }
}

pub(crate) fn merge_env(
    base: &IndexMap<String, EnvValue>,
    ext: &IndexMap<String, EnvValue>,
) -> Result<IndexMap<String, EnvValue>, CargoMakeError> {
    let combined = [base, ext];
    let combined: Vec<_> = env_unique(&combined);

    let mut graph: GraphMap<&str, (), _> = DiGraphMap::new();

    let keys: IndexSet<_> = combined.iter().map(|(key, _)| *key).collect();
    for key in keys {
        graph.add_node(key);
    }

    for (key, val) in &combined {
        // combined is unique (only latest value),
        // which is why we do not need to delete any previously declared outbound edges.

        // if the env variable is in the current scope add an edge,
        // otherwise it is referencing an external variable.
        // also, ignore self reference (such as PATH=${PATH})
        let is_external = envmnt::exists(&key);
        for used in env_depends_on(val).into_iter() {
            if (key != &used || !is_external) && graph.contains_node(used) {
                graph.add_edge(*key, used, ());
            }
        }
    }

    debug!("env dependencies: {:?}", graph);

    let variables = match toposort(&graph, None) {
        Ok(iter) => iter,
        Err(_) => {
            // cycle has been detected, for better performance we now only
            // execute scc.
            // In strongly-connected-components every vertex
            // (node) is reachable from every other node.
            // This means that there **must** be a cycle.
            // This isn't strictly necessary, but aids when debugging.
            for scc in kosaraju_scc(&graph) {
                let render = scc
                    .iter()
                    .chain(scc.first())
                    .map(ToString::to_string)
                    .reduce(|acc, name| format!("{} -> {}", acc, name));

                if let Some(render) = render {
                    return Err(CargoMakeError::EnvVarCycle(format!(" Cycle: {}.", render)));
                }
            }

            return Err(CargoMakeError::EnvVarCycle(String::new()));
        }
    };

    let mut merge = IndexMap::new();
    for name in variables.into_iter().rev() {
        if name.starts_with("CARGO_MAKE_CURRENT_TASK_") {
            // CARGO_MAKE_CURRENT_TASK are handled differently and **always**
            // retain their old value
            if let Some(value) = base.get(name) {
                merge.insert(name.to_owned(), value.clone());
            }

            continue;
        }

        if let Some((key, val)) = combined
            .iter()
            .filter(|(key, _)| *key == name)
            .last()
            .cloned()
        {
            // we need to check if the base and ext both are a profile,
            // in that case we need to do some special handling,
            // by merging them as well.
            match (base.get(key), ext.get(key)) {
                (Some(EnvValue::Profile(base)), Some(EnvValue::Profile(ext))) => {
                    merge.insert(key.to_owned(), EnvValue::Profile(merge_env(base, ext)?));
                }
                _ => {
                    merge.insert(key.to_owned(), val.clone());
                }
            }
        }
    }

    Ok(merge)
}

pub(crate) fn merge_env_files(
    base: &mut Vec<EnvFile>,
    extended: &mut Vec<EnvFile>,
) -> Vec<EnvFile> {
    [&extended[..], &base[..]].concat()
}

pub(crate) fn merge_env_scripts(base: &mut Vec<String>, extended: &mut Vec<String>) -> Vec<String> {
    [&extended[..], &base[..]].concat()
}
