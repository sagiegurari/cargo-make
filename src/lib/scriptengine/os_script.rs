//! # os_script
//!
//! Runs OS scripts.
//!

#[cfg(test)]
#[path = "os_script_test.rs"]
mod os_script_test;

use crate::command;

pub(crate) fn execute(
    script_text: &Vec<String>,
    runner: Option<String>,
    cli_arguments: &Vec<String>,
    validate: bool,
) -> bool {
    let exit_code =
        command::run_script_get_exit_code(&script_text, runner, &cli_arguments, validate);
    exit_code == 0
}
