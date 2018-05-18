//! # cli
//!
//! Handles the command line arguments and executes the runner.
//!

#[cfg(test)]
#[path = "./cli_test.rs"]
mod cli_test;

use ci_info;
use clap::{App, Arg, ArgMatches, SubCommand};
use config;
use descriptor;
use environment;
use logger;
use runner;
use types::{CliArgs, GlobalConfig};
use version;

static NAME: &str = "make";
static VERSION: &str = env!("CARGO_PKG_VERSION");
static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
static DEFAULT_TOML: &str = "Makefile.toml";
static DEFAULT_LOG_LEVEL: &str = "info";
static DEFAULT_TASK_NAME: &str = "default";

fn run(cli_args: CliArgs, global_config: &GlobalConfig) {
    logger::init(&cli_args.log_level);

    info!("cargo-{} {}", &NAME, &VERSION);
    debug!("Written By {}", &AUTHOR);

    debug!("Cli Args {:#?}", &cli_args);
    debug!("Global Configuration {:#?}", &global_config);

    // only run check for updates if we are not in a CI env and user didn't ask to skip the check
    if !cli_args.disable_check_for_updates && !ci_info::is_ci()
        && version::should_check(&global_config)
    {
        version::check();
    }

    let cwd_string_option = match cli_args.cwd.clone() {
        Some(value) => Some(value),
        None => match global_config.search_project_root {
            Some(search) => {
                if search {
                    match environment::get_project_root() {
                        Some(value) => Some(value.clone()),
                        None => None,
                    }
                } else {
                    None
                }
            }
            None => None,
        },
    };
    let cwd = match cwd_string_option {
        Some(ref value) => Some(value.as_ref()),
        None => None,
    };
    environment::setup_cwd(cwd);

    let build_file = &cli_args.build_file;
    let task = &cli_args.task;

    info!("Using Build File: {}", &build_file);
    info!("Task: {}", &task);

    let env_file_entries = environment::parse_env_file(cli_args.env_file.clone());
    let env_cli_entries = cli_args.env.clone();
    let env = match env_file_entries {
        Some(mut env_vec1) => match env_cli_entries {
            Some(mut env_vec2) => {
                env_vec1.append(&mut env_vec2);
                Some(env_vec1)
            }
            None => Some(env_vec1),
        },
        None => env_cli_entries,
    };

    let config = descriptor::load(&build_file, env, cli_args.experimental);

    let env_info = environment::setup_env(&config, &task);

    if cli_args.list_all_steps {
        descriptor::list_steps(&config);
    } else if cli_args.print_only {
        runner::print(&config, &task, cli_args.disable_workspace);
    } else {
        runner::run(config, &task, env_info, &cli_args);
    }
}

/// Handles the command line arguments and executes the runner.
fn run_for_args(matches: ArgMatches, global_config: &GlobalConfig) {
    match matches.subcommand_matches(NAME) {
        Some(cmd_matches) => {
            let mut cli_args = CliArgs::new();

            cli_args.env = cmd_matches.values_of_lossy("env");

            cli_args.build_file = cmd_matches
                .value_of("makefile")
                .unwrap_or(&DEFAULT_TOML)
                .to_string();

            cli_args.cwd = match cmd_matches.value_of("cwd") {
                Some(value) => Some(value.to_string()),
                None => None,
            };

            let default_log_level = match global_config.log_level {
                Some(ref value) => value.as_str().clone(),
                None => &DEFAULT_LOG_LEVEL,
            };
            cli_args.log_level = if cmd_matches.is_present("v") {
                "verbose".to_string()
            } else {
                cmd_matches
                    .value_of("loglevel")
                    .unwrap_or(default_log_level)
                    .to_string()
            };

            cli_args.env_file = match cmd_matches.value_of("envfile") {
                Some(value) => Some(value.to_string()),
                None => None,
            };

            cli_args.disable_check_for_updates =
                cmd_matches.is_present("disable-check-for-updates");
            cli_args.experimental = cmd_matches.is_present("experimental");
            cli_args.print_only = cmd_matches.is_present("print-steps");
            cli_args.disable_workspace = cmd_matches.is_present("no-workspace");
            cli_args.disable_on_error = cmd_matches.is_present("no-on-error");
            cli_args.list_all_steps = cmd_matches.is_present("list-steps");

            let default_task_name = match global_config.default_task_name {
                Some(ref value) => value.as_str().clone(),
                None => &DEFAULT_TASK_NAME,
            };
            let task = cmd_matches.value_of("task").unwrap_or(default_task_name);
            cli_args.task = cmd_matches.value_of("TASK").unwrap_or(task).to_string();

            run(cli_args, global_config);
        }
        None => panic!("cargo-{} not invoked via cargo command.", NAME),
    }
}

fn create_cli<'a, 'b>(global_config: &'a GlobalConfig) -> App<'a, 'b> {
    let default_task_name = match global_config.default_task_name {
        Some(ref value) => value.as_str().clone(),
        None => &DEFAULT_TASK_NAME,
    };
    let default_log_level = match global_config.log_level {
        Some(ref value) => value.as_str().clone(),
        None => &DEFAULT_LOG_LEVEL,
    };

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
                    .default_value(&DEFAULT_TOML),
            )
            .arg(
                Arg::with_name("task")
                    .short("-t")
                    .long("--task")
                    .value_name("TASK")
                    .help(
                        "The task name to execute \
                         (can omit the flag if the task name is the last argument)",
                    )
                    .default_value(default_task_name),
            )
            .arg(
                Arg::with_name("cwd")
                    .long("--cwd")
                    .value_name("DIRECTORY")
                    .help(
                        "Will set the current working directory. \
                         The search for the makefile will be from this directory if defined.",
                    ),
            )
            .arg(Arg::with_name("no-workspace").long("--no-workspace").help(
                "Disable workspace support (tasks are triggered on workspace and not on members)",
            ))
            .arg(
                Arg::with_name("no-on-error")
                    .long("--no-on-error")
                    .help("Disable on error flow even if defined in config sections"),
            )
            .arg(
                Arg::with_name("envfile")
                    .long("--env-file")
                    .value_name("FILE")
                    .help("Set environment variables from provided file path")
                    .default_value(&DEFAULT_TOML),
            )
            .arg(
                Arg::with_name("env")
                    .long("--env")
                    .short("-e")
                    .value_name("ENV")
                    .multiple(true)
                    .takes_value(true)
                    .number_of_values(1)
                    .help("Set environment variables"),
            )
            .arg(
                Arg::from_usage("-l, --loglevel=[LOG LEVEL] 'The log level'")
                    .possible_values(&["verbose", "info", "error"])
                    .default_value(default_log_level),
            )
            .arg(
                Arg::with_name("v")
                    .short("-v")
                    .long("--verbose")
                    .help("Sets the log level to verbose (shorthand for --loglevel verbose)"),
            )
            .arg(
                Arg::with_name("experimental")
                    .long("--experimental")
                    .help("Allows access unsupported experimental predefined tasks."),
            )
            .arg(
                Arg::with_name("disable-check-for-updates")
                    .long("--disable-check-for-updates")
                    .help("Disables the update check during startup"),
            )
            .arg(Arg::with_name("print-steps").long("--print-steps").help(
                "Only prints the steps of the build in the order they will \
                 be invoked but without invoking them",
            ))
            .arg(
                Arg::with_name("list-steps")
                    .long("--list-all-steps")
                    .help("Lists all known steps"),
            )
            .arg(Arg::with_name("TASK")),
    )
}

/// Handles the command line arguments and executes the runner.
pub(crate) fn run_cli() {
    let global_config = config::load();

    let app = create_cli(&global_config);

    let matches = app.get_matches();

    run_for_args(matches, &global_config);
}
