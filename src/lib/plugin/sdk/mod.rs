//! # sdk
//!
//! Loads the cargo-make duckscript plugin SDK.
//!

mod cm_plugin_run_task;

use crate::types::{FlowInfo, FlowState, Step};
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;
use std::cell::RefCell;
use std::rc::Rc;

/// Loads all core commands
pub(crate) fn load(
    flow_info: &FlowInfo,
    flow_state: Rc<RefCell<FlowState>>,
    step: &Step,
    commands: &mut Commands,
) -> Result<(), ScriptError> {
    commands.set(cm_plugin_run_task::create(flow_info, flow_state, step))?;

    Ok(())
}
