//! # log
//!
//! Loads the tasks descriptor.
//!

use log::Log;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use types::{Config, Task};

extern crate serde;
extern crate toml;

fn merge(
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
        merged.insert(key_str, value.clone());
    }

    merged
}

pub fn load(
    file_name: &str,
    logger: &Log,
) -> Config {
    logger.verbose::<()>("Loading default tasks.", &[], None);

    let default_descriptor = include_str!("default.toml");

    let default_config: Config = match toml::from_str(default_descriptor) {
        Ok(value) => value,
        Err(error) => panic!("Unable to parse default descriptor, {}", error),
    };
    logger.verbose("Loaded default config:", &[], Some(&default_config));

    logger.verbose::<()>("Loading tasks from file: ", &[file_name], None);

    let file_path = Path::new(file_name);

    let mut external_tasks: HashMap<String, Task> = if file_path.exists() {
        let mut file = File::open(file_name).unwrap();
        let mut external_descriptor = String::new();
        file.read_to_string(&mut external_descriptor).unwrap();

        let file_config: Config = toml::from_str(&external_descriptor).unwrap();
        logger.verbose("Loaded external config:", &[], Some(&file_config));

        file_config.tasks
    } else {
        logger.info::<()>("External file not found, skipping.", &[], None);

        HashMap::new()
    };

    let mut default_tasks = default_config.tasks;

    // merge configs
    let all_tasks = merge(&mut default_tasks, &mut external_tasks);
    let config = Config { tasks: all_tasks };

    logger.verbose("Loaded merged config:", &[], Some(&config));

    config
}
