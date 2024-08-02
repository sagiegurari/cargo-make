//! # cm_run_task
//!
//! Enables to run cargo-make tasks from within duckscript.
//!

#[cfg(test)]
#[path = "cm_plugin_force_plugin_set_test.rs"]
mod cm_plugin_force_plugin_set_test;

use std::cell::RefCell;
use std::rc::Rc;

use duckscript::types::command::{Command, CommandResult};

use crate::types::{FlowState, Step};

#[derive(Clone)]
pub(crate) struct CommandImpl {
    flow_state: Rc<RefCell<FlowState>>,
    step: Step,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        "cm_plugin_force_plugin_set".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        self.flow_state.borrow_mut().forced_plugin = self.step.config.plugin.clone();

        CommandResult::Continue(Some("true".to_string()))
    }
}

pub(crate) fn create(flow_state: Rc<RefCell<FlowState>>, step: &Step) -> Box<dyn Command> {
    Box::new(CommandImpl {
        flow_state,
        step: step.clone(),
    })
}
