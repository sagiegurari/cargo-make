//! # cm_run_task
//!
//! Enables to run cargo-make tasks from within duckscript.
//!

use crate::runner;
use crate::scriptengine::duck_script::sdk;
use crate::types::{FlowInfo, FlowState};
use duckscript::types::command::{Command, CommandResult, Commands};
use duckscript::types::instruction::Instruction;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;
use std::thread;

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
            let (task_name, async_run) = if arguments.len() > 0 && arguments[0] == "--async" {
                (arguments[1].clone(), true)
            } else {
                (arguments[0].clone(), false)
            };

            sdk::run_in_flow_context(state, &|flow_info: &FlowInfo| -> CommandResult {
                if flow_info.config.tasks.contains_key(&task_name) {
                    // we currently do not support sharing same state
                    // as main flow invocation
                    let mut flow_state = FlowState::new();

                    let mut sub_flow_info = flow_info.clone();
                    sub_flow_info.task = task_name.clone();

                    if async_run {
                        thread::spawn(move || {
                            runner::run_flow(&sub_flow_info, &mut flow_state, true);
                        });
                    } else {
                        runner::run_flow(&sub_flow_info, &mut flow_state, true);
                    }

                    CommandResult::Continue(Some("true".to_string()))
                } else {
                    CommandResult::Error(format!("Task: {} not found.", &arguments[0]).to_string())
                }
            })
        }
    }
}

pub(crate) fn create() -> Box<dyn Command> {
    Box::new(CommandImpl {})
}
