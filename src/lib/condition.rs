//! # condition
//!
//! Evaluates conditions based on task configuration and current env.
//!

#[cfg(test)]
#[path = "condition_test.rs"]
mod condition_test;

use crate::environment;
use crate::profile;
use crate::scriptengine;
use crate::types;
use crate::types::{FlowInfo, RustVersionCondition, ScriptValue, Step, TaskCondition};
use crate::version::{is_newer, is_same};
use envmnt;
use fsio;
use fsio::path::from_path::FromPath;
use glob::glob;
use indexmap::IndexMap;
use rust_info;
use rust_info::types::{RustChannel, RustInfo};
use std::path::Path;

fn validate_env_map(env: Option<IndexMap<String, String>>, equal: bool) -> bool {
    match env {
        Some(env_vars) => {
            let mut all_valid = true;

            for (key, current_value) in env_vars.iter() {
                if (equal && !envmnt::is_equal(key, current_value))
                    || (!equal && !envmnt::contains_ignore_case(key, current_value))
                {
                    all_valid = false;
                    break;
                }
            }

            all_valid
        }
        None => true,
    }
}

fn validate_env(condition: &TaskCondition) -> bool {
    validate_env_map(condition.env.clone(), true)
}

fn validate_env_contains(condition: &TaskCondition) -> bool {
    validate_env_map(condition.env_contains.clone(), false)
}

fn validate_env_set(condition: &TaskCondition) -> bool {
    let env = condition.env_set.clone();

    match env {
        Some(env_vars) => {
            let mut all_valid = true;

            for key in env_vars.iter() {
                if !envmnt::exists(key) {
                    all_valid = false;
                    break;
                }
            }

            all_valid
        }
        None => true,
    }
}

fn validate_env_not_set(condition: &TaskCondition) -> bool {
    let env = condition.env_not_set.clone();

    match env {
        Some(env_vars) => {
            let mut all_valid = true;

            for key in env_vars.iter() {
                if envmnt::exists(key) {
                    all_valid = false;
                    break;
                }
            }

            all_valid
        }
        None => true,
    }
}

fn validate_env_bool(condition: &TaskCondition, truthy: bool) -> bool {
    let env = if truthy {
        condition.env_true.clone()
    } else {
        condition.env_false.clone()
    };

    match env {
        Some(env_vars) => {
            let mut all_valid = true;

            for key in env_vars.iter() {
                let is_true = envmnt::is_or(key, !truthy);

                if is_true != truthy {
                    all_valid = false;
                    break;
                }
            }

            all_valid
        }
        None => true,
    }
}

fn validate_os(condition: &TaskCondition) -> bool {
    let os = condition.os.clone();
    match os {
        Some(os_names) => {
            let os_name = envmnt::get_or("CARGO_MAKE_RUST_TARGET_OS", "");
            let index = os_names.iter().position(|value| *value == os_name);

            match index {
                None => {
                    debug!("Failed OS condition, current OS: {}", &os_name);
                    false
                }
                _ => true,
            }
        }
        None => true,
    }
}

fn validate_platform(condition: &TaskCondition) -> bool {
    let platforms = condition.platforms.clone();
    match platforms {
        Some(platform_names) => {
            let platform_name = types::get_platform_name();

            let index = platform_names
                .iter()
                .position(|value| *value == platform_name);

            match index {
                None => {
                    debug!(
                        "Failed platform condition, current platform: {}",
                        &platform_name
                    );
                    false
                }
                _ => true,
            }
        }
        None => true,
    }
}

fn validate_profile(condition: &TaskCondition) -> bool {
    let profiles = condition.profiles.clone();
    match profiles {
        Some(profile_names) => {
            let profile_name = profile::get();

            let index = profile_names
                .iter()
                .position(|value| *value == profile_name);

            match index {
                None => {
                    debug!(
                        "Failed profile condition, current profile: {}",
                        &profile_name
                    );
                    false
                }
                _ => true,
            }
        }
        None => true,
    }
}

fn validate_channel(condition: &TaskCondition, flow_info_option: Option<&FlowInfo>) -> bool {
    match flow_info_option {
        Some(flow_info) => {
            let channels = condition.channels.clone();
            match channels {
                Some(channel_names) => match flow_info.env_info.rust_info.channel {
                    Some(value) => {
                        let index = match value {
                            RustChannel::Stable => channel_names
                                .iter()
                                .position(|value| *value == "stable".to_string()),
                            RustChannel::Beta => channel_names
                                .iter()
                                .position(|value| *value == "beta".to_string()),
                            RustChannel::Nightly => channel_names
                                .iter()
                                .position(|value| *value == "nightly".to_string()),
                        };

                        match index {
                            None => {
                                debug!("Failed channel condition");
                                false
                            }
                            _ => true,
                        }
                    }
                    None => false,
                },
                None => true,
            }
        }
        None => true,
    }
}

fn validate_rust_version_condition(rustinfo: RustInfo, condition: RustVersionCondition) -> bool {
    if rustinfo.version.is_some() {
        let current_version = rustinfo.version.unwrap();

        let mut valid = match condition.min {
            Some(version) => {
                is_same(&version, &current_version, true, true)
                    || is_newer(&version, &current_version, true, true)
            }
            None => true,
        };

        if valid {
            valid = match condition.max {
                Some(version) => {
                    is_same(&version, &current_version, true, true)
                        || is_newer(&current_version, &version, true, true)
                }
                None => true,
            };
        }

        if valid {
            valid = match condition.equal {
                Some(version) => is_same(&version, &current_version, true, true),
                None => true,
            };
        }

        valid
    } else {
        true
    }
}

fn validate_rust_version(condition: &TaskCondition) -> bool {
    let rust_version = condition.rust_version.clone();
    match rust_version {
        Some(rust_version_condition) => {
            let rustinfo = rust_info::get();

            validate_rust_version_condition(rustinfo, rust_version_condition)
        }
        None => true,
    }
}

fn validate_files(file_paths: &Vec<String>, exist: bool) -> bool {
    for file_path in file_paths.iter() {
        let expanded_file_path = environment::expand_value(file_path);
        let path = Path::new(&expanded_file_path);

        if path.exists() != exist {
            return false;
        }
    }

    true
}

fn validate_files_exist(condition: &TaskCondition) -> bool {
    let files = condition.files_exist.clone();
    match files {
        Some(ref file_paths) => validate_files(file_paths, true),
        None => true,
    }
}

fn validate_files_not_exist(condition: &TaskCondition) -> bool {
    let files = condition.files_not_exist.clone();
    match files {
        Some(ref file_paths) => validate_files(file_paths, false),
        None => true,
    }
}

fn validate_files_modified(condition: &TaskCondition) -> bool {
    match &condition.files_modified {
        Some(files_modified) => {
            if files_modified.input.len() == 0 {
                return true;
            }

            let mut latest_binary = 0;
            for glob_pattern in &files_modified.output {
                let glob_pattern = environment::expand_value(glob_pattern);
                match glob(&glob_pattern) {
                    Ok(paths) => {
                        for entry in paths {
                            match entry {
                                Ok(path_value) => {
                                    if path_value.is_file() {
                                        let value_string: String = FromPath::from_path(&path_value);
                                        match fsio::path::get_last_modified_time(&value_string) {
                                            Ok(last_modified_time) => {
                                                if last_modified_time > latest_binary {
                                                    latest_binary = last_modified_time;
                                                }
                                            }
                                            Err(error) => {
                                                error!(
                                            "Unable to extract last modified time for path: {} {:#?}",
                                            &value_string, &error
                                        )
                                            }
                                        }
                                    }
                                }
                                Err(error) => {
                                    error!(
                                        "Unable to process paths for glob: {} {:#?}",
                                        &glob_pattern, &error
                                    )
                                }
                            }
                        }
                    }
                    Err(error) => {
                        error!(
                            "Unable to fetch paths for glob: {} {:#?}",
                            &glob_pattern, &error
                        )
                    }
                }
            }

            if latest_binary == 0 {
                true
            } else {
                for glob_pattern in &files_modified.input {
                    let glob_pattern = environment::expand_value(glob_pattern);
                    match glob(&glob_pattern) {
                        Ok(paths) => {
                            let mut paths_found = false;
                            for entry in paths {
                                paths_found = true;

                                match entry {
                                    Ok(path_value) => {
                                        if path_value.is_file() {
                                            let value_string: String =
                                                FromPath::from_path(&path_value);
                                            match fsio::path::get_last_modified_time(&value_string)
                                            {
                                                Ok(last_modified_time) => {
                                                    if last_modified_time > latest_binary {
                                                        return true;
                                                    }
                                                }
                                                Err(error) => {
                                                    error!(
                                            "Unable to extract last modified time for path: {} {:#?}",
                                            &value_string, &error
                                        )
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        error!(
                                            "Unable to process paths for glob: {} {:#?}",
                                            &glob_pattern, &error
                                        )
                                    }
                                }
                            }

                            if !paths_found {
                                error!("Unable to find input files for pattern: {}", &glob_pattern);
                            }
                        }
                        Err(error) => {
                            error!(
                                "Unable to fetch paths for glob: {} {:#?}",
                                &glob_pattern, &error
                            )
                        }
                    }
                }

                // all sources (input) are older than binaries (output)
                false
            }
        }
        None => true,
    }
}

fn validate_criteria(flow_info: Option<&FlowInfo>, condition: &Option<TaskCondition>) -> bool {
    match condition {
        Some(ref condition_struct) => {
            debug!("Checking task condition structure.");

            validate_os(&condition_struct)
                && validate_platform(&condition_struct)
                && validate_profile(&condition_struct)
                && validate_channel(&condition_struct, flow_info)
                && validate_env(&condition_struct)
                && validate_env_set(&condition_struct)
                && validate_env_not_set(&condition_struct)
                && validate_env_bool(&condition_struct, true)
                && validate_env_bool(&condition_struct, false)
                && validate_env_contains(&condition_struct)
                && validate_rust_version(&condition_struct)
                && validate_files_exist(&condition_struct)
                && validate_files_not_exist(&condition_struct)
                && validate_files_modified(&condition_struct)
        }
        None => true,
    }
}

fn validate_script(condition_script: &Option<Vec<String>>, script_runner: Option<String>) -> bool {
    match condition_script {
        Some(ref script) => {
            debug!("Checking task condition script.");

            return scriptengine::invoke_script_pre_flow(
                &ScriptValue::Text(script.to_vec()),
                script_runner,
                None,
                None,
                false,
                &vec![],
            );
        }
        None => true,
    }
}

pub(crate) fn validate_conditions_without_context(condition: TaskCondition) -> bool {
    validate_criteria(None, &Some(condition))
}

pub(crate) fn validate_conditions(
    flow_info: &FlowInfo,
    condition: &Option<TaskCondition>,
    condition_script: &Option<Vec<String>>,
    script_runner: Option<String>,
) -> bool {
    validate_criteria(Some(&flow_info), &condition)
        && validate_script(&condition_script, script_runner)
}

pub(crate) fn validate_condition_for_step(flow_info: &FlowInfo, step: &Step) -> bool {
    validate_conditions(
        &flow_info,
        &step.config.condition,
        &step.config.condition_script,
        step.config.script_runner.clone(),
    )
}
