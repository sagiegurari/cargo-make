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
use types::CliArgs;

static NAME: &str = "make";
static VERSION: &str = env!("CARGO_PKG_VERSION");
static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
static DEFAULT_TOML: &str = "Makefile.toml";
static DEFAULT_LOG_LEVEL: &str = "info";
static DEFAULT_TASK_NAME: &str = "default";

fn run(cli_args: CliArgs) {
    let logger = log::create(&cli_args.log_level);

    logger.info::<()>("cargo-", &[&NAME, " ", &VERSION], None);
    logger.verbose::<()>("Written By ", &[&AUTHOR], None);

    logger.verbose("Cli Args", &[], Some(&cli_args));

    let cwd = match cli_args.cwd {
        Some(ref value) => Some(value.as_ref()),
        None => None,
    };
    environment::setup_cwd(&logger, cwd);

    let build_file = &cli_args.build_file;
    let task = &cli_args.task;

    logger.info::<()>("Using Build File: ", &[build_file], None);
    logger.info::<()>("Task: ", &[task], None);

    let env = cli_args.env.clone();

    let config = descriptor::load(&build_file, env, cli_args.experimental, &logger);

    let env_info = environment::setup_env(&logger, &config, &task);

    if cli_args.list_all_steps {
        descriptor::list_steps(&config);
    } else if cli_args.print_only {
        runner::print(&logger, &config, &task, cli_args.disable_workspace);
    } else {
        runner::run(&logger, config, &task, env_info, cli_args.disable_workspace);
    }
}

/// Handles the command line arguments and executes the runner.
fn run_for_args(matches: ArgMatches) {
    match matches.subcommand_matches(NAME) {
        Some(cmd_matches) => {
            let mut cli_args = CliArgs::new();

            cli_args.env = cmd_matches.values_of_lossy("env");

            cli_args.build_file = cmd_matches.value_of("makefile").unwrap_or(&DEFAULT_TOML).to_string();

            cli_args.cwd = match cmd_matches.value_of("cwd") {
                Some(value) => Some(value.to_string()),
                None => None,
            };

            cli_args.log_level = if cmd_matches.is_present("v") {
                "verbose".to_string()
            } else {
                cmd_matches.value_of("loglevel").unwrap_or(&DEFAULT_LOG_LEVEL).to_string()
            };

            cli_args.experimental = cmd_matches.is_present("experimental");
            cli_args.print_only = cmd_matches.is_present("print-steps");
            cli_args.disable_workspace = cmd_matches.is_present("no-workspace");
            cli_args.list_all_steps = cmd_matches.is_present("list-steps");

            let task = cmd_matches.value_of("task").unwrap_or(&DEFAULT_TASK_NAME);
            cli_args.task = cmd_matches.value_of("TASK").unwrap_or(task).to_string();

            run(cli_args);
        }
        None => panic!("cargo-{} not invoked via cargo command.", NAME),
    }
}

fn create_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("cargo").bin_name("cargo").subcommand(
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
                Arg::with_name("env")
                    .long("--env")
                    .short("-e")
                    .value_name("ENV")
                    .multiple(true)
                    .takes_value(true)
                    .number_of_values(1)
                    .help("Set environment variables")
            )
            .arg(
                Arg::from_usage("-l, --loglevel=[LOG LEVEL] 'The log level'")
                    .possible_values(&["verbose", "info", "error"])
                    .default_value(&DEFAULT_LOG_LEVEL)
            )
            .arg(Arg::with_name("v").short("-v").long("--verbose").help(
                "Sets the log level to verbose (shorthand for --loglevel verbose)"
            ))
            .arg(Arg::with_name("experimental").long("--experimental").help(
                "Allows access unsupported experimental predefined tasks."
            ))
            .arg(Arg::with_name("print-steps").long("--print-steps").help(
                "Only prints the steps of the build in the order they will be invoked but without invoking them"
            ))
            .arg(Arg::with_name("list-steps").long("--list-all-steps").help("Lists all known steps"))
            .arg(Arg::with_name("TASK"))
    )
}

/// Handles the command line arguments and executes the runner.
pub fn run_cli() {
    let app = create_cli();

    let matches = app.get_matches();

    run_for_args(matches);
}
