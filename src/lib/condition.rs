//! # condition
//!
//! Evaluates conditions based on task configuration and current env.
//!

#[cfg(test)]
#[path = "condition_test.rs"]
mod condition_test;

use std::path::Path;

use fsio::path::from_path::FromPath;
use glob::glob;
use indexmap::IndexMap;
use rust_info::types::{RustChannel, RustInfo};

use crate::environment;
use crate::error::CargoMakeError;
use crate::profile;
use crate::scriptengine;
use crate::types;
use crate::types::{
    ConditionScriptValue, ConditionType, FlowInfo, RustVersionCondition, ScriptValue, Step,
    TaskCondition,
};
use crate::version::{is_newer, is_same};

fn validate_env_map(
    env: Option<IndexMap<String, String>>,
    equal: bool,
    validate_any: bool,
) -> bool {
    match env {
        Some(env_vars) => {
            let mut found_any = env_vars.is_empty();

            for (key, current_value) in env_vars.iter() {
                let valid = if equal {
                    envmnt::is_equal(key, current_value)
                } else {
                    envmnt::contains_ignore_case(key, current_value)
                };

                if valid {
                    if validate_any {
                        return true;
                    }

                    found_any = true;
                } else if !valid && !validate_any {
                    return false;
                }
            }

            found_any
        }
        None => true,
    }
}

fn validate_env(condition: &TaskCondition, validate_any: bool) -> bool {
    validate_env_map(condition.env.clone(), true, validate_any)
}

fn validate_env_contains(condition: &TaskCondition, validate_any: bool) -> bool {
    validate_env_map(condition.env_contains.clone(), false, validate_any)
}

fn validate_env_set(condition: &TaskCondition, validate_any: bool) -> bool {
    let env = condition.env_set.clone();
    match env {
        Some(env_vars) => {
            let mut found_any = env_vars.is_empty();

            for key in env_vars.iter() {
                let exists = envmnt::exists(key);
                if exists {
                    if validate_any {
                        return true;
                    }

                    found_any = true;
                } else if !exists && !validate_any {
                    return false;
                }
            }

            found_any
        }
        None => true,
    }
}

fn validate_env_not_set(condition: &TaskCondition, validate_any: bool) -> bool {
    let env = condition.env_not_set.clone();

    match env {
        Some(env_vars) => {
            let mut found_any = env_vars.is_empty();

            for key in env_vars.iter() {
                let exists = envmnt::exists(key);
                if !exists {
                    if validate_any {
                        return true;
                    }

                    found_any = true;
                } else if exists && !validate_any {
                    return false;
                }
            }

            found_any
        }
        None => true,
    }
}

fn validate_env_bool(condition: &TaskCondition, truthy: bool, validate_any: bool) -> bool {
    let env = if truthy {
        condition.env_true.clone()
    } else {
        condition.env_false.clone()
    };

    match env {
        Some(env_vars) => {
            let mut found_any = env_vars.is_empty();

            for key in env_vars.iter() {
                let is_true = envmnt::is_or(key, !truthy);
                let is_equal = is_true == truthy;

                if is_equal {
                    if validate_any {
                        return true;
                    }

                    found_any = true;
                } else if !is_equal && !validate_any {
                    return false;
                }
            }

            found_any
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

fn validate_files(file_paths: &Vec<String>, exist: bool, validate_any: bool) -> bool {
    let mut found_any = file_paths.is_empty();

    for file_path in file_paths.iter() {
        let expanded_file_path = environment::expand_value(file_path);
        let path = Path::new(&expanded_file_path);

        let path_exists = path.exists();
        let valid = path_exists == exist;

        if valid {
            if validate_any {
                return true;
            }

            found_any = true;
        } else if !valid && !validate_any {
            return false;
        }
    }

    found_any
}

fn validate_files_exist(condition: &TaskCondition, validate_any: bool) -> bool {
    let files = condition.files_exist.clone();
    match files {
        Some(ref file_paths) => validate_files(file_paths, true, validate_any),
        None => true,
    }
}

fn validate_files_not_exist(condition: &TaskCondition, validate_any: bool) -> bool {
    let files = condition.files_not_exist.clone();
    match files {
        Some(ref file_paths) => validate_files(file_paths, false, validate_any),
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

            let condition_type = condition_struct.get_condition_type();
            let validate_any = condition_type == ConditionType::Or;
            let group_or_condition = condition_type == ConditionType::GroupOr || validate_any;
            let mut not_valid_found = false;

            let mut valid = validate_os(&condition_struct);
            if group_or_condition && valid && condition_struct.os.is_some() {
                return true;
            } else if !group_or_condition && !valid {
                return false;
            } else if group_or_condition && !valid {
                not_valid_found = true;
            }

            valid = validate_platform(&condition_struct);
            if group_or_condition && valid && condition_struct.platforms.is_some() {
                return true;
            } else if !group_or_condition && !valid {
                return false;
            } else if group_or_condition && !valid {
                not_valid_found = true;
            }

            valid = validate_profile(&condition_struct);
            if group_or_condition && valid && condition_struct.profiles.is_some() {
                return true;
            } else if !group_or_condition && !valid {
                return false;
            } else if group_or_condition && !valid {
                not_valid_found = true;
            }

            valid = validate_channel(&condition_struct, flow_info);
            if group_or_condition && valid && condition_struct.channels.is_some() {
                return true;
            } else if !group_or_condition && !valid {
                return false;
            } else if group_or_condition && !valid {
                not_valid_found = true;
            }

            valid = validate_env(&condition_struct, validate_any);
            if group_or_condition && valid && condition_struct.env.is_some() {
                return true;
            } else if !group_or_condition && !valid {
                return false;
            } else if group_or_condition && !valid {
                not_valid_found = true;
            }

            valid = validate_env_set(&condition_struct, validate_any);
            if group_or_condition && valid && condition_struct.env_set.is_some() {
                return true;
            } else if !group_or_condition && !valid {
                return false;
            } else if group_or_condition && !valid {
                not_valid_found = true;
            }

            valid = validate_env_not_set(&condition_struct, validate_any);
            if group_or_condition && valid && condition_struct.env_not_set.is_some() {
                return true;
            } else if !group_or_condition && !valid {
                return false;
            } else if group_or_condition && !valid {
                not_valid_found = true;
            }

            valid = validate_env_bool(&condition_struct, true, validate_any);
            if group_or_condition && valid && condition_struct.env_true.is_some() {
                return true;
            } else if !group_or_condition && !valid {
                return false;
            } else if group_or_condition && !valid {
                not_valid_found = true;
            }

            valid = validate_env_bool(&condition_struct, false, validate_any);
            if group_or_condition && valid && condition_struct.env_false.is_some() {
                return true;
            } else if !group_or_condition && !valid {
                return false;
            } else if group_or_condition && !valid {
                not_valid_found = true;
            }

            valid = validate_env_contains(&condition_struct, validate_any);
            if group_or_condition && valid && condition_struct.env_contains.is_some() {
                return true;
            } else if !group_or_condition && !valid {
                return false;
            } else if group_or_condition && !valid {
                not_valid_found = true;
            }

            valid = validate_rust_version(&condition_struct);
            if group_or_condition && valid && condition_struct.rust_version.is_some() {
                return true;
            } else if !group_or_condition && !valid {
                return false;
            } else if group_or_condition && !valid {
                not_valid_found = true;
            }

            valid = validate_files_exist(&condition_struct, validate_any);
            if group_or_condition && valid && condition_struct.files_exist.is_some() {
                return true;
            } else if !group_or_condition && !valid {
                return false;
            } else if group_or_condition && !valid {
                not_valid_found = true;
            }

            valid = validate_files_not_exist(&condition_struct, validate_any);
            if group_or_condition && valid && condition_struct.files_not_exist.is_some() {
                return true;
            } else if !group_or_condition && !valid {
                return false;
            } else if group_or_condition && !valid {
                not_valid_found = true;
            }

            valid = validate_files_modified(&condition_struct);
            if !valid {
                return false;
            }

            !not_valid_found || !group_or_condition || condition_struct.files_modified.is_some()
        }
        None => true,
    }
}

pub(crate) fn get_script_text(script: &ConditionScriptValue) -> Vec<String> {
    match script {
        ConditionScriptValue::SingleLine(text) => vec![text.clone()],
        ConditionScriptValue::Text(text) => text.clone(),
    }
}

fn validate_script(
    condition_script: &Option<ConditionScriptValue>,
    script_runner: Option<String>,
    script_runner_args: Option<Vec<String>>,
) -> Result<bool, CargoMakeError> {
    match condition_script {
        Some(ref script) => {
            debug!("Checking task condition script.");

            let script_text = get_script_text(script);
            return scriptengine::invoke_script_pre_flow(
                &ScriptValue::Text(script_text),
                script_runner,
                script_runner_args,
                None,
                false,
                &vec![],
            );
        }
        None => Ok(true),
    }
}

pub(crate) fn validate_conditions_without_context(condition: TaskCondition) -> bool {
    validate_criteria(None, &Some(condition))
}

pub(crate) fn validate_conditions(
    flow_info: &FlowInfo,
    condition: &Option<TaskCondition>,
    condition_script: &Option<ConditionScriptValue>,
    script_runner: Option<String>,
    script_runner_args: Option<Vec<String>>,
) -> Result<bool, CargoMakeError> {
    let condition_type = match condition {
        Some(ref value) => value.get_condition_type(),
        None => ConditionType::And,
    };

    let criteria_passed = validate_criteria(Some(&flow_info), &condition);
    if !criteria_passed && condition_type == ConditionType::And {
        Ok(false)
    } else if criteria_passed && condition.is_some() && condition_type != ConditionType::And {
        Ok(true)
    } else {
        if condition_script.is_none() && !criteria_passed {
            Ok(false)
        } else {
            validate_script(&condition_script, script_runner, script_runner_args)
        }
    }
}

pub(crate) fn validate_condition_for_step(
    flow_info: &FlowInfo,
    step: &Step,
) -> Result<bool, CargoMakeError> {
    validate_conditions(
        &flow_info,
        &step.config.condition,
        &step.config.condition_script,
        step.config.script_runner.clone(),
        step.config.condition_script_runner_args.clone(),
    )
}
