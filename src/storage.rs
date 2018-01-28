//! # storage
//!
//! Manages internal cargo-make persistant storage.
//!

#[cfg(test)]
#[path = "./storage_test.rs"]
mod storage_test;

use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use toml;
use environment;
use types::Storage;

fn load_from_path(directory: PathBuf) -> Storage {
    let file_path = Path::new(&directory).join("storage.toml");

    if file_path.exists() {
        match File::open(&file_path) {
            Ok(mut file) => {
                let mut storage_str = String::new();
                file.read_to_string(&mut storage_str).unwrap();

                let mut storage_data: Storage = match toml::from_str(&storage_str) {
                    Ok(mut value) => value,
                    _ => Storage::new(),
                };

                match file_path.to_str() {
                    Some(value) => storage_data.file_name = Some(value.to_string()),
                    None => storage_data.file_name = None,
                };

                storage_data
            }
            _ => Storage::new(),
        }
    } else {
        Storage::new()
    }
}

/// Loads the persisted data
pub(crate) fn load() -> Storage {
    match environment::get_cargo_make_home() {
        Some(directory) => load_from_path(directory),
        None => Storage::new(),
    }
}

/// Stores the data
pub(crate) fn store(storage_data: &Storage) {
    match environment::get_cargo_make_home() {
        Some(directory) => {
            let file_name = directory.join("storage.toml");

            match File::open(&file_name) {
                Ok(mut file) => match toml::to_string_pretty(storage_data) {
                    Ok(toml_str) => {
                        let data = toml_str.as_bytes();

                        match file.write_all(data) {
                            Err(error) => error!(
                                "Error while writing to storage file: {:#?}, error: {:#?}",
                                &file_name, error
                            ),
                            _ => (),
                        }
                    }
                    Err(error) => error!(
                        "Error during serialization of storage, file: {:#?}, error: {:#?}",
                        &file_name, error
                    ),
                },
                Err(error) => error!(
                    "Error while creating/appending to storage file: {:#?}, error: {:#?}",
                    &file_name, error
                ),
            };
        }
        None => (),
    }
}
