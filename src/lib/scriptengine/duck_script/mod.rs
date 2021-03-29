//! # duck_script
//!
//! Compiles and runs duckscript code.
//!

#[cfg(test)]
#[path = "mod_test.rs"]
mod mod_test;

mod sdk;

use crate::environment;
use crate::types::FlowInfo;
use duckscript::runner;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::{Context, StateValue};
use duckscriptsdk;
use envmnt;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) static FLOW_INFO_KEY: &str = "cargo_make::flow_info";

pub(crate) fn execute(
    script: &Vec<String>,
    cli_arguments: &Vec<String>,
    flow_info: Option<&FlowInfo>,
    validate: bool,
) {
    let mut script_text = script.join("\n");
    script_text.insert_str(0, "exit_on_error true\n");

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

    match flow_info {
        Some(info) => {
            context.state.insert(
                FLOW_INFO_KEY.to_string(),
                StateValue::Any(Rc::new(RefCell::new(info.clone()))),
            );
        }
        None => (),
    }

    match load_sdk(&mut context.commands) {
        Ok(_) => {
            let directory = envmnt::get_or("CARGO_MAKE_WORKING_DIRECTORY", "");

            match runner::run_script(&script_text, context) {
                Ok(_) => (),
                Err(error) => {
                    if validate {
                        error!("Error while running duckscript: {}", error);
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
            }
        }
    };
}

fn load_sdk(commands: &mut Commands) -> Result<(), ScriptError> {
    duckscriptsdk::load(commands)?;
    sdk::load(commands)?;

    Ok(())
}
