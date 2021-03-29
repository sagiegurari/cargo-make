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
#[path = "runner_test.rs"]
mod runner_test;

use crate::command;
use crate::condition;
use crate::environment;
use crate::execution_plan::create as create_execution_plan;
use crate::functions;
use crate::installer;
use crate::logger;
use crate::profile;
use crate::proxy_task::create_proxy_task;
use crate::scriptengine;
use crate::time_summary;
use crate::types::{
    CliArgs, Config, DeprecationInfo, EnvInfo, EnvValue, ExecutionPlan, FlowInfo, FlowState,
    RunTaskInfo, RunTaskName, RunTaskRoutingInfo, Step, Task, TaskWatchOptions,
};
use indexmap::IndexMap;
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
) -> (Option<Vec<String>>, bool, bool, Option<String>) {
    let mut task_name = None;

    let mut fork = false;
    let mut parallel = false;
    let mut cleanup_task = None;
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
            cleanup_task = routing_step.cleanup_task.clone();
            break;
        }
    }

    (task_name, fork, parallel, cleanup_task)
}

fn create_fork_step(flow_info: &FlowInfo) -> Step {
    let fork_task = create_proxy_task(&flow_info.task, true, true, None, None);

    Step {
        name: "cargo_make_run_fork".to_string(),
        config: fork_task,
    }
}

fn run_cleanup_task(flow_info: &FlowInfo, flow_state: &mut FlowState, task: &str) {
    match flow_info.config.tasks.get(task) {
        Some(cleanup_task_info) => run_task(
            &flow_info,
            flow_state,
            &Step {
                name: task.to_string(),
                config: cleanup_task_info.clone(),
            },
        ),
        None => error!("Cleanup task: {} not found.", &task),
    }
}

fn run_forked_task(
    flow_info: &FlowInfo,
    flow_state: &mut FlowState,
    cleanup_task: &Option<String>,
) {
    // run task as a sub process
    let step = create_fork_step(&flow_info);

    match cleanup_task {
        Some(cleanup_task_name) => {
            // run the forked task (forked tasks only run a command + args)
            let exit_code =
                command::run_command(&step.config.command.unwrap(), &step.config.args, false);

            if exit_code != 0 {
                run_cleanup_task(&flow_info, flow_state, &cleanup_task_name);
                command::validate_exit_code(exit_code);
            }
        }
        None => run_task(&flow_info, flow_state, &step),
    }
}

/// runs a sub task and returns true/false based if a sub task was actually invoked
fn run_sub_task_and_report(
    flow_info: &FlowInfo,
    flow_state: &mut FlowState,
    sub_task: &RunTaskInfo,
) -> bool {
    let (task_names, fork, parallel, cleanup_task) = match sub_task {
        RunTaskInfo::Name(ref name) => (Some(vec![name.to_string()]), false, false, None),
        RunTaskInfo::Details(ref details) => {
            let task_name_values = match details.name.clone() {
                RunTaskName::Single(name) => vec![name],
                RunTaskName::Multiple(names) => names,
            };
            (
                Some(task_name_values),
                details.fork.unwrap_or(false),
                details.parallel.unwrap_or(false),
                details.cleanup_task.clone(),
            )
        }
        RunTaskInfo::Routing(ref routing_info) => {
            get_sub_task_info_for_routing_info(&flow_info, routing_info)
        }
    };

    if task_names.is_some() {
        let names = task_names.unwrap();
        let mut threads = vec![];

        // clean up task only supported for forked tasks
        if !fork && cleanup_task.is_some() {
            error!("Invalid task, cannot use cleanup_task without fork.");
        }

        for name in names {
            let task_run_fn = move |flow_info: &FlowInfo,
                                    flow_state: &mut FlowState,
                                    fork: bool,
                                    cleanup_task: &Option<String>| {
                let mut sub_flow_info = flow_info.clone();
                sub_flow_info.task = name;

                if fork {
                    run_forked_task(&sub_flow_info, flow_state, cleanup_task);
                } else {
                    run_flow(&sub_flow_info, flow_state, true);
                }
            };

            if parallel {
                let run_flow_info = flow_info.clone();
                // we do not support merging changes back to parent
                let mut cloned_flow_state = flow_state.clone();
                let cloned_cleanup_task = cleanup_task.clone();
                threads.push(thread::spawn(move || {
                    task_run_fn(
                        &run_flow_info,
                        &mut cloned_flow_state,
                        fork,
                        &cloned_cleanup_task,
                    );
                }));
            } else {
                task_run_fn(&flow_info, flow_state, fork, &cleanup_task);
            }
        }

        if threads.len() > 0 {
            for task_thread in threads {
                task_thread.join().unwrap();
            }
        }

        if let Some(cleanup_task_name) = cleanup_task {
            run_cleanup_task(&flow_info, flow_state, &cleanup_task_name);
        }

        true
    } else {
        false
    }
}

fn run_sub_task(flow_info: &FlowInfo, flow_state: &mut FlowState, sub_task: &RunTaskInfo) {
    run_sub_task_and_report(&flow_info, flow_state, &sub_task);
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

fn watch_task(
    flow_info: &FlowInfo,
    flow_state: &mut FlowState,
    task: &str,
    options: Option<TaskWatchOptions>,
) {
    let step = create_watch_step(&task, options);

    run_task(&flow_info, flow_state, &step);
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

pub(crate) fn run_task(flow_info: &FlowInfo, flow_state: &mut FlowState, step: &Step) {
    let start_time = SystemTime::now();

    if step.config.is_actionable() {
        match step.config.env {
            Some(ref env) => environment::set_current_task_meta_info_env(env.clone()),
            None => (),
        };

        if validate_condition(&flow_info, &step) {
            if logger::should_reduce_output(&flow_info) && step.config.script.is_none() {
                debug!("Running Task: {}", &step.name);
            } else {
                info!("Running Task: {}", &step.name);
            }

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
                watch_task(
                    &flow_info,
                    flow_state,
                    &step.name,
                    step.config.watch.clone(),
                );
            } else {
                do_in_task_working_directory(&step, || {
                    installer::install(&updated_step.config, flow_info);
                });

                match step.config.run_task {
                    Some(ref sub_task) => {
                        time_summary::add(&mut flow_state.time_summary, &step.name, start_time);

                        run_sub_task(&flow_info, flow_state, sub_task);
                    }
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

                        time_summary::add(&mut flow_state.time_summary, &step.name, start_time);
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

            if logger::should_reduce_output(&flow_info) && step.config.script.is_none() {
                debug!("Skipping Task: {} {}", &step.name, &fail_message);
            } else {
                info!("Skipping Task: {} {}", &step.name, &fail_message);
            }
        }
    } else {
        debug!("Ignoring Empty Task: {}", &step.name);
    }
}

fn run_task_flow(flow_info: &FlowInfo, flow_state: &mut FlowState, execution_plan: &ExecutionPlan) {
    for step in &execution_plan.steps {
        run_task(&flow_info, flow_state, &step);
    }
}

fn create_watch_task(task: &str, options: Option<TaskWatchOptions>) -> Task {
    let mut task_config = create_proxy_task(&task, true, true, None, None);

    let mut env_map = task_config.env.unwrap_or(IndexMap::new());
    env_map.insert(
        "CARGO_MAKE_DISABLE_WATCH".to_string(),
        EnvValue::Value("true".to_string()),
    );
    task_config.env = Some(env_map);

    let make_args = task_config.args.unwrap();
    let mut make_command = String::new();
    for make_arg in make_args {
        if make_arg.contains(" ") {
            make_command.push_str("\"");
            make_command.push_str(&make_arg);
            make_command.push_str("\"");
        } else {
            make_command.push_str(&make_arg);
        }

        make_command.push(' ');
    }
    make_command = make_command.trim().to_string();

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

pub(crate) fn run_flow(flow_info: &FlowInfo, flow_state: &mut FlowState, sub_flow: bool) {
    let allow_private = sub_flow || flow_info.allow_private;

    let execution_plan = create_execution_plan(
        &flow_info.config,
        &flow_info.task,
        flow_info.disable_workspace,
        allow_private,
        sub_flow,
    );
    debug!("Created execution plan: {:#?}", &execution_plan);

    run_task_flow(&flow_info, flow_state, &execution_plan);
}

fn run_protected_flow(flow_info: &FlowInfo, flow_state: &mut FlowState) {
    let proxy_task = create_proxy_task(
        &flow_info.task,
        flow_info.allow_private,
        flow_info.skip_init_end_tasks,
        None,
        flow_info.cli_arguments.clone(),
    );

    let exit_code = command::run_command(&proxy_task.command.unwrap(), &proxy_task.args, false);

    if exit_code != 0 {
        match flow_info.config.config.on_error_task {
            Some(ref on_error_task) => {
                let mut error_flow_info = flow_info.clone();
                error_flow_info.disable_on_error = true;
                error_flow_info.task = on_error_task.clone();

                run_flow(&error_flow_info, flow_state, false);
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

    time_summary::init(&config, &cli_args);

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
    let mut flow_state = FlowState::new();

    if flow_info.disable_on_error || flow_info.config.config.on_error_task.is_none() {
        run_flow(&flow_info, &mut flow_state, false);
    } else {
        run_protected_flow(&flow_info, &mut flow_state);
    }

    let time_string = match start_time.elapsed() {
        Ok(elapsed) => {
            let time_millies = elapsed.as_millis() as f64 / 1000.0;
            format!(" in {:.2} seconds", time_millies)
        }
        _ => "".to_string(),
    };

    time_summary::print(&flow_state.time_summary);

    info!("Build Done{}.", &time_string);
}
