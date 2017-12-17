//! # gitinfo
//!
//! Loads git information.
//!

#[cfg(test)]
#[path = "./gitinfo_test.rs"]
mod gitinfo_test;

use command;
use std::process::Command;
use types::GitInfo;

fn load_from_git_config(git_info: &mut GitInfo) {
    let result = Command::new("git").arg("config").arg("--list").output();

    match result {
        Ok(output) => {
            let exit_code = command::get_exit_code(Ok(output.status), true);

            if exit_code == 0 {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.split('\n').collect();
                for mut line in lines {
                    line = line.trim();

                    debug!("Checking: {}", &line);

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
        }
        Err(error) => info!(
            "Error while running git config --list command: {:#?}",
            &error
        ),
    };
}

fn load_branch(git_info: &mut GitInfo) {
    let result = Command::new("git").arg("branch").output();

    match result {
        Ok(output) => {
            let exit_code = command::get_exit_code(Ok(output.status), true);
            command::validate_exit_code(exit_code);

            let stdout = String::from_utf8_lossy(&output.stdout);
            let lines: Vec<&str> = stdout.split('\n').collect();
            for mut line in lines {
                line = line.trim();

                debug!("Checking: {}", &line);

                if line.starts_with("*") {
                    let parts: Vec<&str> = line.split(' ').collect();
                    let value = parts[1];
                    git_info.branch = Some(value.to_string());
                }
            }
        }
        Err(error) => info!("Error while running git branch command: {:#?}", &error),
    };
}

pub(crate) fn load() -> GitInfo {
    debug!("Searching for git info.");

    let mut git_info = GitInfo::new();

    load_from_git_config(&mut git_info);
    load_branch(&mut git_info);

    debug!("Loaded git info {:#?}", &git_info);

    git_info
}
