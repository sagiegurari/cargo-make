//! # env
//!
//! Sets up the env vars before running the tasks.
//!

pub(crate) mod crateinfo;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use crate::command;
use crate::condition;
use crate::profile;
use crate::scriptengine;
use crate::types::{
    CliArgs, Config, CrateInfo, EnvFile, EnvInfo, EnvValue, EnvValueConditioned, EnvValueDecode,
    EnvValueScript, PackageInfo, ScriptValue, Step, Task, Workspace,
};
use ci_info::types::CiInfo;
use duckscript;
use duckscriptsdk;
use envmnt;
use envmnt::{ExpandOptions, ExpansionType};
use fsio::path::from_path::FromPath;
use git_info;
use git_info::types::GitInfo;
use indexmap::IndexMap;
use rust_info;
use rust_info::types::{RustChannel, RustInfo};
use std::env;
use std::path::{Path, PathBuf};

fn evaluate_env_value(key: &str, env_value: &EnvValueScript) -> String {
    match command::run_script_get_output(&env_value.script, None, &vec![], true, Some(false)) {
        Ok(output) => {
            let exit_code = output.0;
            let stdout = output.1;
            let stderr = output.2;

            if exit_code != 0 {
                error!(
                    concat!(
                        "Error while evaluating script for env: {}, exit code: {}\n",
                        "Script:\n{:#?}\n",
                        "Stdout:\n{}\n",
                        "Stderr:\n{}\n"
                    ),
                    key, exit_code, env_value.script, &stdout, &stderr
                );
            }

            debug!("Env script stdout:\n{}", &stdout);

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
        }
        _ => "".to_string(),
    }
}

pub(crate) fn expand_value(value: &str) -> String {
    let mut options = ExpandOptions::new();
    options.expansion_type = Some(ExpansionType::UnixBrackets);
    options.default_to_empty = false;

    envmnt::expand(&value, Some(options))
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

fn set_env_for_list(key: &str, list: &Vec<String>) {
    let mut expanded_list = vec![];

    for value in list {
        let env_value = expand_value(&value);
        expanded_list.push(env_value);
    }

    envmnt::set_list(&key, &expanded_list);
}

fn set_env_for_script(key: &str, env_value: &EnvValueScript) {
    let value = evaluate_env_value(&key, &env_value);

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

fn set_env_for_conditional_value(key: &str, conditional_value: &EnvValueConditioned) {
    let valid = match conditional_value.condition {
        Some(ref condition) => condition::validate_conditions_without_context(condition.clone()),
        None => true,
    };

    if valid {
        let value = expand_value(&conditional_value.value);

        evaluate_and_set_env(&key, &value);
    }
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
            EnvValue::Number(value) => evaluate_and_set_env(&key, &value.to_string()),
            EnvValue::List(ref value) => set_env_for_list(&key, value),
            EnvValue::Script(ref script_info) => set_env_for_script(&key, script_info),
            EnvValue::Decode(ref decode_info) => set_env_for_decode_info(&key, decode_info),
            EnvValue::Conditional(ref conditioned_value) => {
                set_env_for_conditional_value(&key, conditioned_value)
            }
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

pub(crate) fn set_env_files(env_files: Vec<EnvFile>) {
    set_env_files_for_config(env_files, None);
}

fn set_env_files_for_config(
    env_files: Vec<EnvFile>,
    additional_profiles: Option<&Vec<String>>,
) -> bool {
    let mut all_loaded = true;
    for env_file in env_files {
        let loaded = match env_file {
            EnvFile::Path(file) => load_env_file(Some(file)),
            EnvFile::Info(info) => {
                let is_valid_profile = match info.profile {
                    Some(profile_name) => {
                        let current_profile_name = profile::get();

                        let found = match additional_profiles {
                            Some(profiles) => profiles.contains(&profile_name),
                            None => false,
                        };

                        current_profile_name == profile_name || found
                    }
                    None => true,
                };

                if is_valid_profile {
                    load_env_file_with_base_directory(Some(info.path), info.base_path)
                } else {
                    false
                }
            }
        };

        all_loaded = all_loaded && loaded;
    }

    all_loaded
}

fn set_env_scripts(env_scripts: Vec<String>, cli_arguments: &Vec<String>) {
    for env_script in env_scripts {
        if !env_script.is_empty() {
            scriptengine::invoke_script_pre_flow(
                &ScriptValue::Text(vec![env_script]),
                None,
                None,
                None,
                true,
                cli_arguments,
            );
        }
    }
}

pub(crate) fn set_current_task_meta_info_env(env: IndexMap<String, EnvValue>) {
    debug!("Setting Up Env.");

    for (key, env_value) in &env {
        if key.starts_with("CARGO_MAKE_CURRENT_TASK_") {
            debug!("Setting env: {} = {:#?}", &key, &env_value);

            match *env_value {
                EnvValue::Value(ref value) => evaluate_and_set_env(&key, value),
                _ => (),
            };
        }
    }
}

/// Updates the env for the current execution based on the descriptor.
fn initialize_env(config: &Config, cli_args: &Vec<String>) {
    debug!("Initializing Env.");

    let additional_profiles = match config.config.additional_profiles {
        Some(ref profiles) => Some(profiles),
        None => None,
    };

    set_env_files_for_config(config.env_files.clone(), additional_profiles);

    set_env_for_config(config.env.clone(), additional_profiles, true);

    set_env_scripts(config.env_scripts.clone(), cli_args);
}

fn setup_env_for_duckscript() {
    let mut version = duckscript::version();
    envmnt::set("CARGO_MAKE_DUCKSCRIPT_VERSION", version);

    version = duckscriptsdk::version();
    envmnt::set("CARGO_MAKE_DUCKSCRIPT_SDK_VERSION", version);
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

    let is_workspace = !crate_info.workspace.is_none();
    envmnt::set_bool("CARGO_MAKE_CRATE_IS_WORKSPACE", is_workspace);
    if is_workspace {
        envmnt::set_bool("CARGO_MAKE_USE_WORKSPACE_PROFILE", true);
    } else if !envmnt::exists("CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER")
        || envmnt::exists("CARGO_MAKE_WORKSPACE_EMULATION_ROOT_DIRECTORY")
    {
        // in case we started the build directly from a workspace member
        // or we are running in a workspace emulation mode, lets
        // search for the workspace root (if any)
        search_and_set_workspace_cwd();
    }

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
    let info = git_info::get();
    let git_info_clone = info.clone();

    envmnt::set_optional("CARGO_MAKE_GIT_BRANCH", &info.current_branch);
    envmnt::set_optional("CARGO_MAKE_GIT_USER_NAME", &info.user_name);
    envmnt::set_optional("CARGO_MAKE_GIT_USER_EMAIL", &info.user_email);
    envmnt::set_optional(
        "CARGO_MAKE_GIT_HEAD_LAST_COMMIT_HASH",
        &info.head.last_commit_hash,
    );
    envmnt::set_optional(
        "CARGO_MAKE_GIT_HEAD_LAST_COMMIT_HASH_PREFIX",
        &info.head.last_commit_hash_short,
    );

    git_info_clone
}

fn setup_env_for_rust(home: Option<PathBuf>) -> RustInfo {
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
    envmnt::set_optional("CARGO_MAKE_RUST_TARGET_TRIPLE", &rustinfo.target_triple);
    envmnt::set_or_remove(
        "CARGO_MAKE_CRATE_TARGET_TRIPLE",
        &crateinfo::crate_target_triple(rustinfo.target_triple, home),
    );

    rust_info_clone
}

fn setup_env_for_ci() -> CiInfo {
    let ci_info_struct = ci_info::get();

    envmnt::set_bool("CARGO_MAKE_CI", ci_info_struct.ci);
    envmnt::set_bool("CARGO_MAKE_PR", ci_info_struct.pr.unwrap_or(false));
    envmnt::set_optional("CARGO_MAKE_CI_BRANCH_NAME", &ci_info_struct.branch_name);
    envmnt::set_optional("CARGO_MAKE_CI_VENDOR", &ci_info_struct.name);

    ci_info_struct
}

fn get_base_directory_name() -> Option<String> {
    match env::current_dir() {
        Ok(path) => match path.file_name() {
            Some(name) => Some(name.to_string_lossy().into_owned()),
            None => None,
        },
        _ => None,
    }
}

fn setup_env_for_project(config: &Config, crate_info: &CrateInfo) {
    let project_name = match crate_info.package {
        Some(ref package) => match package.name {
            Some(ref name) => Some(name.to_string()),
            None => get_base_directory_name(),
        },
        None => get_base_directory_name(),
    };

    envmnt::set_or_remove("CARGO_MAKE_PROJECT_NAME", &project_name);

    let project_version = match crate_info.workspace {
        Some(_) => {
            let main_member = match config.config.main_project_member {
                Some(ref name) => Some(name.to_string()),
                None => match project_name {
                    Some(name) => Some(name),
                    None => None,
                },
            };

            envmnt::set_or_remove("CARGO_MAKE_PROJECT_VERSION_MEMBER", &main_member);

            match main_member {
                Some(member) => {
                    let mut path = PathBuf::new();
                    path.push(member);
                    path.push("Cargo.toml");
                    let member_crate_info = crateinfo::load_from(path);

                    match member_crate_info.package {
                        Some(package) => package.version,
                        None => None,
                    }
                }
                None => None,
            }
        }
        None => match crate_info.package {
            Some(ref package) => package.version.clone(),
            None => None,
        },
    };

    envmnt::set_or_remove("CARGO_MAKE_PROJECT_VERSION", &project_version);
}

/// Sets up the env before the tasks execution.
pub(crate) fn setup_env(
    cli_args: &CliArgs,
    config: &Config,
    task: &str,
    home: Option<PathBuf>,
) -> EnvInfo {
    envmnt::set_bool("CARGO_MAKE", true);
    envmnt::set("CARGO_MAKE_TASK", &task);

    envmnt::set("CARGO_MAKE_COMMAND", &cli_args.command);

    let task_arguments = match cli_args.arguments.clone() {
        Some(args) => args,
        None => vec![],
    };
    envmnt::set_list("CARGO_MAKE_TASK_ARGS", &task_arguments);

    // load duckscript_info
    setup_env_for_duckscript();

    // load crate info
    let crate_info = setup_env_for_crate();

    // load git info
    let gitinfo = setup_env_for_git_repo();

    // load rust info
    let rustinfo = setup_env_for_rust(home);

    // load CI info
    let ci_info_struct = setup_env_for_ci();

    // setup project info
    setup_env_for_project(config, &crate_info);

    // load env vars
    initialize_env(config, &cli_args.arguments.clone().unwrap_or(vec![]));

    EnvInfo {
        rust_info: rustinfo,
        crate_info,
        git_info: gitinfo,
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

fn set_workspace_cwd(directory_path: &Path, force: bool) {
    if force || !envmnt::exists("CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY") {
        let directory_path_string: String = FromPath::from_path(directory_path);

        envmnt::set(
            "CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY",
            directory_path_string,
        );
    }
}

pub(crate) fn search_and_set_workspace_cwd() {
    match crateinfo::search_workspace_root() {
        Some(root_directory) => {
            let root_directory_path_buf = get_directory_path(Some(&root_directory));
            let root_directory_path = root_directory_path_buf.as_path();
            set_workspace_cwd(&root_directory_path, true);
        }
        None => (),
    }
}

fn get_directory_path(path_option: Option<&str>) -> PathBuf {
    let cwd_str = path_option.unwrap_or(".");
    let directory = expand_value(cwd_str);

    let mut directory_path_buf = PathBuf::from(&directory);
    directory_path_buf = directory_path_buf
        .canonicalize()
        .unwrap_or(directory_path_buf);

    // remove UNC path for windows
    if cfg!(windows) {
        directory_path_buf = remove_unc_prefix(&directory_path_buf);
    }

    directory_path_buf
}

pub(crate) fn setup_cwd(cwd: Option<&str>) -> Option<PathBuf> {
    let directory_path_buf = get_directory_path(cwd);
    let directory_path = directory_path_buf.as_path();

    debug!(
        "Changing working directory to: {}",
        directory_path.display()
    );

    match env::set_current_dir(&directory_path) {
        Err(error) => {
            error!(
                "Unable to set current working directory to: {} {:#?}",
                directory_path.display(),
                error
            );
            None
        }
        _ => {
            envmnt::set("CARGO_MAKE_WORKING_DIRECTORY", &directory_path);

            set_workspace_cwd(&directory_path, false);

            debug!("Working directory changed to: {}", directory_path.display());

            let home = home::cargo_home_with_cwd(directory_path).ok();

            envmnt::set_optional("CARGO_MAKE_CARGO_HOME", &home);
            home
        }
    }
}

pub(crate) fn load_env_file(env_file: Option<String>) -> bool {
    load_env_file_with_base_directory(env_file, None)
}

pub(crate) fn load_env_file_with_base_directory(
    env_file: Option<String>,
    base_directory: Option<String>,
) -> bool {
    match env_file {
        Some(file_name) => {
            let file_path = if file_name.starts_with(".") {
                let (base_path, check_relative_path) = match base_directory {
                    Some(file) => (file, true),
                    None => (envmnt::get_or("CARGO_MAKE_WORKING_DIRECTORY", "."), false),
                };

                if check_relative_path && base_path.starts_with(".") {
                    Path::new(&envmnt::get_or("CARGO_MAKE_WORKING_DIRECTORY", "."))
                        .join(&base_path)
                        .join(file_name)
                } else {
                    Path::new(&base_path).join(file_name)
                }
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

fn expand_env_for_script_runner_arguments(task: &mut Task) {
    let updated_args = match task.script_runner_args {
        Some(ref args) => {
            let mut expanded_args = vec![];

            for index in 0..args.len() {
                expanded_args.push(expand_value(&args[index]));
            }

            Some(expanded_args)
        }
        None => None,
    };

    task.script_runner_args = updated_args;
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
    expand_env_for_script_runner_arguments(&mut config);

    Step {
        name: step.name.clone(),
        config,
    }
}
