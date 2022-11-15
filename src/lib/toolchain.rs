//! # toolchain
//!
//! Toolchain related utilify functions.
//!

#[cfg(test)]
#[path = "toolchain_test.rs"]
mod toolchain_test;

use crate::environment::expand_value;
use crate::types::{CommandSpec, ToolchainSpecifier};
use envmnt;
use semver::{Prerelease, Version};
use std::process::{Command, Stdio};

pub(crate) fn get_channel(toolchain: &ToolchainSpecifier) -> String {
    let channel = toolchain.channel().to_string();
    expand_value(&channel)
}

pub(crate) fn wrap_command(
    toolchain: &ToolchainSpecifier,
    command: &str,
    args: &Option<Vec<String>>,
) -> CommandSpec {
    check_toolchain(toolchain);

    let channel = get_channel(&toolchain);
    if channel.is_empty() {
        let mut command_args = vec![];
        if let Some(array) = args {
            for arg in array.iter() {
                command_args.push(arg.to_string());
            }
        };

        CommandSpec {
            command: command.to_string(),
            args: Some(command_args),
        }
    } else {
        let mut rustup_args = vec!["run".to_string(), channel, command.to_string()];
        if let Some(array) = args {
            for arg in array.iter() {
                rustup_args.push(arg.to_string());
            }
        };

        CommandSpec {
            command: "rustup".to_string(),
            args: Some(rustup_args),
        }
    }
}

fn get_specified_min_version(toolchain: &ToolchainSpecifier) -> Option<Version> {
    let min_version = toolchain.min_version()?;
    let spec_min_version = min_version.parse::<Version>();
    if let Err(_) = spec_min_version {
        warn!("Unable to parse min version value: {}", &min_version);
    }
    spec_min_version.ok()
}

fn check_toolchain(toolchain: &ToolchainSpecifier) {
    let channel = get_channel(&toolchain);

    if channel.is_empty() {
        return;
    }

    let output = Command::new("rustup")
        .args(&["run", &channel, "rustc", "--version"])
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to check rustup toolchain");
    if !output.status.success() {
        error!(
            "Missing toolchain {}! Please install it using rustup.",
            &channel
        );
        return;
    }

    let spec_min_version = get_specified_min_version(toolchain);
    if let Some(ref spec_min_version) = spec_min_version {
        let rustc_version = String::from_utf8_lossy(&output.stdout);
        let rustc_version = rustc_version
            .split(" ")
            .nth(1)
            .expect("expected a version in rustc output");
        let mut rustc_version = rustc_version
            .parse::<Version>()
            .expect("unexpected version format");
        // Remove prerelease identifiers from the output of rustc. Specifying a toolchain
        // channel means the user actively chooses beta or nightly (or a custom one).
        //
        // Direct comparison with rustc output would otherwise produce unintended results:
        // `{ channel = "beta", min_version = "1.56" }` is expected to work with
        // `rustup run beta rustc --version` ==> "rustc 1.56.0-beta.4 (e6e620e1c 2021-10-04)"
        // so we would have 1.56.0-beta.4 < 1.56 according to semver
        rustc_version.pre = Prerelease::EMPTY;

        if &rustc_version < spec_min_version {
            error!(
                "Installed toolchain {} is required to satisfy version {}, found {}! Please upgrade it using rustup.",
                &channel,
                &spec_min_version,
                rustc_version,
            );
        }
    }
}

pub(crate) fn get_cargo_binary_path(toolchain: &ToolchainSpecifier) -> Option<String> {
    let command_spec = wrap_command(
        toolchain,
        "rustup",
        &Some(vec!["which".to_string(), "cargo".to_string()]),
    );
    let mut command = Command::new(&command_spec.command);
    match command_spec.args {
        Some(ref args_vec) => {
            command.args(args_vec);
        }
        None => debug!("No command args defined."),
    };

    let output = command
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to check rustup toolchain");
    if !output.status.success() {
        error!(
            "Missing toolchain {}! Please install it using rustup.",
            &toolchain
        );
        return None;
    }

    let binary_path = String::from_utf8_lossy(&output.stdout);
    if binary_path.is_empty() {
        None
    } else {
        Some(binary_path.to_string())
    }
}

pub(crate) fn remove_rust_env_vars() {
    envmnt::remove_all(&vec!["RUSTC", "RUSTDOC", "RUSTFLAGS"]);
}
