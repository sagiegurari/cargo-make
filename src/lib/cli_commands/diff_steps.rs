//! # diff_steps
//!
//! Prints a diff of execution plans.
//!

#[cfg(test)]
#[path = "./diff_steps_test.rs"]
mod diff_steps_test;

use crate::command;
use crate::execution_plan::create as create_execution_plan;
use crate::io::{create_file, delete_file};
use crate::types::{CliArgs, Config, ExecutionPlan};
use std::fs::File;
use std::io::{BufWriter, Write};

fn write_as_string(execution_plan: &ExecutionPlan, file: &File, file_path: &str) {
    let mut writer = BufWriter::new(file);
    match writeln!(&mut writer, "{:#?}", &execution_plan.steps) {
        Err(error) => {
            error!("Unable to write to file: {} {:#?}", &file_path, &error);
            panic!("Unable to write to file, error: {}", error);
        }
        _ => (),
    };
}

/// Runs the execution plan diff
pub(crate) fn run(
    internal_config: &Config,
    external_config: &Config,
    task: &str,
    cli_args: &CliArgs,
) {
    let internal_execution_plan = create_execution_plan(
        internal_config,
        &task,
        cli_args.disable_workspace,
        true,
        false,
    );

    let external_execution_plan = create_execution_plan(
        external_config,
        &task,
        cli_args.disable_workspace,
        true,
        false,
    );

    let internal_file = create_file(
        &move |file: &mut File, file_path: &str| {
            write_as_string(&internal_execution_plan, &file, &file_path)
        },
        "toml",
    );
    let external_file = create_file(
        &move |file: &mut File, file_path: &str| {
            write_as_string(&external_execution_plan, &file, &file_path)
        },
        "toml",
    );

    info!("Printing diff...");
    command::run_command(
        "git",
        &Some(vec![
            "diff".to_string(),
            "--no-index".to_string(),
            internal_file.to_string(),
            external_file.to_string(),
        ]),
        false,
    );

    delete_file(&internal_file);
    delete_file(&external_file);

    info!("Done");
}
