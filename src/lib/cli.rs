//! # cli
//!
//! The cargo-make cli
//!

#[cfg(test)]
#[path = "cli_test.rs"]
mod cli_test;

use crate::cli_commands;
use crate::config;
use crate::descriptor;
use crate::environment;
use crate::logger;
use crate::logger::LoggerOptions;
use crate::profile;
use crate::recursion_level;
use crate::runner;
use crate::time_summary;
use crate::types::{CliArgs, GlobalConfig};
use crate::version;
use clap::{Arg, ArgMatches, Command};
use std::time::SystemTime;

static VERSION: &str = env!("CARGO_PKG_VERSION");
static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
static DEFAULT_TOML: &str = "Makefile.toml";
static DEFAULT_LOG_LEVEL: &str = "info";
static DEFAULT_TASK_NAME: &str = "default";
static DEFAULT_OUTPUT_FORMAT: &str = "default";

fn run(cli_args: CliArgs, global_config: &GlobalConfig) {
    let start_time = SystemTime::now();

    recursion_level::increment();

    logger::init(&LoggerOptions {
        level: cli_args.log_level.clone(),
        color: !cli_args.disable_color,
    });

    if recursion_level::is_top() {
        info!("{} {}", &cli_args.command, &VERSION);
        debug!("Written By {}", &AUTHOR);
    }

    debug!("Cli Args {:#?}", &cli_args);
    debug!("Global Configuration {:#?}", &global_config);

    if version::should_check(&cli_args, &global_config) {
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
    let home = environment::setup_cwd(cwd);

    let force_makefile = cli_args.build_file.is_some();
    let build_file = &cli_args
        .build_file
        .clone()
        .unwrap_or(DEFAULT_TOML.to_string());
    let task = &cli_args.task;
    let profile_name = &cli_args
        .profile
        .clone()
        .unwrap_or(profile::DEFAULT_PROFILE.to_string());
    let normalized_profile_name = profile::set(&profile_name);

    environment::load_env_file(cli_args.env_file.clone());

    let env = cli_args.env.clone();

    let experimental = cli_args.experimental;
    let descriptor_load_result = descriptor::load(&build_file, force_makefile, env, experimental);

    let config = match descriptor_load_result {
        Ok(config) => config,
        Err(ref min_version) => {
            error!(
                "{} version: {} does not meet minimum required version: {}",
                &cli_args.command, &VERSION, &min_version
            );
            panic!(
                "{} version: {} does not meet minimum required version: {}",
                &cli_args.command, &VERSION, &min_version
            );
        }
    };
    let mut time_summary_vec = vec![];
    time_summary::add(
        &mut time_summary_vec,
        "[Load Makefiles]",
        start_time.clone(),
    );
    let step_time = SystemTime::now();

    match config.config.additional_profiles {
        Some(ref profiles) => profile::set_additional(profiles),
        None => profile::set_additional(&vec![]),
    };

    let env_info = environment::setup_env(&cli_args, &config, &task, home);
    time_summary::add(&mut time_summary_vec, "[Setup Env]", step_time);

    let crate_name = envmnt::get_or("CARGO_MAKE_CRATE_NAME", "");
    if crate_name.len() > 0 {
        info!("Project: {}", &crate_name);
    }
    info!("Build File: {}", &build_file);
    info!("Task: {}", &task);
    info!("Profile: {}", &normalized_profile_name);

    // ensure profile env was not overridden
    profile::set(&normalized_profile_name);

    if cli_args.list_all_steps || cli_args.list_category_steps.is_some() {
        cli_commands::list_steps::run(
            &config,
            &cli_args.output_format,
            &cli_args.output_file,
            cli_args.list_category_steps,
        );
    } else if cli_args.diff_execution_plan {
        let default_config = descriptor::load_internal_descriptors(true, experimental, None);
        cli_commands::diff_steps::run(&default_config, &config, &task, &cli_args);
    } else if cli_args.print_only {
        cli_commands::print_steps::print(
            &config,
            &task,
            &cli_args.output_format,
            cli_args.disable_workspace,
            cli_args.skip_tasks_pattern,
        );
    } else {
        runner::run(
            config,
            &task,
            env_info,
            &cli_args,
            start_time,
            time_summary_vec,
        );
    }
}

/// Handles the command line arguments and executes the runner.
fn run_for_args(
    matches: ArgMatches,
    global_config: &GlobalConfig,
    command_name: &String,
    sub_command: bool,
) {
    let cmd_matches = if sub_command {
        match matches.subcommand_matches(command_name) {
            Some(value) => value,
            None => panic!("cargo-{} not invoked via cargo command.", &command_name),
        }
    } else {
        &matches
    };

    let mut cli_args = CliArgs::new();

    cli_args.command = if sub_command {
        let mut binary = "cargo ".to_string();
        binary.push_str(&command_name);
        binary
    } else {
        command_name.clone()
    };

    cli_args.env = cmd_matches.values_of_lossy("env");

    cli_args.build_file = if cmd_matches.occurrences_of("makefile") == 0 {
        None
    } else {
        let makefile = cmd_matches
            .value_of("makefile")
            .unwrap_or(&DEFAULT_TOML)
            .to_string();
        Some(makefile)
    };

    cli_args.cwd = match cmd_matches.value_of("cwd") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    let default_log_level = match global_config.log_level {
        Some(ref value) => value.as_str(),
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

    let default_disable_color = match global_config.disable_color {
        Some(value) => value,
        None => false,
    };
    cli_args.disable_color = cmd_matches.is_present("no-color")
        || envmnt::is("CARGO_MAKE_DISABLE_COLOR")
        || default_disable_color;

    cli_args.print_time_summary =
        cmd_matches.is_present("time-summary") || envmnt::is("CARGO_MAKE_PRINT_TIME_SUMMARY");

    cli_args.env_file = match cmd_matches.value_of("envfile") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    cli_args.output_format = cmd_matches
        .value_of("output-format")
        .unwrap_or(DEFAULT_OUTPUT_FORMAT)
        .to_string();

    cli_args.list_category_steps = match cmd_matches.value_of("list-category-steps") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    cli_args.output_file = match cmd_matches.value_of("output_file") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    let profile_name = cmd_matches
        .value_of("profile".to_string())
        .unwrap_or(profile::DEFAULT_PROFILE);
    cli_args.profile = Some(profile_name.to_string());

    cli_args.disable_check_for_updates = cmd_matches.is_present("disable-check-for-updates");
    cli_args.experimental = cmd_matches.is_present("experimental");
    cli_args.print_only = cmd_matches.is_present("print-steps");
    cli_args.disable_workspace = cmd_matches.is_present("no-workspace");
    cli_args.disable_on_error = cmd_matches.is_present("no-on-error");
    cli_args.allow_private = cmd_matches.is_present("allow-private");
    cli_args.skip_init_end_tasks = cmd_matches.is_present("skip-init-end-tasks");
    cli_args.list_all_steps = cmd_matches.is_present("list-steps");
    cli_args.diff_execution_plan = cmd_matches.is_present("diff-steps");

    cli_args.skip_tasks_pattern = match cmd_matches.value_of("skip-tasks-pattern") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    let default_task_name = match global_config.default_task_name {
        Some(ref value) => value.as_str(),
        None => &DEFAULT_TASK_NAME,
    };
    let task = cmd_matches.value_of("task").unwrap_or(default_task_name);
    let task_cmd = cmd_matches
        .values_of("TASK_CMD")
        .unwrap_or_default()
        .collect::<Vec<_>>();
    let (task, arguments) = match task_cmd.as_slice() {
        &[] => (task, None),
        &[task_name, ref task_args @ ..] => {
            let args_strings = task_args.iter().map(|item| item.to_string()).collect();
            (task_name, Some(args_strings))
        }
    };
    cli_args.task = task.to_string();
    cli_args.arguments = arguments;

    run(cli_args, global_config);
}

fn create_cli<'a>(
    global_config: &'a GlobalConfig,
    command_name: &String,
    sub_command: bool,
) -> Command<'a> {
    let default_task_name = match global_config.default_task_name {
        Some(ref value) => value.as_str(),
        None => &DEFAULT_TASK_NAME,
    };
    let default_log_level = match global_config.log_level {
        Some(ref value) => value.as_str(),
        None => &DEFAULT_LOG_LEVEL,
    };

    let mut cli_app = if sub_command {
        Command::new(command_name)
    } else {
        let name = command_name.as_str();
        Command::new(name).bin_name(name)
    };

    cli_app = cli_app
        .version(VERSION)
        .author(AUTHOR)
        .about(DESCRIPTION)
        .allow_hyphen_values(true)
        .trailing_var_arg(true)
        .help_expected(true)
        .arg(
            Arg::new("makefile")
                .long("--makefile")
                .value_name("FILE")
                .help("The optional toml file containing the tasks definitions")
                .default_value(&DEFAULT_TOML),
        )
        .arg(
            Arg::new("task")
                .short('t')
                .long("--task")
                .value_name("TASK")
                .help(
                    "The task name to execute \
                     (can omit the flag if the task name is the last argument)",
                )
                .default_value(default_task_name),
        )
        .arg(
            Arg::new("profile")
                .short('p')
                .long("--profile")
                .value_name("PROFILE")
                .help(
                    "The profile name (will be converted to lower case)",
                )
                .default_value(&profile::DEFAULT_PROFILE),
        )
        .arg(
            Arg::new("cwd")
                .long("--cwd")
                .value_name("DIRECTORY")
                .help(
                    "Will set the current working directory. \
                     The search for the makefile will be from this directory if defined.",
                ),
        )
        .arg(Arg::new("no-workspace").long("--no-workspace").help(
            "Disable workspace support (tasks are triggered on workspace and not on members)",
        ))
        .arg(
            Arg::new("no-on-error")
                .long("--no-on-error")
                .help("Disable on error flow even if defined in config sections"),
        )
        .arg(
            Arg::new("allow-private")
                .long("--allow-private")
                .help("Allow invocation of private tasks"),
        )
        .arg(
            Arg::new("skip-init-end-tasks")
                .long("--skip-init-end-tasks")
                .help("If set, init and end tasks are skipped"),
        )
        .arg(
            Arg::new("skip-tasks-pattern")
                .long("--skip-tasks")
                .value_name("SKIP_TASK_PATTERNS")
                .help("Skip all tasks that match the provided regex (example: pre.*|post.*)"),
        )
        .arg(
            Arg::new("envfile")
                .long("--env-file")
                .value_name("FILE")
                .help("Set environment variables from provided file"),
        )
        .arg(
            Arg::new("env")
                .long("--env")
                .short('e')
                .value_name("ENV")
                .multiple_occurrences(true)
                .takes_value(true)
                .number_of_values(1)
                .allow_invalid_utf8(true)
                .help("Set environment variables"),
        )
        .arg(
            Arg::new("loglevel")
                .long("--loglevel")
                .short('l')
                .value_name("LOG LEVEL")
                .possible_values(&["verbose", "info", "error"])
                .default_value(default_log_level)
                .help("The log level"),
        )
        .arg(
            Arg::new("v")
                .short('v')
                .long("--verbose")
                .help("Sets the log level to verbose (shorthand for --loglevel verbose)"),
        )
        .arg(
            Arg::new("no-color")
                .long("--no-color")
                .help("Disables colorful output"),
        )
        .arg(
            Arg::new("time-summary")
                .long("--time-summary")
                .help("Print task level time summary at end of flow"),
        )
        .arg(
            Arg::new("experimental")
                .long("--experimental")
                .help("Allows access unsupported experimental predefined tasks."),
        )
        .arg(
            Arg::new("disable-check-for-updates")
                .long("--disable-check-for-updates")
                .help("Disables the update check during startup"),
        )
        .arg(
            Arg::new("output-format")
                .long("--output-format")
                .value_name("OUTPUT FORMAT")
                .possible_values(&["default", "short-description", "markdown", "markdown-single-page", "markdown-sub-section", "autocomplete"])
                .default_value(DEFAULT_OUTPUT_FORMAT)
                .help("The print/list steps format (some operations do not support all formats)"),
        )
        .arg(
            Arg::new("output_file")
                .long("--output-file")
                .value_name("OUTPUT_FILE")
                .help("The list steps output file name"),
        )
        .arg(Arg::new("print-steps").long("--print-steps").help(
            "Only prints the steps of the build in the order they will \
             be invoked but without invoking them",
        ))
        .arg(
            Arg::new("list-steps")
                .long("--list-all-steps")
                .help("Lists all known steps"),
        )
        .arg(
            Arg::new("list-category-steps")
                .long("--list-category-steps")
                .value_name("CATEGORY")
                .help("List steps for a given category"),
        )
        .arg(
            Arg::new("diff-steps")
                .long("--diff-steps")
                .help("Runs diff between custom flow and prebuilt flow (requires git)"),
        )
        .arg(Arg::new("TASK_CMD")
                .multiple_occurrences(true)
                .help("The task to execute, potentially including arguments which can be accessed in the task itself.")
        );

    if sub_command {
        Command::new("cargo").bin_name("cargo").subcommand(cli_app)
    } else {
        cli_app
    }
}

/// Handles the command line arguments and executes the runner.
pub(crate) fn run_cli(command_name: String, sub_command: bool) {
    let global_config = config::load();

    let app = create_cli(&global_config, &command_name, sub_command);

    let matches = app.get_matches();

    run_for_args(matches, &global_config, &command_name, sub_command);
}
