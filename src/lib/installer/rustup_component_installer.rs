//! # rustup_component_installer
//!
//! Installs rustup components.
//!

#[cfg(test)]
#[path = "rustup_component_installer_test.rs"]
mod rustup_component_installer_test;

use crate::command;
use crate::toolchain::wrap_command;
use crate::types::{InstallRustupComponentInfo, ToolchainSpecifier};
use std::process::Command;

pub(crate) fn is_installed(
    toolchain: &Option<ToolchainSpecifier>,
    binary: &str,
    test_args: &[String],
) -> bool {
    let mut command_struct = match toolchain {
        Some(ref toolchain_string) => {
            let command_spec = wrap_command(toolchain_string, binary, &None);
            let mut cmd = Command::new(command_spec.command);
            cmd.args(command_spec.args.unwrap());

            cmd
        }
        None => Command::new(binary),
    };

    debug!(
        "Validating installation using command: {} args: {:#?}",
        binary, &test_args
    );
    let result = command_struct.args(test_args).output();

    match result {
        Ok(output) => {
            let exit_code = command::get_exit_code(Ok(output.status), false);
            debug!("Installation validation test exit code: {}", exit_code);

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

pub(crate) fn invoke_rustup_install(
    toolchain: &Option<ToolchainSpecifier>,
    info: &InstallRustupComponentInfo,
) -> bool {
    let mut command_spec = Command::new("rustup");
    command_spec.arg("component");
    command_spec.arg("add");

    match toolchain {
        Some(ref toolchain) => {
            command_spec.arg("--toolchain");
            command_spec.arg(toolchain.channel());
        }
        None => {}
    };

    let result = command_spec.arg(&info.rustup_component_name).output();

    match result {
        Ok(output) => {
            let exit_code = command::get_exit_code(Ok(output.status), false);

            if exit_code != 0 {
                debug!(
                    "Failed to add component: {} via rustup",
                    &info.rustup_component_name
                );

                false
            } else {
                debug!(
                    "Component: {} added via rustup",
                    &info.rustup_component_name
                );

                true
            }
        }
        Err(error) => {
            debug!(
                "Failed to add component: {} via rustup, error: {:#?}",
                &info.rustup_component_name, &error
            );
            false
        }
    }
}

pub(crate) fn install(
    toolchain: &Option<ToolchainSpecifier>,
    info: &InstallRustupComponentInfo,
    validate: bool,
) -> bool {
    let mut installed = match info.binary {
        Some(ref binary) => match info.test_arg {
            Some(ref test_arg) => is_installed(&toolchain, binary, test_arg),
            None => false,
        },
        None => false,
    };

    if !installed {
        debug!(
            "Rustup Component: {} not installed.",
            &info.rustup_component_name
        );

        installed = invoke_rustup_install(&toolchain, &info);

        if validate && !installed {
            error!(
                "Failed to add rustup component: {}",
                &info.rustup_component_name
            );
        }

        installed
    } else {
        true
    }
}
