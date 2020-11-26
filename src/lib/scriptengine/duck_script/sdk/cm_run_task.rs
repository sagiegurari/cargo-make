//! # cm_run_task
//!
//! Enables to run cargo-make tasks from within duckscript.
//!

use crate::runner;
use crate::scriptengine::duck_script::sdk;
use crate::types::{FlowInfo, FlowState, Step};
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
                match flow_info.config.tasks.get(&task_name) {
                    Some(task) => {
                        // we currently do not support sharing same state
                        // as main flow invocation
                        let mut flow_state = FlowState::new();

                        let step = Step {
                            name: arguments[0].clone(),
                            config: task.clone(),
                        };

                        if async_run {
                            let flow_info_clone = flow_info.clone();
                            thread::spawn(move || {
                                runner::run_task(&flow_info_clone, &mut flow_state, &step);
                            });
                        } else {
                            runner::run_task(flow_info, &mut flow_state, &step);
                        }

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
