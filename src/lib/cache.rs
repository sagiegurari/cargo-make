//! # cache
//!
//! Manages internal cargo-make persistant cache.
//!

#[cfg(test)]
#[path = "cache_test.rs"]
mod cache_test;

use crate::storage;
use crate::types::Cache;
use dirs_next;
use fsio;
use fsio::file::{read_text_file, write_text_file};
use std::path::{Path, PathBuf};
use toml;

static CACHE_FILE: &'static str = "cache.toml";

fn load_from_path(directory: PathBuf) -> Cache {
    let file_path = Path::new(&directory).join(CACHE_FILE);

    let mut cache_data = if file_path.exists() {
        match read_text_file(&file_path) {
            Ok(cache_str) => {
                let cache_data: Cache = match toml::from_str(&cache_str) {
                    Ok(value) => value,
                    Err(error) => {
                        info!("Unable to parse cache file, {}", error);
                        Cache::new()
                    }
                };

                cache_data
            }
            Err(error) => {
                info!(
                    "Unable to read cache file: {:?} error: {}",
                    &file_path,
                    error.to_string()
                );
                Cache::new()
            }
        }
    } else {
        Cache::new()
    };

    match file_path.to_str() {
        Some(value) => cache_data.file_name = Some(value.to_string()),
        None => cache_data.file_name = None,
    };

    cache_data
}

fn get_cache_directory(migrate: bool) -> Option<PathBuf> {
    let os_directory = dirs_next::cache_dir();
    storage::get_storage_directory(os_directory, CACHE_FILE, migrate)
}

/// Loads the persisted data
pub(crate) fn load() -> Cache {
    match get_cache_directory(true) {
        Some(directory) => load_from_path(directory),
        None => Cache::new(),
    }
}

/// Stores the data
pub(crate) fn store(cache_data: &Cache) {
    match get_cache_directory(false) {
        Some(directory) => {
            let exists = if directory.exists() {
                true
            } else {
                match fsio::directory::create(&directory) {
                    Ok(_) => true,
                    _ => false,
                }
            };

            if exists {
                let file_name = directory.join(CACHE_FILE);

                match toml::to_string_pretty(cache_data) {
                    Ok(toml_str) => match write_text_file(&file_name, &toml_str) {
                        Err(error) => info!(
                            "Error while writing to cache file: {:#?}, error: {:#?}",
                            &file_name, error
                        ),
                        _ => (),
                    },
                    Err(error) => info!(
                        "Error during serialization of cache, file: {:#?}, error: {:#?}",
                        &file_name, error
                    ),
                };
            }
        }
        None => (),
    }
}
