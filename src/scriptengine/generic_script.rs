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
        Some(s) => s,
        None => {
            match extract_runner_from_script(
                script_text.clone()) {
                Some(r) => r,
                None => panic!("Script runner not specified in toml file or shebang line")
            }
        }
    };

    let valid = run_file(&file, &runner_string);

    delete_file(&file);

    if !valid {
        error!("Unable to execute generic script.");
    }
}

pub(crate) fn extract_runner_from_script(script: Vec<String>) -> Option<String> {
   match script.first() {
       Some(s) => {
           let m: Vec<&str> = s.matches("#!").collect();
           if m.len() == 1 {
               Some(extract_runner_from_shebang(s.to_string()))
           } else {
               None
           }
       },
       None => None,
   }
}

pub(crate) fn extract_runner_from_shebang(shebang: String) -> String {
    shebang.replace("#!", "" )
}


