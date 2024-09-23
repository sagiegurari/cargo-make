//! # cm_run_task
//!
//! Enables to run cargo-make tasks from within duckscript.
//!

use std::cell::RefCell;
use std::rc::Rc;

use duckscript::types::command::{Command, CommandResult};

use crate::types::FlowState;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    flow_state: Rc<RefCell<FlowState>>,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        "cm_plugin_force_plugin_clear".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        self.flow_state.borrow_mut().forced_plugin = None;

        CommandResult::Continue(Some("true".to_string()))
    }
}

pub(crate) fn create(flow_state: Rc<RefCell<FlowState>>) -> Box<dyn Command> {
    Box::new(CommandImpl { flow_state })
}
