//! # env
//!
//! Sets up the env vars before running the tasks.
//!

mod gitinfo;
mod rustinfo;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use self::rustinfo::Channel;
use log::Logger;
use std::env;
use std::path::PathBuf;
use types::{Config, CrateInfo, PackageInfo};

/// Updates the env for the current execution based on the descriptor.
fn set_env(
    logger: &Logger,
    config: &Config,
) {
    logger.info::<()>("Setting Up Env.", &[], None);

    for (key, value) in &config.env {
        logger.verbose::<()>("Setting env: ", &[&key, "=", &value], None);
        env::set_var(&key, &value);
    }
}

fn setup_env_for_crate(logger: &Logger) {
    let crate_info = CrateInfo::load(&logger);

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
}

fn setup_env_for_git_repo(logger: &Logger) {
    let git_info = gitinfo::load(&logger);

    if git_info.branch.is_some() {
        env::set_var("CARGO_MAKE_GIT_BRANCH", &git_info.branch.unwrap());
    }

    if git_info.user_name.is_some() {
        env::set_var("CARGO_MAKE_GIT_USER_NAME", &git_info.user_name.unwrap());
    }

    if git_info.user_email.is_some() {
        env::set_var("CARGO_MAKE_GIT_USER_EMAIL", &git_info.user_email.unwrap());
    }
}

fn setup_env_for_rust(logger: &Logger) {
    let rust_info = rustinfo::load(&logger);

    if rust_info.version.is_some() {
        env::set_var("CARGO_MAKE_RUST_VERSION", &rust_info.version.unwrap());
    }

    if rust_info.channel.is_some() {
        let channel_option = rust_info.channel.unwrap();

        let channel = match channel_option {
            Channel::Stable => "stable",
            Channel::Beta => "beta",
            Channel::Nightly => "nightly",
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
}

/// Sets up the env before the tasks execution.
pub fn setup_env(
    logger: &Logger,
    config: &Config,
    task: &str,
) {
    set_env(logger, config);

    env::set_var("CARGO_MAKE", "true");
    env::set_var("CARGO_MAKE_TASK", &task);

    let log_level = if logger.is_verbose_enabled() {
        "verbose"
    } else if logger.is_info_enabled() {
        "info"
    } else {
        "error"
    };

    env::set_var("CARGO_MAKE_LOG_LEVEL", log_level);

    // load crate info
    setup_env_for_crate(&logger);

    // load git info
    setup_env_for_git_repo(&logger);

    // load rust info
    setup_env_for_rust(&logger);
}

pub fn setup_cwd(
    logger: &Logger,
    cwd: Option<&str>,
) {
    let directory = cwd.unwrap_or(".");

    logger.verbose::<()>("Changing working directory to: ", &[&directory], None);

    let mut directory_path_buf = PathBuf::from(&directory);
    directory_path_buf = directory_path_buf.canonicalize().unwrap_or(directory_path_buf);
    let directory_path = directory_path_buf.as_path();

    match env::set_current_dir(&directory_path) {
        Err(error) => logger.error("Unable to set current working directory to: ", &[&directory], Some(error)),
        _ => {
            env::set_var("CARGO_MAKE_WORKING_DIRECTORY", directory_path);

            logger.verbose::<()>("Working directory changed to: ", &[&directory], None);
        }
    }
}
