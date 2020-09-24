//! # toolchain
//!
//! Toolchain related utilify functions.
//!

#[cfg(test)]
#[path = "./toolchain_test.rs"]
mod toolchain_test;

use crate::types::{CommandSpec, FlowInfo, Step};
use std::process::{Command, Stdio};

#[cfg(test)]
fn should_validate_installed_toolchain() -> bool {
    use crate::test;

    return test::is_not_rust_stable();
}

#[cfg(not(test))]
fn should_validate_installed_toolchain() -> bool {
    return true;
}

pub(crate) fn wrap_command(
    toolchain: &str,
    command: &str,
    args: &Option<Vec<String>>,
) -> CommandSpec {
    let validate = should_validate_installed_toolchain();

    if validate && !has_toolchain(toolchain) {
        error!(
            "Missing toolchain {}! Please install it using rustup.",
            &toolchain
        );
    }

    let mut rustup_args = vec![
        "run".to_string(),
        toolchain.to_string(),
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

fn has_toolchain(toolchain: &str) -> bool {
    Command::new("rustup")
        .args(&["run", toolchain, "rustc"])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status()
        .expect("Failed to check rustup toolchain")
        .success()
}

pub(crate) fn expand(flow_info: &FlowInfo, step: &mut Step) {
    println!("{:?}", flow_info.config.config.toolchain);
    if step.config.toolchain.is_none() && flow_info.config.config.toolchain.is_some() {
        step.config.toolchain = flow_info.config.config.toolchain.clone();
    }
}
