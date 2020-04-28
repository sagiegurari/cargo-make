//! # sdk
//!
//! Loads the cargo-make duckscript SDK.
//!

mod cm_run_task;

use crate::scriptengine::duck_script;
use crate::types::FlowInfo;
use duckscript::types::command::{CommandResult, Commands};
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::StateValue;
use std::collections::HashMap;

/// Loads all core commands
pub(crate) fn load(commands: &mut Commands) -> Result<(), ScriptError> {
    commands.set(cm_run_task::create())?;

    Ok(())
}

pub(crate) fn run_in_flow_context(
    state: &mut HashMap<String, StateValue>,
    func: &Fn(&FlowInfo) -> CommandResult,
) -> CommandResult {
    match state.get(duck_script::FLOW_INFO_KEY) {
        Some(state_value) => match state_value {
            StateValue::Any(rc_value) => {
                let value_any = rc_value.borrow();

                match value_any.downcast_ref::<FlowInfo>() {
                    Some(flow_info) => func(flow_info),
                    None => CommandResult::Error(
                        "Flow information not available, invalid type.".to_string(),
                    ),
                }
            }
            _ => CommandResult::Error(
                "Flow information not available, invalid state value type.".to_string(),
            ),
        },
        None => CommandResult::Error("Flow information not available.".to_string()),
    }
}
