//! # env
//!
//! Sets up the env vars before running the tasks.
//!

pub(crate) mod crateinfo;
mod gitinfo;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use crate::command;
use crate::profile;
use crate::types::{
    CliArgs, Config, CrateInfo, EnvInfo, EnvValue, EnvValueDecode, EnvValueScript, GitInfo,
    PackageInfo, Step, Task, Workspace,
};
use ci_info::types::CiInfo;
use envmnt;
use indexmap::IndexMap;
use rust_info;
use rust_info::types::{RustChannel, RustInfo};
use std::env;
use std::path::{Path, PathBuf};

fn evaluate_env_value(env_value: &EnvValueScript) -> String {
    match command::run_script_get_output(&env_value.script, None, &vec![], true, Some(false)) {
        Ok(output) => {
            let exit_code = output.0;
            let stdout = output.1;

            command::validate_exit_code(exit_code);

            debug!("Env script stdout:\n{}", &stdout);

            if exit_code == 0 {
                let multi_line = match env_value.multi_line {
                    Some(bool_value) => bool_value,
                    None => false,
                };

                if multi_line {
                    stdout.to_string()
                } else {
                    let mut lines: Vec<&str> = stdout.split("\n").collect();
                    lines.retain(|&line| line.len() > 0);

                    if lines.len() > 0 {
                        let line = lines[lines.len() - 1].to_string();

                        let line_str = str::replace(&line, "\r", "");

                        line_str.to_string()
                    } else {
                        "".to_string()
                    }
                }
            } else {
                "".to_string()
            }
        }
        _ => "".to_string(),
    }
}

pub(crate) fn expand_value(value: &str) -> String {
    let mut value_string = value.to_string();

    match value_string.find("${") {
        Some(_) => {
            for (existing_key, existing_value) in env::vars() {
                let mut key_pattern = "${".to_string();
                key_pattern.push_str(&existing_key);
                key_pattern.push_str("}");

                value_string = str::replace(&value_string, &key_pattern, &existing_value);
            }

            value_string
        }
        None => value_string,
    }
}

fn evaluate_and_set_env(key: &str, value: &str) {
    let env_value = expand_value(&value);

    debug!("Setting Env: {} Value: {}", &key, &env_value);
    envmnt::set(&key, &env_value);
}

fn set_env_for_bool(key: &str, value: bool) {
    debug!("Setting Env: {} Value: {}", &key, &value);
    envmnt::set_bool(&key, value);
}

fn set_env_for_script(key: &str, env_value: &EnvValueScript) {
    let value = evaluate_env_value(&env_value);

    evaluate_and_set_env(&key, &value);
}

fn set_env_for_decode_info(key: &str, decode_info: &EnvValueDecode) {
    let source_value = expand_value(&decode_info.source);

    let mapped_value = match decode_info.mapping.get(&source_value) {
        Some(value) => value.to_string(),
        None => match decode_info.default_value {
            Some(ref value) => value.clone().to_string(),
            None => source_value.clone(),
        },
    };

    evaluate_and_set_env(&key, &mapped_value);
}

fn set_env_for_profile(
    profile_name: &str,
    sub_env: &IndexMap<String, EnvValue>,
    additional_profiles: Option<&Vec<String>>,
) {
    let current_profile_name = profile::get();
    let profile_name_string = profile_name.to_string();

    let found = match additional_profiles {
        Some(profiles) => profiles.contains(&profile_name_string),
        None => false,
    };

    if current_profile_name == profile_name_string || found {
        debug!("Setting Up Profile: {} Env.", &profile_name);

        set_env_for_config(sub_env.clone(), None, false);
    }
}

/// Updates the env based on the provided data
pub(crate) fn set_env(env: IndexMap<String, EnvValue>) {
    set_env_for_config(env, None, true)
}

fn unset_env(key: &str) {
    envmnt::remove(key);
}

/// Updates the env based on the provided data
fn set_env_for_config(
    env: IndexMap<String, EnvValue>,
    additional_profiles: Option<&Vec<String>>,
    allow_sub_env: bool,
) {
    debug!("Setting Up Env.");

    for (key, env_value) in &env {
        debug!("Setting env: {} = {:#?}", &key, &env_value);

        match *env_value {
            EnvValue::Value(ref value) => evaluate_and_set_env(&key, value),
            EnvValue::Boolean(value) => set_env_for_bool(&key, value),
            EnvValue::Script(ref script_info) => set_env_for_script(&key, script_info),
            EnvValue::Decode(ref decode_info) => set_env_for_decode_info(&key, decode_info),
            EnvValue::Profile(ref sub_env) => {
                if allow_sub_env {
                    set_env_for_profile(&key, sub_env, additional_profiles)
                }
            }
            EnvValue::Unset(ref value) => {
                if value.unset {
                    unset_env(&key);
                }
            }
        };
    }

    if allow_sub_env {
        let profile_name = profile::get();

        if env.contains_key(&profile_name) {
            match env.get(&profile_name) {
                Some(ref env_value) => {
                    match *env_value {
                        EnvValue::Profile(ref sub_env) => {
                            set_env_for_profile(&profile_name, sub_env, None)
                        }
                        _ => (),
                    };
                }
                None => (),
            };
        }
    }
}

/// Updates the env for the current execution based on the descriptor.
fn initialize_env(config: &Config) {
    debug!("Initializing Env.");

    let additional_profiles = match config.config.additional_profiles {
        Some(ref profiles) => Some(profiles),
        None => None,
    };

    set_env_for_config(config.env.clone(), additional_profiles, true);
}

fn setup_env_for_crate() -> CrateInfo {
    let crate_info = crateinfo::load();
    let crate_info_clone = crate_info.clone();

    let package_info = crate_info.package.unwrap_or(PackageInfo::new());

    if package_info.name.is_some() {
        let crate_name = package_info.name.unwrap();
        envmnt::set("CARGO_MAKE_CRATE_NAME", &crate_name);

        let crate_fs_name = str::replace(&crate_name, "-", "_");
        envmnt::set("CARGO_MAKE_CRATE_FS_NAME", &crate_fs_name);
    }

    envmnt::set_optional("CARGO_MAKE_CRATE_VERSION", &package_info.version);
    envmnt::set_optional("CARGO_MAKE_CRATE_DESCRIPTION", &package_info.description);
    envmnt::set_optional("CARGO_MAKE_CRATE_LICENSE", &package_info.license);
    envmnt::set_optional(
        "CARGO_MAKE_CRATE_DOCUMENTATION",
        &package_info.documentation,
    );
    envmnt::set_optional("CARGO_MAKE_CRATE_HOMEPAGE", &package_info.homepage);
    envmnt::set_optional("CARGO_MAKE_CRATE_REPOSITORY", &package_info.repository);

    let has_dependencies = match crate_info.dependencies {
        Some(ref dependencies) => dependencies.len() > 0,
        None => crate_info.workspace.is_some(),
    };

    envmnt::set_bool("CARGO_MAKE_CRATE_HAS_DEPENDENCIES", has_dependencies);

    let is_workspace_var_value = !crate_info.workspace.is_none();
    envmnt::set_bool("CARGO_MAKE_CRATE_IS_WORKSPACE", is_workspace_var_value);

    let workspace = crate_info.workspace.unwrap_or(Workspace::new());
    let members = workspace.members.unwrap_or(vec![]);
    let mut env_options = envmnt::ListOptions::new();
    env_options.separator = Some(",".to_string());
    envmnt::set_list_with_options("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS", &members, &env_options);

    // check if Cargo.lock file exists in working directory
    let lock_file = Path::new("Cargo.lock");
    let lock_file_exists = lock_file.exists();
    envmnt::set_bool("CARGO_MAKE_CRATE_LOCK_FILE_EXISTS", lock_file_exists);

    crate_info_clone
}

fn setup_env_for_git_repo() -> GitInfo {
    let git_info = gitinfo::load();
    let git_info_clone = git_info.clone();

    envmnt::set_optional("CARGO_MAKE_GIT_BRANCH", &git_info.branch);
    envmnt::set_optional("CARGO_MAKE_GIT_USER_NAME", &git_info.user_name);
    envmnt::set_optional("CARGO_MAKE_GIT_USER_EMAIL", &git_info.user_email);

    git_info_clone
}

fn setup_env_for_rust() -> RustInfo {
    let rustinfo = rust_info::get();
    let rust_info_clone = rustinfo.clone();

    envmnt::set_optional("CARGO_MAKE_RUST_VERSION", &rustinfo.version);

    if rustinfo.channel.is_some() {
        let channel_option = rustinfo.channel.unwrap();

        let channel = match channel_option {
            RustChannel::Stable => "stable",
            RustChannel::Beta => "beta",
            RustChannel::Nightly => "nightly",
        };

        envmnt::set("CARGO_MAKE_RUST_CHANNEL", channel);
    }

    envmnt::set(
        "CARGO_MAKE_RUST_TARGET_ARCH",
        &rustinfo.target_arch.unwrap_or("unknown".to_string()),
    );
    envmnt::set(
        "CARGO_MAKE_RUST_TARGET_ENV",
        &rustinfo.target_env.unwrap_or("unknown".to_string()),
    );
    envmnt::set(
        "CARGO_MAKE_RUST_TARGET_OS",
        &rustinfo.target_os.unwrap_or("unknown".to_string()),
    );
    envmnt::set(
        "CARGO_MAKE_RUST_TARGET_POINTER_WIDTH",
        &rustinfo
            .target_pointer_width
            .unwrap_or("unknown".to_string()),
    );
    envmnt::set(
        "CARGO_MAKE_RUST_TARGET_VENDOR",
        &rustinfo.target_vendor.unwrap_or("unknown".to_string()),
    );

    rust_info_clone
}

fn setup_env_for_ci() -> CiInfo {
    let ci_info_struct = ci_info::get();

    envmnt::set_bool("CARGO_MAKE_CI", ci_info_struct.ci);
    envmnt::set_bool("CARGO_MAKE_PR", ci_info_struct.pr.unwrap_or(false));

    ci_info_struct
}

/// Sets up the env before the tasks execution.
pub(crate) fn setup_env(cli_args: &CliArgs, config: &Config, task: &str) -> EnvInfo {
    envmnt::set("CARGO_MAKE", "true");
    envmnt::set("CARGO_MAKE_TASK", &task);

    envmnt::set("CARGO_MAKE_COMMAND", &cli_args.command);

    let task_arguments = match cli_args.arguments.clone() {
        Some(args) => args,
        None => vec![],
    };
    envmnt::set_list("CARGO_MAKE_TASK_ARGS", &task_arguments);

    envmnt::set("CARGO_MAKE_USE_WORKSPACE_PROFILE", "false");

    // load crate info
    let crate_info = setup_env_for_crate();

    // load git info
    let git_info = setup_env_for_git_repo();

    // load rust info
    let rust_info = setup_env_for_rust();

    // load CI info
    let ci_info_struct = setup_env_for_ci();

    // load env vars
    initialize_env(config);

    EnvInfo {
        rust_info,
        crate_info,
        git_info,
        ci_info: ci_info_struct,
    }
}

fn remove_unc_prefix(directory_path_buf: &PathBuf) -> PathBuf {
    let mut path_str = directory_path_buf.to_str().unwrap_or(".");

    let prefix = r"\\?\";
    if path_str.starts_with(prefix) {
        path_str = &path_str[prefix.len()..];
        PathBuf::from(path_str)
    } else {
        directory_path_buf.clone()
    }
}

pub(crate) fn setup_cwd(cwd: Option<&str>) {
    let directory = cwd.unwrap_or(".");

    debug!("Changing working directory to: {}", &directory);

    let mut directory_path_buf = PathBuf::from(&directory);
    directory_path_buf = directory_path_buf
        .canonicalize()
        .unwrap_or(directory_path_buf);

    // remove UNC path for windows
    if cfg!(windows) {
        directory_path_buf = remove_unc_prefix(&directory_path_buf);
    }

    let directory_path = directory_path_buf.as_path();

    match env::set_current_dir(&directory_path) {
        Err(error) => error!(
            "Unable to set current working directory to: {} {:#?}",
            &directory, error
        ),
        _ => {
            envmnt::set("CARGO_MAKE_WORKING_DIRECTORY", directory_path);

            debug!("Working directory changed to: {}", &directory);
        }
    }
}

pub(crate) fn load_env_file(env_file: Option<String>) -> bool {
    match env_file {
        Some(file_name) => {
            let file_path = if file_name.starts_with(".") {
                let base_path = envmnt::get_or("CARGO_MAKE_WORKING_DIRECTORY", ".");
                Path::new(&base_path).join(file_name)
            } else {
                Path::new(&file_name).to_path_buf()
            };

            match file_path.to_str() {
                Some(file_path_str) => {
                    let evaluate_env_var = |value: String| expand_value(&value);

                    match envmnt::evaluate_and_load_file(file_path_str, evaluate_env_var) {
                        Err(error) => {
                            error!(
                                "Unable to load env file: {} Error: {:#?}",
                                &file_path_str, error
                            );
                            false
                        }
                        _ => {
                            debug!("Loaded env file: {}", &file_path_str);
                            true
                        }
                    }
                }
                None => false,
            }
        }
        None => false,
    }
}

fn get_project_root_for_path(directory: &PathBuf) -> Option<String> {
    let file_path = Path::new(directory).join("Cargo.toml");

    if file_path.exists() {
        match directory.to_str() {
            Some(directory_string) => Some(directory_string.to_string()),
            _ => None,
        }
    } else {
        match directory.parent() {
            Some(parent_directory) => {
                let parent_directory_path = parent_directory.to_path_buf();
                get_project_root_for_path(&parent_directory_path)
            }
            None => None,
        }
    }
}

pub(crate) fn get_project_root() -> Option<String> {
    match env::current_dir() {
        Ok(directory) => get_project_root_for_path(&directory),
        _ => None,
    }
}

fn expand_env_for_arguments(task: &mut Task) {
    //update args by replacing any env vars
    let updated_args = match task.args {
        Some(ref args) => {
            let mut expanded_args = vec![];

            let task_args = match envmnt::get_list("CARGO_MAKE_TASK_ARGS") {
                Some(list) => list,
                None => vec![],
            };

            for index in 0..args.len() {
                if args[index].contains("${@}") {
                    if task_args.len() > 0 {
                        if args[index] == "${@}" {
                            for arg_index in 0..task_args.len() {
                                expanded_args.push(task_args[arg_index].clone());
                            }
                        } else {
                            for arg_index in 0..task_args.len() {
                                let value_string =
                                    str::replace(&args[index], "${@}", &task_args[arg_index]);
                                expanded_args.push(value_string);
                            }
                        }
                    }
                } else {
                    expanded_args.push(args[index].clone());
                }
            }

            for index in 0..expanded_args.len() {
                expanded_args[index] = expand_value(&expanded_args[index]);
            }

            Some(expanded_args)
        }
        None => None,
    };

    task.args = updated_args;
}

pub(crate) fn expand_env(step: &Step) -> Step {
    //clone data before modify
    let mut config = step.config.clone();

    //update command by replacing any env vars
    match config.command {
        Some(value) => {
            config.command = Some(expand_value(&value));
        }
        None => {}
    };

    //update args by replacing any env vars
    expand_env_for_arguments(&mut config);

    Step {
        name: step.name.clone(),
        config,
    }
}
