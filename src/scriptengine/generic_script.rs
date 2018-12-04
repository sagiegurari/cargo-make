//! # generic_script
//!
//! Runs generic scripts for other languages based on provided config.
//!

#[cfg(test)]
#[path = "./generic_script_test.rs"]
mod generic_script_test;

use crate::command;
use crate::scriptengine::script_utils::{create_script_file, delete_file};

fn run_file(file: &str, runner: &String) -> bool {
    let exit_code = command::run_command(runner, &Some(vec![file.to_string()]), false);

    debug!("Executed generic script, exit code: {}", exit_code);

    exit_code == 0
}

pub(crate) fn execute(script_text: &Vec<String>, runner: Option<String>, extension: String) {
    let file = create_script_file(script_text, &extension);

    let runner_string = match runner {
        Some(script) => script,
        None => match extract_runner_from_script(script_text.clone()) {
            Some(script) => script,
            None => {
                error!("Script runner not specified in toml file or shebang line");
                panic!("Script runner not specified in toml file or shebang line")
            }
        },
    };

    let valid = run_file(&file, &runner_string);

    delete_file(&file);

    if !valid {
        error!("Unable to execute generic script.");
    }
}

fn extract_runner_from_script(script: Vec<String>) -> Option<String> {
    if cfg!(windows) {
        return None
    }
    match script.first() {
        Some(line) => {
            let shebang: Vec<&str> = line.matches("#!").collect();
            Some(extract_runner_from_shebang(shebang.first().unwrap().to_string()))
        },
        None => None,
    }
}

fn extract_runner_from_shebang(shebang: String) -> String {
    shebang.replace("#!", "")
}
