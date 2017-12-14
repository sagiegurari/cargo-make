//! # env
//!
//! Sets up the env vars before running the tasks.
//!

mod gitinfo;
pub(crate) mod crateinfo;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use command;
use rust_info;
use rust_info::types::{RustChannel, RustInfo};
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use types::{Config, CrateInfo, EnvInfo, EnvValue, EnvValueInfo, GitInfo, PackageInfo, Workspace};

fn evaluate_env_value(env_value: &EnvValueInfo) -> String {
    match command::run_script_get_output(&env_value.script, None, true) {
        Ok(output) => {
            let exit_code = output.0;
            let stdout = output.1;

            command::validate_exit_code(exit_code);

            if exit_code == 0 {
                let mut lines: Vec<&str> = stdout.split("\n").collect();
                lines.retain(|&line| line.len() > 0);

                if lines.len() > 0 {
                    let line = lines[lines.len() - 1].to_string();

                    let line_str = str::replace(&line, "\r", "");

                    line_str.to_string()
                } else {
                    "".to_string()
                }
            } else {
                "".to_string()
            }
        }
        _ => "".to_string(),
    }
}

fn evaluate_and_set_env(
    key: &str,
    value: &str,
) {
    let mut value_string = value.to_string();
    let env_value = match value_string.find("${") {
        Some(_) => {
            for (existing_key, existing_value) in env::vars() {
                let mut key_pattern = "${".to_string();
                key_pattern.push_str(&existing_key);
                key_pattern.push_str("}");

                value_string = str::replace(&value_string, &key_pattern, &existing_value);
            }

            value_string.as_str()
        }
        None => value,
    };

    env::set_var(&key, &env_value);
}

fn set_env_for_info(
    key: &str,
    env_value: &EnvValueInfo,
) {
    let value = evaluate_env_value(&env_value);

    evaluate_and_set_env(&key, &value);
}

/// Updates the env based on the provided data
pub(crate) fn set_env(env: HashMap<String, EnvValue>) {
    debug!("Setting Up Env.");

    for (key, env_value) in &env {
        debug!("Setting env: {} = {:#?}", &key, &env_value);

        match *env_value {
            EnvValue::Value(ref value) => evaluate_and_set_env(&key, value),
            EnvValue::Info(ref info) => set_env_for_info(&key, info),
        };
    }
}

/// Updates the env for the current execution based on the descriptor.
fn initialize_env(config: &Config) {
    info!("Setting Up Env.");

    set_env(config.env.clone());
}

fn setup_env_for_crate() -> CrateInfo {
    let crate_info = crateinfo::load();
    let crate_info_clone = crate_info.clone();

    let package_info = crate_info.package.unwrap_or(PackageInfo::new());

    if package_info.name.is_some() {
        let crate_name = package_info.name.unwrap();
        env::set_var("CARGO_MAKE_CRATE_NAME", &crate_name);

        let crate_fs_name = str::replace(&crate_name, "-", "_");
        env::set_var("CARGO_MAKE_CRATE_FS_NAME", &crate_fs_name);
    }

    if package_info.version.is_some() {
        env::set_var("CARGO_MAKE_CRATE_VERSION", &package_info.version.unwrap());
    }

    if package_info.description.is_some() {
        env::set_var("CARGO_MAKE_CRATE_DESCRIPTION", &package_info.description.unwrap());
    }

    if package_info.license.is_some() {
        env::set_var("CARGO_MAKE_CRATE_LICENSE", &package_info.license.unwrap());
    }

    if package_info.documentation.is_some() {
        env::set_var("CARGO_MAKE_CRATE_DOCUMENTATION", &package_info.documentation.unwrap());
    }

    if package_info.homepage.is_some() {
        env::set_var("CARGO_MAKE_CRATE_HOMEPAGE", &package_info.homepage.unwrap());
    }

    if package_info.repository.is_some() {
        env::set_var("CARGO_MAKE_CRATE_REPOSITORY", &package_info.repository.unwrap());
    }

    let has_dependencies = match crate_info.dependencies {
        Some(ref dependencies) => dependencies.len() > 0,
        None => false,
    };

    let has_dependencies_var_value = if has_dependencies {
        "TRUE"
    } else {
        "FALSE"
    };
    env::set_var("CARGO_MAKE_CRATE_HAS_DEPENDENCIES", has_dependencies_var_value);

    let is_workspace_var_value = if crate_info.workspace.is_none() {
        "FALSE"
    } else {
        "TRUE"
    };
    env::set_var("CARGO_MAKE_CRATE_IS_WORKSPACE", is_workspace_var_value);

    let workspace = crate_info.workspace.unwrap_or(Workspace::new());
    let members = workspace.members.unwrap_or(vec![]);
    let members_string = members.join(",");
    env::set_var("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS", &members_string);

    crate_info_clone
}

fn setup_env_for_git_repo() -> GitInfo {
    let git_info = gitinfo::load();
    let git_info_clone = git_info.clone();

    if git_info.branch.is_some() {
        env::set_var("CARGO_MAKE_GIT_BRANCH", &git_info.branch.unwrap());
    }

    if git_info.user_name.is_some() {
        env::set_var("CARGO_MAKE_GIT_USER_NAME", &git_info.user_name.unwrap());
    }

    if git_info.user_email.is_some() {
        env::set_var("CARGO_MAKE_GIT_USER_EMAIL", &git_info.user_email.unwrap());
    }

    git_info_clone
}

fn setup_env_for_rust() -> RustInfo {
    let rustinfo = rust_info::get();
    let rust_info_clone = rustinfo.clone();

    if rustinfo.version.is_some() {
        env::set_var("CARGO_MAKE_RUST_VERSION", &rustinfo.version.unwrap());
    }

    if rustinfo.channel.is_some() {
        let channel_option = rustinfo.channel.unwrap();

        let channel = match channel_option {
            RustChannel::Stable => "stable",
            RustChannel::Beta => "beta",
            RustChannel::Nightly => "nightly",
        };

        env::set_var("CARGO_MAKE_RUST_CHANNEL", channel.to_string());
    }

    env::set_var("CARGO_MAKE_RUST_TARGET_ARCH", &rustinfo.target_arch.unwrap_or("unknown".to_string()));
    env::set_var("CARGO_MAKE_RUST_TARGET_ENV", &rustinfo.target_env.unwrap_or("unknown".to_string()));
    env::set_var("CARGO_MAKE_RUST_TARGET_OS", &rustinfo.target_os.unwrap_or("unknown".to_string()));
    env::set_var("CARGO_MAKE_RUST_TARGET_POINTER_WIDTH", &rustinfo.target_pointer_width.unwrap_or("unknown".to_string()));
    env::set_var("CARGO_MAKE_RUST_TARGET_VENDOR", &rustinfo.target_vendor.unwrap_or("unknown".to_string()));

    rust_info_clone
}

/// Sets up the env before the tasks execution.
pub(crate) fn setup_env(
    config: &Config,
    task: &str,
) -> EnvInfo {
    initialize_env(config);

    env::set_var("CARGO_MAKE", "true");
    env::set_var("CARGO_MAKE_TASK", &task);

    // load crate info
    let crate_info = setup_env_for_crate();

    // load git info
    let git_info = setup_env_for_git_repo();

    // load rust info
    let rust_info = setup_env_for_rust();

    EnvInfo { rust_info, crate_info, git_info }
}

pub(crate) fn setup_cwd(cwd: Option<&str>) {
    let directory = cwd.unwrap_or(".");

    debug!("Changing working directory to: {}", &directory);

    let mut directory_path_buf = PathBuf::from(&directory);
    if !cfg!(windows) {
        directory_path_buf = directory_path_buf.canonicalize().unwrap_or(directory_path_buf);
    }
    let directory_path = directory_path_buf.as_path();

    match env::set_current_dir(&directory_path) {
        Err(error) => error!("Unable to set current working directory to: {} {:#?}", &directory, error),
        _ => {
            env::set_var("CARGO_MAKE_WORKING_DIRECTORY", directory_path);

            debug!("Working directory changed to: {}", &directory);
        }
    }
}

pub(crate) fn get_env(
    key: &str,
    default: &str,
) -> String {
    match env::var(key) {
        Ok(value) => value.to_string(),
        _ => default.to_string(),
    }
}
