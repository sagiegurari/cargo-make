//! # duck_script
//!
//! Compiles and runs duckscript code.
//!

#[cfg(test)]
#[path = "./duck_script_test.rs"]
mod duck_script_test;

use crate::environment;
use duckscript::runner;
use duckscript::types::runtime::Context;
use duckscriptsdk;
use envmnt;

pub(crate) fn execute(script: &Vec<String>, cli_arguments: &Vec<String>, validate: bool) {
    let script_text = script.join("\n");

    let mut context = Context::new();
    let mut index = 0;
    for argument in cli_arguments {
        index = index + 1;

        context
            .variables
            .insert(index.to_string(), argument.to_string());
    }

    let all_vars = envmnt::vars();

    for (key, value) in all_vars {
        context.variables.insert(key, value);
    }

    match duckscriptsdk::load(&mut context.commands) {
        Ok(_) => {
            let directory = envmnt::get_or("CARGO_MAKE_WORKING_DIRECTORY", "");

            match runner::run_script(&script_text, context) {
                Ok(_) => (),
                Err(error) => {
                    if validate {
                        error!("Error while running duckscript: {}", error);
                        panic!("Error while running duckscript: {}", error)
                    }
                }
            };

            // revert to originl working directory
            if !directory.is_empty() {
                environment::setup_cwd(Some(&directory));
            }
        }
        Err(error) => {
            if validate {
                error!("Unable to load duckscript SDK: {}", error);
                panic!("Unable to load duckscript SDK: {}", error)
            }
        }
    };
}
