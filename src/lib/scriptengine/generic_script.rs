//! # generic_script
//!
//! Runs generic scripts for other languages based on provided config.
//!

#[cfg(test)]
#[path = "./generic_script_test.rs"]
mod generic_script_test;

use crate::command;
use crate::io::delete_file;
use crate::scriptengine::script_utils::create_script_file;

fn run_file(file: &str, runner: &String) -> bool {
    let exit_code = command::run_command(runner, &Some(vec![file.to_string()]), false);
    debug!("Executed generic script, exit code: {}", exit_code);

    exit_code == 0
}

pub(crate) fn execute(
    script_text: &Vec<String>,
    runner: String,
    extension: String,
    validate: bool,
) {
    let file = create_script_file(script_text, &extension);

    let valid = run_file(&file, &runner);

    delete_file(&file);

    if validate && !valid {
        error!("Unable to execute generic script.");
    }
}
