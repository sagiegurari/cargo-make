//! # cargo_alias
//!
//! Dynamically creates tasks based on alias information in the cargo config.
//!

#[cfg(test)]
#[path = "./cargo_alias_test.rs"]
mod cargo_alias_test;

use crate::io;
use crate::types::{InstallCrate, Task};
use std::collections::HashMap;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum AliasValue {
    String(String),
    List(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
struct CargoConfig {
    alias: Option<HashMap<String, AliasValue>>,
}

fn load_from_file(file: &str) -> Vec<(String, Task)> {
    let file_path = Path::new(file);

    let mut tasks = vec![];
    if file_path.exists() {
        if file_path.is_file() {
            let text = io::read_text_file(&file_path.to_path_buf());

            if !text.is_empty() {
                let cargo_config: CargoConfig = match toml::from_str(&text) {
                    Ok(value) => value,
                    Err(error) => {
                        warn!("Unable to parse cargo config file, {}", error);
                        CargoConfig { alias: None }
                    }
                };

                if let Some(aliases) = cargo_config.alias {
                    for (key, _value) in aliases {
                        let mut task = Task::new();
                        task.command = Some("cargo".to_string());
                        task.args = Some(vec![key.to_string()]);
                        task.install_crate = Some(InstallCrate::Enabled(false));

                        tasks.push((key, task));
                    }
                }
            }
        } else {
            error!("Invalid config file path provided: {}", &file);
        }
    }

    tasks
}

pub(crate) fn load() -> Vec<(String, Task)> {
    load_from_file("./.cargo/config.toml")
}
