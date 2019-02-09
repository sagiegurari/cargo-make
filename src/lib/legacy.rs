//! # legacy
//!
//! Support legacy features.
//!

#[cfg(test)]
#[path = "./legacy_test.rs"]
mod legacy_test;

use dirs;
use std::env;
use std::fs::{copy, create_dir_all, remove_dir, remove_file};
use std::path::PathBuf;

fn get_legacy_cargo_make_home() -> Option<PathBuf> {
    match dirs::home_dir() {
        Some(directory) => Some(directory.join(".cargo-make")),
        None => None,
    }
}

pub(crate) fn get_cargo_make_home() -> Option<PathBuf> {
    match env::var("CARGO_MAKE_HOME") {
        Ok(directory) => Some(PathBuf::from(directory)),
        _ => get_legacy_cargo_make_home(),
    }
}

fn migrate_from_directory(
    target_directory: PathBuf,
    file: &str,
    legacy_directory: &PathBuf,
) -> bool {
    let legacy_file = legacy_directory.join(file);

    if legacy_file.exists() {
        let exists = if target_directory.exists() {
            true
        } else {
            match create_dir_all(&target_directory) {
                Ok(_) => true,
                _ => false,
            }
        };

        if exists {
            let target_file = target_directory.join(file);
            info!("Legacy cargo-make file: {:#?} exists, target directory: {:#?} exists, copy to: {:#?}", &legacy_file, &target_directory, &target_file);

            match copy(&legacy_file, &target_file) {
                Ok(_) => {
                    info!("Delete legacy cargo-make file: {:#?}", &legacy_file);
                    remove_file(&legacy_file).unwrap_or(());

                    // delete old directory (will only work if empty)
                    remove_dir(&legacy_directory).unwrap_or(());

                    true
                }
                Err(error) => {
                    info!(
                        "Error while copying legacy file: {:#?} to: {:#?}, error: {:#?}",
                        &legacy_file, &target_file, &error
                    );
                    false
                }
            }
        } else {
            false
        }
    } else {
        true
    }
}

pub(crate) fn migrate(target_directory: PathBuf, file: &str) -> bool {
    debug!(
        "Legacy cargo-make target_directory: {:#?} file: {:#?} ",
        &target_directory, &file
    );
    return match get_legacy_cargo_make_home() {
        Some(directory) => migrate_from_directory(target_directory, &file, &directory),
        None => true,
    };
}

pub(crate) fn show_deprecated_attriute_warning(old_attribute: &str, new_attribute: &str) {
    warn!(
        "[DEPRECATED] The attribute '{}' has been replaced with '{}'. Please update your makefile.",
        old_attribute, new_attribute
    );
}
