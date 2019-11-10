//! # execution_plan
//!
//! Creates execution plan for a given task and makefile.
//!

#[cfg(test)]
#[path = "./execution_plan_test.rs"]
mod execution_plan_test;

use crate::environment;
use crate::logger;
use crate::profile;
use crate::types::{Config, CrateInfo, EnvValue, ExecutionPlan, ScriptValue, Step, Task};
use envmnt;
use glob::Pattern;
use indexmap::IndexMap;
use std::collections::HashSet;
use std::env;
use std::path;
use std::path::Path;
use std::vec::Vec;

/// Resolve aliases to different tasks, checking for cycles
fn get_task_name_recursive(config: &Config, name: &str, seen: &mut Vec<String>) -> String {
    seen.push(name.to_string());

    match config.tasks.get(name) {
        Some(task_config) => {
            let alias = task_config.get_alias();

            match alias {
                Some(ref alias) if seen.contains(alias) => {
                    let chain = seen.join(" -> ");
                    error!("Detected cycle while resolving alias {}: {}", &name, chain);
                    panic!("Detected cycle while resolving alias {}: {}", &name, chain);
                }
                Some(ref alias) => get_task_name_recursive(config, alias, seen),
                _ => name.to_string(),
            }
        }
        None => {
            error!("Task not found: {}", &name);
            panic!("Task not found: {}", &name);
        }
    }
}

/// Returns the actual task name to invoke as tasks may have aliases
fn get_task_name(config: &Config, name: &str) -> String {
    let mut seen = Vec::new();

    get_task_name_recursive(config, name, &mut seen)
}

pub(crate) fn get_normalized_task(config: &Config, name: &str, support_alias: bool) -> Task {
    let actual_task_name = if support_alias {
        get_task_name(config, name)
    } else {
        name.to_string()
    };

    match config.tasks.get(&actual_task_name) {
        Some(task_config) => {
            let mut clone_task = task_config.clone();
            let normalized_task = clone_task.get_normalized_task();

            match normalized_task.extend {
                Some(ref extended_task_name) => {
                    let mut extended_task =
                        get_normalized_task(config, extended_task_name, support_alias);

                    extended_task.extend(&normalized_task);

                    extended_task
                }
                None => normalized_task,
            }
        }
        None => {
            error!("Task {} not found using name: {}", &name, &actual_task_name);
            panic!("Task {} not found using name: {}", &name, &actual_task_name);
        }
    }
}

fn get_skipped_workspace_members(skip_members_config: String) -> HashSet<String> {
    let mut members = HashSet::new();

    let members_list: Vec<&str> = skip_members_config.split(';').collect();

    for member in members_list.iter() {
        if member.len() > 0 {
            members.insert(member.to_string());
        }
    }

    return members;
}

fn should_skip_workspace_member(member: &str, skipped_members: &HashSet<String>) -> bool {
    if skipped_members.contains(member) {
        true
    } else {
        // search for globs
        let mut skip = false;
        for skipped_member in skipped_members {
            if skipped_member.contains("*") {
                skip = match Pattern::new(skipped_member) {
                    Ok(pattern) => pattern.matches(&member),
                    _ => false,
                };

                if skip {
                    break;
                }
            }
        }

        skip
    }
}

fn update_member_path(member: &str) -> String {
    let os_separator = path::MAIN_SEPARATOR.to_string();

    //convert to OS path separators
    let mut member_path = str::replace(&member, "\\", &os_separator);
    member_path = str::replace(&member_path, "/", &os_separator);

    member_path
}

fn create_workspace_task(crate_info: CrateInfo, task: &str) -> Task {
    let workspace = crate_info.workspace.unwrap();
    let members = workspace.members.unwrap_or(vec![]);

    let log_level = logger::get_log_level();

    let profile_name = if envmnt::is_or("CARGO_MAKE_USE_WORKSPACE_PROFILE", true) {
        profile::get()
    } else {
        profile::DEFAULT_PROFILE.to_string()
    };

    let skip_members_config = envmnt::get_or("CARGO_MAKE_WORKSPACE_SKIP_MEMBERS", "");
    let skip_members = get_skipped_workspace_members(skip_members_config);

    let cargo_make_command = "cargo make";

    let mut script_lines = vec![];
    for member in &members {
        if !should_skip_workspace_member(&member, &skip_members) {
            //convert to OS path separators
            let member_path = update_member_path(&member);

            let mut cd_line = if cfg!(windows) {
                "PUSHD ".to_string()
            } else {
                "cd ./".to_string()
            };
            cd_line.push_str(&member_path);
            script_lines.push(cd_line);

            //get member name
            let member_name = match Path::new(&member_path).file_name() {
                Some(name) => String::from(name.to_string_lossy()),
                None => member_path.clone(),
            };

            debug!("Adding Member: {} Path: {}", &member_name, &member_path);

            let mut make_line = cargo_make_command.to_string();
            make_line
                .push_str(" --disable-check-for-updates --allow-private --no-on-error --loglevel=");
            make_line.push_str(&log_level);
            make_line.push_str(" --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=");
            make_line.push_str(&member_name);
            make_line.push_str(" --profile ");
            make_line.push_str(&profile_name);
            make_line.push_str(" ");
            make_line.push_str(&task);

            if let Some(args) = envmnt::get_list("CARGO_MAKE_TASK_ARGS") {
                for arg in args {
                    make_line.push_str(" ");
                    make_line.push_str(&arg);
                }
            }

            script_lines.push(make_line);

            if cfg!(windows) {
                script_lines.push("if %errorlevel% neq 0 exit /b %errorlevel%".to_string());
                script_lines.push("POPD".to_string());
            } else {
                script_lines.push("cd -".to_string());
            };
        } else {
            debug!("Skipping Member: {}.", &member);
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

    let mut workspace_task = Task::new();
    workspace_task.script = Some(ScriptValue::Text(script_lines));
    workspace_task.env = task_env;

    workspace_task
}

fn is_workspace_flow(
    config: &Config,
    task: &str,
    disable_workspace: bool,
    crate_info: &CrateInfo,
) -> bool {
    // if project is not a workspace or if workspace is disabled via cli, return no workspace flow
    if disable_workspace || crate_info.workspace.is_none() {
        false
    } else {
        // project is a workspace and wasn't disabled via cli, need to check requested task
        let cli_task = get_normalized_task(config, task, true);

        // check for configured default workspace flag
        let default_to_workspace = config.config.default_to_workspace.unwrap_or(true);

        cli_task.workspace.unwrap_or(default_to_workspace)
    }
}

/// Creates an execution plan for the given step based on existing execution plan data
fn create_for_step(
    config: &Config,
    task: &str,
    steps: &mut Vec<Step>,
    task_names: &mut HashSet<String>,
    root: bool,
    allow_private: bool,
) {
    let task_config = get_normalized_task(config, task, true);

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
                        create_for_step(&config, &dependency, steps, task_names, false, true);
                    }
                }
                _ => debug!("No dependencies found for task: {}", &task),
            };

            if !task_names.contains(task) {
                steps.push(Step {
                    name: task.to_string(),
                    config: task_config,
                });
                task_names.insert(task.to_string());
            } else if root {
                error!("Circular reference found for task: {}", &task);
            }
        }
    } else {
        error!("Task {} is private", &task);
        panic!("Task {} is private", &task);
    }
}

/// Creates the full execution plan
pub(crate) fn create(
    config: &Config,
    task: &str,
    disable_workspace: bool,
    allow_private: bool,
    sub_flow: bool,
) -> ExecutionPlan {
    let mut task_names = HashSet::new();
    let mut steps = Vec::new();

    if !sub_flow {
        match config.config.init_task {
            Some(ref task) => {
                let task_config = get_normalized_task(config, task, false);
                let add = !task_config.disabled.unwrap_or(false);

                if add {
                    steps.push(Step {
                        name: task.to_string(),
                        config: task_config,
                    });
                }
            }
            None => debug!("Init task not defined."),
        };
    }

    // load crate info and look for workspace info
    let crate_info = environment::crateinfo::load();

    let workspace_flow = is_workspace_flow(&config, &task, disable_workspace, &crate_info);

    if workspace_flow {
        let workspace_task = create_workspace_task(crate_info, task);

        steps.push(Step {
            name: "workspace".to_string(),
            config: workspace_task,
        });
    } else {
        create_for_step(
            &config,
            &task,
            &mut steps,
            &mut task_names,
            true,
            allow_private,
        );
    }

    if !sub_flow {
        // always add end task even if already executed due to some depedency
        match config.config.end_task {
            Some(ref task) => {
                let task_config = get_normalized_task(config, task, false);
                let add = !task_config.disabled.unwrap_or(false);

                if add {
                    steps.push(Step {
                        name: task.to_string(),
                        config: task_config,
                    });
                }
            }
            None => debug!("End task not defined."),
        };
    }

    ExecutionPlan { steps }
}
