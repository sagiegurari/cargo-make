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
use crate::profile;
use crate::scriptengine;
use crate::types::{
    CliArgs, Config, EnvInfo, EnvValue, ExecutionPlan, FlowInfo, RunTaskInfo, RunTaskRoutingInfo,
    Step, Task,
};
use indexmap::IndexMap;
use std::env;
use std::time::SystemTime;

fn validate_condition(flow_info: &FlowInfo, step: &Step) -> bool {
    condition::validate_condition_for_step(&flow_info, &step)
}

fn get_sub_task_name_for_routing_info(
    flow_info: &FlowInfo,
    routing_info: &Vec<RunTaskRoutingInfo>,
) -> Option<String> {
    let mut task_name = None;

    for routing_step in routing_info {
        let invoke = condition::validate_conditions(
            &flow_info,
            &routing_step.condition,
            &routing_step.condition_script,
            None,
        );

        if invoke {
            task_name = Some(routing_step.name.clone());
            break;
        }
    }

    task_name
}

/// runs a sub task and returns true/false based if a sub task was actually invoked
fn run_sub_task_and_report(flow_info: &FlowInfo, sub_task: &RunTaskInfo) -> bool {
    let mut sub_flow_info = flow_info.clone();

    let task_name = match sub_task {
        RunTaskInfo::Name(ref name) => Some(name.to_string()),
        RunTaskInfo::Routing(ref routing_info) => {
            get_sub_task_name_for_routing_info(&flow_info, routing_info)
        }
    };

    if task_name.is_some() {
        sub_flow_info.task = task_name.unwrap();

        run_flow(&sub_flow_info, true);

        true
    } else {
        false
    }
}

fn run_sub_task(flow_info: &FlowInfo, sub_task: &RunTaskInfo) {
    run_sub_task_and_report(&flow_info, &sub_task);
}

fn create_watch_task_name(task: &str) -> String {
    let mut watch_task_name = "".to_string();
    watch_task_name.push_str(&task);
    watch_task_name.push_str("-watch");

    watch_task_name
}

fn create_watch_step(task: &str) -> Step {
    let watch_task = create_watch_task(&task);

    let watch_task_name = create_watch_task_name(&task);

    Step {
        name: watch_task_name,
        config: watch_task,
    }
}

fn watch_task(flow_info: &FlowInfo, task: &str) {
    let step = create_watch_step(&task);

    run_task(&flow_info, &step);
}

fn should_watch(task: &Task) -> bool {
    match task.watch {
        Some(watch_bool) => {
            if watch_bool {
                let disable_watch_env = environment::get_env("CARGO_MAKE_DISABLE_WATCH", "FALSE");
                disable_watch_env != "TRUE"
            } else {
                false
            }
        }
        None => false,
    }
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

        //get profile
        let profile_name = profile::get();

        let env = match step.config.env {
            Some(ref env) => env.clone(),
            None => IndexMap::new(),
        };
        environment::set_env(env);

        //make sure profile env is not overwritten
        profile::set(&profile_name);

        let updated_step = environment::expand_env(&step);

        let watch = should_watch(&step.config);

        if watch {
            watch_task(&flow_info, &step.name);
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

                    // run script
                    let script_runner_done =
                        scriptengine::invoke(&updated_step.config, &cli_arguments);

                    // run command
                    if !script_runner_done {
                        command::run(&updated_step);
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

fn create_watch_task(task: &str) -> Task {
    let mut task_config = create_proxy_task(&task);

    let mut env_map = task_config.env.unwrap_or(IndexMap::new());
    env_map.insert(
        "CARGO_MAKE_DISABLE_WATCH".to_string(),
        EnvValue::Value("TRUE".to_string()),
    );
    task_config.env = Some(env_map);

    let make_args = task_config.args.unwrap();
    let make_command = &make_args.join(" ");

    let watch_args = vec![
        "watch".to_string(),
        "-q".to_string(),
        "-x".to_string(),
        make_command.to_string(),
    ];
    task_config.args = Some(watch_args);

    task_config
}

fn create_proxy_task(task: &str) -> Task {
    //get log level name
    let log_level = logger::get_log_level();

    let mut log_level_arg = "--loglevel=".to_string();
    log_level_arg.push_str(&log_level);

    //get profile
    let profile_name = profile::get();

    let mut profile_arg = "--profile=\"".to_string();
    profile_arg.push_str(&profile_name);
    profile_arg.push_str("\"");

    //setup common args
    let mut args = vec![
        "make".to_string(),
        "--disable-check-for-updates".to_string(),
        "--no-on-error".to_string(),
        log_level_arg.to_string(),
        profile_arg.to_string(),
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

fn run_flow(flow_info: &FlowInfo, sub_flow: bool) {
    let allow_private = sub_flow;

    let execution_plan = create_execution_plan(
        &flow_info.config,
        &flow_info.task,
        flow_info.disable_workspace,
        allow_private,
        sub_flow,
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
