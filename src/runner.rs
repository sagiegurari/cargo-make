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
use crate::execution_plan::create as create_execution_plan;
use crate::installer;
use crate::logger;
use crate::scriptengine;
use crate::types::{CliArgs, Config, EnvInfo, ExecutionPlan, FlowInfo, Step, Task};
use indexmap::IndexMap;
use std::env;
use std::time::SystemTime;

fn validate_condition(flow_info: &FlowInfo, step: &Step) -> bool {
    condition::validate_condition(&flow_info, &step)
}

fn run_sub_task(flow_info: &FlowInfo, sub_task: &str) {
    let mut sub_flow_info = flow_info.clone();
    sub_flow_info.task = sub_task.to_string();

    run_flow(&sub_flow_info, true);
}

fn watch_task() {
    //TODO impl
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

        let watch = match step.config.watch {
            Some(watch_bool) => watch_bool,
            None => false,
        };

        if watch {
            watch_task();
        } else {
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
                    let script_runner_done =
                        scriptengine::invoke(&updated_step.config, &cli_arguments);

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
        }
    } else {
        debug!("Task: {} disabled", &step.name);
    }
}

fn run_task_flow(flow_info: &FlowInfo, execution_plan: &ExecutionPlan) {
    for step in &execution_plan.steps {
        run_task(&flow_info, &step);
    }
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
