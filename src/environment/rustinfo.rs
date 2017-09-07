//! # rustinfo
//!
//! Loads rust compiler information.
//!

#[cfg(test)]
#[path = "./rustinfo_test.rs"]
mod rustinfo_test;

use command;
use std::collections::HashMap;
use std::process::Command;
use types::{RustChannel, RustInfo};

pub fn load() -> RustInfo {
    let mut rust_info = RustInfo::new();

    let mut result = Command::new("rustc").arg("--version").output();

    match result {
        Ok(output) => {
            let exit_code = command::get_exit_code(Ok(output.status), true);

            if exit_code == 0 {
                let stdout = String::from_utf8_lossy(&output.stdout);
                let parts: Vec<&str> = stdout.split(' ').collect();

                if (parts.len() >= 3) && (parts[0] == "rustc") {
                    let version_part = parts[1];

                    let version_parts: Vec<&str> = version_part.split('-').collect();

                    if version_parts.len() > 0 {
                        rust_info.version = Some(version_parts[0].to_string());

                        if version_parts.len() == 1 {
                            rust_info.channel = Some(RustChannel::Stable);
                        } else if version_parts[1].contains("beta") {
                            rust_info.channel = Some(RustChannel::Beta);
                        } else if version_parts[1].contains("nightly") {
                            rust_info.channel = Some(RustChannel::Nightly);
                        }
                    }
                }
            }
        }
        Err(error) => info!("Error while running rustc --version command: {:#?}", &error),
    };

    result = Command::new("rustc").arg("--print").arg("cfg").output();

    match result {
        Ok(output) => {
            let exit_code = command::get_exit_code(Ok(output.status), true);

            if exit_code == 0 {
                let mut values = HashMap::<String, String>::new();

                let stdout = String::from_utf8_lossy(&output.stdout);
                let lines: Vec<&str> = stdout.split('\n').collect();
                for mut line in lines {
                    line = line.trim();

                    debug!("Checking: {}", &line);

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
        Err(error) => info!("Error while running rustc --version command: {:#?}", &error),
    };

    rust_info
}
