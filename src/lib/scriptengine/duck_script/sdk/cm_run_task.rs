//! # cm_run_task
//!
//! Enables to run cargo-make tasks from within duckscript.
//!

use crate::error::CargoMakeError;
use crate::runner;
use crate::types::{FlowInfo, FlowState};
use duckscript::types::command::{Command, CommandArgs, CommandResult};
use std::cell::RefCell;
use std::rc::Rc;
use std::thread;

#[derive(Clone)]
pub(crate) struct CommandImpl {
    flow_info: FlowInfo,
    flow_state: Rc<RefCell<FlowState>>,
}

impl Command for CommandImpl {
    fn name(&self) -> String {
        "cm_run_task".to_string()
    }

    fn clone_and_box(&self) -> Box<dyn Command> {
        Box::new((*self).clone())
    }

    fn run(&self, arguments: CommandArgs) -> CommandResult {
        if arguments.args.is_empty() {
            CommandResult::Error(
                CargoMakeError::NotFound(String::from("No task name provided.")).to_string(),
            )
        } else {
            let (task_name, async_run) =
                if arguments.args.len() > 0 && arguments.args[0] == "--async" {
                    (arguments.args[1].clone(), true)
                } else {
                    (arguments.args[0].clone(), false)
                };

            if self.flow_info.config.tasks.contains_key(&task_name) {
                let mut sub_flow_info = self.flow_info.clone();
                sub_flow_info.task = task_name.clone();

                if async_run {
                    let cloned_flow_state = self.flow_state.borrow().clone();

                    thread::spawn(move || -> Result<(), CargoMakeError> {
                        runner::run_flow(
                            &sub_flow_info,
                            Rc::new(RefCell::new(cloned_flow_state)),
                            true,
                        )
                    });
                } else {
                    if let Err(e) = runner::run_flow(&sub_flow_info, self.flow_state.clone(), true)
                    {
                        return CommandResult::Error(e.to_string());
                    }
                }

                CommandResult::Continue(Some("true".to_string()))
            } else {
                CommandResult::Error(format!("Task: {} not found.", &arguments.args[0]).to_string())
            }
        }
    }
}

pub(crate) fn create(flow_info: &FlowInfo, flow_state: Rc<RefCell<FlowState>>) -> Box<dyn Command> {
    Box::new(CommandImpl {
        flow_info: flow_info.clone(),
        flow_state,
    })
}
