//! # print_steps
//!
//! Prints the execution plan in multiple formats.
//!

#[cfg(test)]
#[path = "print_steps_test.rs"]
mod print_steps_test;

use crate::error::CargoMakeError;
use std::io;

use crate::execution_plan::ExecutionPlanBuilder;
use crate::types::{Config, CrateInfo, ExecutionPlan, SerdeRegex};
use regex::Regex;

#[derive(Debug)]
enum PrintFormat {
    /// The default format
    Default,
    /// Prints a short description of the task
    ShortDescription,
}

impl PartialEq for PrintFormat {
    fn eq(&self, other: &PrintFormat) -> bool {
        match self {
            PrintFormat::Default => match other {
                PrintFormat::Default => true,
                _ => false,
            },
            PrintFormat::ShortDescription => match other {
                PrintFormat::ShortDescription => true,
                _ => false,
            },
        }
    }
}

fn get_format_type(output_format: &str) -> PrintFormat {
    if output_format == "short-description" {
        PrintFormat::ShortDescription
    } else {
        PrintFormat::Default
    }
}

fn print_short_description(
    output_buffer: &mut impl io::Write,
    execution_plan: &ExecutionPlan,
) -> io::Result<()> {
    let mut counter = 1;
    for step in &execution_plan.steps[execution_plan.steps_to_run.clone()] {
        let task = &step.config;
        let description = match &task.description {
            Some(value) => value,
            None => "no description",
        };
        writeln!(
            output_buffer,
            "{}. {} - {}",
            counter, &step.name, &description
        )?;

        counter += 1;
    }
    Ok(())
}

fn print_default(
    output_buffer: &mut impl io::Write,
    execution_plan: &ExecutionPlan,
) -> io::Result<()> {
    let steps: Vec<crate::types::Step> = {
        let mut _steps =
            Vec::<crate::types::Step>::with_capacity(execution_plan.steps_to_run.len());
        execution_plan.steps[execution_plan.steps_to_run.clone()].clone_into(&mut _steps);
        _steps
    };
    writeln!(
        output_buffer,
        "{:#?}",
        ExecutionPlan {
            steps,
            ..ExecutionPlan::default()
        }
    )
}

/// Only prints the execution plan
pub fn print(
    output_buffer: &mut impl io::Write,
    config: &Config,
    task: &str,
    output_format: &str,
    disable_workspace: bool,
    skip_tasks_pattern: &Option<String>,
    crateinfo: &CrateInfo,
    skip_init_end_tasks: bool,
) -> Result<(), CargoMakeError> {
    let skip_tasks_pattern_regex = match skip_tasks_pattern {
        Some(ref pattern) => match Regex::new(pattern) {
            Ok(reg) => Some(SerdeRegex(reg)),
            Err(_) => {
                warn!("Invalid skip tasks pattern provided: {}", pattern);
                None
            }
        },
        None => None,
    };

    let execution_plan = ExecutionPlanBuilder {
        crate_info: Some(crateinfo),
        disable_workspace,
        skip_tasks_pattern: skip_tasks_pattern_regex.as_ref(),
        skip_init_end_tasks,
        ..ExecutionPlanBuilder::new(&config, &task)
    }
    .build()?;
    debug!("Created execution plan: {:#?}", &execution_plan);

    let print_format = get_format_type(&output_format);

    match print_format {
        PrintFormat::ShortDescription => print_short_description(output_buffer, &execution_plan)?,
        PrintFormat::Default => print_default(output_buffer, &execution_plan)?,
    };
    Ok(())
}
