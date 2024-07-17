//! # os_script
//!
//! Runs OS scripts.
//!

#[cfg(test)]
#[path = "os_script_test.rs"]
mod os_script_test;

use crate::command;
use crate::error::CargoMakeError;

pub(crate) fn execute(
    script_text: &Vec<String>,
    runner: Option<String>,
    cli_arguments: &Vec<String>,
    validate: bool,
) -> Result<bool, CargoMakeError> {
    let exit_code =
        command::run_script_get_exit_code(&script_text, runner, &cli_arguments, validate)?;
    Ok(exit_code == 0)
}
