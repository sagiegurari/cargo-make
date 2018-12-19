//! # print
//!
//! Prints the execution plan in multiple formats.
//!

#[cfg(test)]
#[path = "./print_test.rs"]
mod print_test;

use crate::execution_plan::create as create_execution_plan;
use crate::types::Config;

/// Only prints the execution plan
pub(crate) fn print(config: &Config, task: &str, disable_workspace: bool) {
    let execution_plan = create_execution_plan(&config, &task, disable_workspace, false);
    debug!("Created execution plan: {:#?}", &execution_plan);

    println!("{:#?}", &execution_plan);
}
