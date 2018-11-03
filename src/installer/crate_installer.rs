//! # crate_installer
//!
//! Installs crates via rustup/cargo.
//!

#[cfg(test)]
#[path = "./crate_installer_test.rs"]
mod crate_installer_test;

use crate::command;
use crate::installer::cargo_plugin_installer;
use crate::types::InstallCrateInfo;
use std::process::Command;

fn is_crate_installed(binary: &str, test_arg: &str) -> bool {
    let result = Command::new(binary).arg(test_arg).output();

    match result {
        Ok(output) => {
            let exit_code = command::get_exit_code(Ok(output.status), false);

            if exit_code != 0 {
                false
            } else {
                true
            }
        }
        Err(error) => {
            debug!(
                "Unable to check if crate is installed: {} {:#?}",
                binary, &error
            );
            false
        }
    }
}

fn invoke_rustup_install(info: &InstallCrateInfo) -> bool {
    match info.rustup_component_name {
        Some(ref component) => {
            let result = Command::new("rustup")
                .arg("component")
                .arg("add")
                .arg(&component)
                .output();

            match result {
                Ok(output) => {
                    let exit_code = command::get_exit_code(Ok(output.status), false);

                    if exit_code != 0 {
                        debug!("Failed to add component: {} via rustup", &component);

                        false
                    } else {
                        debug!("Component: {} added via rustup", &component);

                        true
                    }
                }
                Err(error) => {
                    debug!(
                        "Failed to add component: {} via rustup, error: {:#?}",
                        &component, &error
                    );
                    false
                }
            }
        }
        None => false,
    }
}

fn invoke_cargo_install(info: &InstallCrateInfo, args: &Option<Vec<String>>, validate: bool) {
    let install_args =
        cargo_plugin_installer::get_install_crate_args(&info.crate_name, true, &args);

    command::run_command("cargo", &Some(install_args), validate);
}

pub(crate) fn install_crate(info: &InstallCrateInfo, args: &Option<Vec<String>>, validate: bool) {
    if !is_crate_installed(&info.binary, &info.test_arg) {
        debug!("Crate: {} not installed.", &info.crate_name);

        if !invoke_rustup_install(&info) {
            invoke_cargo_install(&info, &args, validate);
        }
    }
}
