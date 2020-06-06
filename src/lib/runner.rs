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
use crate::functions;
use crate::installer;
use crate::logger;
use crate::profile;
use crate::scriptengine;
use crate::types::{
    CliArgs, Config, DeprecationInfo, EnvInfo, EnvValue, ExecutionPlan, FlowInfo, RunTaskInfo,
    RunTaskName, RunTaskRoutingInfo, Step, Task, TaskWatchOptions,
};
use indexmap::IndexMap;
use std::env;
use std::thread;
use std::time::SystemTime;

fn do_in_task_working_directory<F>(step: &Step, mut action: F)
where
    F: FnMut(),
{
    let revert_directory = match step.config.cwd {
        Some(ref cwd) => {
            let expanded_cwd = environment::expand_value(cwd);

            if expanded_cwd.len() > 0 {
                let directory = envmnt::get_or("CARGO_MAKE_WORKING_DIRECTORY", "");

                environment::setup_cwd(Some(&expanded_cwd));

                directory
            } else {
                "".to_string()
            }
        }
        None => "".to_string(),
    };

    action();

    // revert to original cwd
    match step.config.cwd {
        Some(_) => {
            environment::setup_cwd(Some(&revert_directory));
        }
        _ => (),
    };
}

fn validate_condition(flow_info: &FlowInfo, step: &Step) -> bool {
    let mut valid = true;

    let do_validate = || {
        valid = condition::validate_condition_for_step(&flow_info, &step);
    };

    do_in_task_working_directory(&step, do_validate);

    valid
}

pub(crate) fn get_sub_task_info_for_routing_info(
    flow_info: &FlowInfo,
    routing_info: &Vec<RunTaskRoutingInfo>,
) -> (Option<Vec<String>>, bool, bool) {
    let mut task_name = None;

    let mut fork = false;
    let mut parallel = false;
    for routing_step in routing_info {
        let invoke = condition::validate_conditions(
            &flow_info,
            &routing_step.condition,
            &routing_step.condition_script,
            None,
        );

        if invoke {
            let task_name_values = match routing_step.name.clone() {
                RunTaskName::Single(name) => vec![name],
                RunTaskName::Multiple(names) => names,
            };
            task_name = Some(task_name_values);
            fork = routing_step.fork.unwrap_or(false);
            parallel = routing_step.parallel.unwrap_or(false);
            break;
        }
    }

    (task_name, fork, parallel)
}

fn create_fork_step(flow_info: &FlowInfo) -> Step {
    let fork_task = create_proxy_task(&flow_info.task, true, true);

    Step {
        name: "cargo_make_run_fork".to_string(),
        config: fork_task,
    }
}

fn run_forked_task(flow_info: &FlowInfo) {
    // run task as a sub process
    let step = create_fork_step(&flow_info);
    run_task(&flow_info, &step);
}

/// runs a sub task and returns true/false based if a sub task was actually invoked
fn run_sub_task_and_report(flow_info: &FlowInfo, sub_task: &RunTaskInfo) -> bool {
    let (task_names, fork, parallel) = match sub_task {
        RunTaskInfo::Name(ref name) => (Some(vec![name.to_string()]), false, false),
        RunTaskInfo::Details(ref details) => {
            let task_name_values = match details.name.clone() {
                RunTaskName::Single(name) => vec![name],
                RunTaskName::Multiple(names) => names,
            };
            (
                Some(task_name_values),
                details.fork.unwrap_or(false),
                details.parallel.unwrap_or(false),
            )
        }
        RunTaskInfo::Routing(ref routing_info) => {
            get_sub_task_info_for_routing_info(&flow_info, routing_info)
        }
    };

    if task_names.is_some() {
        let names = task_names.unwrap();
        let mut threads = vec![];

        for name in names {
            let task_run_fn = |flow_info: &FlowInfo, fork: bool| {
                let mut sub_flow_info = flow_info.clone();
                sub_flow_info.task = name;

                if fork {
                    run_forked_task(&sub_flow_info);
                } else {
                    run_flow(&sub_flow_info, true);
                }
            };

            if parallel {
                let run_flow_info = flow_info.clone();
                threads.push(thread::spawn(move || {
                    task_run_fn(&run_flow_info, fork);
                }));
            } else {
                task_run_fn(&flow_info, fork);
            }
        }

        if threads.len() > 0 {
            for task_thread in threads {
                task_thread.join().unwrap();
            }
        }

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

fn create_watch_step(task: &str, options: Option<TaskWatchOptions>) -> Step {
    let watch_task = create_watch_task(&task, options);

    let watch_task_name = create_watch_task_name(&task);

    Step {
        name: watch_task_name,
        config: watch_task,
    }
}

fn watch_task(flow_info: &FlowInfo, task: &str, options: Option<TaskWatchOptions>) {
    let step = create_watch_step(&task, options);

    run_task(&flow_info, &step);
}

fn is_watch_enabled() -> bool {
    !envmnt::is_or("CARGO_MAKE_DISABLE_WATCH", false)
}

fn should_watch(task: &Task) -> bool {
    match task.watch {
        Some(ref watch_value) => match watch_value {
            TaskWatchOptions::Boolean(watch_bool) => {
                if *watch_bool {
                    is_watch_enabled()
                } else {
                    false
                }
            }
            TaskWatchOptions::Options(_) => is_watch_enabled(),
        },
        None => false,
    }
}

pub(crate) fn run_task(flow_info: &FlowInfo, step: &Step) {
    if step.config.is_actionable() {
        match step.config.env {
            Some(ref env) => environment::set_current_task_meta_info_env(env.clone()),
            None => (),
        };

        if validate_condition(&flow_info, &step) {
            info!("Running Task: {}", &step.name);

            if !step.config.is_valid() {
                error!(
                    "Invalid task: {}, contains multiple actions.\n{:#?}",
                    &step.name, &step.config
                );
            }

            let deprecated_info = step.config.deprecated.clone();
            match deprecated_info {
                Some(deprecated) => match deprecated {
                    DeprecationInfo::Boolean(value) => {
                        if value {
                            warn!("Task: {} is deprecated.", &step.name);
                        }

                        ()
                    }
                    DeprecationInfo::Message(ref message) => {
                        warn!("Task: {} is deprecated - {}", &step.name, message);

                        ()
                    }
                },
                None => (),
            };

            //get profile
            let profile_name = profile::get();

            match step.config.env_files {
                Some(ref env_files) => environment::set_env_files(env_files.clone()),
                None => (),
            };
            match step.config.env {
                Some(ref env) => environment::set_env(env.clone()),
                None => (),
            };

            envmnt::set("CARGO_MAKE_CURRENT_TASK_NAME", &step.name);

            //make sure profile env is not overwritten
            profile::set(&profile_name);

            // modify step using env and functions
            let mut updated_step = functions::run(&step);
            updated_step = environment::expand_env(&updated_step);

            let watch = should_watch(&step.config);

            if watch {
                watch_task(&flow_info, &step.name, step.config.watch.clone());
            } else {
                do_in_task_working_directory(&step, || {
                    installer::install(&updated_step.config, flow_info);
                });

                match step.config.run_task {
                    Some(ref sub_task) => run_sub_task(&flow_info, sub_task),
                    None => {
                        do_in_task_working_directory(&step, || {
                            // run script
                            let script_runner_done =
                                scriptengine::invoke(&updated_step.config, flow_info);

                            // run command
                            if !script_runner_done {
                                command::run(&updated_step);
                            };
                        });
                    }
                };
            }
        } else {
            let fail_message = match step.config.condition {
                Some(ref condition) => match condition.fail_message {
                    Some(ref value) => value.to_string(),
                    None => "".to_string(),
                },
                None => "".to_string(),
            };
            info!("Skipping Task: {} {}", &step.name, &fail_message);
        }
    } else {
        debug!("Ignoring Empty Task: {}", &step.name);
    }
}

fn run_task_flow(flow_info: &FlowInfo, execution_plan: &ExecutionPlan) {
    for step in &execution_plan.steps {
        run_task(&flow_info, &step);
    }
}

fn create_watch_task(task: &str, options: Option<TaskWatchOptions>) -> Task {
    let mut task_config = create_proxy_task(&task, true, true);

    let mut env_map = task_config.env.unwrap_or(IndexMap::new());
    env_map.insert(
        "CARGO_MAKE_DISABLE_WATCH".to_string(),
        EnvValue::Value("true".to_string()),
    );
    task_config.env = Some(env_map);

    let make_args = task_config.args.unwrap();
    let make_command = &make_args.join(" ");

    let mut watch_args = vec!["watch".to_string(), "-q".to_string()];

    match options {
        Some(task_watch_options) => match task_watch_options {
            TaskWatchOptions::Options(watch_options) => {
                let watch_version = match watch_options.version {
                    Some(value) => value.to_string(),
                    _ => "7.4.1".to_string(), // current version
                };
                task_config.install_crate_args = Some(vec!["--version".to_string(), watch_version]);

                match watch_options.postpone {
                    Some(value) => {
                        if value {
                            watch_args.push("--postpone".to_string());
                        }
                    }
                    _ => (),
                };

                match watch_options.ignore_pattern {
                    Some(value) => watch_args.extend_from_slice(&["-i".to_string(), value]),
                    _ => (),
                };

                match watch_options.no_git_ignore {
                    Some(value) => {
                        if value {
                            watch_args.push("--no-gitignore".to_string());
                        }
                    }
                    _ => (),
                };

                match watch_options.watch {
                    Some(paths) => {
                        for watch_path in paths {
                            watch_args.extend_from_slice(&["-w".to_string(), watch_path])
                        }
                    }
                    _ => (),
                };
            }
            _ => (),
        },
        _ => (),
    }

    watch_args.extend_from_slice(&["-x".to_string(), make_command.to_string()]);

    task_config.args = Some(watch_args);

    task_config
}

fn create_proxy_task(task: &str, allow_private: bool, skip_init_end_tasks: bool) -> Task {
    //get log level name
    let log_level = logger::get_log_level();

    let mut log_level_arg = "--loglevel=".to_string();
    log_level_arg.push_str(&log_level);

    //get profile
    let profile_name = profile::get();

    let mut profile_arg = "--profile=".to_string();
    profile_arg.push_str(&profile_name);

    //setup common args
    let mut args = vec![
        "make".to_string(),
        "--disable-check-for-updates".to_string(),
        "--no-on-error".to_string(),
        log_level_arg.to_string(),
        profile_arg.to_string(),
    ];

    if allow_private {
        args.push("--allow-private".to_string());
    }

    if skip_init_end_tasks {
        args.push("--skip-init-end-tasks".to_string());
    }

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
    let allow_private = sub_flow || flow_info.allow_private;

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
    let proxy_task = create_proxy_task(
        &flow_info.task,
        flow_info.allow_private,
        flow_info.skip_init_end_tasks,
    );

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
        allow_private: cli_args.allow_private,
        skip_init_end_tasks: cli_args.skip_init_end_tasks,
        cli_arguments: cli_args.arguments.clone(),
    };

    if flow_info.disable_on_error || flow_info.config.config.on_error_task.is_none() {
        run_flow(&flow_info, false);
    } else {
        run_protected_flow(&flow_info);
    }

    let time_string = match start_time.elapsed() {
        Ok(elapsed) => {
            let mut string = "in ".to_string();
            string.push_str(&elapsed.as_secs().to_string());
            string.push_str(" seconds");

            string
        }
        _ => "".to_string(),
    };

    info!("Build Done {}.", &time_string);
}
