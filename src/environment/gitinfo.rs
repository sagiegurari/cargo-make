//! # gitinfo
//!
//! Loads git information.
//!

#[cfg(test)]
#[path = "./gitinfo_test.rs"]
mod gitinfo_test;

use command;
use log::Logger;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds git info for the given repo directory
pub struct GitInfo {
    /// branch name
    pub branch: Option<String>,
    /// user.name
    pub user_name: Option<String>,
    /// user.email
    pub user_email: Option<String>
}

impl GitInfo {
    pub fn new() -> GitInfo {
        GitInfo { branch: None, user_name: None, user_email: None }
    }
}

fn load_from_git_config(
    logger: &Logger,
    git_info: &mut GitInfo,
) {
    let result = Command::new("git").arg("config").arg("--list").output();

    match result {
        Ok(output) => {
            command::validate_exit_code(Ok(output.status), logger);

            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.split('\n').collect();
            for mut line in lines {
                line = line.trim();

                logger.verbose::<()>("Checking: ", &[&line], None);

                if line.starts_with("user.name=") {
                    let parts: Vec<&str> = line.split('=').collect();
                    let value = parts[1];
                    git_info.user_name = Some(value.to_string());
                } else if line.starts_with("user.email=") {
                    let parts: Vec<&str> = line.split('=').collect();
                    let value = parts[1];
                    git_info.user_email = Some(value.to_string());
                }
            }
        }
        Err(error) => logger.info("Error while running git config --list command.: ", &[], Some(&error)),
    };
}

fn load_branch(
    logger: &Logger,
    git_info: &mut GitInfo,
) {
    let result = Command::new("git").arg("branch").output();

    match result {
        Ok(output) => {
            command::validate_exit_code(Ok(output.status), logger);

            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.split('\n').collect();
            for mut line in lines {
                line = line.trim();

                logger.verbose::<()>("Checking: ", &[&line], None);

                if line.starts_with("*") {
                    let parts: Vec<&str> = line.split(' ').collect();
                    let value = parts[1];
                    git_info.branch = Some(value.to_string());
                }
            }
        }
        Err(error) => logger.info("Error while running git branch command.: ", &[], Some(&error)),
    };
}

pub fn load(logger: &Logger) -> GitInfo {
    logger.verbose::<()>("Searching for git info.", &[], None);

    let mut git_info = GitInfo::new();

    load_from_git_config(&logger, &mut git_info);
    load_branch(&logger, &mut git_info);

    logger.verbose("Loaded git info.", &[], Some(&git_info));

    git_info
}
