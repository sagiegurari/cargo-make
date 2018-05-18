//! # config
//!
//! Enable to load/store user level configuration for cargo-make.
//!

#[cfg(test)]
#[path = "./config_test.rs"]
mod config_test;

use environment;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use toml;
use types::GlobalConfig;

fn load_from_path(directory: PathBuf) -> GlobalConfig {
    let file_path = Path::new(&directory).join("config.toml");

    if file_path.exists() {
        let mut file = match File::open(&file_path) {
            Ok(value) => value,
            Err(error) => panic!(
                "Unable to open config file, directory: {:#?} error: {}",
                &directory, error
            ),
        };

        let mut config_str = String::new();
        file.read_to_string(&mut config_str).unwrap();

        let mut global_config: GlobalConfig = match toml::from_str(&config_str) {
            Ok(value) => value,
            Err(error) => panic!("Unable to parse global configuration file, {}", error),
        };

        match file_path.to_str() {
            Some(value) => global_config.file_name = Some(value.to_string()),
            None => global_config.file_name = None,
        };

        global_config
    } else {
        GlobalConfig::new()
    }
}

/// Returns the configuration
pub(crate) fn load() -> GlobalConfig {
    match environment::get_cargo_make_home() {
        Some(directory) => load_from_path(directory),
        None => GlobalConfig::new(),
    }
}
