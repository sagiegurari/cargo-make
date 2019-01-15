//! # print_steps
//!
//! Prints the execution plan in multiple formats.
//!

#[cfg(test)]
#[path = "./print_steps_test.rs"]
mod print_steps_test;

use crate::execution_plan::create as create_execution_plan;
use crate::types::{Config, ExecutionPlan};

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

fn print_short_description(execution_plan: &ExecutionPlan) {
    let mut counter = 1;
    for step in &execution_plan.steps {
        let task = &step.config;
        let description = match &task.description {
            Some(value) => value,
            None => "no description",
        };
        println!("{}. {} - {}", counter, &step.name, &description);

        counter = counter + 1;
    }
}

fn print_default(execution_plan: &ExecutionPlan) {
    println!("{:#?}", &execution_plan);
}

/// Only prints the execution plan
pub(crate) fn print(config: &Config, task: &str, output_format: &str, disable_workspace: bool) {
    let execution_plan = create_execution_plan(&config, &task, disable_workspace, false);
    debug!("Created execution plan: {:#?}", &execution_plan);

    let print_format = get_format_type(&output_format);

    match print_format {
        PrintFormat::ShortDescription => print_short_description(&execution_plan),
        PrintFormat::Default => print_default(&execution_plan),
    };
}
