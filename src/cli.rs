//! # cli
//!
//! Handles the command line arguments and executes the runner.
//!

#[cfg(test)]
#[path = "./cli_test.rs"]
mod cli_test;

use clap::{App, Arg, ArgMatches, SubCommand};
use descriptor;
use environment;
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
    cwd: Option<&str>,
    disable_workspace: bool,
    print_only: bool,
) {
    let logger = log::create(log_level);

    logger.info::<()>("cargo-", &[&NAME, " ", &VERSION], None);
    logger.verbose::<()>("Written By ", &[&AUTHOR], None);

    environment::setup_cwd(&logger, cwd);

    logger.info::<()>("Using Build File: ", &[build_file], None);
    logger.info::<()>("Task: ", &[task], None);

    let config = descriptor::load(&build_file, &logger);

    environment::setup_env(&logger, &config, &task);

    if print_only {
        runner::print(&logger, &config, &task, disable_workspace);
    } else {
        runner::run(&logger, &config, &task, disable_workspace);
    }
}

/// Handles the command line arguments and executes the runner.
fn run_for_args(matches: ArgMatches) {
    match matches.subcommand_matches(NAME) {
        Some(cmd_matches) => {
            let build_file = cmd_matches.value_of("makefile").unwrap_or(&DEFAULT_TOML);
            let mut task = cmd_matches.value_of("task").unwrap_or(&DEFAULT_TASK_NAME);
            task = cmd_matches.value_of("TASK").unwrap_or(task);
            let cwd = cmd_matches.value_of("cwd");

            let log_level = if cmd_matches.is_present("v") {
                "verbose"
            } else {
                cmd_matches.value_of("loglevel").unwrap_or(&DEFAULT_LOG_LEVEL)
            };

            let print_only = cmd_matches.is_present("print-steps");
            let disable_workspace = cmd_matches.is_present("no-workspace");

            run(build_file, task, log_level, cwd, disable_workspace, print_only);
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
                    .value_name("TASK")
                    .help("The task name to execute (can omit the flag if the task name is the last argument)")
                    .default_value(&DEFAULT_TASK_NAME)
            )
            .arg(Arg::with_name("cwd").long("--cwd").value_name("DIRECTORY").help(
                "Will set the current working directory. The search for the makefile will be from this directory if defined."
            ))
            .arg(Arg::with_name("no-workspace").long("--no-workspace").help(
                "Disable workspace support (tasks are triggered on workspace and not on members)"
            ))
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
