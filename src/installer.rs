//! # installer
//!
//! Installs external dependencies for tasks.<br>
//! There are 2 types of dependencies: install_crate, install_script.<br>
//! install_crate ensures the crate command is available and if not installs the crate based on the provided name.<br>
//! install_script always gets executed before the task command.
//!

#[cfg(test)]
#[path = "./installer_test.rs"]
mod installer_test;

use command;
use std::process::Command;
use types::Task;

fn is_crate_installed(crate_name: &str) -> bool {
    debug!("Getting list of installed cargo commands.");
    let result = Command::new("cargo").arg("--list").output();

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

fn get_install_crate_args(crate_name: &str, args: &Option<Vec<String>>) -> Vec<String> {
    let mut install_args = vec!["install".to_string()];

    match *args {
        Some(ref args_vec) => for arg in args_vec.iter() {
            install_args.push(arg.to_string());
        },
        None => debug!("No crate installation args defined."),
    };

    install_args.push(crate_name.to_string());

    install_args
}

pub(crate) fn install_crate(
    cargo_command: &str,
    crate_name: &str,
    args: &Option<Vec<String>>,
    validate: bool,
) {
    if !is_crate_installed(cargo_command) {
        let install_args = get_install_crate_args(crate_name, args);

        command::run_command("cargo", &Some(install_args), validate);
    }
}

pub(crate) fn install(task_config: &Task) {
    let validate = !task_config.is_force();

    match task_config.install_crate {
        Some(ref crate_name) => {
            let cargo_command = match task_config.args {
                Some(ref args) => &args[0],
                None => {
                    error!("Missing cargo command to invoke.");
                    panic!("Missing cargo command to invoke.");
                }
            };

            install_crate(
                cargo_command,
                crate_name,
                &task_config.install_crate_args,
                validate,
            );
        }
        None => {
            match task_config.install_script {
                Some(ref script) => {
                    command::run_script(
                        &script,
                        task_config.script_runner.clone(),
                        &vec![],
                        validate,
                    );
                    ()
                }
                None => {
                    match task_config.command {
                        Some(ref command) => {
                            if command == "cargo" {
                                match task_config.args {
                                    Some(ref args) => {
                                        // create crate name
                                        let mut crate_name = "cargo-".to_string();
                                        crate_name = crate_name + &args[0];

                                        install_crate(
                                            &args[0],
                                            &crate_name,
                                            &task_config.install_crate_args,
                                            validate,
                                        );
                                    }
                                    None => debug!("No installation script defined."),
                                }
                            }
                        }
                        None => debug!("No installation script defined."),
                    }
                }
            }
        }
    }
}
