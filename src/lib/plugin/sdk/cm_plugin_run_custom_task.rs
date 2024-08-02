//! # cm_run_task
//!
//! Enables to run cargo-make tasks from within duckscript.
//!

#[cfg(test)]
#[path = "cm_plugin_run_custom_task_test.rs"]
mod cm_plugin_run_custom_task_test;

use std::cell::RefCell;
use std::rc::Rc;

use duckscript::types::command::{Command, CommandResult};

use crate::runner;
use crate::types::{FlowInfo, FlowState, RunTaskOptions, Step, Task};

#[derive(Clone)]
pub(crate) struct CommandImpl {
    flow_info: FlowInfo,
    flow_state: Rc<RefCell<FlowState>>,
    step: Step,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        "cm_plugin_run_custom_task".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: Vec<String>) -> CommandResult {
        if arguments.is_empty() {
            CommandResult::Error("No task data provided.".to_string())
        } else {
            let task: Task = match serde_json::from_str(&arguments[0]) {
                Ok(value) => value,
                Err(error) => return CommandResult::Error(error.to_string()),
            };

            let custom_step = Step {
                name: self.step.name.clone(),
                config: task,
            };

            let options = RunTaskOptions {
                plugins_enabled: false,
            };

            if let Err(e) = runner::run_task_with_options(
                &self.flow_info,
                self.flow_state.clone(),
                &custom_step,
                &options,
            ) {
                return CommandResult::Error(e.to_string());
            }

            CommandResult::Continue(Some("true".to_string()))
        }
    }
}

pub(crate) fn create(
    flow_info: &FlowInfo,
    flow_state: Rc<RefCell<FlowState>>,
    step: &Step,
) -> Box<dyn Command> {
    Box::new(CommandImpl {
        flow_info: flow_info.clone(),
        flow_state,
        step: step.clone(),
    })
}
