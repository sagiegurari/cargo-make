//! # shell_to_batch
//!
//! Converts shell scripts to windows batch and invokes them.
//!

#[cfg(test)]
#[path = "shell_to_batch_test.rs"]
mod shell_to_batch_test;

use crate::command;

pub(crate) fn execute(script: &Vec<String>, cli_arguments: &Vec<String>, validate: bool) -> bool {
    let exit_code = if cfg!(windows) {
        let shell_script = script.join("\n");
        let windows_batch = shell2batch::convert(&shell_script);

        let windows_script_lines = windows_batch
            .split("\n")
            .map(|string| string.to_string())
            .collect();

        command::run_script_get_exit_code(&windows_script_lines, None, cli_arguments, validate)
    } else {
        command::run_script_get_exit_code(script, None, cli_arguments, validate)
    };

    exit_code == 0
}
