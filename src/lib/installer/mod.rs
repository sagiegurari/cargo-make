//! # installer
//!
//! Installs external dependencies for tasks.<br>
//! There are 2 types of dependencies: install_crate, install_script.<br>
//! install_crate ensures the crate command is available and if not installs the crate based on the provided name.<br>
//! install_script always gets executed before the task command.
//!

pub(crate) mod cargo_plugin_installer;
pub(crate) mod crate_installer;
pub(crate) mod crate_version_check;
pub(crate) mod rustup_component_installer;

#[cfg(test)]
#[path = "mod_test.rs"]
mod mod_test;

use crate::scriptengine;
use crate::types::{FlowInfo, FlowState, InstallCrate, Task};
use std::cell::RefCell;
use std::rc::Rc;

fn get_cargo_plugin_info_from_command(task_config: &Task) -> Option<(String, String)> {
    match task_config.command {
        Some(ref command) => {
            if command == "cargo" {
                match task_config.args {
                    Some(ref args) => {
                        if args.len() > 0 && !args[0].starts_with("-") {
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

fn get_first_command_arg(task_config: &Task) -> Option<String> {
    match task_config.args {
        Some(ref args) => {
            if args.is_empty() {
                None
            } else {
                Some(args[0].to_string())
            }
        }
        None => None,
    }
}

pub(crate) fn install(
    task_config: &Task,
    flow_info: &FlowInfo,
    flow_state: Rc<RefCell<FlowState>>,
) {
    let validate = !task_config.should_ignore_errors();

    let toolchain = task_config.toolchain.clone();

    let mut install_crate = task_config.install_crate.clone();
    if let Some(ref install_crate_value) = install_crate {
        if let InstallCrate::Enabled(enabled) = install_crate_value {
            if *enabled {
                // enabled true is the same as no install_crate defined
                install_crate = None;
            }
        }
    }

    match install_crate {
        Some(ref install_crate_info) => match install_crate_info {
            InstallCrate::Enabled(_) => (),
            InstallCrate::Value(ref crate_name) => {
                let first_arg = get_first_command_arg(task_config);
                let cargo_command = match first_arg {
                    Some(ref arg) => arg,
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
                    &None,
                );
            }
            InstallCrate::CargoPluginInfo(ref install_info) => {
                let (cargo_command, crate_name) =
                    match get_cargo_plugin_info_from_command(&task_config) {
                        Some(cargo_plugin_info) => cargo_plugin_info,
                        None => match get_first_command_arg(task_config) {
                            Some(arg) => match install_info.crate_name {
                                Some(ref crate_name) => (arg, crate_name.to_string()),
                                None => {
                                    error!("Missing crate name to invoke.");
                                    panic!("Missing crate name to invoke.");
                                }
                            },
                            None => match install_info.crate_name {
                                Some(ref crate_name) => {
                                    (crate_name.to_string(), crate_name.to_string())
                                }
                                None => {
                                    error!("Missing cargo command to invoke.");
                                    panic!("Missing crate command to invoke.");
                                }
                            },
                        },
                    };

                cargo_plugin_installer::install_crate(
                    &toolchain,
                    &cargo_command,
                    &crate_name,
                    &task_config.install_crate_args,
                    validate,
                    &install_info.min_version,
                    &install_info.install_command,
                    &install_info.force,
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
                    &script,
                    task_config.script_runner.clone(),
                    task_config.script_runner_args.clone(),
                    task_config.script_extension.clone(),
                    validate,
                    Some(flow_info),
                    Some(flow_state),
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
                        &None,
                    );
                }
                None => debug!("No installation script defined."),
            },
        },
    }
}
