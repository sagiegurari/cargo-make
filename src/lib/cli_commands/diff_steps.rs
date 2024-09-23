//! # diff_steps
//!
//! Prints a diff of execution plans.
//!

#[cfg(test)]
#[path = "diff_steps_test.rs"]
mod diff_steps_test;

use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

use regex::Regex;

use crate::command;
use crate::error::CargoMakeError;
use crate::execution_plan::ExecutionPlanBuilder;
use crate::io::{create_file, delete_file};
use crate::types::{CliArgs, Config, CrateInfo, ExecutionPlan};

fn write_as_string(execution_plan: &ExecutionPlan, file: &File) -> io::Result<()> {
    let mut writer = BufWriter::new(file);
    writeln!(&mut writer, "{:#?}", &execution_plan.steps)
}

/// Runs the execution plan diff
pub(crate) fn run(
    internal_config: &Config,
    external_config: &Config,
    task: &str,
    cli_args: &CliArgs,
    crateinfo: &CrateInfo,
) -> Result<(), CargoMakeError> {
    let skip_tasks_pattern = match cli_args.skip_tasks_pattern {
        Some(ref pattern) => match Regex::new(pattern) {
            Ok(reg) => Some(reg),
            Err(_) => {
                warn!("Invalid skip tasks pattern provided: {}", pattern);
                None
            }
        },
        None => None,
    };

    let internal_execution_plan = ExecutionPlanBuilder {
        crate_info: Some(crateinfo),
        disable_workspace: cli_args.disable_workspace,
        allow_private: true,
        skip_tasks_pattern: skip_tasks_pattern.as_ref(),
        ..ExecutionPlanBuilder::new(internal_config, &task)
    }
    .build()?;

    let external_execution_plan = ExecutionPlanBuilder {
        crate_info: Some(crateinfo),
        disable_workspace: cli_args.disable_workspace,
        allow_private: true,
        skip_tasks_pattern: skip_tasks_pattern.as_ref(),
        ..ExecutionPlanBuilder::new(external_config, &task)
    }
    .build()?;

    let internal_file = create_file(
        &move |file: &mut File| write_as_string(&internal_execution_plan, &file),
        "toml",
    )?;
    let external_file = create_file(
        &move |file: &mut File| write_as_string(&external_execution_plan, &file),
        "toml",
    )?;

    info!("Printing diff...");
    command::run_command(
        "git",
        &Some(vec![
            "--no-pager".to_string(),
            "diff".to_string(),
            "--no-index".to_string(),
            internal_file.to_string(),
            external_file.to_string(),
        ]),
        false,
    )?;

    delete_file(&internal_file);
    delete_file(&external_file);

    info!("Done");

    Ok(())
}
