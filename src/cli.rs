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
) {
    let logger = log::create(log_level);

    logger.info::<()>("Using Build File: ", &[build_file], None);
    logger.info::<()>("Task: ", &[task], None);

    let config = descriptor::load(&build_file, &logger);

    runner::run(&logger, &config, &task);
}

/// Handles the command line arguments and executes the runner.
fn run_for_args(matches: ArgMatches) {
    match matches.subcommand_matches(NAME) {
        Some(cmd_matches) => {
            let build_file = cmd_matches.value_of("buildFile").unwrap_or(&DEFAULT_TOML);
            let task = cmd_matches.value_of("task").unwrap_or(&DEFAULT_TASK_NAME);
            let mut log_level = cmd_matches.value_of("loglevel").unwrap_or(&DEFAULT_LOG_LEVEL);

            if cmd_matches.is_present("v") {
                log_level = "verbose";
            }

            run(build_file, task, log_level);
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
            .arg(Arg::from_usage("-b, --buildFile=[FILE] 'The optional toml file containing the build descriptor'").default_value(&DEFAULT_TOML))
            .arg(Arg::from_usage("-t, --task=[TASK NAME] 'The task name to execute'").default_value(&DEFAULT_TASK_NAME))
            .arg(
                Arg::from_usage("-l, --loglevel=[LOG LEVEL] 'The log level'")
                    .possible_values(&["verbose", "info", "error"])
                    .default_value(&DEFAULT_LOG_LEVEL)
            )
            .arg(Arg::with_name("v").short("-v").help(
                "Sets the log level to verbose (shorthand for --loglevel verbose)"
            ))
    )
}

/// Handles the command line arguments and executes the runner.
pub fn run_cli() {
    let app = create_cli();

    let matches = app.get_matches();

    run_for_args(matches);
}
