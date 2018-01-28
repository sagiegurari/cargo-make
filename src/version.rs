//! # version
//!
//! Checks if the currently running version is the most up to date version and
//! if not, it will print a notification message.
//!

#[cfg(test)]
#[path = "./version_test.rs"]
mod version_test;

use command;
use semver::Version;
use std::process::Command;
use types::GlobalConfig;
use storage;
use std::time::{SystemTime, UNIX_EPOCH};

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn get_version_from_output(line: &str) -> Option<String> {
    let parts: Vec<&str> = line.split(' ').collect();

    if parts.len() >= 3 {
        let version_part = parts[2];
        let version = str::replace(version_part, "\"", "");

        Some(version)
    } else {
        None
    }
}

fn get_latest_version() -> Option<String> {
    let result = Command::new("cargo")
        .arg("search")
        .arg("cargo-make")
        .output();

    match result {
        Ok(output) => {
            let exit_code = command::get_exit_code(Ok(output.status), false);
            if exit_code == 0 {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.split('\n').collect();

                let mut output = None;
                for mut line in lines {
                    line = line.trim();

                    debug!("Checking: {}", &line);

                    if line.starts_with("cargo-make = ") {
                        output = get_version_from_output(line);

                        break;
                    }
                }

                output
            } else {
                None
            }
        }
        _ => None,
    }
}

fn is_newer_found(latest_string: &str) -> bool {
    debug!("Checking Version: {}", &latest_string);

    let current = Version::parse(VERSION);
    match current {
        Ok(current_values) => {
            let latest = Version::parse(latest_string);

            match latest {
                Ok(latest_values) => {
                    if latest_values.major > current_values.major {
                        true
                    } else if latest_values.major == current_values.major {
                        if latest_values.minor > current_values.minor {
                            true
                        } else {
                            latest_values.minor == current_values.minor
                                && latest_values.patch > current_values.patch
                        }
                    } else {
                        false
                    }
                }
                _ => false,
            }
        }
        _ => false,
    }
}

fn print_notification(latest_string: &str) {
    warn!("#####################################################################");
    warn!("#                                                                   #");
    warn!("#                                                                   #");
    warn!("#                  NEW CARGO-MAKE VERSION FOUND!!!                  #");
    warn!(
        "#                  Current: {}, Latest: {}\t\t\t#",
        VERSION, latest_string
    );
    warn!("#    Run 'cargo install --force cargo-make' to get latest version   #");
    warn!("#                                                                   #");
    warn!("#                                                                   #");
    warn!("#####################################################################");
}

pub(crate) fn should_check(global_config: &GlobalConfig) -> bool {
    match global_config.update_check_minimum_interval {
        Some(ref value) => {
            if value == "daily" || value == "weekly" || value == "monthly" {
                false
            } else {
                true
            }
        }
        None => true,
    }
}

pub(crate) fn check() {
    let latest = get_latest_version();

    match latest {
        Some(value) => {
            if is_newer_found(&value) {
                print_notification(&value);

                let mut storage_data = storage::load();
                let now = SystemTime::now();
                match now.duration_since(UNIX_EPOCH) {
                    Ok(duration) => {
                        storage_data.last_update_check = Some(duration.as_secs());
                        storage::store(&storage_data);
                    }
                    _ => (),
                }
            }
        }
        None => (),
    }
}
