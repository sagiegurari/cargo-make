//! # condition
//!
//! Evaluates conditions based on task configuration and current env.
//!

#[cfg(test)]
#[path = "./condition_test.rs"]
mod condition_test;

use crate::command;
use crate::profile;
use crate::types;
use crate::types::{FlowInfo, RustVersionCondition, Step, TaskCondition};
use crate::version::is_newer;
use envmnt;
use rust_info;
use rust_info::types::{RustChannel, RustInfo};

fn validate_env(condition: &TaskCondition) -> bool {
    let env = condition.env.clone();

    match env {
        Some(env_vars) => {
            let mut all_valid = true;

            for (key, current_value) in env_vars.iter() {
                if !envmnt::is_equal(key, current_value) {
                    all_valid = false;
                    break;
                }
            }

            all_valid
        }
        None => true,
    }
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

fn validate_channel(condition: &TaskCondition, flow_info: &FlowInfo) -> bool {
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

fn validate_rust_version_condition(rustinfo: RustInfo, condition: RustVersionCondition) -> bool {
    if rustinfo.version.is_some() {
        let current_version = rustinfo.version.unwrap();

        let mut valid = match condition.min {
            Some(version) => {
                version == current_version || is_newer(&version, &current_version, true)
            }
            None => true,
        };

        if valid {
            valid = match condition.max {
                Some(version) => {
                    version == current_version || is_newer(&current_version, &version, true)
                }
                None => true,
            };
        }

        if valid {
            valid = match condition.equal {
                Some(version) => version == current_version,
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

fn validate_criteria(flow_info: &FlowInfo, condition: &Option<TaskCondition>) -> bool {
    match condition {
        Some(ref condition_struct) => {
            debug!("Checking task condition structure.");

            validate_platform(&condition_struct)
                && validate_profile(&condition_struct)
                && validate_channel(&condition_struct, &flow_info)
                && validate_env(&condition_struct)
                && validate_env_set(&condition_struct)
                && validate_env_not_set(&condition_struct)
                && validate_env_bool(&condition_struct, true)
                && validate_env_bool(&condition_struct, false)
                && validate_rust_version(&condition_struct)
        }
        None => true,
    }
}

fn validate_script(condition_script: &Option<Vec<String>>, script_runner: Option<String>) -> bool {
    match condition_script {
        Some(ref script) => {
            debug!("Checking task condition script.");

            let exit_code =
                command::run_script_get_exit_code(&script, script_runner, &vec![], false);

            if exit_code == 0 {
                true
            } else {
                false
            }
        }
        None => true,
    }
}

pub(crate) fn validate_conditions(
    flow_info: &FlowInfo,
    condition: &Option<TaskCondition>,
    condition_script: &Option<Vec<String>>,
    script_runner: Option<String>,
) -> bool {
    validate_criteria(&flow_info, &condition) && validate_script(&condition_script, script_runner)
}

pub(crate) fn validate_condition_for_step(flow_info: &FlowInfo, step: &Step) -> bool {
    validate_conditions(
        &flow_info,
        &step.config.condition,
        &step.config.condition_script,
        step.config.script_runner.clone(),
    )
}
