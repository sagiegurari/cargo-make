//! # toolchain
//!
//! Toolchain related utilify functions.
//!

#[cfg(test)]
#[path = "toolchain_test.rs"]
mod toolchain_test;

use cargo_metadata::Version;

use crate::types::{CommandSpec, ToolchainSpecifier};
use std::process::{Command, Stdio};

#[cfg(test)]
fn should_validate_installed_toolchain(toolchain: &ToolchainSpecifier) -> bool {
    use crate::test;

    return toolchain.min_version().is_some() || test::is_not_rust_stable();
}

#[cfg(not(test))]
fn should_validate_installed_toolchain(_: &ToolchainSpecifier) -> bool {
    return true;
}

pub(crate) fn wrap_command(
    toolchain: &ToolchainSpecifier,
    command: &str,
    args: &Option<Vec<String>>,
) -> CommandSpec {
    let validate = should_validate_installed_toolchain(toolchain);

    if validate && !has_toolchain(toolchain) {
        error!(
            "Missing toolchain {}! Please install it using rustup.",
            &toolchain
        );
    }

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

fn has_toolchain(toolchain: &ToolchainSpecifier) -> bool {
    let output = Command::new("rustup")
        .args(&["run", toolchain.channel(), "rustc", "--version"])
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .output()
        .expect("Failed to check rustup toolchain");
    if !output.status.success() {
        return false;
    }

    let spec_min_version = toolchain.min_version().and_then(|v| {
        let parsed = v.parse::<Version>();
        if !parsed.is_ok() {
            warn!("Unable to parse min version value: {}", &v);
        }
        parsed.ok()
    });
    if let Some(ref spec_min_version) = spec_min_version {
        let version = String::from_utf8_lossy(&output.stdout);
        let version = version
            .split(" ")
            .nth(1)
            .expect("expected a version in rustc output");
        let version = version
            .parse::<Version>()
            .expect("unexpected version format");
        spec_min_version <= &version
    } else {
        true
    }
}
