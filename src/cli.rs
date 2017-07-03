//! # cli
//!
//! Handles the command line arguments and executes the runner.
//!

#[cfg(test)]
#[path = "./cli_test.rs"]
mod cli_test;

use clap::{App, Arg, ArgMatches, SubCommand};
use descriptor;
use log;
use runner;

static NAME: &str = "make";
static VERSION: &str = env!("CARGO_PKG_VERSION");
static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
static DEFAULT_TOML: &str = "Makefile.toml";
static DEFAULT_LOG_LEVEL: &str = "info";
static DEFAULT_TASK_NAME: &str = "default";

fn run(
    build_file: &str,
    task: &str,
    log_level: &str,
    print_only: bool,
) {
    let logger = log::create(log_level);

    logger.info::<()>("Using Build File: ", &[build_file], None);
    logger.info::<()>("Task: ", &[task], None);

    let config = descriptor::load(&build_file, &logger);

    if print_only {
        runner::print(&logger, &config, &task);
    } else {
        runner::run(&logger, &config, &task);
    }
}

/// Handles the command line arguments and executes the runner.
fn run_for_args(matches: ArgMatches) {
    match matches.subcommand_matches(NAME) {
        Some(cmd_matches) => {
            let build_file = cmd_matches.value_of("makefile").unwrap_or(&DEFAULT_TOML);
            let mut task = cmd_matches.value_of("task").unwrap_or(&DEFAULT_TASK_NAME);
            task = cmd_matches.value_of("TASK").unwrap_or(task);

            let log_level = if cmd_matches.is_present("v") {
                "verbose"
            } else {
                cmd_matches.value_of("loglevel").unwrap_or(&DEFAULT_LOG_LEVEL)
            };

            let print_only = cmd_matches.is_present("print-steps");

            run(build_file, task, log_level, print_only);
        }
        None => panic!("cargo-{} not invoked via cargo command.", NAME),
    }
}

fn create_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("cargo").subcommand(
        SubCommand::with_name(NAME)
            .version(VERSION)
            .author(AUTHOR)
            .about(DESCRIPTION)
            .arg(
                Arg::with_name("makefile")
                    .long("--makefile")
                    .value_name("FILE")
                    .help("The optional toml file containing the tasks definitions")
                    .default_value(&DEFAULT_TOML)
            )
            .arg(
                Arg::with_name("task")
                    .short("-t")
                    .long("--task")
                    .value_name("TASK NAME")
                    .help("The task name to execute (can omit the flag if the task name is the last argument)")
                    .default_value(&DEFAULT_TASK_NAME)
            )
            .arg(
                Arg::from_usage("-l, --loglevel=[LOG LEVEL] 'The log level'")
                    .possible_values(&["verbose", "info", "error"])
                    .default_value(&DEFAULT_LOG_LEVEL)
            )
            .arg(Arg::with_name("v").short("-v").long("--verbose").help(
                "Sets the log level to verbose (shorthand for --loglevel verbose)"
            ))
            .arg(Arg::with_name("print-steps").long("--print-steps").help(
                "Only prints the steps of the build in the order they will be invoked but without invoking them"
            ))
            .arg(Arg::with_name("TASK"))
    )
}

/// Handles the command line arguments and executes the runner.
pub fn run_cli() {
    let app = create_cli();

    let matches = app.get_matches();

    run_for_args(matches);
}
