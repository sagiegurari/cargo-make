//! # generic_script
//!
//! Runs generic scripts for other languages based on provided config.
//!

#[cfg(test)]
#[path = "generic_script_test.rs"]
mod generic_script_test;

use crate::command;
use crate::error::CargoMakeError;
use crate::io::delete_file;
use crate::scriptengine::script_utils::create_script_file;

fn run_file(
    file: &str,
    runner: &String,
    arguments: Option<Vec<String>>,
    cli_arguments: &mut Vec<String>,
) -> Result<bool, CargoMakeError> {
    let mut args = match arguments {
        Some(values) => values,
        None => vec![],
    };

    args.push(file.to_string());

    args.append(cli_arguments);

    let exit_code = command::run_command(runner, &Some(args), false)?;
    debug!("Executed script, exit code: {}", exit_code);

    Ok(exit_code == 0)
}

pub(crate) fn execute(
    script_text: &Vec<String>,
    runner: String,
    extension: String,
    arguments: Option<Vec<String>>,
    cli_arguments: &Vec<String>,
    validate: bool,
) -> Result<bool, CargoMakeError> {
    let file = create_script_file(script_text, &extension);

    let valid = run_file(&file, &runner, arguments, &mut cli_arguments.clone())?;

    delete_file(&file);

    if validate && !valid {
        error!("Unable to execute script.");
    }

    Ok(valid)
}
