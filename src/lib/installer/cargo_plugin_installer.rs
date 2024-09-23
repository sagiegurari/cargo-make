//! # cargo_plugin_installer
//!
//! Installs cargo plugins automatically if needed.
//!

#[cfg(test)]
#[path = "cargo_plugin_installer_test.rs"]
mod cargo_plugin_installer_test;

use std::process::Command;
use strip_ansi_escapes::strip_str;

use crate::command;
use crate::error::CargoMakeError;
use crate::installer::crate_version_check;
use crate::toolchain::wrap_command;
use crate::types::ToolchainSpecifier;

fn is_crate_in_list_output_legacy(crate_name: &str, output: &str) -> bool {
    let lines: Vec<&str> = output.split(' ').collect();
    for mut line in lines {
        line = line.trim();

        debug!("Checking (legacy): {}", &line);

        if line.contains(crate_name) && crate_name.contains(line) {
            debug!("Found installed cratei (legacy).");

            return true;
        }
    }

    false
}

fn is_crate_in_list_output(crate_name: &str, output: &str) -> bool {
    let lines: Vec<&str> = output.split('\n').collect();
    for mut line in lines {
        line = line.trim();

        let words: Vec<&str> = line.split(' ').collect();
        let plugin_name = words[0].trim();
        let found = crate_name.eq(plugin_name);
        debug!(
            "Checking Line: {}\nPlugin: <{}> Expected: <{}> Sizes: {}/{} Found: {}",
            &line,
            &plugin_name,
            &crate_name,
            plugin_name.len(),
            crate_name.len(),
            found
        );

        if found {
            debug!("Found installed crate.");

            return true;
        }
    }

    false
}

fn is_crate_installed(
    toolchain: &Option<ToolchainSpecifier>,
    crate_name: &str,
) -> Result<bool, CargoMakeError> {
    debug!("Getting list of installed cargo commands.");

    let mut command_struct = match toolchain {
        Some(ref toolchain_string) => {
            let command_spec = wrap_command(toolchain_string, "cargo", &None);
            let mut cmd = Command::new(command_spec.command);
            cmd.args(command_spec.args.unwrap());

            cmd
        }
        None => Command::new("cargo"),
    };

    let result = command_struct.arg("--list").output();

    match result {
        Ok(output) => {
            let exit_code = command::get_exit_code(Ok(output.status), false);
            command::validate_exit_code(exit_code)?;

            let stdout = strip_str(String::from_utf8_lossy(&output.stdout));
            let crate_name_trimmed = crate_name.trim();
            Ok(is_crate_in_list_output(&crate_name_trimmed, &stdout)
                || is_crate_in_list_output_legacy(&crate_name_trimmed, &stdout))
        }
        Err(error) => {
            error!(
                "Unable to check if crate is installed: {} {:#?}",
                crate_name, &error
            );
            Ok(false)
        }
    }
}

fn should_skip_crate_name(args: &Option<Vec<String>>) -> bool {
    match *args {
        Some(ref args_vec) => args_vec.contains(&"--git".to_string()),
        None => false,
    }
}

pub(crate) fn get_install_crate_args(
    crate_name: &str,
    force: bool,
    args: &Option<Vec<String>>,
    version_option: &Option<String>,
    install_command: &Option<String>,
) -> Vec<String> {
    let install_command_str = match install_command {
        Some(value) => value.clone(),
        None => "install".to_string(),
    };
    let mut install_args = vec![install_command_str.to_string()];

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

    let skip_crate_name = should_skip_crate_name(&args);
    if !skip_crate_name {
        // add locked flags
        if let Some(version) = version_option {
            if envmnt::is("CARGO_MAKE_CRATE_INSTALLATION_LOCKED") {
                install_args.push("--locked".to_string());
                install_args.push("--version".to_string());
                install_args.push(version.to_string());
            }
        }

        install_args.push(crate_name.to_string());
    }

    install_args
}

pub(crate) fn install_crate(
    toolchain: &Option<ToolchainSpecifier>,
    cargo_command: Option<&str>,
    crate_name: &str,
    args: &Option<Vec<String>>,
    validate: bool,
    min_version: &Option<String>,
    install_command: &Option<String>,
    allow_force: &Option<bool>,
) -> Result<(), CargoMakeError> {
    let installed = match cargo_command {
        Some(cargo_command) => is_crate_installed(&toolchain, cargo_command),
        None => Ok(false),
    }?;
    let mut force = false;
    let allow_force_value = allow_force.unwrap_or(true);
    let run_installation = if !installed {
        true
    } else if toolchain.is_none() {
        match *min_version {
            Some(ref version) => {
                if crate_version_check::is_min_version_valid(&crate_name, version, None) {
                    false
                } else {
                    force = allow_force_value;
                    true
                }
            }
            None => false,
        }
    } else {
        false
    };

    if run_installation {
        let install_args =
            get_install_crate_args(crate_name, force, args, &min_version, install_command);

        match toolchain {
            Some(ref toolchain_string) => {
                let command_spec = wrap_command(&toolchain_string, "cargo", &Some(install_args));
                command::run_command(&command_spec.command, &command_spec.args, validate)?
            }
            None => command::run_command("cargo", &Some(install_args), validate)?,
        };
    }
    Ok(())
}
