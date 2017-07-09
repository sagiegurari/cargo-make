//! # rustinfo
//!
//! Loads rust compiler information.
//!

#[cfg(test)]
#[path = "./rustinfo_test.rs"]
mod rustinfo_test;

use command;
use log::Logger;
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
    pub channel: Option<Channel>
}

impl RustInfo {
    pub fn new() -> RustInfo {
        RustInfo { version: None, channel: None }
    }
}

pub fn load(logger: &Logger) -> RustInfo {
    let mut rust_info = RustInfo::new();

    let result = Command::new("rustc").arg("--version").output();

    match result {
        Ok(output) => {
            command::validate_exit_code(Ok(output.status), logger);

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
        Err(error) => logger.info("Error while running rustc --version command.: ", &[], Some(&error)),
    };

    rust_info
}
