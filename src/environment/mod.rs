//! # env
//!
//! Sets up the env vars before running the tasks.
//!

mod gitinfo;
mod rustinfo;
pub mod crateinfo;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use std::collections::HashMap;
use std::env;
use std::path::PathBuf;
use types::{Config, CrateInfo, EnvInfo, GitInfo, PackageInfo, RustChannel, RustInfo, Workspace};

/// Updates the env based on the provided data
pub fn set_env(env: HashMap<String, String>) {
    debug!("Setting Up Env.");

    for (key, value) in &env {
        debug!("Setting env: {} = {}", &key, &value);
        env::set_var(&key, &value);
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
    let rust_info = rustinfo::load();
    let rust_info_clone = rust_info.clone();

    if rust_info.version.is_some() {
        env::set_var("CARGO_MAKE_RUST_VERSION", &rust_info.version.unwrap());
    }

    if rust_info.channel.is_some() {
        let channel_option = rust_info.channel.unwrap();

        let channel = match channel_option {
            RustChannel::Stable => "stable",
            RustChannel::Beta => "beta",
            RustChannel::Nightly => "nightly",
        };

        env::set_var("CARGO_MAKE_RUST_CHANNEL", channel.to_string());
    }

    if rust_info.target_arch.is_some() {
        env::set_var("CARGO_MAKE_RUST_TARGET_ARCH", &rust_info.target_arch.unwrap());
    }

    if rust_info.target_env.is_some() {
        env::set_var("CARGO_MAKE_RUST_TARGET_ENV", &rust_info.target_env.unwrap());
    }

    if rust_info.target_os.is_some() {
        env::set_var("CARGO_MAKE_RUST_TARGET_OS", &rust_info.target_os.unwrap());
    }

    if rust_info.target_pointer_width.is_some() {
        env::set_var("CARGO_MAKE_RUST_TARGET_POINTER_WIDTH", &rust_info.target_pointer_width.unwrap());
    }

    if rust_info.target_vendor.is_some() {
        env::set_var("CARGO_MAKE_RUST_TARGET_VENDOR", &rust_info.target_vendor.unwrap());
    }

    rust_info_clone
}

/// Sets up the env before the tasks execution.
pub fn setup_env(
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

pub fn setup_cwd(cwd: Option<&str>) {
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
