//! # runner
//!
//! Runs the requested tasks.<br>
//! The flow is as follows:
//!
//! * Load env variables
//! * Create an execution plan based on the requested task and its dependencies
//! * Run all tasks defined in the execution plan
//!

use command;
use installer;
use log::Logger;
use std::collections::HashSet;
use std::env;
use std::time::SystemTime;
use types::{Config, ExecutionPlan, Step};

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
            let alias = if cfg!(windows) {
                match task_config.windows_alias {
                    Some(ref value) => Some(value),
                    _ => None,
                }
            } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
                match task_config.mac_alias {
                    Some(ref value) => Some(value),
                    _ => None,
                }
            } else {
                match task_config.linux_alias {
                    Some(ref value) => Some(value),
                    _ => None,
                }
            };

            match alias {
                Some(ref os_alias) => get_task_name(logger, config, os_alias),
                _ => {
                    match task_config.alias {
                        Some(ref alias) => get_task_name(logger, config, alias),
                        _ => name.to_string(),
                    }
                }
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
                    for depedency in dependencies {
                        create_execution_plan_for_step(&logger, &config, &depedency, steps, task_names, false);
                    }
                }
                _ => logger.verbose::<()>("No dependencies found for task: ", &[&task], None),
            };

            if !task_names.contains(task) {
                let add = match task_config.disabled {
                    Some(ref disabled) => !disabled,
                    None => true,
                };

                if add {
                    steps.push(Step { name: task.to_string(), config: task_config.clone() });
                    task_names.insert(task.to_string());
                }
            } else if root {
                logger.error::<()>("Circular reference found for task: ", &[&task], None);
            }
        }
        None => logger.error::<()>("Task not found: ", &[&task], None),
    }
}

/// Creates the full execution plan
fn create_execution_plan(
    logger: &Logger,
    config: &Config,
    task: &str,
) -> ExecutionPlan {
    let mut task_names = HashSet::new();
    let mut steps = Vec::new();

    create_execution_plan_for_step(&logger, &config, &task, &mut steps, &mut task_names, true);

    ExecutionPlan { steps }
}

/// Updates the env for the current execution
fn set_env(
    logger: &Logger,
    config: &Config,
) {
    logger.info::<()>("Setting Up Env.", &[], None);

    for (key, value) in &config.env {
        logger.verbose::<()>("Setting env: ", &[&key, "=", &value], None);
        env::set_var(&key, &value);
    }
}

/// Runs the requested tasks.<br>
/// The flow is as follows:
///
/// * Load env variables
/// * Create an execution plan based on the requested task and its dependencies
/// * Run all tasks defined in the execution plan
pub fn run(
    logger: &Logger,
    config: &Config,
    task: &str,
) {
    let start_time = SystemTime::now();

    set_env(logger, config);

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

    logger.info::<()>("Build done", &[&time_string, "."], None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use log;
    use std::collections::HashMap;
    use types::Task;

    #[test]
    fn set_env_empty() {
        let logger = log::create("error");
        let config = Config { env: HashMap::new(), tasks: HashMap::new() };

        set_env(&logger, &config);
    }

    #[test]
    fn set_env_values() {
        let logger = log::create("error");
        let mut config = Config { env: HashMap::new(), tasks: HashMap::new() };
        config.env.insert("MY_ENV_KEY".to_string(), "MY_ENV_VALUE".to_string());

        assert_eq!(env::var("MY_ENV_KEY").unwrap_or("NONE".to_string()), "NONE".to_string());

        set_env(&logger, &config);

        assert_eq!(env::var("MY_ENV_KEY").unwrap(), "MY_ENV_VALUE");
    }

    #[test]
    #[should_panic]
    fn get_task_name_not_found() {
        let logger = log::create("error");
        let config = Config { env: HashMap::new(), tasks: HashMap::new() };

        get_task_name(&logger, &config, "test");
    }

    #[test]
    fn get_task_name_no_alias() {
        let logger = log::create("error");
        let mut config = Config { env: HashMap::new(), tasks: HashMap::new() };

        config.tasks.insert(
            "test".to_string(),
            Task {
                alias: None,
                linux_alias: None,
                windows_alias: None,
                mac_alias: None,
                disabled: None,
                install_crate: None,
                install_script: None,
                command: None,
                args: None,
                script: None,
                dependencies: None
            }
        );

        let name = get_task_name(&logger, &config, "test");

        assert_eq!(name, "test");
    }

    #[test]
    fn get_task_name_alias() {
        let logger = log::create("error");
        let mut config = Config { env: HashMap::new(), tasks: HashMap::new() };

        config.tasks.insert(
            "test".to_string(),
            Task {
                alias: Some("test2".to_string()),
                linux_alias: None,
                windows_alias: None,
                mac_alias: None,
                disabled: None,
                install_crate: None,
                install_script: None,
                command: None,
                args: None,
                script: None,
                dependencies: None
            }
        );

        config.tasks.insert(
            "test2".to_string(),
            Task {
                alias: None,
                linux_alias: None,
                windows_alias: None,
                mac_alias: None,
                disabled: None,
                install_crate: None,
                install_script: None,
                command: None,
                args: None,
                script: None,
                dependencies: None
            }
        );

        let name = get_task_name(&logger, &config, "test");

        assert_eq!(name, "test2");
    }

    #[test]
    fn get_task_name_platform_alias() {
        let logger = log::create("error");
        let mut config = Config { env: HashMap::new(), tasks: HashMap::new() };

        let mut task = Task {
            alias: None,
            linux_alias: None,
            windows_alias: None,
            mac_alias: None,
            disabled: None,
            install_crate: None,
            install_script: None,
            command: None,
            args: None,
            script: None,
            dependencies: None
        };
        if cfg!(windows) {
            task.windows_alias = Some("test2".to_string());
        } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
            task.mac_alias = Some("test2".to_string());
        } else {
            task.linux_alias = Some("test2".to_string());
        };

        config.tasks.insert("test".to_string(), task);

        config.tasks.insert(
            "test2".to_string(),
            Task {
                alias: None,
                linux_alias: None,
                windows_alias: None,
                mac_alias: None,
                disabled: None,
                install_crate: None,
                install_script: None,
                command: None,
                args: None,
                script: None,
                dependencies: None
            }
        );

        let name = get_task_name(&logger, &config, "test");

        assert_eq!(name, "test2");
    }
}
