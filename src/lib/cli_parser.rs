//! # cli_parser
//!
//! Defines the cli args and parsers them.
//!

#[cfg(test)]
#[path = "cli_parser_test.rs"]
mod cli_parser_test;

use crate::cli::{
    AUTHOR, DEFAULT_LOG_LEVEL, DEFAULT_OUTPUT_FORMAT, DEFAULT_TASK_NAME, DESCRIPTION, VERSION,
};
use crate::profile;
use crate::types::{CliArgs, GlobalConfig};
use cliparser::{Argument, ArgumentHelp, ArgumentOccurrence, ArgumentValueType, CliSpec};

fn get_args(
    cli_parsed: &CliParsed,
    global_config: &GlobalConfig,
    command_name: &str,
    sub_command: bool,
) -> CliArgs {
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
        binary.push_str(command_name);
        binary
    } else {
        command_name.to_string()
    };

    cli_args.env = cli_parsed
        .argument_values
        .get("env")
        .unwrap_or(vec![])
        .clone();

    cli_args.build_file = match cli_parsed.argument_values.get("makefile") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    cli_args.cwd = match cmd_matches.get_one::<String>("cwd") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    let default_log_level = match global_config.log_level {
        Some(ref value) => value.to_string(),
        None => DEFAULT_LOG_LEVEL.to_string(),
    };
    cli_args.log_level = if cmd_matches.contains_id("v") {
        "verbose".to_string()
    } else {
        cmd_matches
            .get_one::<String>("loglevel")
            .unwrap_or(&default_log_level)
            .to_string()
    };

    let default_disable_color = match global_config.disable_color {
        Some(value) => value,
        None => false,
    };
    cli_args.disable_color = cmd_matches.contains_id("no-color")
        || envmnt::is("CARGO_MAKE_DISABLE_COLOR")
        || default_disable_color;

    cli_args.print_time_summary =
        cmd_matches.contains_id("time-summary") || envmnt::is("CARGO_MAKE_PRINT_TIME_SUMMARY");

    cli_args.env_file = match cmd_matches.get_one::<String>("envfile") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    cli_args.output_format = cmd_matches
        .get_one::<String>("output-format")
        .unwrap_or(&DEFAULT_OUTPUT_FORMAT.to_string())
        .to_string();

    cli_args.list_category_steps = match cmd_matches.get_one::<String>("list-category-steps") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    cli_args.output_file = match cmd_matches.get_one::<String>("output_file") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    let default_profile = profile::DEFAULT_PROFILE.to_string();
    let profile_name = cmd_matches
        .get_one::<String>("profile")
        .unwrap_or(&default_profile);
    cli_args.profile = Some(profile_name.to_string());

    cli_args.disable_check_for_updates = cmd_matches.contains_id("disable-check-for-updates");
    cli_args.experimental = cmd_matches.contains_id("experimental");
    cli_args.print_only = cmd_matches.contains_id("print-steps");
    cli_args.disable_workspace = cmd_matches.contains_id("no-workspace");
    cli_args.disable_on_error = cmd_matches.contains_id("no-on-error");
    cli_args.allow_private = cmd_matches.contains_id("allow-private");
    cli_args.skip_init_end_tasks = cmd_matches.contains_id("skip-init-end-tasks");
    cli_args.list_all_steps = cmd_matches.contains_id("list-steps");
    cli_args.diff_execution_plan = cmd_matches.contains_id("diff-steps");

    cli_args.skip_tasks_pattern = match cmd_matches.get_one::<String>("skip-tasks-pattern") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    let default_task_name = match global_config.default_task_name {
        Some(ref value) => value.to_string(),
        None => DEFAULT_TASK_NAME.to_string(),
    };
    let task = cmd_matches
        .get_one::<String>("task")
        .unwrap_or(&default_task_name);
    let task_cmd = get_string_vec(cmd_matches, "TASK_CMD").unwrap_or_default();
    let task_cmd_slice = task_cmd.as_slice();
    let (task, arguments) = match task_cmd_slice {
        &[] => (task, None),
        &[ref task_name, ref task_args @ ..] => {
            let args_strings = task_args.iter().map(|item| item.to_string()).collect();
            (task_name, Some(args_strings))
        }
    };
    cli_args.task = task.to_string();
    cli_args.arguments = arguments;

    cli_args
}

fn create_cli(global_config: &GlobalConfig, command_name: &str, sub_command: bool) -> CliSpec {
    let default_task_name = match global_config.default_task_name {
        Some(ref value) => value.as_str(),
        None => &DEFAULT_TASK_NAME,
    };
    let default_log_level = match global_config.log_level {
        Some(ref value) => value.as_str(),
        None => &DEFAULT_LOG_LEVEL,
    };

    let mut spec = CliSpec::new();

    spec = spec
        .set_meta_info(Some(CliSpecMetaInfo {
            author: Some(AUTHOR),
            version: Some(VERSION),
            description: Some(DESCRIPTION),
            project: Some("cargo-make".to_string()),
            help_post_text: Some(
                "See more info at: https://github.com/sagiegurari/cargo-make".to_string(),
            ),
        }))
        .add_command("makers")
        .add_subcommand(vec!["cargo".to_string(), "make".to_string()]);

    cli_app = cli_app
        .arg(
            Arg::new("makefile")
                .long("--makefile")
                .value_name("FILE")
                .value_parser(value_parser!(String))
                .help("The optional toml file containing the tasks definitions"),
        )
        .arg(
            Arg::new("task")
                .short('t')
                .long("--task")
                .value_name("TASK")
                .value_parser(value_parser!(String))
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
                .value_parser(value_parser!(String))
                .help(
                    "The profile name (will be converted to lower case)",
                )
                .default_value(&profile::DEFAULT_PROFILE),
        )
        .arg(
            Arg::new("cwd")
                .long("--cwd")
                .value_name("DIRECTORY")
                .value_parser(value_parser!(String))
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
                .value_parser(value_parser!(String))
                .value_name("SKIP_TASK_PATTERNS")
                .help("Skip all tasks that match the provided regex (example: pre.*|post.*)"),
        )
        .arg(
            Arg::new("envfile")
                .long("--env-file")
                .value_name("FILE")
                .value_parser(value_parser!(String))
                .help("Set environment variables from provided file"),
        )
        .arg(
            Arg::new("env")
                .long("--env")
                .short('e')
                .value_name("ENV")
                .value_parser(value_parser!(String))
                .action(ArgAction::Append)
                .takes_value(true)
                .number_of_values(1)
                .help("Set environment variables"),
        )
        .arg(
            Arg::new("loglevel")
                .long("--loglevel")
                .short('l')
                .value_name("LOG LEVEL")
                .value_parser(["verbose", "info", "error"])
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
                .value_parser(["default", "short-description", "markdown", "markdown-single-page", "markdown-sub-section", "autocomplete"])
                .default_value(DEFAULT_OUTPUT_FORMAT)
                .help("The print/list steps format (some operations do not support all formats)"),
        )
        .arg(
            Arg::new("output_file")
                .long("--output-file")
                .value_name("OUTPUT_FILE")
                .value_parser(value_parser!(String))
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
                .value_parser(value_parser!(String))
                .help("List steps for a given category"),
        )
        .arg(
            Arg::new("diff-steps")
                .long("--diff-steps")
                .help("Runs diff between custom flow and prebuilt flow (requires git)"),
        )
        .arg(Arg::new("TASK_CMD")
                .value_parser(value_parser!(String))
                .takes_value(true)
                .multiple_values(true)
                .help("The task to execute, potentially including arguments which can be accessed in the task itself.")
        );

    spec
}

pub(crate) fn parse_args(
    global_config: &GlobalConfig,
    command_name: &str,
    sub_command: bool,
    args: Option<Vec<&str>>,
) -> CliArgs {
    let spec = create_cli(&global_config, command_name, sub_command);

    let cli_parsed = match args {
        Some(args_vec) => parse(args_vec, &spec),
        None => parse_process(&spec),
    };

    // TODO HANDLE HELP/VERSION COMMANDS HERE!!!

    get_args(&cli_parsed, &global_config, command_name, sub_command)
}

pub(crate) fn parse(
    global_config: &GlobalConfig,
    command_name: &str,
    sub_command: bool,
) -> CliArgs {
    parse_args(global_config, command_name, sub_command, None)
}
