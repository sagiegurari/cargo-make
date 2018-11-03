//! # installer
//!
//! Installs external dependencies for tasks.<br>
//! There are 2 types of dependencies: install_crate, install_script.<br>
//! install_crate ensures the crate command is available and if not installs the crate based on the provided name.<br>
//! install_script always gets executed before the task command.
//!

pub(crate) mod cargo_plugin_installer;
mod crate_installer;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use crate::command;
use crate::types::{InstallCrate, Task};

pub(crate) fn install(task_config: &Task) {
    let validate = !task_config.is_force();

    match task_config.install_crate {
        Some(ref install_crate_info) => match install_crate_info {
            InstallCrate::Value(ref crate_name) => {
                let cargo_command = match task_config.args {
                    Some(ref args) => &args[0],
                    None => {
                        error!("Missing cargo command to invoke.");
                        panic!("Missing cargo command to invoke.");
                    }
                };

                cargo_plugin_installer::install_crate(
                    cargo_command,
                    crate_name,
                    &task_config.install_crate_args,
                    validate,
                );
            }
            InstallCrate::Info(ref install_info) => crate_installer::install_crate(
                install_info,
                &task_config.install_crate_args,
                validate,
            ),
        },
        None => {
            match task_config.install_script {
                Some(ref script) => {
                    command::run_script(
                        &script,
                        task_config.script_runner.clone(),
                        &vec![],
                        validate,
                    );
                    ()
                }
                None => {
                    match task_config.command {
                        Some(ref command) => {
                            if command == "cargo" {
                                match task_config.args {
                                    Some(ref args) => {
                                        // create crate name
                                        let mut crate_name = "cargo-".to_string();
                                        crate_name = crate_name + &args[0];

                                        cargo_plugin_installer::install_crate(
                                            &args[0],
                                            &crate_name,
                                            &task_config.install_crate_args,
                                            validate,
                                        );
                                    }
                                    None => debug!("No installation script defined."),
                                }
                            }
                        }
                        None => debug!("No installation script defined."),
                    }
                }
            }
        }
    }
}
