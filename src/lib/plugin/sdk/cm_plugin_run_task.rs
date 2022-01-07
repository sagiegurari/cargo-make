//! # cm_run_task
//!
//! Enables to run cargo-make tasks from within duckscript.
//!

use crate::runner;
use crate::types::{FlowInfo, FlowState, RunTaskOptions, Step};
use duckscript::types::command::{Command, CommandResult};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    flow_info: FlowInfo,
    flow_state: Rc<RefCell<FlowState>>,
    step: Step,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        "cm_plugin_run_task".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, _arguments: Vec<String>) -> CommandResult {
        let options = RunTaskOptions {
            plugins_enabled: false,
        };

        runner::run_task_with_options(
            &self.flow_info,
            self.flow_state.clone(),
            &self.step,
            &options,
        );

        CommandResult::Continue(Some("true".to_string()))
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
