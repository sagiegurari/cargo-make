//! # runner
//!
//! Runs the requested tasks.<br>
//! The flow is as follows:
//!
//! * Load env variables
//! * Create an execution plan based on the requested task and its dependencies
//! * Run all tasks defined in the execution plan
//!

#[cfg(test)]
#[path = "./runner_test.rs"]
mod runner_test;

use crate::command;
use crate::condition;
use crate::environment;
use crate::installer;
use crate::logger;
use crate::scriptengine;
use crate::types::{
    CliArgs, Config, CrateInfo, EnvInfo, EnvValue, ExecutionPlan, FlowInfo, Step, Task,
};
use indexmap::IndexMap;
use std::collections::HashSet;
use std::env;
use std::path;
use std::time::SystemTime;

fn validate_condition(flow_info: &FlowInfo, step: &Step) -> bool {
    condition::validate_condition(&flow_info, &step)
}

fn run_sub_task(flow_info: &FlowInfo, sub_task: &str) {
    let mut sub_flow_info = flow_info.clone();
    sub_flow_info.task = sub_task.to_string();

    run_flow(&sub_flow_info, true);
}

fn run_task(flow_info: &FlowInfo, step: &Step) {
    info!("Running Task: {}", &step.name);

    if validate_condition(&flow_info, &step) {
        if !step.config.is_valid() {
            error!(
                "Invalid task, contains multiple actions.\n{:#?}",
                &step.config
            );
        }

        let env = match step.config.env {
            Some(ref env) => env.clone(),
            None => IndexMap::new(),
        };
        environment::set_env(env);

        let updated_step = environment::expand_env(&step);

        installer::install(&updated_step.config);

        match step.config.run_task {
            Some(ref sub_task) => run_sub_task(&flow_info, sub_task),
            None => {
                let revert_directory = match step.config.cwd {
                    Some(ref cwd) => {
                        if cwd.len() > 0 {
                            let directory =
                                environment::get_env("CARGO_MAKE_WORKING_DIRECTORY", "");

                            environment::setup_cwd(Some(cwd));

                            directory
                        } else {
                            "".to_string()
                        }
                    }
                    None => "".to_string(),
                };

                // get cli arguments
                let cli_arguments = match flow_info.cli_arguments {
                    Some(ref args) => args.clone(),
                    None => vec![],
                };

                // try to invoke it as a none OS script
                let script_runner_done = scriptengine::invoke(&updated_step.config, &cli_arguments);

                // run as command or OS script
                if !script_runner_done {
                    command::run(&updated_step, &cli_arguments);
                };

                // revert to original cwd
                match step.config.cwd {
                    Some(_) => environment::setup_cwd(Some(&revert_directory)),
                    _ => (),
                };
            }
        };
    } else {
        debug!("Task: {} disabled", &step.name);
    }
}

fn run_task_flow(flow_info: &FlowInfo, execution_plan: &ExecutionPlan) {
    for step in &execution_plan.steps {
        run_task(&flow_info, &step);
    }
}

/// Returns the actual task name to invoke as tasks may have aliases
fn get_task_name(config: &Config, name: &str) -> String {
    match config.tasks.get(name) {
        Some(task_config) => {
            let alias = task_config.get_alias();

            match alias {
                Some(ref alias) => get_task_name(config, alias),
                _ => name.to_string(),
            }
        }
        None => {
            // This will actually panic
            error!("Task not found: {}", &name);

            name.to_string()
        }
    }
}

/// Creates an execution plan for the given step based on existing execution plan data
fn create_execution_plan_for_step(
    config: &Config,
    task: &str,
    steps: &mut Vec<Step>,
    task_names: &mut HashSet<String>,
    root: bool,
    allow_private: bool,
) {
    let actual_task = get_task_name(config, task);

    match config.tasks.get(&actual_task) {
        Some(task_config) => {
            let is_private = match task_config.private {
                Some(value) => value,
                None => false,
            };

            if allow_private || !is_private {
                let mut clone_task = task_config.clone();
                let normalized_task = clone_task.get_normalized_task();
                let add = !normalized_task.disabled.unwrap_or(false);

                if add {
                    match task_config.dependencies {
                        Some(ref dependencies) => for dependency in dependencies {
                            create_execution_plan_for_step(
                                &config,
                                &dependency,
                                steps,
                                task_names,
                                false,
                                true,
                            );
                        },
                        _ => debug!("No dependencies found for task: {}", &task),
                    };

                    if !task_names.contains(task) {
                        steps.push(Step {
                            name: task.to_string(),
                            config: normalized_task,
                        });
                        task_names.insert(task.to_string());
                    } else if root {
                        error!("Circular reference found for task: {}", &task);
                    }
                }
            } else {
                error!("Task not found: {}", &task);
            }
        }
        None => error!("Task not found: {}", &task),
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

    let skip_members_config = environment::get_env("CARGO_MAKE_WORKSPACE_SKIP_MEMBERS", "");
    let skip_members = get_skipped_workspace_members(skip_members_config);

    let cargo_make_command = environment::get_env("CARGO_MAKE_COMMAND", "cargo make");

    let mut script_lines = vec![];
    for member in &members {
        if !skip_members.contains(member) {
            info!("Adding Member: {}.", &member);

            //convert to OS path separators
            let member_path = update_member_path(&member);

            let mut cd_line = if cfg!(windows) {
                "PUSHD ".to_string()
            } else {
                "cd ./".to_string()
            };
            cd_line.push_str(&member_path);
            script_lines.push(cd_line);

            let mut make_line = cargo_make_command.to_string();
            make_line.push_str(" --disable-check-for-updates --no-on-error --loglevel=");
            make_line.push_str(&log_level);
            make_line.push_str(" ");
            make_line.push_str(&task);
            script_lines.push(make_line);

            if cfg!(windows) {
                script_lines.push("if %errorlevel% neq 0 exit /b %errorlevel%".to_string());
                script_lines.push("POPD".to_string());
            } else {
                script_lines.push("cd -".to_string());
            };
        } else {
            info!("Skipping Member: {}.", &member);
        }
    }

    //only if environment variable is set
    let task_env = if environment::get_env_as_bool("CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE", false) {
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
    workspace_task.script = Some(script_lines);
    workspace_task.env = task_env;

    workspace_task
}

fn create_proxy_task(task: &str) -> Task {
    //get log level name
    let log_level = logger::get_log_level();

    let mut log_level_arg = "--loglevel=".to_string();
    log_level_arg.push_str(&log_level);

    //setup common args
    let mut args = vec![
        "make".to_string(),
        "--disable-check-for-updates".to_string(),
        "--no-on-error".to_string(),
        log_level_arg.to_string(),
    ];

    //get makefile location
    match env::var("CARGO_MAKE_MAKEFILE_PATH") {
        Ok(makefile_path) => {
            if makefile_path.len() > 0 {
                let mut makefile_arg = "--makefile=".to_string();
                makefile_arg.push_str(&makefile_path);

                args.push(makefile_arg.to_string());
            }
        }
        _ => {}
    };

    args.push(task.to_string());

    let mut proxy_task = Task::new();
    proxy_task.command = Some("cargo".to_string());
    proxy_task.args = Some(args);

    proxy_task.get_normalized_task()
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
        let cli_task = match config.tasks.get(task) {
            Some(task_config) => {
                let mut clone_task = task_config.clone();
                clone_task.get_normalized_task()
            }
            None => {
                error!("Task not found: {}", &task);
                panic!("Task not found: {}", &task);
            }
        };

        cli_task.workspace.unwrap_or(true)
    }
}

/// Creates the full execution plan
fn create_execution_plan(
    config: &Config,
    task: &str,
    disable_workspace: bool,
    allow_private: bool,
) -> ExecutionPlan {
    let mut task_names = HashSet::new();
    let mut steps = Vec::new();

    match config.config.init_task {
        Some(ref task) => match config.tasks.get(task) {
            Some(task_config) => {
                let mut clone_task = task_config.clone();
                let normalized_task = clone_task.get_normalized_task();
                let add = !normalized_task.disabled.unwrap_or(false);

                if add {
                    steps.push(Step {
                        name: task.to_string(),
                        config: normalized_task,
                    });
                }
            }
            None => error!("Task not found: {}", &task),
        },
        None => debug!("Init task not defined."),
    };

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
        create_execution_plan_for_step(
            &config,
            &task,
            &mut steps,
            &mut task_names,
            true,
            allow_private,
        );
    }

    // always add end task even if already executed due to some depedency
    match config.config.end_task {
        Some(ref task) => match config.tasks.get(task) {
            Some(task_config) => {
                let mut clone_task = task_config.clone();
                let normalized_task = clone_task.get_normalized_task();
                let add = !normalized_task.disabled.unwrap_or(false);

                if add {
                    steps.push(Step {
                        name: task.to_string(),
                        config: normalized_task,
                    });
                }
            }
            None => error!("Task not found: {}", &task),
        },
        None => debug!("End task not defined."),
    };

    ExecutionPlan { steps }
}

fn run_flow(flow_info: &FlowInfo, allow_private: bool) {
    let execution_plan = create_execution_plan(
        &flow_info.config,
        &flow_info.task,
        flow_info.disable_workspace,
        allow_private,
    );
    debug!("Created execution plan: {:#?}", &execution_plan);

    run_task_flow(&flow_info, &execution_plan);
}

fn run_protected_flow(flow_info: &FlowInfo) {
    let proxy_task = create_proxy_task(&flow_info.task);

    let exit_code = command::run_command(&proxy_task.command.unwrap(), &proxy_task.args, false);

    if exit_code != 0 {
        match flow_info.config.config.on_error_task {
            Some(ref on_error_task) => {
                let mut error_flow_info = flow_info.clone();
                error_flow_info.disable_on_error = true;
                error_flow_info.task = on_error_task.clone();

                run_flow(&error_flow_info, false);
            }
            _ => (),
        };

        error!("Task error detected, exit code: {}", &exit_code);
    }
}

/// Runs the requested tasks.<br>
/// The flow is as follows:
///
/// * Create an execution plan based on the requested task and its dependencies
/// * Run all tasks defined in the execution plan
pub(crate) fn run(config: Config, task: &str, env_info: EnvInfo, cli_args: &CliArgs) {
    let start_time = SystemTime::now();

    let flow_info = FlowInfo {
        config,
        task: task.to_string(),
        env_info,
        disable_workspace: cli_args.disable_workspace,
        disable_on_error: cli_args.disable_on_error,
        cli_arguments: cli_args.arguments.clone(),
    };

    if flow_info.disable_on_error || flow_info.config.config.on_error_task.is_none() {
        run_flow(&flow_info, false);
    } else {
        run_protected_flow(&flow_info);
    }

    let time_string = match start_time.elapsed() {
        Ok(elapsed) => {
            let mut string = " in ".to_string();
            string.push_str(&elapsed.as_secs().to_string());
            string.push_str(" seconds");

            string
        }
        _ => "".to_string(),
    };

    info!("Build Done {}.", &time_string);
}

/// Only prints the execution plan
pub(crate) fn print(config: &Config, task: &str, disable_workspace: bool) {
    let execution_plan = create_execution_plan(&config, &task, disable_workspace, false);
    debug!("Created execution plan: {:#?}", &execution_plan);

    println!("{:#?}", &execution_plan);
}
