//! # descriptor
//!
//! Loads the tasks descriptor.<br>
//! It will first load the default descriptor which is defined in cargo-make internally and
//! afterwards tries to find the external descriptor and load it as well.<br>
//! If an extenal descriptor exists, it will be loaded and extend the default descriptor.
//!

#[cfg(test)]
#[path = "./descriptor_test.rs"]
mod descriptor_test;

use log::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;
use types::{Config, ConfigSection, ExternalConfig, Task};

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
                let mut merged_task = Task::new();

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
    base_path: &str,
    file_name: &str,
    logger: &Logger,
) -> ExternalConfig {
    logger.verbose::<()>("Loading tasks from file: ", &[file_name, " base directory: ", &base_path], None);

    let file_path = Path::new(base_path).join(file_name);

    if file_path.exists() {
        logger.verbose("Opening file:", &[], Some(&file_path));
        let mut file = match File::open(&file_path) {
            Ok(value) => value,
            Err(error) => panic!("Unable to open file, base path: {} file name: {} error: {}", base_path, file_name, error),
        };
        let mut external_descriptor = String::new();
        file.read_to_string(&mut external_descriptor).unwrap();

        let file_config: ExternalConfig = match toml::from_str(&external_descriptor) {
            Ok(value) => value,
            Err(error) => panic!("Unable to parse external descriptor, {}", error),
        };
        logger.verbose("Loaded external config:", &[], Some(&file_config));

        match file_config.extend {
            Some(ref base_file) => {
                let parent_path_buf = Path::new(base_path).join(file_name).join("..");
                let parent_path = file_path.parent().unwrap_or(&parent_path_buf).to_str().unwrap_or(".");
                logger.verbose::<()>("External config parent path:", &[&parent_path], None);
                let base_file_config = load_external_descriptor(parent_path, base_file, logger);

                // merge env
                let mut base_env = match base_file_config.env {
                    Some(env) => env,
                    None => HashMap::new(),
                };
                let mut extended_env = match file_config.env {
                    Some(env) => env,
                    None => HashMap::new(),
                };
                let all_env = merge_maps(&mut base_env, &mut extended_env);

                // merge tasks
                let mut base_tasks = match base_file_config.tasks {
                    Some(tasks) => tasks,
                    None => HashMap::new(),
                };
                let mut extended_tasks = match file_config.tasks {
                    Some(tasks) => tasks,
                    None => HashMap::new(),
                };
                let all_tasks = merge_tasks(&mut base_tasks, &mut extended_tasks);

                let mut config_section = ConfigSection::new();
                if base_file_config.config.is_some() {
                    config_section.extend(&mut base_file_config.config.unwrap());
                }
                if file_config.config.is_some() {
                    config_section.extend(&mut file_config.config.unwrap());
                }

                ExternalConfig { extend: None, config: Some(config_section), env: Some(all_env), tasks: Some(all_tasks) }
            }
            None => file_config,
        }
    } else {
        logger.info::<()>("External file not found, skipping.", &[], None);

        ExternalConfig::new()
    }
}

fn load_default(
    experimental: bool,
    logger: &Logger,
) -> Config {
    logger.verbose::<()>("Loading default tasks.", &[], None);

    let default_descriptor = include_str!("Makefile.stable.toml");

    let mut default_config: Config = match toml::from_str(default_descriptor) {
        Ok(value) => value,
        Err(error) => panic!("Unable to parse default descriptor, {}", error),
    };
    logger.verbose("Loaded default config:", &[], Some(&default_config));

    if experimental {
        let experimental_descriptor = include_str!("Makefile.beta.toml");

        let experimental_config: Config = match toml::from_str(experimental_descriptor) {
            Ok(value) => value,
            Err(error) => panic!("Unable to parse experimental descriptor, {}", error),
        };
        logger.verbose("Loaded experimental config:", &[], Some(&experimental_config));

        let mut default_tasks = default_config.tasks;
        let mut experimental_tasks = experimental_config.tasks;
        let all_tasks = merge_tasks(&mut default_tasks, &mut experimental_tasks);

        default_config.tasks = all_tasks;
    }

    default_config
}

/// Loads the tasks descriptor.<br>
/// It will first load the default descriptor which is defined in cargo-make internally and
/// afterwards tries to find the external descriptor and load it as well.<br>
/// If an extenal descriptor exists, it will be loaded and extend the default descriptor.
pub fn load(
    file_name: &str,
    env: Option<Vec<String>>,
    experimental: bool,
    logger: &Logger,
) -> Config {
    let default_config = load_default(experimental, &logger);

    let external_config: ExternalConfig = load_external_descriptor(".", file_name, logger);

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
    let mut all_env = merge_maps(&mut default_env, &mut external_env);
    all_env = match env {
        Some(values) => {
            let mut cli_env = HashMap::new();

            for env_pair in &values {
                let env_part: Vec<&str> = env_pair.split('=').collect();
                logger.verbose::<()>("Checking env pair: ", &[&env_pair], None);

                if env_part.len() == 2 {
                    cli_env.insert(env_part[0].to_string(), env_part[1].to_string());
                }
            }

            merge_maps(&mut all_env, &mut cli_env)
        }
        None => all_env,
    };

    let all_tasks = merge_tasks(&mut default_tasks, &mut external_tasks);

    let mut config_section = default_config.config.clone();
    config_section.extend(&mut external_config.config.unwrap_or(ConfigSection::new()));

    let config = Config { config: config_section, env: all_env, tasks: all_tasks };

    logger.verbose("Loaded merged config:", &[], Some(&config));

    config
}

pub fn list_steps(config: &Config) {
    for (key, value) in config.tasks.iter() {
        let description = match value.description {
            Some(ref value) => value,
            None => "No Description.",
        };

        println!("{}: {} ", &key, &description);
    }
}
