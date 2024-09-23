//! # version
//!
//! Checks if the currently running version is the most up to date version and
//! if not, it will print a notification message.
//!

#[cfg(test)]
#[path = "version_test.rs"]
mod version_test;

use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use semver::Version;

use crate::cache;
use crate::command;
use crate::types::{Cache, CliArgs, GlobalConfig};

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
        .arg("--limit=1")
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

fn parse(version_string: &str, allow_partial_version_string: bool) -> Result<Version, ()> {
    match Version::parse(version_string) {
        Ok(version) => Ok(version),
        Err(_) => {
            if allow_partial_version_string {
                match lenient_semver::parse(version_string) {
                    Ok(version) => Ok(version),
                    Err(_) => Err(()),
                }
            } else {
                Err(())
            }
        }
    }
}

pub(crate) fn is_same(
    version_string1: &str,
    version_string2: &str,
    allow_partial_version_string: bool,
    default_result: bool,
) -> bool {
    let version1 = parse(version_string1, allow_partial_version_string);
    match version1 {
        Ok(values1) => {
            let version2 = parse(version_string2, allow_partial_version_string);

            match version2 {
                Ok(values2) => {
                    values1.major == values2.major
                        && values1.minor == values2.minor
                        && values1.patch == values2.patch
                }
                _ => default_result,
            }
        }
        _ => default_result,
    }
}

pub(crate) fn is_newer(
    old_string: &str,
    new_string: &str,
    allow_partial_version_string: bool,
    default_result: bool,
) -> bool {
    let old_version = parse(old_string, allow_partial_version_string);
    match old_version {
        Ok(old_values) => {
            let new_version = parse(new_string, allow_partial_version_string);

            match new_version {
                Ok(new_values) => {
                    if new_values.major > old_values.major {
                        true
                    } else if new_values.major == old_values.major {
                        if new_values.minor > old_values.minor {
                            true
                        } else {
                            new_values.minor == old_values.minor
                                && new_values.patch > old_values.patch
                        }
                    } else {
                        false
                    }
                }
                _ => default_result,
            }
        }
        _ => default_result,
    }
}

pub(crate) fn is_newer_found(version_string: &str) -> bool {
    debug!("Checking Version: {}", &version_string);

    is_newer(&VERSION, &version_string, false, false)
}

fn print_notification(latest_string: &str) {
    warn!("#####################################################################");
    warn!("#                                                                   #");
    warn!("#                                                                   #");
    warn!("#                  NEW CARGO-MAKE VERSION FOUND!!!                  #");
    warn!(
        "#{:^67}#",
        format!("Current: {}, Latest: {}", VERSION, latest_string)
    );
    warn!("#    Run 'cargo install --force cargo-make' to get latest version   #");
    warn!("#                                                                   #");
    warn!("#                                                                   #");
    warn!("#####################################################################");
}

fn get_now_as_seconds() -> u64 {
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => duration.as_secs(),
        _ => 0,
    }
}

fn has_amount_of_days_passed_from_last_check(days: u64, last_check_seconds: u64) -> bool {
    let now_seconds = get_now_as_seconds();
    if now_seconds > 0 && days > 0 {
        if last_check_seconds > now_seconds {
            false
        } else {
            let diff_seconds = now_seconds - last_check_seconds;
            let minimum_diff_seconds = days * 24 * 60 * 60;

            diff_seconds >= minimum_diff_seconds
        }
    } else {
        true
    }
}

fn has_amount_of_days_passed(days: u64, cache_data: &Cache) -> bool {
    match cache_data.last_update_check {
        Some(last_check_seconds) => {
            has_amount_of_days_passed_from_last_check(days, last_check_seconds)
        }
        None => true,
    }
}

fn get_days(global_config: &GlobalConfig) -> u64 {
    match global_config.update_check_minimum_interval {
        Some(ref value) => {
            if value == "always" {
                0
            } else if value == "daily" {
                1
            } else if value == "monthly" {
                30
            } else {
                // default to weekly
                7
            }
        }
        None => 7, // default to weekly
    }
}

fn should_check_overdue(global_config: &GlobalConfig) -> bool {
    let days = get_days(global_config);

    if days > 0 {
        let cache_data = cache::load();
        has_amount_of_days_passed(days, &cache_data)
    } else {
        true
    }
}

fn should_check_for_args(cli_args: &CliArgs, global_config: &GlobalConfig, is_ci: bool) -> bool {
    // only run check for updates if we are not in a CI env and user didn't ask to skip the check
    !cli_args.disable_check_for_updates
        && !is_ci
        && !envmnt::is_or("CARGO_MAKE_DISABLE_UPDATE_CHECK", false)
        && should_check_overdue(&global_config)
}

pub(crate) fn should_check(cli_args: &CliArgs, global_config: &GlobalConfig) -> bool {
    // only run check for updates if we are not in a CI env and user didn't ask to skip the check
    should_check_for_args(cli_args, global_config, ci_info::is_ci())
}

pub(crate) fn check() {
    let latest = get_latest_version();

    let mut cache_data = cache::load();
    let now = get_now_as_seconds();
    if now > 0 {
        cache_data.last_update_check = Some(now);
        cache::store(&cache_data);
    }

    match latest {
        Some(value) => {
            if is_newer_found(&value) {
                print_notification(&value);
            }
        }
        None => (),
    }
}
