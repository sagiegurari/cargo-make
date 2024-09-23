//! # crate_version_check
//!
//! Checks crate versions.
//!

#[cfg(test)]
#[path = "crate_version_check_test.rs"]
mod crate_version_check_test;

use std::collections::HashMap;
use std::env;
use std::path::Path;

use semver::Version;

use crate::command;

#[derive(Deserialize, Debug)]
struct CratesRegistryInfo {
    v1: Option<HashMap<String, Vec<String>>>,
}

enum VersionParseOutput {
    Version(Version),
    InvalidVersion,
    WrongCrate,
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

fn parse_version_from_string(version_line: &str, crate_name: &str) -> VersionParseOutput {
    let parts: Vec<&str> = version_line.split(' ').collect();

    if parts.len() >= 2 && parts[0] == crate_name {
        match Version::parse(parts[1]) {
            Ok(version) => VersionParseOutput::Version(version),
            _ => VersionParseOutput::InvalidVersion,
        }
    } else {
        VersionParseOutput::WrongCrate
    }
}

fn get_crate_version_from_info(crate_name: &str, info: &CratesRegistryInfo) -> Option<Version> {
    match info.v1 {
        Some(ref keys) => {
            for key in keys.keys() {
                let output = parse_version_from_string(&key, crate_name);

                match output {
                    VersionParseOutput::Version(version) => return Some(version),
                    VersionParseOutput::InvalidVersion => return None,
                    VersionParseOutput::WrongCrate => (),
                };
            }

            None
        }
        None => None,
    }
}

pub(crate) fn get_crate_version(crate_name: &str, binary: Option<&str>) -> Option<Version> {
    let cargo_home = get_cargo_home();
    let version = match cargo_home {
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
    };

    match version {
        Some(value) => Some(value),
        None => match binary {
            Some(value) => {
                let result = command::run_command_get_output_string(
                    &value,
                    &Some(vec!["--version".to_string()]),
                );

                match result {
                    Some(ref output) => {
                        debug!("Version CLI output: {}", output);

                        match parse_version_from_string(output, crate_name) {
                            VersionParseOutput::Version(version) => Some(version),
                            VersionParseOutput::InvalidVersion => None,
                            VersionParseOutput::WrongCrate => None,
                        }
                    }
                    None => None,
                }
            }
            None => None,
        },
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

pub(crate) fn is_min_version_valid(
    crate_name: &str,
    min_version: &str,
    binary: Option<&str>,
) -> bool {
    let semver_value = Version::parse(min_version);
    match semver_value {
        Ok(version_values) => match get_crate_version(crate_name, binary) {
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

fn is_version_valid_for_versions(version: &Version, installed_version: &Version) -> bool {
    version.major == installed_version.major
        && version.minor == installed_version.minor
        && version.patch == installed_version.patch
}

pub(crate) fn is_version_valid(crate_name: &str, version: &str, binary: Option<&str>) -> bool {
    let semver_value = Version::parse(version);
    match semver_value {
        Ok(version_values) => match get_crate_version(crate_name, binary) {
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
