//! # generic_script
//!
//! Runs generic scripts for other languages based on provided config.
//!

#[cfg(test)]
#[path = "./generic_script_test.rs"]
mod generic_script_test;

use crate::command;
use crate::scriptengine::script_utils::{create_script_file, delete_file};

fn run_file(file: &str, runner: &String) -> (bool, String) {
    let output = command::run_command_get_output(runner, &Some(vec![file.to_string()]), false);
    let exit_code = command::get_exit_code_from_output(&output, false);
    debug!("Executed generic script, exit code: {}", exit_code);

    let valid = exit_code == 0;

    match output {
        Ok(ref output_struct) => {
            let stdout = String::from_utf8_lossy(&output_struct.stdout).into_owned();
            (valid, stdout)
        }
        _ => (valid, "".to_string()),
    }
}

pub(crate) fn execute(script_text: &Vec<String>, runner: String, extension: String) -> String {
    let file = create_script_file(script_text, &extension);

    let output = run_file(&file, &runner);
    let valid = output.0;

    delete_file(&file);

    if !valid {
        error!("Unable to execute generic script.");
    }

    output.1
}
