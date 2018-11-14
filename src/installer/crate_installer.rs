//! # crate_installer
//!
//! Installs crates via rustup/cargo.
//!

#[cfg(test)]
#[path = "./crate_installer_test.rs"]
mod crate_installer_test;

use crate::command;
use crate::installer::{cargo_plugin_installer, rustup_component_installer};
use crate::types::{InstallCrateInfo, InstallRustupComponentInfo};

fn invoke_rustup_install(info: &InstallCrateInfo) -> bool {
    match info.rustup_component_name {
        Some(ref component) => {
            let rustup_component_info = InstallRustupComponentInfo {
                rustup_component_name: component.to_string(),
                binary: Some(info.binary.clone()),
                test_arg: Some(info.test_arg.clone()),
            };
            rustup_component_installer::invoke_rustup_install(&rustup_component_info)
        }
        None => false,
    }
}

fn invoke_cargo_install(info: &InstallCrateInfo, args: &Option<Vec<String>>, validate: bool) {
    let install_args =
        cargo_plugin_installer::get_install_crate_args(&info.crate_name, true, &args);

    command::run_command("cargo", &Some(install_args), validate);
}

pub(crate) fn install(info: &InstallCrateInfo, args: &Option<Vec<String>>, validate: bool) {
    if !rustup_component_installer::is_installed(&info.binary, &info.test_arg) {
        debug!("Crate: {} not installed.", &info.crate_name);

        if !invoke_rustup_install(&info) {
            invoke_cargo_install(&info, &args, validate);
        }
    }
}
