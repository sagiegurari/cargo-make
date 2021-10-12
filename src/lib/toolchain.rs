//! # toolchain
//!
//! Toolchain related utilify functions.
//!

#[cfg(test)]
#[path = "toolchain_test.rs"]
mod toolchain_test;

use cargo_metadata::Version;
use semver::Prerelease;

use crate::types::{CommandSpec, ToolchainSpecifier};
use std::{
    process::{Command, Stdio},
    str::FromStr,
};

pub(crate) fn wrap_command(
    toolchain: &ToolchainSpecifier,
    command: &str,
    args: &Option<Vec<String>>,
) -> CommandSpec {
    check_toolchain(toolchain);

    let mut rustup_args = vec![
        "run".to_string(),
        toolchain.channel().to_string(),
        command.to_string(),
    ];

    match args {
        Some(array) => {
            for arg in array.iter() {
                rustup_args.push(arg.to_string());
            }
        }
        None => (),
    };

    CommandSpec {
        command: "rustup".to_string(),
        args: Some(rustup_args),
    }
}

fn get_specified_min_version(toolchain: &ToolchainSpecifier) -> Option<Version> {
    let min_version = toolchain.min_version()?;
    if let Some(mut min_version) = min_version.parse::<Version>().ok() {
        if min_version.pre.is_empty() && toolchain.is_prerelease() {
            // If no explicit prerelease identifier has been specified, but a prerelease
            // version has been selected, add it to the parsed version.
            // Comparison with rustc output would otherwise produce unintended results:
            // `{ channel = "beta", min_version = "1.56" }` is expected to work with
            // `rustup run beta rustc --version` ==> "rustc 1.56.0-beta.4 (e6e620e1c 2021-10-04)"
            // so we have 1.56-beta < 1.56.0-beta.4 < 1.56 according to semver
            match Prerelease::from_str(toolchain.channel()) {
                Err(_) => {
                    warn!(
                        "Unable to parse channel pre-release identifier: {}",
                        &toolchain.channel()
                    );
                }
                Ok(contrived_pre) => min_version.pre = contrived_pre,
            }
        }
        Some(min_version)
    } else {
        warn!("Unable to parse min version value: {}", &min_version);
        None
    }
}

fn check_toolchain(toolchain: &ToolchainSpecifier) {
    let output = Command::new("rustup")
        .args(&["run", toolchain.channel(), "rustc", "--version"])
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to check rustup toolchain");
    if !output.status.success() {
        error!(
            "Missing toolchain {}! Please install it using rustup.",
            &toolchain
        );
        return;
    }

    let spec_min_version = get_specified_min_version(toolchain);
    if let Some(ref spec_min_version) = spec_min_version {
        let version = String::from_utf8_lossy(&output.stdout);
        let version = version
            .split(" ")
            .nth(1)
            .expect("expected a version in rustc output");
        let version = version
            .parse::<Version>()
            .expect("unexpected version format");
        if &version < spec_min_version {
            error!(
                "Installed toolchain {} is required to satisfy version {}, found {}! Please upgrade it using rustup.",
                toolchain.channel(),
                &spec_min_version,
                version,
            );
        }
    }
}
