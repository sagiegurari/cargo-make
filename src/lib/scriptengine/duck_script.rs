//! # duck_script
//!
//! Compiles and runs duckscript code.
//!

#[cfg(test)]
#[path = "./duck_script_test.rs"]
mod duck_script_test;

use duckscript::runner;
use duckscript::types::runtime::Context;
use duckscriptsdk;

pub(crate) fn execute(script: &Vec<String>, cli_arguments: &Vec<String>, validate: bool) {
    let script_text = script.join("\n");

    let mut context = Context::new();
    let mut index = 0;
    for argument in cli_arguments {
        index = index + 1;

        let mut key = String::from("$");
        key.push_str(&index.to_string());
        context.variables.insert(key, argument.to_string());
    }

    match duckscriptsdk::load(&mut context.commands) {
        Ok(_) => match runner::run_script(&script_text, context) {
            Ok(_) => (),
            Err(error) => {
                if validate {
                    error!("Error while running duckscript: {}", error)
                }
            }
        },
        Err(error) => {
            if validate {
                error!("Unable to load duckscript SDK: {}", error)
            }
        }
    };
}
