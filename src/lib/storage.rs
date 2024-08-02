//! # storage
//!
//! Provides storage related utilities such as config/cache directory locations.
//!

#[cfg(test)]
#[path = "storage_test.rs"]
mod storage_test;

use std::env;
use std::path::{Path, PathBuf};

use crate::legacy;

fn get_legacy_directory() -> Option<PathBuf> {
    legacy::get_cargo_make_home()
}

pub(crate) fn get_storage_directory(
    os_directory: Option<PathBuf>,
    file_name: &str,
    migrate: bool,
) -> Option<PathBuf> {
    match env::var("CARGO_MAKE_HOME") {
        // if env is defined, it is taken as highest priority
        Ok(directory) => Some(PathBuf::from(directory)),
        _ => {
            match os_directory {
                Some(directory) => {
                    let home_directory = directory.join("cargo-make");

                    let file_path = Path::new(&directory).join(file_name);

                    // migrate old data to new directory
                    if !file_path.exists() && migrate {
                        legacy::migrate(home_directory.clone(), file_name);
                    }

                    Some(home_directory)
                }
                None => get_legacy_directory(), // in case no dir is defined for system, default to old approach
            }
        }
    }
}
