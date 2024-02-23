//! # duck_script
//!
//! Compiles and runs duckscript code.
//!

#[cfg(test)]
#[path = "mod_test.rs"]
mod mod_test;

mod sdk;

use crate::environment;
use crate::logger::{get_level, get_log_level, LogLevel};
use crate::types::{FlowInfo, FlowState};
use duckscript::runner;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::Context;
use std::cell::RefCell;
use std::rc::Rc;

pub(crate) fn execute(
    script: &Vec<String>,
    cli_arguments: &Vec<String>,
    flow_info: Option<&FlowInfo>,
    flow_state: Option<Rc<RefCell<FlowState>>>,
    validate: bool,
) -> bool {
    let mut array_command = "@ = array".to_string();
    let mut index = 0;
    for _ in cli_arguments {
        index = index + 1;
        array_command.push_str(format!(" ${{{}}}", index).as_str());
    }

    let log_level = get_log_level();
    let level = get_level(&log_level);
    if level == LogLevel::OFF {
        array_command.push_str("alias echo noop");
    }

    let mut script_text = script.join("\n");
    script_text.insert_str(
        0,
        format!("exit_on_error true\n{}\n", &array_command).as_str(),
    );

    let mut context = create_common_context(cli_arguments);

    match load_sdk(&mut context.commands, flow_info, flow_state) {
        Ok(_) => {
            let directory = envmnt::get_or("CARGO_MAKE_WORKING_DIRECTORY", "");

            match runner::run_script(&script_text, context) {
                Ok(_) => (),
                Err(error) => {
                    if validate {
                        error!("Error while running duckscript: {}", error);
                    }

                    return false;
                }
            };

            // revert to originl working directory
            if !directory.is_empty() {
                environment::setup_cwd(Some(&directory));
            }

            true
        }
        Err(error) => {
            if validate {
                error!("Unable to load duckscript SDK: {}", error);
            }

            false
        }
    }
}

pub(crate) fn create_common_context(cli_arguments: &Vec<String>) -> Context {
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

    context
}

pub(crate) fn load_sdk(
    commands: &mut Commands,
    flow_info: Option<&FlowInfo>,
    flow_state: Option<Rc<RefCell<FlowState>>>,
) -> Result<(), ScriptError> {
    duckscriptsdk::load(commands)?;
    sdk::load(commands, flow_info, flow_state)?;

    Ok(())
}
