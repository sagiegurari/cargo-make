//! # descriptor
//!
//! Loads the tasks descriptor.<br>
//! It will first load the default descriptor which is defined in cargo-make internally and
//! afterwards tries to find the external descriptor and load it as well.<br>
//! If an extenal descriptor exists, it will be loaded and extend the default descriptor.
//!

use log::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use types::{Config, ExternalConfig, Task};

extern crate toml;

fn merge_maps(
    base: &mut HashMap<String, String>,
    extended: &mut HashMap<String, String>,
) -> HashMap<String, String> {
    let mut merged = HashMap::<String, String>::new();

    for (key, value) in base.iter() {
        let key_str = key.to_string();
        let value_str = value.to_string();
        merged.insert(key_str, value_str);
    }

    for (key, value) in extended.iter() {
        let key_str = key.to_string();
        let value_str = value.to_string();
        merged.insert(key_str, value_str);
    }

    merged
}

fn merge_tasks(
    base: &mut HashMap<String, Task>,
    extended: &mut HashMap<String, Task>,
) -> HashMap<String, Task> {
    let mut merged = HashMap::<String, Task>::new();

    for (key, value) in base.iter() {
        let key_str = key.to_string();
        merged.insert(key_str, value.clone());
    }

    for (key, value) in extended.iter() {
        let key_str = key.to_string();
        let mut task = value.clone();

        task = match base.get(key) {
            Some(ref value) => {
                let mut merged_task = Task {
                    disabled: None,
                    alias: None,
                    install_crate: None,
                    install_script: None,
                    command: None,
                    args: None,
                    script: None,
                    dependencies: None
                };

                merged_task.extend(value);
                merged_task.extend(&task);

                merged_task
            }
            _ => task,
        };

        merged.insert(key_str, task);
    }

    merged
}

fn load_external_descriptor(
    file_name: &str,
    logger: &Logger,
) -> ExternalConfig {
    logger.verbose::<()>("Loading tasks from file: ", &[file_name], None);

    let file_path = Path::new(file_name);

    if file_path.exists() {
        let mut file = File::open(file_name).unwrap();
        let mut external_descriptor = String::new();
        file.read_to_string(&mut external_descriptor).unwrap();

        let file_config: ExternalConfig = match toml::from_str(&external_descriptor) {
            Ok(value) => value,
            Err(error) => panic!("Unable to parse external descriptor, {}", error),
        };
        logger.verbose("Loaded external config:", &[], Some(&file_config));

        file_config
    } else {
        logger.info::<()>("External file not found, skipping.", &[], None);

        ExternalConfig { env: None, tasks: None }
    }
}

/// Loads the tasks descriptor.<br>
/// It will first load the default descriptor which is defined in cargo-make internally and
/// afterwards tries to find the external descriptor and load it as well.<br>
/// If an extenal descriptor exists, it will be loaded and extend the default descriptor.
pub fn load(
    file_name: &str,
    logger: &Logger,
) -> Config {
    logger.verbose::<()>("Loading default tasks.", &[], None);

    let default_descriptor = include_str!("default.toml");

    let default_config: Config = match toml::from_str(default_descriptor) {
        Ok(value) => value,
        Err(error) => panic!("Unable to parse default descriptor, {}", error),
    };
    logger.verbose("Loaded default config:", &[], Some(&default_config));

    let external_config: ExternalConfig = load_external_descriptor(file_name, logger);

    let mut external_tasks = match external_config.tasks {
        Some(tasks) => tasks,
        None => HashMap::new(),
    };
    let mut default_tasks = default_config.tasks;

    let mut external_env = match external_config.env {
        Some(env) => env,
        None => HashMap::new(),
    };
    let mut default_env = default_config.env;

    // merge configs
    let all_env = merge_maps(&mut default_env, &mut external_env);
    let all_tasks = merge_tasks(&mut default_tasks, &mut external_tasks);

    let config = Config { env: all_env, tasks: all_tasks };

    logger.verbose("Loaded merged config:", &[], Some(&config));

    config
}

#[cfg(test)]
mod tests {
    use super::*;
    use log;

    #[test]
    fn merge_maps_both_empty() {
        let mut map1 = HashMap::<String, String>::new();
        let mut map2 = HashMap::<String, String>::new();

        let output = merge_maps(&mut map1, &mut map2);
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn merge_maps_first_empty() {
        let mut map1 = HashMap::<String, String>::new();
        let mut map2 = HashMap::<String, String>::new();

        map2.insert("test".to_string(), "value".to_string());

        let output = merge_maps(&mut map1, &mut map2);
        assert_eq!(output.len(), 1);
        assert_eq!(output.get("test").unwrap(), &"value".to_string());
    }

    #[test]
    fn merge_maps_second_empty() {
        let mut map1 = HashMap::<String, String>::new();
        let mut map2 = HashMap::<String, String>::new();

        map1.insert("test".to_string(), "value".to_string());

        let output = merge_maps(&mut map1, &mut map2);
        assert_eq!(output.len(), 1);
        assert_eq!(output.get("test").unwrap(), &"value".to_string());
    }

    #[test]
    fn merge_maps_both_with_values() {
        let mut map1 = HashMap::<String, String>::new();
        let mut map2 = HashMap::<String, String>::new();

        map1.insert("test1".to_string(), "value1".to_string());
        map2.insert("test21".to_string(), "value21".to_string());
        map2.insert("test22".to_string(), "value22".to_string());

        let output = merge_maps(&mut map1, &mut map2);
        assert_eq!(output.len(), 3);
        assert_eq!(output.get("test1").unwrap(), &"value1".to_string());
        assert_eq!(output.get("test21").unwrap(), &"value21".to_string());
        assert_eq!(output.get("test22").unwrap(), &"value22".to_string());
    }

    #[test]
    fn merge_tasks_both_empty() {
        let mut map1 = HashMap::<String, Task>::new();
        let mut map2 = HashMap::<String, Task>::new();

        let output = merge_tasks(&mut map1, &mut map2);
        assert_eq!(output.len(), 0);
    }

    #[test]
    fn merge_tasks_first_empty() {
        let mut map1 = HashMap::<String, Task>::new();
        let mut map2 = HashMap::<String, Task>::new();

        map2.insert(
            "test".to_string(),
            Task {
                disabled: None,
                alias: None,
                install_crate: Some("my crate".to_string()),
                install_script: None,
                command: Some("test".to_string()),
                args: None,
                script: None,
                dependencies: None
            }
        );

        let output = merge_tasks(&mut map1, &mut map2);
        assert_eq!(output.len(), 1);
        let task = output.get("test").unwrap();
        assert!(task.disabled.is_none());
        assert!(task.alias.is_none());
        assert!(task.install_crate.is_some());
        assert!(task.install_script.is_none());
        assert!(task.command.is_some());
        assert!(task.args.is_none());
        assert!(task.script.is_none());
        assert!(task.dependencies.is_none());
    }

    #[test]
    fn merge_tasks_second_empty() {
        let mut map1 = HashMap::<String, Task>::new();
        let mut map2 = HashMap::<String, Task>::new();

        map1.insert(
            "test".to_string(),
            Task {
                disabled: None,
                alias: None,
                install_crate: Some("my crate".to_string()),
                install_script: None,
                command: Some("test".to_string()),
                args: None,
                script: None,
                dependencies: None
            }
        );

        let output = merge_tasks(&mut map1, &mut map2);
        assert_eq!(output.len(), 1);
        let task = output.get("test").unwrap();
        assert!(task.disabled.is_none());
        assert!(task.alias.is_none());
        assert!(task.install_crate.is_some());
        assert!(task.install_script.is_none());
        assert!(task.command.is_some());
        assert!(task.args.is_none());
        assert!(task.script.is_none());
        assert!(task.dependencies.is_none());
    }

    #[test]
    fn merge_tasks_both_with_values() {
        let mut map1 = HashMap::<String, Task>::new();
        let mut map2 = HashMap::<String, Task>::new();

        map1.insert(
            "test".to_string(),
            Task {
                disabled: None,
                alias: None,
                install_crate: Some("my crate".to_string()),
                install_script: None,
                command: Some("test".to_string()),
                args: None,
                script: None,
                dependencies: None
            }
        );

        map2.insert(
            "test2".to_string(),
            Task {
                disabled: None,
                alias: None,
                install_crate: None,
                install_script: None,
                command: Some("test".to_string()),
                args: None,
                script: None,
                dependencies: None
            }
        );

        let output = merge_tasks(&mut map1, &mut map2);
        assert_eq!(output.len(), 2);

        let mut task = output.get("test").unwrap();
        assert!(task.disabled.is_none());
        assert!(task.alias.is_none());
        assert!(task.install_crate.is_some());
        assert!(task.install_script.is_none());
        assert!(task.command.is_some());
        assert!(task.args.is_none());
        assert!(task.script.is_none());
        assert!(task.dependencies.is_none());

        task = output.get("test2").unwrap();
        assert!(task.disabled.is_none());
        assert!(task.alias.is_none());
        assert!(task.install_crate.is_none());
        assert!(task.install_script.is_none());
        assert!(task.command.is_some());
        assert!(task.args.is_none());
        assert!(task.script.is_none());
        assert!(task.dependencies.is_none());
    }

    #[test]
    fn merge_tasks_extend_task() {
        let mut map1 = HashMap::<String, Task>::new();
        let mut map2 = HashMap::<String, Task>::new();

        map1.insert(
            "test".to_string(),
            Task {
                disabled: None,
                alias: None,
                install_crate: Some("my crate".to_string()),
                install_script: None,
                command: Some("test1".to_string()),
                args: None,
                script: None,
                dependencies: None
            }
        );

        map2.insert(
            "test".to_string(),
            Task {
                disabled: Some(true),
                alias: None,
                install_crate: None,
                install_script: None,
                command: Some("test2".to_string()),
                args: None,
                script: None,
                dependencies: None
            }
        );

        let output = merge_tasks(&mut map1, &mut map2);
        assert_eq!(output.len(), 1);

        let task = output.get("test").unwrap();
        assert!(task.disabled.is_some());
        assert!(task.alias.is_none());
        assert!(task.install_crate.is_some());
        assert!(task.install_script.is_none());
        assert!(task.command.is_some());
        assert!(task.args.is_none());
        assert!(task.script.is_none());
        assert!(task.dependencies.is_none());

        let task_clone = task.clone();
        assert!(task_clone.disabled.unwrap());
        assert_eq!(task_clone.install_crate.unwrap(), "my crate");
        assert_eq!(task_clone.command.unwrap(), "test2");
    }

    #[test]
    fn load_external_descriptor_no_file() {
        let logger = log::create("error");
        let config = load_external_descriptor("bad_file.toml2", &logger);

        assert!(config.env.is_none());
        assert!(config.tasks.is_none());
    }
}
