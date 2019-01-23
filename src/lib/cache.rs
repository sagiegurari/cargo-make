//! # cache
//!
//! Manages internal cargo-make persistant cache.
//!

#[cfg(test)]
#[path = "./cache_test.rs"]
mod cache_test;

use crate::storage;
use crate::types::Cache;
use dirs;
use std::fs::{create_dir_all, File};
use std::io::prelude::*;
use std::io::Read;
use std::path::{Path, PathBuf};
use toml;

static CACHE_FILE: &'static str = "cache.toml";

fn load_from_path(directory: PathBuf) -> Cache {
    let file_path = Path::new(&directory).join(CACHE_FILE);

    let mut cache_data = if file_path.exists() {
        match File::open(&file_path) {
            Ok(mut file) => {
                let mut cache_str = String::new();
                file.read_to_string(&mut cache_str).unwrap();

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
                    "Unable to open cache file, directory: {:#?} error: {}",
                    &directory, error
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
    let os_directory = dirs::cache_dir();
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
                match create_dir_all(&directory) {
                    Ok(_) => true,
                    _ => false,
                }
            };

            if exists {
                let file_name = directory.join(CACHE_FILE);

                let file_descriptor = match File::create(&file_name) {
                    Ok(file) => Some(file),
                    Err(error) => {
                        info!(
                            "Error while creating cache file: {:#?}, error: {:#?}",
                            &file_name, error
                        );
                        None
                    }
                };

                match file_descriptor {
                    Some(mut file) => match toml::to_string_pretty(cache_data) {
                        Ok(toml_str) => {
                            let data = toml_str.as_bytes();

                            match file.write_all(data) {
                                Err(error) => info!(
                                    "Error while writing to cache file: {:#?}, error: {:#?}",
                                    &file_name, error
                                ),
                                _ => (),
                            }
                        }
                        Err(error) => info!(
                            "Error during serialization of cache, file: {:#?}, error: {:#?}",
                            &file_name, error
                        ),
                    },
                    _ => (),
                };
            }
        }
        None => (),
    }
}
