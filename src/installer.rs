//! # installer
//!
//! Installs external dependencies for tasks.<br>
//! There are 2 types of dependencies: install_crate, install_script.<br>
//! install_crate ensures the crate command is available and if not installs the crate based on the provided name.<br>
//! install_script always gets executed before the task command.
//!

use command;
use log::Logger;
use std::process::Command;
use types::Task;

fn is_crate_installed(
    logger: &Logger,
    crate_name: &str,
) -> bool {
    logger.verbose::<()>("Getting list of installed cargo commands.", &[], None);
    let result = Command::new("cargo").arg("--list").output();

    match result {
        Ok(output) => {
            let mut found = false;

            command::validate_exit_code(Ok(output.status));

            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.split(' ').collect();
            for mut line in lines {
                line = line.trim();

                logger.verbose::<()>("Checking: ", &[&line], None);

                if line.contains(crate_name) && crate_name.contains(line) {
                    found = true;
                    logger.verbose::<()>("Found installed crate.", &[], None);

                    break;

                }
            }

            found
        }
        Err(error) => {
            logger.error("Unable to check if crate is installed: ", &[crate_name], Some(&error));
            false
        }
    }
}

pub fn install(
    logger: &Logger,
    task_config: &Task,
) {
    match task_config.install_crate {
        Some(ref crate_name) => {
            let cargo_command = match task_config.args {
                Some(ref args) => &args[0],
                None => {
                    logger.error::<()>("Missing cargo command to invoke.", &[], None);
                    panic!("Missing cargo command to invoke.");
                }
            };

            if !is_crate_installed(&logger, cargo_command) {
                command::run_command(&logger, "cargo", &Some(vec!["install".to_string(), crate_name.to_string()]));
            }
        }
        None => {
            match task_config.install_script {
                Some(ref script) => command::run_script(&logger, &script),
                None => logger.verbose::<()>("No installation script defined.", &[], None),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use log;

    #[test]
    fn is_crate_installed_true() {
        let logger = log::create("error");
        let output = is_crate_installed(&logger, "test");
        assert!(output);
    }

    #[test]
    fn is_crate_installed_false() {
        let logger = log::create("error");
        let output = is_crate_installed(&logger, "badbadbad");
        assert!(!output);
    }

    #[test]
    fn install_empty() {
        let logger = log::create("error");
        let task = Task {
            install_crate: None,
            command: None,
            args: None,
            disabled: None,
            alias: None,
            install_script: None,
            script: None,
            dependencies: None
        };

        install(&logger, &task);
    }

    #[test]
    fn install_crate_already_installed() {
        let logger = log::create("error");
        let task = Task {
            install_crate: Some("test".to_string()),
            command: Some("cargo".to_string()),
            args: Some(vec!["test".to_string()]),
            disabled: None,
            alias: None,
            install_script: None,
            script: None,
            dependencies: None
        };

        install(&logger, &task);
    }

    #[test]
    #[should_panic]
    fn install_crate_missing_cargo_command() {
        let logger = log::create("error");
        let task = Task {
            install_crate: Some("test".to_string()),
            command: Some("cargo".to_string()),
            args: None,
            disabled: None,
            alias: None,
            install_script: None,
            script: None,
            dependencies: None
        };

        install(&logger, &task);
    }

    #[test]
    fn install_script_ok() {
        let logger = log::create("error");
        let task = Task {
            install_script: Some(vec!["exit 0".to_string()]),
            install_crate: None,
            command: None,
            args: None,
            disabled: None,
            alias: None,
            script: None,
            dependencies: None
        };

        install(&logger, &task);
    }
}
