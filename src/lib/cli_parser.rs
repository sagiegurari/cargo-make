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
use cliparser::types::{
    Argument, ArgumentHelp, ArgumentOccurrence, ArgumentValueType, CliParsed, CliSpec,
    CliSpecMetaInfo, PositionalArgument,
};

#[cfg(test)]
fn exit() -> ! {
    panic!("{}", "exit test");
}

#[cfg(not(test))]
use std::process;

#[cfg(not(test))]
fn exit() -> ! {
    process::exit(0);
}

fn get_args(
    cli_parsed: &CliParsed,
    global_config: &GlobalConfig,
    command_name: &str,
    sub_command: bool,
) -> CliArgs {
    let mut cli_args = CliArgs::new();

    cli_args.command = if sub_command {
        let mut binary = "cargo ".to_string();
        binary.push_str(command_name);
        binary
    } else {
        command_name.to_string()
    };

    cli_args.env = to_owned_vec(cli_parsed.argument_values.get("env"));

    cli_args.build_file = match cli_parsed.get_first_value("makefile") {
        Some(value) => Some(value),
        None => None,
    };

    cli_args.cwd = cli_parsed.get_first_value("cwd");

    let default_log_level = match global_config.log_level {
        Some(ref value) => value.to_string(),
        None => DEFAULT_LOG_LEVEL.to_string(),
    };
    cli_args.log_level = if cli_parsed.arguments.contains("verbose") {
        "verbose".to_string()
    } else if cli_parsed.arguments.contains("quiet") {
        "error".to_string()
    } else if cli_parsed.arguments.contains("silent") {
        "off".to_string()
    } else {
        cli_parsed
            .get_first_value("loglevel")
            .unwrap_or(default_log_level)
            .to_string()
    };

    let default_disable_color = match global_config.disable_color {
        Some(value) => value,
        None => false,
    };
    cli_args.disable_color = cli_parsed.arguments.contains("no-color")
        || envmnt::is("CARGO_MAKE_DISABLE_COLOR")
        || default_disable_color;

    cli_args.print_time_summary = cli_parsed.arguments.contains("time-summary")
        || envmnt::is("CARGO_MAKE_PRINT_TIME_SUMMARY");

    cli_args.env_file = match cli_parsed.get_first_value("envfile") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    cli_args.output_format = cli_parsed
        .get_first_value("output-format")
        .unwrap_or(DEFAULT_OUTPUT_FORMAT.to_string())
        .to_string();

    cli_args.list_category_steps = match cli_parsed.get_first_value("list-category-steps") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    cli_args.output_file = match cli_parsed.get_first_value("output-file") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    let profile_name = cli_parsed
        .get_first_value("profile")
        .unwrap_or_else(profile::default_profile);
    cli_args.profile = Some(profile_name.to_string());

    cli_args.disable_check_for_updates = cli_parsed.arguments.contains("disable-check-for-updates");
    cli_args.experimental = cli_parsed.arguments.contains("experimental");
    cli_args.print_only = cli_parsed.arguments.contains("print-steps");
    cli_args.disable_workspace = cli_parsed.arguments.contains("no-workspace");
    cli_args.disable_on_error = cli_parsed.arguments.contains("no-on-error");
    cli_args.allow_private = cli_parsed.arguments.contains("allow-private");
    cli_args.skip_init_end_tasks = cli_parsed.arguments.contains("skip-init-end-tasks");
    cli_args.list_all_steps = cli_parsed.arguments.contains("list-steps");
    cli_args.diff_execution_plan = cli_parsed.arguments.contains("diff-steps");
    cli_args.hide_uninteresting = cli_parsed.arguments.contains("hide-uninteresting");

    cli_args.skip_tasks_pattern = match cli_parsed.get_first_value("skip-tasks-pattern") {
        Some(value) => Some(value.to_string()),
        None => None,
    };

    let default_task_name = match global_config.default_task_name {
        Some(ref value) => value.to_string(),
        None => DEFAULT_TASK_NAME.to_string(),
    };
    let task = cli_parsed
        .get_first_value("task")
        .unwrap_or(default_task_name);
    let task_cmd = to_owned_vec(cli_parsed.argument_values.get("TASK_CMD")).unwrap_or(vec![]);
    let task_cmd_slice = task_cmd.as_slice();
    let (task, arguments) = match task_cmd_slice {
        &[] => (task, None),
        &[ref task_name, ref task_args @ ..] => {
            let args_strings = task_args.iter().map(|item| item.to_string()).collect();
            (task_name.to_string(), Some(args_strings))
        }
    };
    cli_args.task = task;
    cli_args.arguments = arguments;

    cli_args
}

pub fn create_cli(global_config: &GlobalConfig) -> CliSpec {
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
            author: Some(AUTHOR.to_string()),
            version: Some(VERSION.to_string()),
            description: Some(DESCRIPTION.to_string()),
            project: Some("cargo-make".to_string()),
            help_post_text: Some(
                "See more info at: https://github.com/sagiegurari/cargo-make".to_string(),
            ),
        }))
        .add_command("makers")
        .add_subcommand(vec!["cargo", "make"])
        .add_subcommand(vec!["cargo-make", "make"]) // done by cargo
        .add_argument(Argument {
            name: "help".to_string(),
            key: vec!["--help".to_string(), "-h".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text("Print help information".to_string())),
        })
        .add_argument(Argument {
            name: "version".to_string(),
            key: vec!["--version".to_string(), "-V".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text("Print version information".to_string())),
        })
        .add_argument(Argument {
            name: "makefile".to_string(),
            key: vec!["--makefile".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: None,
            help: Some(ArgumentHelp::TextAndParam(
                "The optional toml file containing the tasks definitions".to_string(),
                "FILE".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "task".to_string(),
            key: vec!["--task".to_string(), "-t".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: Some(default_task_name.to_string()),
            help: Some(ArgumentHelp::TextAndParam(
                "The task name to execute (can omit the flag if the task name is the last argument)".to_string(),
                "TASK".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "profile".to_string(),
            key: vec!["--profile".to_string(), "-p".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: Some(profile::default_profile()),
            help: Some(ArgumentHelp::TextAndParam(
                "The profile name (will be converted to lower case)".to_string(),
                "PROFILE".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "cwd".to_string(),
            key: vec!["--cwd".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: None,
            help: Some(ArgumentHelp::TextAndParam(
                "Will set the current working directory. The search for the makefile will be from this directory if defined.".to_string(),
                "DIRECTORY".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "no-workspace".to_string(),
            key: vec!["--no-workspace".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Disable workspace support (tasks are triggered on workspace and not on members)".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "no-on-error".to_string(),
            key: vec!["--no-on-error".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Disable on error flow even if defined in config sections".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "allow-private".to_string(),
            key: vec!["--allow-private".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Allow invocation of private tasks".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "skip-init-end-tasks".to_string(),
            key: vec!["--skip-init-end-tasks".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "If set, init and end tasks are skipped".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "skip-tasks-pattern".to_string(),
            key: vec!["--skip-tasks".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: None,
            help: Some(ArgumentHelp::TextAndParam(
                "Skip all tasks that match the provided regex (example: pre.*|post.*)".to_string(),
                "SKIP_TASK_PATTERNS".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "envfile".to_string(),
            key: vec!["--env-file".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: None,
            help: Some(ArgumentHelp::TextAndParam(
                "Set environment variables from provided file".to_string(),
                "FILE".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "env".to_string(),
            key: vec!["--env".to_string(), "-e".to_string()],
            argument_occurrence: ArgumentOccurrence::Multiple,
            value_type: ArgumentValueType::Single,
            default_value: None,
            help: Some(ArgumentHelp::TextAndParam(
                "Set environment variables".to_string(),
                "ENV".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "loglevel".to_string(),
            key: vec!["--loglevel".to_string(), "-l".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: Some(default_log_level.to_string()),
            help: Some(ArgumentHelp::TextAndParam(
                "The log level (verbose, info, error, off)".to_string(),
                "LOG LEVEL".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "verbose".to_string(),
            key: vec!["--verbose".to_string(), "-v".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Sets the log level to verbose (shorthand for --loglevel verbose)".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "quiet".to_string(),
            key: vec!["--quiet".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Sets the log level to error (shorthand for --loglevel error)".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "silent".to_string(),
            key: vec!["--silent".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Sets the log level to off (shorthand for --loglevel off)".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "no-color".to_string(),
            key: vec!["--no-color".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Disables colorful output".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "time-summary".to_string(),
            key: vec!["--time-summary".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Print task level time summary at end of flow".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "experimental".to_string(),
            key: vec!["--experimental".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Allows access unsupported experimental predefined tasks.".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "disable-check-for-updates".to_string(),
            key: vec!["--disable-check-for-updates".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Disables the update check during startup".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "output-format".to_string(),
            key: vec!["--output-format".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: None,
            help: Some(ArgumentHelp::TextAndParam(
                "The print/list steps format (some operations do not support all formats) (default, short-description, markdown, markdown-single-page, markdown-sub-section, autocomplete)".to_string(),
                "OUTPUT FORMAT".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "output-file".to_string(),
            key: vec!["--output-file".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: None,
            help: Some(ArgumentHelp::TextAndParam(
                "The list steps output file name".to_string(),
                "OUTPUT_FILE".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "hide-uninteresting".to_string(),
            key: vec!["--hide-uninteresting".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Hide any minor tasks such as pre/post hooks.".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "print-steps".to_string(),
            key: vec!["--print-steps".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Only prints the steps of the build in the order they will be invoked but without invoking them".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "list-steps".to_string(),
            key: vec!["--list-all-steps".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Lists all known steps".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "list-category-steps".to_string(),
            key: vec!["--list-category-steps".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::Single,
            default_value: None,
            help: Some(ArgumentHelp::TextAndParam(
                "List steps for a given category".to_string(),
                "CATEGORY".to_string(),
            )),
        })
        .add_argument(Argument {
            name: "diff-steps".to_string(),
            key: vec!["--diff-steps".to_string()],
            argument_occurrence: ArgumentOccurrence::Single,
            value_type: ArgumentValueType::None,
            default_value: None,
            help: Some(ArgumentHelp::Text(
                "Runs diff between custom flow and prebuilt flow (requires git)".to_string(),
            )),
        })
        .set_positional_argument(Some(PositionalArgument {
            name: "TASK_CMD".to_string(),
            help: Some(ArgumentHelp::Text(
                "The task to execute, potentially including arguments which can be accessed in the task itself.".to_string(),
            )),
        }));

    spec
}

pub fn parse_args(
    global_config: &GlobalConfig,
    command_name: &str,
    sub_command: bool,
    args: Option<Vec<&str>>,
) -> CliArgs {
    let spec = create_cli(&global_config);

    let parse_result = match args {
        Some(args_vec) => cliparser::parse(&args_vec, &spec),
        None => cliparser::parse_process(&spec),
    };

    match parse_result {
        Ok(cli_parsed) => {
            if cli_parsed.arguments.contains("help") {
                // generate help text
                let help_text = cliparser::help(&spec);
                println!("{}", help_text);
                exit();
            } else if cli_parsed.arguments.contains("version") {
                // generate version text
                let version_text = cliparser::version(&spec);
                println!("{}", version_text);
                exit();
            }

            get_args(&cli_parsed, &global_config, command_name, sub_command)
        }
        Err(error) => {
            let help_text = cliparser::help(&spec);
            println!("{}\n{}", &error, help_text);
            exit()
        }
    }
}

pub fn parse(
    global_config: &GlobalConfig,
    command_name: &str,
    sub_command: bool,
) -> CliArgs {
    parse_args(global_config, command_name, sub_command, None)
}

fn to_owned_vec(vec_option: Option<&Vec<String>>) -> Option<Vec<String>> {
    match vec_option {
        Some(vec) => Some(vec.to_owned()),
        None => None,
    }
}
