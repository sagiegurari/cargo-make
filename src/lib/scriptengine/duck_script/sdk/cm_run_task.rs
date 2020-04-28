//! # cm_run_task
//!
//! Enables to run cargo-make tasks from within duckscript.
//!

use crate::runner;
use crate::scriptengine::duck_script::sdk;
use crate::types::{FlowInfo, Step};
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

#[derive(Clone)]
pub(crate) struct CommandImpl {}

impl Command for CommandImpl {
    fn name(&self) -> String {
        "cm_run_task".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn requires_context(&self) -> bool {
        true
    }

    fn run_with_context(
        &self,
        arguments: Vec<String>,
        state: &mut HashMap<String, StateValue>,
        _variables: &mut HashMap<String, String>,
        _output_variable: Option<String>,
        _instructions: &Vec<Instruction>,
        _commands: &mut Commands,
        _line: usize,
    ) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("No task name provided.".to_string())
        } else {
            sdk::run_in_flow_context(state, &|flow_info: &FlowInfo| -> CommandResult {
                match flow_info.config.tasks.get(&arguments[0]) {
                    Some(task) => {
                        runner::run_task(
                            flow_info,
                            &Step {
                                name: arguments[0].clone(),
                                config: task.clone(),
                            },
                        );

                        CommandResult::Continue(Some("true".to_string()))
                    }
                    None => CommandResult::Error(
                        format!("Task: {} not found.", &arguments[0]).to_string(),
                    ),
                }
            })
        }
    }
}

pub(crate) fn create() -> Box<dyn Command> {
    Box::new(CommandImpl {})
}
