//! # cache
//!
//! Manages internal cargo-make persistant cache.
//!

#[cfg(test)]
#[path = "./cache_test.rs"]
mod cache_test;

use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use toml;
use environment;
use types::Cache;

fn load_from_path(directory: PathBuf) -> Cache {
    let file_path = Path::new(&directory).join("cache.toml");

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

/// Loads the persisted data
pub(crate) fn load() -> Cache {
    match environment::get_cargo_make_home() {
        Some(directory) => load_from_path(directory),
        None => Cache::new(),
    }
}

/// Stores the data
pub(crate) fn store(cache_data: &Cache) {
    match environment::get_cargo_make_home() {
        Some(directory) => {
            let file_name = directory.join("cache.toml");

            match File::open(&file_name) {
                Ok(mut file) => match toml::to_string_pretty(cache_data) {
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
                Err(error) => info!(
                    "Error while creating/appending to cache file: {:#?}, error: {:#?}",
                    &file_name, error
                ),
            };
        }
        None => (),
    }
}
