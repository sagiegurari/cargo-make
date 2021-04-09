//! # crate_installer
//!
//! Installs crates via rustup/cargo.
//!

#[cfg(test)]
#[path = "crate_installer_test.rs"]
mod crate_installer_test;

use crate::command;
use crate::installer::crate_version_check;
use crate::installer::{cargo_plugin_installer, rustup_component_installer};
use crate::toolchain::wrap_command;
use crate::types::{CommandSpec, InstallCrateInfo, InstallRustupComponentInfo};

fn invoke_rustup_install(toolchain: &Option<String>, info: &InstallCrateInfo) -> bool {
    match info.rustup_component_name {
        Some(ref component) => {
            let rustup_component_info = InstallRustupComponentInfo {
                rustup_component_name: component.to_string(),
                binary: Some(info.binary.clone()),
                test_arg: Some(info.test_arg.clone()),
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
    let (automatic_lock_version, version_option) = if info.min_version.is_some() {
        (false, &info.min_version)
    } else {
        (info.version.is_some(), &info.version)
    };

    let remove_lock =
        if automatic_lock_version && !envmnt::is("CARGO_MAKE_CRATE_INSTALLATION_LOCKED") {
            envmnt::set_bool("CARGO_MAKE_CRATE_INSTALLATION_LOCKED", true);
            true
        } else {
            false
        };

    let install_args = cargo_plugin_installer::get_install_crate_args(
        &info.crate_name,
        true,
        &args,
        version_option,
    );

    let command_spec = match toolchain {
        Some(ref toolchain_string) => wrap_command(toolchain_string, "cargo", &Some(install_args)),
        None => CommandSpec {
            command: "cargo".to_string(),
            args: Some(install_args),
        },
    };

    command::run_command(&command_spec.command, &command_spec.args, validate);

    if remove_lock {
        envmnt::remove("CARGO_MAKE_CRATE_INSTALLATION_LOCKED");
    }
}

fn is_crate_only_info(info: &InstallCrateInfo) -> bool {
    match info.rustup_component_name {
        Some(_) => false,
        None => true,
    }
}

pub(crate) fn install(
    toolchain: &Option<String>,
    info: &InstallCrateInfo,
    args: &Option<Vec<String>>,
    validate: bool,
) {
    let installed =
        rustup_component_installer::is_installed(&toolchain, &info.binary, &info.test_arg);
    let crate_only_info = is_crate_only_info(&info);
    let run_installation = if !installed {
        true
    } else if crate_only_info && toolchain.is_none() {
        match info.min_version {
            Some(ref version) => {
                !crate_version_check::is_min_version_valid(&info.crate_name, version)
            }
            None => match info.version {
                Some(ref version) => {
                    !crate_version_check::is_version_valid(&info.crate_name, version)
                }
                None => false,
            },
        }
    } else {
        false
    };

    if run_installation {
        debug!("Crate: {} not installed.", &info.crate_name);

        if !invoke_rustup_install(&toolchain, &info) {
            invoke_cargo_install(&toolchain, &info, &args, validate);
        }
    }
}
