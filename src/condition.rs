//! # condition
//!
//! Evaluates conditions based on task configuration and current env.
//!

#[cfg(test)]
#[path = "./condition_test.rs"]
mod condition_test;

use command;
use rust_info::types::RustChannel;
use std::env;
use types;
use types::{FlowInfo, Step, TaskCondition};

fn validate_env(condition: &TaskCondition) -> bool {
    let env = condition.env.clone();

    match env {
        Some(env_vars) => {
            let mut all_valid = true;

            for (key, current_value) in env_vars.iter() {
                match env::var(key) {
                    Ok(value) => {
                        all_valid = value == current_value.to_string();
                    }
                    _ => {
                        all_valid = false;
                    }
                };

                if !all_valid {
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
                match env::var(key) {
                    Err(_) => {
                        all_valid = false;
                    }
                    _ => (),
                };

                if !all_valid {
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
                match env::var(key) {
                    Ok(_) => {
                        all_valid = false;
                    }
                    _ => (),
                };

                if !all_valid {
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

fn validate_criteria(flow_info: &FlowInfo, step: &Step) -> bool {
    match step.config.condition {
        Some(ref condition) => {
            debug!("Checking task condition structure.");

            validate_platform(&condition) && validate_channel(&condition, &flow_info)
                && validate_env(&condition) && validate_env_set(&condition)
                && validate_env_not_set(&condition)
        }
        None => true,
    }
}

fn validate_script(step: &Step) -> bool {
    match step.config.condition_script {
        Some(ref script) => {
            debug!("Checking task condition script.");

            let exit_code =
                command::run_script(&script, step.config.script_runner.clone(), &vec![], false);

            if exit_code == 0 {
                true
            } else {
                false
            }
        }
        None => true,
    }
}

pub(crate) fn validate_condition(flow_info: &FlowInfo, step: &Step) -> bool {
    validate_criteria(&flow_info, &step) && validate_script(&step)
}
