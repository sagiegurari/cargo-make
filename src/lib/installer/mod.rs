//! # installer
//!
//! Installs external dependencies for tasks.<br>
//! There are 2 types of dependencies: install_crate, install_script.<br>
//! install_crate ensures the crate command is available and if not installs the crate based on the provided name.<br>
//! install_script always gets executed before the task command.
//!

pub(crate) mod cargo_plugin_installer;
mod crate_installer;
pub(crate) mod crate_version_check;
pub(crate) mod rustup_component_installer;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use crate::scriptengine;
use crate::types::{FlowInfo, InstallCrate, ScriptValue, Task};

fn get_cargo_plugin_info_from_command(task_config: &Task) -> Option<(String, String)> {
    match task_config.command {
        Some(ref command) => {
            if command == "cargo" {
                match task_config.args {
                    Some(ref args) => {
                        if args.len() > 0 {
                            // create crate name
                            let mut crate_name = "cargo-".to_string();
                            crate_name = crate_name + &args[0];

                            Some((args[0].clone(), crate_name))
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            } else {
                None
            }
        }
        None => None,
    }
}

pub(crate) fn install(task_config: &Task, flow_info: &FlowInfo) {
    let validate = !task_config.should_ignore_errors();

    let toolchain = match task_config.toolchain {
        Some(ref value) => Some(value.to_string()),
        None => None,
    };

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
                    &toolchain,
                    cargo_command,
                    crate_name,
                    &task_config.install_crate_args,
                    validate,
                    &None,
                );
            }
            InstallCrate::CargoPluginInfo(ref install_info) => {
                let (cargo_command, crate_name) =
                    match get_cargo_plugin_info_from_command(&task_config) {
                        Some(cargo_plugin_info) => cargo_plugin_info,
                        None => match task_config.args {
                            Some(ref args) => match install_info.crate_name {
                                Some(ref crate_name) => {
                                    (args[0].to_string(), crate_name.to_string())
                                }
                                None => {
                                    error!("Missing crate name to invoke.");
                                    panic!("Missing crate name to invoke.");
                                }
                            },
                            None => {
                                error!("Missing cargo command to invoke.");
                                panic!("Missing cargo command to invoke.");
                            }
                        },
                    };

                cargo_plugin_installer::install_crate(
                    &toolchain,
                    &cargo_command,
                    &crate_name,
                    &task_config.install_crate_args,
                    validate,
                    &Some(install_info.min_version.clone()),
                );
            }
            InstallCrate::CrateInfo(ref install_info) => crate_installer::install(
                &toolchain,
                install_info,
                &task_config.install_crate_args,
                validate,
            ),
            InstallCrate::RustupComponentInfo(ref install_info) => {
                rustup_component_installer::install(&toolchain, install_info, validate);
            }
        },
        None => match task_config.install_script {
            Some(ref script) => {
                scriptengine::invoke_script_in_flow_context(
                    &ScriptValue::Text(script.to_vec()),
                    task_config.script_runner.clone(),
                    task_config.script_extension.clone(),
                    validate,
                    Some(flow_info),
                );
                ()
            }
            None => match get_cargo_plugin_info_from_command(&task_config) {
                Some((cargo_command, crate_name)) => {
                    cargo_plugin_installer::install_crate(
                        &toolchain,
                        &cargo_command,
                        &crate_name,
                        &task_config.install_crate_args,
                        validate,
                        &None,
                    );
                }
                None => debug!("No installation script defined."),
            },
        },
    }
}
