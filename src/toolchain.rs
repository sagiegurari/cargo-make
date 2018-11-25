//! # toolchain
//!
//! Toolchain related utilify functions.
//!

#[cfg(test)]
#[path = "./toolchain_test.rs"]
mod toolchain_test;

use crate::types::CommandSpec;

pub(crate) fn wrap_command(
    toolchain: &str,
    command: &str,
    args: &Option<Vec<String>>,
) -> CommandSpec {
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
