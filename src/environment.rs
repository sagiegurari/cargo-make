//! # env
//!
//! Sets up the env vars before running the tasks.
//!

#[cfg(test)]
#[path = "./environment_test.rs"]
mod environment_test;

use log::Logger;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;
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
    // load crate info
    let file_path = Path::new("Cargo.toml");

    if file_path.exists() {
        logger.verbose("Opening file:", &[], Some(&file_path));
        let mut file = match File::open(&file_path) {
            Ok(value) => value,
            Err(error) => panic!("Unable to open Cargo.toml, error: {}", error),
        };
        let mut crate_info_string = String::new();
        file.read_to_string(&mut crate_info_string).unwrap();

        let crate_info: CrateInfo = match toml::from_str(&crate_info_string) {
            Ok(value) => value,
            Err(error) => panic!("Unable to parse Cargo.toml, {}", error),
        };
        logger.verbose("Loaded Cargo.toml:", &[], Some(&crate_info));

        let package_info = crate_info.package.unwrap_or(PackageInfo::new());

        if package_info.name.is_some() {
            env::set_var("CARGO_MAKE_CRATE_NAME", &package_info.name.unwrap());
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
    } else {
        logger.info::<()>("Cargo.toml not found, skipping.", &[], None);
    };
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

    // load crate info
    setup_env_for_crate(&logger);
}

pub fn setup_cwd(
    logger: &Logger,
    cwd: Option<&str>,
) {
    match cwd {
        Some(directory) => {
            let directory_path = Path::new(directory);
            logger.verbose::<()>("Changing working directory to: ", &[&directory], None);

            match env::set_current_dir(&directory_path) {
                Err(error) => logger.error("Unable to set current working directory to: ", &[&directory], Some(error)),
                _ => logger.verbose::<()>("Working directory changed to: ", &[&directory], None),
            }
        }
        None => (),
    };
}
