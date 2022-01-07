//! # sdk
//!
//! Loads the cargo-make duckscript SDK.
//!

mod cm_run_task;

use crate::types::{FlowInfo, FlowState};
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;
use std::cell::RefCell;
use std::rc::Rc;

/// Loads all core commands
pub(crate) fn load(
    commands: &mut Commands,
    flow_info_option: Option<&FlowInfo>,
    flow_state_option: Option<Rc<RefCell<FlowState>>>,
) -> Result<(), ScriptError> {
    if let (Some(flow_info), Some(flow_state)) = (flow_info_option, flow_state_option) {
        commands.set(cm_run_task::create(flow_info, flow_state))?;
    }

    Ok(())
}
