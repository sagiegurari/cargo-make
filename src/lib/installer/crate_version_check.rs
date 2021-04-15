//! # crate_version_check
//!
//! Checks crate versions.
//!

#[cfg(test)]
#[path = "crate_version_check_test.rs"]
mod crate_version_check_test;

use dirs_next;
use fsio;
use semver::Version;
use std::collections::HashMap;
use std::env;
use std::path::Path;
use toml;

#[derive(Deserialize, Debug)]
struct CratesRegistryInfo {
    v1: Option<HashMap<String, Vec<String>>>,
}

fn get_cargo_home() -> Option<String> {
    let cargo_home = match env::var("CARGO_HOME") {
        Ok(value) => Some(value.to_string()),
        Err(_) => match dirs_next::home_dir() {
            Some(directory) => match directory.join(".cargo").to_str() {
                Some(value) => Some(value.to_string()),
                None => None,
            },
            None => None,
        },
    };

    match cargo_home {
        Some(directory) => {
            let directory_path = Path::new(&directory);
            if directory_path.exists() && directory_path.is_dir() {
                Some(directory)
            } else {
                None
            }
        }
        None => None,
    }
}

fn load_crates_toml(cargo_home: &str) -> Option<CratesRegistryInfo> {
    let file_path = Path::new(cargo_home).join(".crates.toml");

    if file_path.exists() && file_path.is_file() {
        match fsio::file::read_text_file(&file_path) {
            Ok(file_content) => match toml::from_str(&file_content) {
                Ok(info) => Some(info),
                Err(error) => {
                    warn!("Unable to parse crates descriptor, error: {}", &error);
                    None
                }
            },
            Err(error) => {
                warn!("Unable to open crates descriptor, error: {}", &error);
                None
            }
        }
    } else {
        None
    }
}

fn get_crate_version_from_info(crate_name: &str, info: &CratesRegistryInfo) -> Option<Version> {
    match info.v1 {
        Some(ref keys) => {
            let mut output = None;

            for key in keys.keys() {
                let parts: Vec<&str> = key.split(' ').collect();

                if parts.len() >= 2 && parts[0] == crate_name {
                    output = match Version::parse(parts[1]) {
                        Ok(version) => Some(version),
                        _ => None,
                    };

                    break;
                }
            }

            output
        }
        None => None,
    }
}

pub(crate) fn get_crate_version(crate_name: &str) -> Option<Version> {
    let cargo_home = get_cargo_home();
    match cargo_home {
        Some(directory) => match load_crates_toml(&directory) {
            Some(info) => get_crate_version_from_info(&crate_name, &info),
            None => {
                warn!(
                    "Unable to read crates registry information to validate {} crate version.",
                    &crate_name
                );
                None
            }
        },
        None => {
            warn!(
                "Unable to find cargo home directory to validate {} crate version.",
                &crate_name
            );
            None
        }
    }
}

pub(crate) fn is_min_version_valid_for_versions(
    min_version: &Version,
    installed_version: &Version,
) -> bool {
    if min_version.major > installed_version.major {
        false
    } else if min_version.major < installed_version.major {
        true
    } else if min_version.minor > installed_version.minor {
        false
    } else if min_version.minor < installed_version.minor {
        true
    } else {
        installed_version.patch >= min_version.patch
    }
}

pub(crate) fn is_min_version_valid(crate_name: &str, min_version: &str) -> bool {
    let semver_value = Version::parse(min_version);
    match semver_value {
        Ok(version_values) => match get_crate_version(crate_name) {
            Some(installed_version_values) => {
                is_min_version_valid_for_versions(&version_values, &installed_version_values)
            }
            None => {
                warn!(
                    "Unable to read currently installed version for crate: {}",
                    &crate_name
                );
                true
            }
        },
        _ => {
            warn!("Unable to parse min version value: {}", &min_version);
            true
        }
    }
}

pub(crate) fn is_version_valid_for_versions(
    version: &Version,
    installed_version: &Version,
) -> bool {
    version.major == installed_version.major
        && version.minor == installed_version.minor
        && version.patch == installed_version.patch
}

pub(crate) fn is_version_valid(crate_name: &str, version: &str) -> bool {
    let semver_value = Version::parse(version);
    match semver_value {
        Ok(version_values) => match get_crate_version(crate_name) {
            Some(installed_version_values) => {
                is_version_valid_for_versions(&version_values, &installed_version_values)
            }
            None => {
                warn!(
                    "Unable to read currently installed version for crate: {}",
                    &crate_name
                );
                true
            }
        },
        _ => {
            warn!("Unable to parse version value: {}", &version);
            true
        }
    }
}
