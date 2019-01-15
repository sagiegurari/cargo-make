#![deny(
    absolute_paths_not_starting_with_crate,
    anonymous_parameters,
    const_err,
    dead_code,
    deprecated,
    deprecated_in_future,
    duplicate_macro_exports,
    ellipsis_inclusive_range_patterns,
    exceeding_bitshifts,
    explicit_outlives_requirements,
    illegal_floating_point_literal_pattern,
    improper_ctypes,
    incoherent_fundamental_impls,
    intra_doc_link_resolution_failure,
    invalid_type_param_default,
    irrefutable_let_patterns,
    keyword_idents,
    late_bound_lifetime_arguments,
    legacy_constructor_visibility,
    legacy_directory_ownership,
    macro_expanded_macro_exports_accessed_by_absolute_paths,
    missing_copy_implementations,
    missing_docs,
    missing_fragment_specifier,
    mutable_transmutes,
    no_mangle_const_items,
    no_mangle_generic_items,
    non_camel_case_types,
    non_shorthand_field_patterns,
    non_snake_case,
    non_upper_case_globals,
    order_dependent_trait_objects,
    overflowing_literals,
    parenthesized_params_in_types_and_modules,
    path_statements,
    patterns_in_fns_without_body,
    plugin_as_library,
    private_doc_tests,
    private_in_public,
    proc_macro_derive_resolution_fallback,
    pub_use_of_private_extern_crate,
    question_mark_macro_sep,
    safe_extern_statics,
    safe_packed_borrows,
    stable_features,
    trivial_bounds,
    trivial_casts,
    trivial_numeric_casts,
    type_alias_bounds,
    tyvar_behind_raw_pointer,
    unconditional_recursion,
    unions_with_drop_fields,
    unknown_crate_types,
    unnameable_test_items,
    unreachable_code,
    unreachable_patterns,
    unreachable_pub,
    unsafe_code,
    unstable_features,
    unstable_name_collisions,
    unused_allocation,
    unused_assignments,
    unused_attributes,
    unused_comparisons,
    unused_doc_comments,
    unused_extern_crates,
    unused_features,
    unused_import_braces,
    unused_imports,
    unused_labels,
    unused_lifetimes,
    unused_macros,
    unused_must_use,
    unused_mut,
    unused_parens,
    unused_qualifications,
    unused_unsafe,
    unused_variables,
    where_clauses_object_safety,
    while_true
)]
#![warn(macro_use_extern_crate, unknown_lints)]
#![allow(
    bare_trait_objects,
    box_pointers,
    elided_lifetimes_in_paths,
    missing_doc_code_examples,
    missing_debug_implementations,
    single_use_lifetimes,
    unused_results,
    variant_size_differences,
    warnings,
    renamed_and_removed_lints
)]
#![cfg_attr(feature = "clippy", feature(plugin))]

//! # lib
//!
//! Wraps the cargo-make task runner as an internal library.
//!

extern crate ci_info;
extern crate clap;
extern crate dirs;
extern crate fern;
extern crate glob;
extern crate indexmap;
#[macro_use]
extern crate log;
extern crate rand;
extern crate run_script;
extern crate rust_info;
extern crate semver;
#[macro_use]
extern crate serde_derive;
extern crate shell2batch;
extern crate toml;

#[cfg(test)]
#[path = "./test.rs"]
mod test;

#[cfg(test)]
#[path = "./lib_test.rs"]
mod lib_test;

// make types public for docs
pub mod types;

mod cache;
mod command;
mod condition;
mod config;
mod descriptor;
mod environment;
mod execution_plan;
mod installer;
mod legacy;
mod logger;
mod print;
mod runner;
mod scriptengine;
mod storage;
mod toolchain;
mod version;

use crate::types::{CliArgs, GlobalConfig};
use clap::{App, Arg, ArgMatches, SubCommand};

static VERSION: &str = env!("CARGO_PKG_VERSION");
static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
static DEFAULT_TOML: &str = "Makefile.toml";
static DEFAULT_LOG_LEVEL: &str = "info";
static DEFAULT_TASK_NAME: &str = "default";
static DEFAULT_OUTPUT_FORMAT: &str = "default";

fn run(cli_args: CliArgs, global_config: &GlobalConfig) {
    logger::init(&cli_args.log_level);

    info!("{} {}", &cli_args.command, &VERSION);
    debug!("Written By {}", &AUTHOR);

    debug!("Cli Args {:#?}", &cli_args);
    debug!("Global Configuration {:#?}", &global_config);

    // only run check for updates if we are not in a CI env and user didn't ask to skip the check
    if !cli_args.disable_check_for_updates
        && !ci_info::is_ci()
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

    let force_makefile = cli_args.build_file.is_some();
    let build_file = &cli_args
        .build_file
        .clone()
        .unwrap_or(DEFAULT_TOML.to_string());
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

    let config = descriptor::load(&build_file, force_makefile, env, cli_args.experimental);

    let env_info = environment::setup_env(&cli_args, &config, &task);

    if cli_args.list_all_steps {
        descriptor::list_steps(&config, &cli_args.output_format);
    } else if cli_args.print_only {
        print::print(
            &config,
            &task,
            &cli_args.output_format,
            cli_args.disable_workspace,
        );
    } else {
        runner::run(config, &task, env_info, &cli_args);
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

    cli_args.output_format = cmd_matches
        .value_of("output-format")
        .unwrap_or(DEFAULT_OUTPUT_FORMAT)
        .to_string();

    cli_args.disable_check_for_updates = cmd_matches.is_present("disable-check-for-updates");
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

    cli_args.arguments = match cmd_matches.values_of("TASK_ARGS") {
        Some(values) => {
            let args_str: Vec<&str> = values.collect();
            let args_strings = args_str.iter().map(|item| item.to_string()).collect();
            Some(args_strings)
        }
        None => None,
    };

    run(cli_args, global_config);
}

fn create_cli<'a, 'b>(
    global_config: &'a GlobalConfig,
    command_name: &String,
    sub_command: bool,
) -> App<'a, 'b> {
    let default_task_name = match global_config.default_task_name {
        Some(ref value) => value.as_str().clone(),
        None => &DEFAULT_TASK_NAME,
    };
    let default_log_level = match global_config.log_level {
        Some(ref value) => value.as_str().clone(),
        None => &DEFAULT_LOG_LEVEL,
    };

    let mut cli_app = if sub_command {
        SubCommand::with_name(&command_name)
    } else {
        let name = command_name.as_str();
        App::new(name.clone()).bin_name(name.clone())
    };

    cli_app = cli_app
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
                .help("Set environment variables from provided file"),
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
        .arg(
            Arg::from_usage("--output-format=[OUTPUT FORMAT] 'The print/list steps format (some operations do not support all formats)'")
                .possible_values(&["default", "short-description", "markdown"])
                .default_value(DEFAULT_OUTPUT_FORMAT),
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
        .arg(Arg::with_name("TASK").help("The task name to execute"))
        .arg(
            Arg::with_name("TASK_ARGS")
                .multiple(true)
                .help("Task arguments which can be accessed in the task itself."),
        );

    if sub_command {
        App::new("cargo").bin_name("cargo").subcommand(cli_app)
    } else {
        cli_app
    }
}

/// Handles the command line arguments and executes the runner.
pub fn run_cli(command_name: String, sub_command: bool) {
    let global_config = config::load();

    let app = create_cli(&global_config, &command_name, sub_command);

    let matches = app.get_matches();

    run_for_args(matches, &global_config, &command_name, sub_command);
}
