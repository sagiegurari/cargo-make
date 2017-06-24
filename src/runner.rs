//! # runner
//!
//! Runs the requested tasks
//!

use command;
use installer;
use log::Log;
use std::collections::HashSet;
use std::env;
use types::{Config, ExecutionPlan, Step};

fn run_task(
    logger: &Log,
    step: &Step,
) {
    logger.info::<()>("Running Task: ", &[&step.name], None);

    installer::install(&logger, &step.config);

    command::run(&logger, &step);
}

fn run_task_flow(
    logger: &Log,
    execution_plan: &ExecutionPlan,
) {
    for step in &execution_plan.steps {
        run_task(&logger, &step);
    }
}

fn get_task_name(
    logger: &Log,
    config: &Config,
    name: &str,
) -> String {
    match config.tasks.get(name) {
        Some(task_config) => {
            match task_config.alias {
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

fn create_execution_plan_for_step(
    logger: &Log,
    config: &Config,
    task: &str,
    steps: &mut Vec<Step>,
    task_names: &mut HashSet<String>,
    root: bool,
) {
    let actual_task = get_task_name(logger, config, task);

    match config.tasks.get(&actual_task) {
        Some(task_config) => {
            match task_config.depedencies {
                Some(ref depedencies) => {
                    for depedency in depedencies {
                        create_execution_plan_for_step(&logger, &config, &depedency, steps, task_names, false);
                    }
                }
                _ => logger.verbose::<()>("No depedencies found for task: ", &[&task], None),
            };

            if !task_names.contains(task) {
                steps.push(Step { name: task.to_string(), config: task_config.clone() });
                task_names.insert(task.to_string());
            } else if root {
                logger.error::<()>("Circular reference found for task: ", &[&task], None);
            }
        }
        None => logger.error::<()>("Task not found: ", &[&task], None),
    }
}

fn create_execution_plan(
    logger: &Log,
    config: &Config,
    task: &str,
) -> ExecutionPlan {
    let mut task_names = HashSet::new();
    let mut steps = Vec::new();

    create_execution_plan_for_step(&logger, &config, &task, &mut steps, &mut task_names, true);

    ExecutionPlan { steps }
}

fn set_env(
    logger: &Log,
    config: &Config,
) {
    logger.info::<()>("Setting Up Env.", &[], None);

    for (key, value) in &config.env {
        logger.verbose::<()>("Setting env: ", &[&key, "=", &value], None);
        env::set_var(&key, &value);
    }
}

pub fn run(
    logger: &Log,
    config: &Config,
    task: &str,
) {
    set_env(logger, config);

    let execution_plan = create_execution_plan(&logger, &config, &task);
    logger.verbose("Created execution plan: ", &[], Some(&execution_plan));

    run_task_flow(logger, &execution_plan);
}
