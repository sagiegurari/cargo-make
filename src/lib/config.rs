//! # config
//!
//! Enable to load/store user level configuration for cargo-make.
//!

#[cfg(test)]
#[path = "config_test.rs"]
mod config_test;

use crate::storage;
use crate::types::GlobalConfig;
use fsio::file::read_text_file;
use fsio::path::from_path::FromPath;
use std::path::{Path, PathBuf};

pub static CONFIG_FILE: &'static str = "config.toml";

pub fn get_config_directory() -> Option<PathBuf> {
    let os_directory = dirs_next::config_dir();
    storage::get_storage_directory(os_directory, CONFIG_FILE, true)
}

pub fn load_from_path(directory: PathBuf) -> GlobalConfig {
    let file_path = Path::new(&directory).join(CONFIG_FILE);
    debug!("Loading config from: {:#?}", &file_path);

    if file_path.exists() {
        match read_text_file(&file_path) {
            Ok(config_str) => {
                let mut global_config: GlobalConfig = match toml::from_str(&config_str) {
                    Ok(value) => value,
                    Err(error) => panic!("Unable to parse global configuration file, {}", error),
                };

                global_config.file_name = Some(FromPath::from_path(&file_path));

                global_config
            }
            Err(error) => panic!(
                "Unable to read config file: {:?} error: {}",
                &file_path, error
            ),
        }
    } else {
        GlobalConfig::new()
    }
}

/// Returns the configuration
pub fn load() -> GlobalConfig {
    match get_config_directory() {
        Some(directory) => load_from_path(directory),
        None => GlobalConfig::new(),
    }
}
