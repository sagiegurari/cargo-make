//! # execution_plan
//!
//! Creates execution plan for a given task and makefile.
//!

#[cfg(test)]
#[path = "execution_plan_test.rs"]
mod execution_plan_test;

use crate::environment;
use crate::error::CargoMakeError;
use crate::logger;
use crate::profile;
use crate::proxy_task::create_proxy_task;
use crate::types::{
    Config, CrateInfo, EnvValue, ExecutionPlan, ScriptValue, Step, Task, TaskIdentifier, Workspace,
};
use fsio::path::{get_basename, get_parent_directory};
use glob::Pattern;
use indexmap::IndexMap;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashSet;
use std::env;
use std::path::Path;

/// Resolve aliases recursively until a task without alias is found.
fn get_task_name_recursive(
    config: &Config,
    name: &str,
    seen: &mut Vec<String>,
) -> Result<String, CargoMakeError> {
    seen.push(name.to_string());

    match config.tasks.get(name) {
        Some(task_config) => {
            let alias = task_config.get_alias();

            match alias {
                Some(ref alias) if seen.contains(alias) => {
                    let chain = seen.join(" -> ");
                    error!("Detected cycle while resolving alias {}: {}", &name, chain);
                    return Err(CargoMakeError::AliasCycle(name.to_string(), chain));
                }
                Some(ref alias) => get_task_name_recursive(config, alias, seen),
                None => Ok(name.to_string()),
            }
        }
        None => Err(CargoMakeError::NotFound(format!(
            "Task {:#?} not found",
            name
        ))),
    }
}

/// Returns the actual task name to invoke as tasks may have aliases.
///
/// # Panics
/// if there is a cycle in the alias chain.
pub(crate) fn get_actual_task_name(config: &Config, name: &str) -> Result<String, CargoMakeError> {
    let mut seen = Vec::new();

    get_task_name_recursive(config, name, &mut seen)
}

/// Resolves alias and normalizes task.
///
/// # Panics
/// if task is not found or there is a cycle in the alias chain.
pub(crate) fn get_normalized_task(
    config: &Config,
    name: &str,
    support_alias: bool,
) -> Result<Task, CargoMakeError> {
    get_optional_normalized_task(config, name, support_alias)
}

/// Resolves alias and normalizes task.
///
/// # Panics
/// if there is a cycle in the alias chain.
fn get_optional_normalized_task(
    config: &Config,
    name: &str,
    support_alias: bool,
) -> Result<Task, CargoMakeError> {
    let actual_task_name = if support_alias {
        get_actual_task_name(config, name)
    } else {
        Ok(name.to_string())
    }?;

    match config.tasks.get(&actual_task_name) {
        Some(task_config) => {
            let mut clone_task = task_config.clone();
            let mut normalized_task = clone_task.get_normalized_task();

            normalized_task = match normalized_task.extend {
                Some(ref extended_task_name) => {
                    let mut extended_task =
                        get_normalized_task(config, extended_task_name, support_alias)?;

                    if let Some(ref env) = normalized_task.env {
                        if env.len() == 2
                            && env.contains_key("CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE")
                            && env
                                .contains_key("CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE_DIRECTORY")
                        {
                            normalized_task.env = None;
                        }
                    }
                    extended_task.extend(&normalized_task);

                    extended_task
                }
                None => normalized_task,
            };

            Ok(normalized_task)
        }
        None => Err(CargoMakeError::NotFound(format!(
            "Task {:#?} not found",
            name
        ))),
    }
}

fn get_workspace_members_config(members_config: String) -> HashSet<String> {
    let mut members = HashSet::new();

    let members_list: Vec<&str> = members_config.split(';').collect();

    for member in members_list.iter() {
        if member.len() > 0 {
            members.insert(member.to_string());
        }
    }

    members
}

fn is_workspace_member_found(member: &str, members_map: &HashSet<String>) -> bool {
    if members_map.contains(member) {
        true
    } else {
        // search for globs
        let mut found = false;

        for member_iter in members_map {
            if member_iter.contains("*") {
                found = match Pattern::new(member_iter) {
                    Ok(pattern) => pattern.matches(&member),
                    _ => false,
                };

                if found {
                    break;
                }
            }
        }

        found
    }
}

fn should_skip_workspace_member(member: &str, skipped_members: &HashSet<String>) -> bool {
    is_workspace_member_found(member, skipped_members)
}

fn should_include_workspace_member(member: &str, include_members: &HashSet<String>) -> bool {
    if include_members.is_empty() {
        true
    } else {
        is_workspace_member_found(member, include_members)
    }
}

fn filter_workspace_members(members: &Vec<String>) -> Vec<String> {
    let skip_members_config = envmnt::get_or("CARGO_MAKE_WORKSPACE_SKIP_MEMBERS", "");
    let skip_members = get_workspace_members_config(skip_members_config);

    let include_members_config = envmnt::get_or("CARGO_MAKE_WORKSPACE_INCLUDE_MEMBERS", "");
    let include_members = get_workspace_members_config(include_members_config);

    let mut filtered_members = vec![];
    for member in members {
        if !should_skip_workspace_member(&member, &skip_members)
            && should_include_workspace_member(&member, &include_members)
        {
            filtered_members.push(member.to_string());
        } else {
            debug!("Skipping Member: {}.", &member);
        }
    }

    filtered_members
}

fn create_workspace_task(crate_info: &CrateInfo, task: &str) -> Task {
    let set_workspace_emulation = crate_info.workspace.is_none()
        && envmnt::is("CARGO_MAKE_WORKSPACE_EMULATION")
        && !envmnt::exists("CARGO_MAKE_WORKSPACE_EMULATION_ROOT_DIRECTORY");
    if set_workspace_emulation {
        environment::search_and_set_workspace_cwd();
        let root_directory = envmnt::get_or_panic("CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY");
        envmnt::set(
            "CARGO_MAKE_WORKSPACE_EMULATION_ROOT_DIRECTORY",
            &root_directory,
        );
    }

    let members = if crate_info.workspace.is_some() {
        let workspace_clone = crate_info.workspace.clone();
        let workspace = workspace_clone.unwrap_or(Workspace::new());
        workspace.members.unwrap_or(vec![])
    } else {
        envmnt::get_list("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS").unwrap_or(vec![])
    };

    let log_level = logger::get_log_level();

    let profile_name = if envmnt::is_or("CARGO_MAKE_USE_WORKSPACE_PROFILE", true) {
        profile::get()
    } else {
        profile::default_profile()
    };

    let filtered_members = filter_workspace_members(&members);
    let mut script_lines = vec![];

    if !filtered_members.is_empty() {
        let cargo_make_command = "cargo make";

        script_lines.push("workspace_directory = pwd".to_string());
        for member in &filtered_members {
            let mut cd_line = "cd ./".to_string();
            cd_line.push_str(&member.replace("\\", "/"));
            script_lines.push(cd_line);

            //get member name
            let member_name = match Path::new(&member).file_name() {
                Some(name) => String::from(name.to_string_lossy()),
                None => member.clone(),
            };

            debug!("Adding Member: {} Path: {}", &member_name, &member);

            let mut make_line = "exec --fail-on-error ".to_string();
            make_line.push_str(&cargo_make_command);
            make_line
                .push_str(" --disable-check-for-updates --allow-private --no-on-error --loglevel=");
            make_line.push_str(&log_level);
            make_line.push_str(" --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=");
            make_line.push_str(&member_name);
            make_line.push_str(" --profile ");
            make_line.push_str(&profile_name);
            make_line.push_str(" -- ");
            make_line.push_str(&task);

            if let Some(args) = envmnt::get_list("CARGO_MAKE_TASK_ARGS") {
                for arg in args {
                    make_line.push_str(" ");
                    make_line.push_str(&arg);
                }
            }

            script_lines.push(make_line);

            script_lines.push("cd ${workspace_directory}".to_string());
        }
    }

    //only if environment variable is set
    let task_env = if envmnt::is_or("CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE", false) {
        match env::var("CARGO_MAKE_MAKEFILE_PATH") {
            Ok(makefile) => {
                let mut env_map = IndexMap::new();
                env_map.insert(
                    "CARGO_MAKE_WORKSPACE_MAKEFILE".to_string(),
                    EnvValue::Value(makefile.to_string()),
                );

                Some(env_map)
            }
            _ => None,
        }
    } else {
        None
    };

    debug!("Workspace Task Script: {:#?}", &script_lines);

    let mut workspace_task = Task::new();
    workspace_task.script_runner = Some("@duckscript".to_string());
    workspace_task.script = Some(ScriptValue::Text(script_lines));
    workspace_task.env = task_env;

    workspace_task
}

fn is_workspace_flow(
    config: &Config,
    task: &str,
    disable_workspace: bool,
    crate_info: &CrateInfo,
    sub_flow: bool,
) -> bool {
    // determine if workspace flow is explicitly set and enabled in the requested task
    let (task_set_workspace, task_enable_workspace) =
        match get_optional_normalized_task(config, task, true) {
            Ok(normalized_task) => match normalized_task.workspace {
                Some(enable_workspace) => (true, enable_workspace),
                None => (false, false),
            },
            Err(_) => (false, false),
        };

    // if project is not a workspace or if workspace is disabled via cli, return no workspace flow
    if disable_workspace
        || (sub_flow && !task_enable_workspace)
        || (crate_info.workspace.is_none() && !envmnt::is("CARGO_MAKE_WORKSPACE_EMULATION"))
        || envmnt::exists("CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER")
    {
        false
    } else {
        // project is a workspace and wasn't disabled via cli, need to check requested task

        // use requested task's workspace flag if set
        if task_set_workspace {
            task_enable_workspace
        } else {
            // use configured default workspace flag if set
            config.config.default_to_workspace.unwrap_or(true)
        }
    }
}

/// Creates an execution plan for the given step based on existing execution plan data
fn create_for_step(
    config: &Config,
    task: &TaskIdentifier,
    steps: &mut Vec<Step>,
    task_names: &mut HashSet<String>,
    root: bool,
    allow_private: bool,
    skip_tasks_pattern: Option<&Regex>,
) -> Result<(), CargoMakeError> {
    if let Some(skip_tasks_pattern_regex) = skip_tasks_pattern {
        if skip_tasks_pattern_regex.is_match(&task.name) {
            debug!("Skipping task: {} due to skip pattern.", &task.name);
            return Ok(());
        }
    }

    if let Some(path) = &task.path {
        // this is referring to a task in another file
        // so we create a proxy task to invoke it
        let proxy_name = format!("{}_proxy", task.name);

        let path_obj = Path::new(path);
        let (working_directory, makefile) = if path_obj.is_file() {
            let filename = get_basename(&path_obj);
            let working_directory = get_parent_directory(&path_obj);
            (working_directory, filename)
        } else {
            (Some(path.to_string()), Some("Makefile.toml".to_string()))
        };

        let mut proxy_task = create_proxy_task(&task.name, true, false, makefile, None);
        proxy_task.cwd = working_directory;

        let step = Step {
            name: proxy_name,
            config: proxy_task,
        };

        debug!("Created external dependency step: {:#?}", &step);

        steps.push(step);
        task_names.insert(task.to_string());
        return Ok(());
    }

    let task_config = get_normalized_task(config, &task.name, true)?;

    debug!("Normalized Task: {} config: {:#?}", &task, &task_config);

    let is_private = match task_config.private {
        Some(value) => value,
        None => false,
    };

    if allow_private || !is_private {
        let add = !task_config.disabled.unwrap_or(false);

        if add {
            match task_config.dependencies {
                Some(ref dependencies) => {
                    for dependency in dependencies {
                        create_for_step(
                            &config,
                            &dependency.to_owned().into(),
                            steps,
                            task_names,
                            false,
                            true,
                            skip_tasks_pattern,
                        )?;
                    }
                }
                _ => debug!("No dependencies found for task: {}", &task),
            };

            if !task_names.contains(&task.name) {
                steps.push(Step {
                    name: task.to_string(),
                    config: task_config,
                });
                task_names.insert(task.to_string());
            } else if root {
                return Err(CargoMakeError::CircularReference(format!("{}", task)));
            }
        }
        Ok(())
    } else {
        error!("Task {} is private", &task);
        return Err(CargoMakeError::TaskIs(format!("{}", task), "private"));
    }
}

fn add_predefined_step(
    config: &Config,
    task: &str,
    steps: &mut Vec<Step>,
) -> Result<(), CargoMakeError> {
    let task_config = get_normalized_task(config, task, false)?;
    let add = !task_config.disabled.unwrap_or(false);

    if add {
        steps.push(Step {
            name: task.to_string(),
            config: task_config,
        });
    }
    Ok(())
}

#[derive(Clone, Debug)]
pub(crate) struct ExecutionPlanBuilder<'a> {
    pub config: &'a Config,
    pub task: &'a str,
    pub crate_info: Option<&'a CrateInfo>,
    pub disable_workspace: bool,
    pub allow_private: bool,
    pub sub_flow: bool,
    pub skip_tasks_pattern: Option<&'a Regex>,
    pub skip_init_end_tasks: bool,
}

impl<'a> ExecutionPlanBuilder<'a> {
    pub fn new(config: &'a Config, task: &'a str) -> Self {
        Self {
            config,
            task,
            crate_info: None,
            disable_workspace: false,
            allow_private: false,
            sub_flow: false,
            skip_tasks_pattern: None,
            skip_init_end_tasks: false,
        }
    }

    pub fn build(&self) -> Result<ExecutionPlan, CargoMakeError> {
        let Self {
            config,
            task,
            crate_info,
            disable_workspace,
            allow_private,
            sub_flow,
            skip_tasks_pattern,
            skip_init_end_tasks,
        } = *self;
        let mut task_names = HashSet::<String>::new();
        let mut steps = Vec::<Step>::new();
        let default_crate_info = CrateInfo::new();
        let crate_info = crate_info.unwrap_or(&default_crate_info);
        let skip_init_end_tasks = skip_init_end_tasks || sub_flow;

        if !skip_init_end_tasks {
            match config.config.legacy_migration_task {
                Some(ref task) => add_predefined_step(config, task, &mut steps)?,
                None => {
                    /* return Err(CargoMakeError::NotFound(String::from(
                        "Legacy migration task not defined.",
                    ))) */
                }
            };
            match config.config.init_task {
                Some(ref task) => add_predefined_step(config, task, &mut steps)?,
                None => {
                    /* return Err(CargoMakeError::NotFound(String::from(
                        "Init task not defined.",
                    ))) */
                }
            };
        }

        let skip = match skip_tasks_pattern {
            Some(pattern) => pattern.is_match(task),
            None => false,
        };

        if !skip {
            let workspace_flow =
                is_workspace_flow(&config, &task, disable_workspace, &crate_info, sub_flow);

            if workspace_flow {
                let workspace_task = create_workspace_task(crate_info, task);

                steps.push(Step {
                    name: "workspace".to_string(),
                    config: workspace_task,
                });
            } else {
                create_for_step(
                    &config,
                    &TaskIdentifier::from_name(task),
                    &mut steps,
                    &mut task_names,
                    true,
                    allow_private,
                    skip_tasks_pattern,
                )?;
            }
        } else {
            debug!("Skipping task: {} due to skip pattern.", &task);
        }

        if !skip_init_end_tasks {
            // always add end task even if already executed due to some dependency
            match config.config.end_task {
                Some(ref task) => add_predefined_step(config, task, &mut steps)?,
                None => {
                    /* return Err(CargoMakeError::NotFound(String::from(
                        "Ent task not defined.",
                    ))) */
                }
            };
        }
        let mut before_each_task: Option<String> = None;
        let mut after_each_task: Option<String> = None;
        for step in steps.iter() {
            match step.name.as_str() {
                "before_each" => before_each_task = Some(step.name.clone()),
                "after_each" => after_each_task = Some(step.name.clone()),
                _ => {}
            }
        }

        Ok(ExecutionPlan {
            steps: Self::handle_before_each_and_after_each_tasks(
                &config,
                before_each_task,
                after_each_task,
                &steps,
            )?
            .into_owned(),
        })
    }

    /// `before_each` and `after_each` interspersal for the steps vector
    fn handle_before_each_and_after_each_tasks<'b>(
        config: &Config,
        before_each_task: Option<String>,
        after_each_task: Option<String>,
        steps: &'b Vec<Step>,
    ) -> Result<Cow<'b, Vec<Step>>, CargoMakeError> {
        let mut before_and_after_each = Vec::<Step>::with_capacity(2);
        let scale = {
            let mut handle_some = |name: &String| -> Result<u8, CargoMakeError> {
                let task_config = get_normalized_task(config, name, false)?;
                if !task_config.disabled.unwrap_or(false) {
                    before_and_after_each.push(Step {
                        name: name.to_string(),
                        config: task_config,
                    });
                    Ok(1)
                } else {
                    Ok(0)
                }
            };
            let _has_after_each: u8 = match after_each_task {
                None => 0,
                Some(after_each) => handle_some(&after_each)?,
            };
            let _has_before_each: u8 = match before_each_task {
                None => 0,
                Some(before_each) => handle_some(&before_each)?,
            };
            _has_before_each + _has_after_each
        };
        if scale > 0 {
            let before_and_after_each_len = scale as usize + 1;
            let mut interspersed_steps =
                Vec::<Step>::with_capacity(steps.len() * before_and_after_each_len);
            interspersed_steps.extend(steps.into_iter().flat_map(|e| -> Vec<Step> {
                let mut _steps = Vec::<Step>::with_capacity(before_and_after_each_len + 1);
                _steps.clone_from_slice(&before_and_after_each);
                _steps.push(e.to_owned());
                _steps
            }));
            Ok(Cow::Owned(interspersed_steps))
        } else {
            Ok(Cow::Borrowed(&steps))
        }
    }
}
