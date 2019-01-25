//! # cargo_plugin_installer
//!
//! Installs cargo plugins automatically if needed.
//!

#[cfg(test)]
#[path = "./cargo_plugin_installer_test.rs"]
mod cargo_plugin_installer_test;

use crate::command;
use crate::toolchain::wrap_command;
use std::process::Command;

fn is_crate_installed(toolchain: &Option<String>, crate_name: &str) -> bool {
    debug!("Getting list of installed cargo commands.");

    let mut command_struct = match toolchain {
        Some(ref toolchain_string) => {
            let command_spec = wrap_command(toolchain_string, "cargo", &None);
            let mut cmd = Command::new(command_spec.command);

            let args_vec = command_spec.args.unwrap();
            for arg in args_vec.iter() {
                cmd.arg(arg);
            }

            cmd
        }
        None => Command::new("cargo"),
    };

    let result = command_struct.arg("--list").output();

    match result {
        Ok(output) => {
            let mut found = false;

            let exit_code = command::get_exit_code(Ok(output.status), false);
            command::validate_exit_code(exit_code);

            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.split(' ').collect();
            for mut line in lines {
                line = line.trim();

                debug!("Checking: {}", &line);

                if line.contains(crate_name) && crate_name.contains(line) {
                    found = true;
                    debug!("Found installed crate.");

                    break;
                }
            }

            found
        }
        Err(error) => {
            error!(
                "Unable to check if crate is installed: {} {:#?}",
                crate_name, &error
            );
            false
        }
    }
}

pub(crate) fn get_install_crate_args(
    crate_name: &str,
    force: bool,
    args: &Option<Vec<String>>,
) -> Vec<String> {
    let mut install_args = vec!["install".to_string()];

    if force {
        install_args.push("--force".to_string());
    }

    match *args {
        Some(ref args_vec) => {
            for arg in args_vec.iter() {
                install_args.push(arg.to_string());
            }
        }
        None => debug!("No crate installation args defined."),
    };

    install_args.push(crate_name.to_string());

    install_args
}

pub(crate) fn install_crate(
    toolchain: &Option<String>,
    cargo_command: &str,
    crate_name: &str,
    args: &Option<Vec<String>>,
    validate: bool,
) {
    if !is_crate_installed(&toolchain, cargo_command) {
        let install_args = get_install_crate_args(crate_name, false, args);

        match toolchain {
            Some(ref toolchain_string) => {
                let command_spec = wrap_command(&toolchain_string, "cargo", &Some(install_args));
                command::run_command(&command_spec.command, &command_spec.args, validate)
            }
            None => command::run_command("cargo", &Some(install_args), validate),
        };
    }
}
