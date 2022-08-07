use crate::types::{EnvFile, EnvValue, EnvValueConditioned, EnvValueDecode, EnvValuePathGlob};
use indexmap::IndexMap;
use once_cell::sync::Lazy;
use petgraph::algo::{kosaraju_scc, toposort};
use petgraph::graphmap::{DiGraphMap, GraphMap};
use regex::Regex;
use std::collections::HashSet;

#[cfg(test)]
#[path = "env_test.rs"]
mod env_test;

static RE_VARIABLE: Lazy<Regex> = Lazy::new(|| Regex::new(r"\$\{.*?}").unwrap());

pub(crate) fn merge_env_depends_on_extract(val: &str) -> Vec<&str> {
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

fn merge_env_unique<'a>(
    vals: &'a [&'a IndexMap<String, EnvValue>],
) -> Vec<(&'a str, &'a EnvValue)> {
    let mut visited = HashSet::new();
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

fn merge_env_depends_on(val: &EnvValue) -> Vec<&str> {
    match val {
        EnvValue::Value(value) => merge_env_depends_on_extract(value),
        EnvValue::Decode(EnvValueDecode { source, .. }) => merge_env_depends_on_extract(source),
        EnvValue::List(values) => values
            .iter()
            .map(|value| merge_env_depends_on_extract(value))
            .reduce(|mut acc, mut other| {
                acc.append(&mut other);
                acc
            })
            .unwrap_or_default(),
        EnvValue::Conditional(EnvValueConditioned { value, .. }) => {
            merge_env_depends_on_extract(value)
        }
        EnvValue::PathGlob(EnvValuePathGlob { glob, .. }) => merge_env_depends_on_extract(glob),
        _ => vec![],
    }
}

pub(crate) fn merge_env(
    base: &IndexMap<String, EnvValue>,
    ext: &IndexMap<String, EnvValue>,
) -> Result<IndexMap<String, EnvValue>, String> {
    let combined = [base, ext];
    let combined: Vec<_> = merge_env_unique(&combined);

    let mut graph: GraphMap<&str, (), _> = DiGraphMap::new();

    let keys: HashSet<_> = combined.iter().map(|(key, _)| *key).collect();
    for key in keys {
        graph.add_node(key);
    }

    debug!("initial graph: {:?}", graph);

    for (key, val) in &combined {
        // combined is unique (only latest value),
        // which is why we do not need to delete any previously declared outbound edges.

        // if the env variable is in the current scope add add an edge,
        // otherwise it is referencing and external variable.
        for used in merge_env_depends_on(val).into_iter() {
            if graph.contains_node(used) {
                graph.add_edge(*key, used, ());
            }
        }
    }

    let variables = match toposort(&graph, None) {
        Ok(iter) => iter,
        Err(_) => {
            // cycle has been detected, for better performance we now only
            // execute scc.
            // In strongly-connected-components every vertex
            // (node) is reachable from every other node.
            // This means that there **must** be a cycle.
            // This isn't strictly necessary, but aids when debugging.
            let mut err = "A cycle between different env variables has been detected.".to_owned();
            for scc in kosaraju_scc(&graph) {
                let render = scc
                    .iter()
                    .chain(scc.first())
                    .map(ToString::to_string)
                    .reduce(|acc, name| format!("{acc} -> {name}"));

                if let Some(render) = render {
                    err.push_str(&format!(" Cycle: {}.", render));
                }
            }

            return Err(err);
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

    println!("{:#?}", merge);
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
