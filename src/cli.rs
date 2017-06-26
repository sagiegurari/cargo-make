//! # cli
//!
//! Handles the command line arguments and executes the runner.
//!

use clap::{App, Arg, SubCommand};
use descriptor;
use log;
use runner;

static NAME: &str = "make";
static VERSION: &str = env!("CARGO_PKG_VERSION");
static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
static DEFAULT_TOML: &str = "Makefile.toml";

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
pub fn run_cli() {
    let mut build_file_arg = "-b, --buildFile=[FILE] 'Build toml file containing the build descriptor (default: ".to_string();
    build_file_arg.push_str(&DEFAULT_TOML);
    build_file_arg.push_str(" if exists)'");

    let matches = App::new("cargo")
        .subcommand(
            SubCommand::with_name(NAME)
                .version(VERSION)
                .author(AUTHOR)
                .about(DESCRIPTION)
                .arg_from_usage(&build_file_arg)
                .arg_from_usage("-t, --task=[TASK NAME] 'The task name to execute (default: default)'")
                .arg(Arg::from_usage("-l, --loglevel=[LOG LEVEL] 'The log level (default: info)'").possible_values(&["verbose", "info", "error"]))
        )
        .get_matches();

    match matches.subcommand_matches(NAME) {
        Some(cmd_matches) => {
            let build_file = cmd_matches.value_of("buildFile").unwrap_or(&DEFAULT_TOML);
            let task = cmd_matches.value_of("task").unwrap_or("default");
            let log_level = cmd_matches.value_of("loglevel").unwrap_or("info");

            run(build_file, task, log_level);
        }
        None => panic!("cargo-{} not invoked via cargo command.", NAME),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_empty_task() {
        run("bad.toml", "empty", "error");
    }

    #[test]
    fn run_file_and_task() {
        run("./examples/dependencies.toml", "A", "error");
    }
}
