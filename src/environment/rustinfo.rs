//! # rustinfo
//!
//! Loads rust compiler information.
//!

#[cfg(test)]
#[path = "./rustinfo_test.rs"]
mod rustinfo_test;

use command;
use log::Logger;
use std::collections::HashMap;
use std::process::Command;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Rust channel type
pub enum Channel {
    Stable,
    Beta,
    Nightly
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds rust info for the current runtime
pub struct RustInfo {
    /// version
    pub version: Option<String>,
    /// channel
    pub channel: Option<Channel>,
    /// target arch cfg value
    pub target_arch: Option<String>,
    /// target env cfg value
    pub target_env: Option<String>,
    /// target OS cfg value
    pub target_os: Option<String>,
    /// target pointer width cfg value
    pub target_pointer_width: Option<String>,
    /// target vendor cfg value
    pub target_vendor: Option<String>
}

impl RustInfo {
    pub fn new() -> RustInfo {
        RustInfo {
            version: None,
            channel: None,
            target_arch: None,
            target_env: None,
            target_os: None,
            target_pointer_width: None,
            target_vendor: None
        }
    }
}

pub fn load(logger: &Logger) -> RustInfo {
    let mut rust_info = RustInfo::new();

    let mut result = Command::new("rustc").arg("--version").output();

    match result {
        Ok(output) => {
            let exit_code = command::get_exit_code(Ok(output.status), logger, true);

            if exit_code == 0 {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let parts: Vec<&str> = stdout.split(' ').collect();

                if (parts.len() >= 3) && (parts[0] == "rustc") {
                    let version_part = parts[1];

                    let version_parts: Vec<&str> = version_part.split('-').collect();

                    if version_parts.len() > 0 {
                        rust_info.version = Some(version_parts[0].to_string());

                        if version_parts.len() == 1 {
                            rust_info.channel = Some(Channel::Stable);
                        } else if version_parts[1].contains("beta") {
                            rust_info.channel = Some(Channel::Beta);
                        } else if version_parts[1].contains("nightly") {
                            rust_info.channel = Some(Channel::Nightly);
                        }
                    }
                }
            }
        }
        Err(error) => logger.info("Error while running rustc --version command.: ", &[], Some(&error)),
    };

    result = Command::new("rustc").arg("--print").arg("cfg").output();

    match result {
        Ok(output) => {
            let exit_code = command::get_exit_code(Ok(output.status), logger, true);

            if exit_code == 0 {
                let mut values = HashMap::<String, String>::new();

                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.split('\n').collect();
                for mut line in lines {
                    line = line.trim();

                    logger.verbose::<()>("Checking: ", &[&line], None);

                    if line.contains("=") {
                        let parts: Vec<&str> = line.split('=').collect();
                        let value = str::replace(parts[1], "\"", "");
                        values.insert(parts[0].to_string(), value.to_string());
                    }
                }

                rust_info.target_arch = Some(values.remove("target_arch").unwrap_or("unknown".to_string()));
                rust_info.target_env = Some(values.remove("target_env").unwrap_or("unknown".to_string()));
                rust_info.target_os = Some(values.remove("target_os").unwrap_or("unknown".to_string()));
                rust_info.target_pointer_width = Some(values.remove("target_pointer_width").unwrap_or("unknown".to_string()));
                rust_info.target_vendor = Some(values.remove("target_vendor").unwrap_or("unknown".to_string()));
            }
        }
        Err(error) => logger.info("Error while running rustc --version command.: ", &[], Some(&error)),
    };

    rust_info
}
