//! # rustup_component_installer
//!
//! Installs rustup components.
//!

#[cfg(test)]
#[path = "./rustup_component_installer_test.rs"]
mod rustup_component_installer_test;

use crate::command;
use crate::types::InstallRustupComponentInfo;
use std::process::Command;

pub(crate) fn is_installed(binary: &str, test_arg: &str) -> bool {
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

pub(crate) fn invoke_rustup_install(info: &InstallRustupComponentInfo) -> bool {
    let result = Command::new("rustup")
        .arg("component")
        .arg("add")
        .arg(&info.rustup_component_name)
        .output();

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

pub(crate) fn install(info: &InstallRustupComponentInfo, validate: bool) -> bool {
    let mut installed = match info.binary {
        Some(ref binary) => match info.test_arg {
            Some(ref test_arg) => is_installed(binary, test_arg),
            None => false,
        },
        None => false,
    };

    if !installed {
        debug!(
            "Rustup Component: {} not installed.",
            &info.rustup_component_name
        );

        installed = invoke_rustup_install(&info);

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
