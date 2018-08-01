//! # config
//!
//! Enable to load/store user level configuration for cargo-make.
//!

#[cfg(test)]
#[path = "./config_test.rs"]
mod config_test;

use dirs;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use storage;
use toml;
use types::GlobalConfig;

static CONFIG_FILE: &'static str = "config.toml";

fn get_config_directory() -> Option<PathBuf> {
    let os_directory = dirs::config_dir();
    storage::get_storage_directory(os_directory, CONFIG_FILE, true)
}

fn load_from_path(directory: PathBuf) -> GlobalConfig {
    let file_path = Path::new(&directory).join(CONFIG_FILE);
    info!("Loading config from: {:#?}", &file_path);

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
    match get_config_directory() {
        Some(directory) => load_from_path(directory),
        None => GlobalConfig::new(),
    }
}
