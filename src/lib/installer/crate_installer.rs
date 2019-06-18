//! # crate_installer
//!
//! Installs crates via rustup/cargo.
//!

#[cfg(test)]
#[path = "./crate_installer_test.rs"]
mod crate_installer_test;

use crate::command;
use crate::installer::{cargo_plugin_installer, rustup_component_installer};
use crate::toolchain::wrap_command;
use crate::types::{CommandSpec, InstallCrateInfo, InstallRustupComponentInfo};

fn invoke_rustup_install(toolchain: &Option<String>, info: &InstallCrateInfo) -> bool {
    match info.rustup_component_name {
        Some(ref component) => {
            let rustup_component_info = InstallRustupComponentInfo {
                rustup_component_name: component.to_string(),
                binary: Some(info.binary.clone()),
                // InstallRustupComponentInfo only supports one argument right now.
                test_arg: info.test_arg.get(0).cloned(),
            };
            rustup_component_installer::invoke_rustup_install(&toolchain, &rustup_component_info)
        }
        None => false,
    }
}

fn invoke_cargo_install(
    toolchain: &Option<String>,
    info: &InstallCrateInfo,
    args: &Option<Vec<String>>,
    validate: bool,
) {
    let install_args =
        cargo_plugin_installer::get_install_crate_args(&info.crate_name, true, &args);

    let command_spec = match toolchain {
        Some(ref toolchain_string) => wrap_command(toolchain_string, "cargo", &Some(install_args)),
        None => CommandSpec {
            command: "cargo".to_string(),
            args: Some(install_args),
        },
    };

    command::run_command(&command_spec.command, &command_spec.args, validate);
}

pub(crate) fn install(
    toolchain: &Option<String>,
    info: &InstallCrateInfo,
    args: &Option<Vec<String>>,
    validate: bool,
) {
    if !rustup_component_installer::is_installed(&toolchain, &info.binary, &info.test_arg) {
        debug!("Crate: {} not installed.", &info.crate_name);

        if !invoke_rustup_install(&toolchain, &info) {
            invoke_cargo_install(&toolchain, &info, &args, validate);
        }
    }
}
