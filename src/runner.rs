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

use command;
use installer;
use log::Logger;
use std::collections::HashSet;
use std::time::SystemTime;
use types::{Config, CrateInfo, ExecutionPlan, Step, Task};

fn run_task(
    logger: &Logger,
    step: &Step,
) {
    logger.info::<()>("Running Task: ", &[&step.name], None);

    installer::install(&logger, &step.config);

    command::run(&logger, &step);
}

fn run_task_flow(
    logger: &Logger,
    execution_plan: &ExecutionPlan,
) {
    for step in &execution_plan.steps {
        run_task(&logger, &step);
    }
}

/// Returns the actual task name to invoke as tasks may have aliases
fn get_task_name(
    logger: &Logger,
    config: &Config,
    name: &str,
) -> String {
    match config.tasks.get(name) {
        Some(task_config) => {
            let alias = task_config.get_alias();

            match alias {
                Some(ref alias) => get_task_name(logger, config, alias),
                _ => name.to_string(),
            }
        }
        None => {
            // This will actually panic
            logger.error::<()>("Task not found: ", &[&name], None);

            name.to_string()
        }
    }
}

/// Creates an execution plan for the given step based on existing execution plan data
fn create_execution_plan_for_step(
    logger: &Logger,
    config: &Config,
    task: &str,
    steps: &mut Vec<Step>,
    task_names: &mut HashSet<String>,
    root: bool,
) {
    let actual_task = get_task_name(logger, config, task);

    match config.tasks.get(&actual_task) {
        Some(task_config) => {
            match task_config.dependencies {
                Some(ref dependencies) => {
                    for dependency in dependencies {
                        create_execution_plan_for_step(&logger, &config, &dependency, steps, task_names, false);
                    }
                }
                _ => logger.verbose::<()>("No dependencies found for task: ", &[&task], None),
            };

            if !task_names.contains(task) {
                let mut clone_task = task_config.clone();
                let normalized_task = clone_task.get_normalized_task();
                let add = !normalized_task.disabled.unwrap_or(false);

                if add {
                    steps.push(Step { name: task.to_string(), config: normalized_task });
                    task_names.insert(task.to_string());
                }
            } else if root {
                logger.error::<()>("Circular reference found for task: ", &[&task], None);
            }
        }
        None => logger.error::<()>("Task not found: ", &[&task], None),
    }
}

fn create_workspace_task(
    crate_info: CrateInfo,
    task: &str,
) -> Task {
    let workspace = crate_info.workspace.unwrap();
    let members = workspace.members.unwrap_or(vec![]);

    let mut script_lines = vec![];
    for member in &members {
        let mut cd_line = "cd ./".to_string();
        cd_line.push_str(&member);
        script_lines.push(cd_line);

        let mut make_line = "cargo make ".to_string();
        make_line.push_str(&task);
        script_lines.push(make_line);

        if cfg!(windows) {
            script_lines.push("cd %CARGO_MAKE_WORKING_DIRECTORY%".to_string());
        } else {
            script_lines.push("cd ${CARGO_MAKE_WORKING_DIRECTORY}".to_string());
        };
    }

    let mut workspace_task = Task::new();
    workspace_task.script = Some(script_lines);

    workspace_task
}

/// Creates the full execution plan
fn create_execution_plan(
    logger: &Logger,
    config: &Config,
    task: &str,
) -> ExecutionPlan {
    let mut task_names = HashSet::new();
    let mut steps = Vec::new();

    match config.config.init_task {
        Some(ref task) => {
            match config.tasks.get(task) {
                Some(task_config) => {
                    let mut clone_task = task_config.clone();
                    let normalized_task = clone_task.get_normalized_task();
                    let add = !normalized_task.disabled.unwrap_or(false);

                    if add {
                        steps.push(Step { name: task.to_string(), config: normalized_task });
                    }
                }
                None => logger.error::<()>("Task not found: ", &[task], None),
            }
        }
        None => logger.verbose::<()>("Init task not defined.", &[], None),
    };

    // load crate info and look for workspace info
    let crate_info = CrateInfo::load(&logger);

    if crate_info.workspace.is_some() {
        let workspace_task = create_workspace_task(crate_info, task);

        steps.push(Step { name: "workspace".to_string(), config: workspace_task });
    } else {
        create_execution_plan_for_step(&logger, &config, &task, &mut steps, &mut task_names, true);
    }

    // always add end task even if already executed due to some depedency
    match config.config.end_task {
        Some(ref task) => {
            match config.tasks.get(task) {
                Some(task_config) => {
                    let mut clone_task = task_config.clone();
                    let normalized_task = clone_task.get_normalized_task();
                    let add = !normalized_task.disabled.unwrap_or(false);

                    if add {
                        steps.push(Step { name: task.to_string(), config: normalized_task });
                    }
                }
                None => logger.error::<()>("Task not found: ", &[task], None),
            }
        }
        None => logger.verbose::<()>("End task not defined.", &[], None),
    };

    ExecutionPlan { steps }
}

/// Runs the requested tasks.<br>
/// The flow is as follows:
///
/// * Create an execution plan based on the requested task and its dependencies
/// * Run all tasks defined in the execution plan
pub fn run(
    logger: &Logger,
    config: &Config,
    task: &str,
) {
    let start_time = SystemTime::now();

    let execution_plan = create_execution_plan(&logger, &config, &task);
    logger.verbose("Created execution plan: ", &[], Some(&execution_plan));

    run_task_flow(logger, &execution_plan);

    let time_string = match start_time.elapsed() {
        Ok(elapsed) => {
            let mut string = " in ".to_string();
            string.push_str(&elapsed.as_secs().to_string());
            string.push_str(" seconds");

            string
        }
        _ => "".to_string(),
    };

    logger.info::<()>("Build Done", &[&time_string, "."], None);
}


/// Only prints the execution plan
pub fn print(
    logger: &Logger,
    config: &Config,
    task: &str,
) {
    let execution_plan = create_execution_plan(&logger, &config, &task);
    logger.verbose("Created execution plan: ", &[], Some(&execution_plan));

    println!("{:#?}", &execution_plan);
}
