//! # cm_check_task_condition
//!
//! Enables to run cargo-make tasks from within duckscript.
//!

#[cfg(test)]
#[path = "cm_plugin_check_task_condition_test.rs"]
mod cm_plugin_check_task_condition_test;

use crate::runner;
use crate::types::{FlowInfo, Step};
use duckscript::types::command::{Command, CommandArgs, CommandResult};

#[derive(Clone)]
pub(crate) struct CommandImpl {
    flow_info: FlowInfo,
    step: Step,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        "cm_plugin_check_task_condition".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: CommandArgs) -> CommandResult {
        let passed = runner::validate_condition(&self.flow_info, &self.step);

        match passed {
            Ok(r) => CommandResult::Continue(Some(r.to_string())),
            Err(e) => CommandResult::Error(e.to_string()),
        }
    }
}

pub(crate) fn create(flow_info: &FlowInfo, step: &Step) -> Box<dyn Command> {
    Box::new(CommandImpl {
        flow_info: flow_info.clone(),
        step: step.clone(),
    })
}
