//! # cli
//!
//! The cargo-make cli
//!

#[cfg(test)]
#[path = "cli_test.rs"]
mod cli_test;

use crate::cli_commands;
use crate::cli_parser;
use crate::config;
use crate::descriptor;
use crate::environment;
use crate::error::CargoMakeError;
use crate::logger;
use crate::logger::LoggerOptions;
use crate::profile;
use crate::recursion_level;
use crate::runner;
use crate::time_summary;
use crate::toolchain;
use crate::types::{CliArgs, GlobalConfig};
use crate::version;
use std::time::SystemTime;

pub(crate) static VERSION: &str = env!("CARGO_PKG_VERSION");
pub(crate) static AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
pub(crate) static DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub(crate) static DEFAULT_TOML: &str = "Makefile.toml";
pub(crate) static DEFAULT_LOG_LEVEL: &str = "info";
pub(crate) static DEFAULT_TASK_NAME: &str = "default";
pub(crate) static DEFAULT_OUTPUT_FORMAT: &str = "default";

pub fn run(
    cli_args: &CliArgs,
    global_config: &GlobalConfig,
    logger_options: Option<LoggerOptions>,
) -> Result<(), CargoMakeError> {
    let start_time = SystemTime::now();

    recursion_level::increment();

    logger::init(&logger_options.unwrap_or(LoggerOptions {
        name: String::from(env!("CARGO_PKG_NAME")),
        level: cli_args.log_level.clone(),
        color: !cli_args.disable_color,
    }));

    if recursion_level::is_top() {
        info!("{} {}", &cli_args.command, &VERSION);
        debug!("Written By {}", &AUTHOR);
    }

    debug!("Cli Args {:#?}", &cli_args);
    debug!("Global Configuration {:#?}", &global_config);

    if version::should_check(&cli_args, &global_config) {
        version::check();
    }

    // clear env vars (see https://github.com/rust-lang/rustup/issues/3029)
    toolchain::remove_rust_env_vars();

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
        .unwrap_or_else(profile::default_profile);
    let normalized_profile_name = profile::set(&profile_name);

    environment::load_env_file(cli_args.env_file.clone());

    let env = cli_args.env.clone();

    let experimental = cli_args.experimental;
    let config = descriptor::load(&build_file, force_makefile, env, experimental)?;

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

    let env_info = environment::setup_env(&cli_args, &config, &task, home, &mut time_summary_vec)?;
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
            &cli_args.list_category_steps,
            cli_args.hide_uninteresting,
        )
    } else if cli_args.diff_execution_plan {
        let default_config = descriptor::load_internal_descriptors(true, experimental, None)?;
        cli_commands::diff_steps::run(
            &default_config,
            &config,
            &task,
            &cli_args,
            &env_info.crate_info,
        )
    } else if cli_args.print_only {
        cli_commands::print_steps::print(
            &mut std::io::stdout(),
            &config,
            &task,
            &cli_args.output_format,
            cli_args.disable_workspace,
            &cli_args.skip_tasks_pattern,
            &env_info.crate_info,
            cli_args.skip_init_end_tasks,
        )
    } else {
        runner::run(
            config,
            &task,
            env_info,
            &cli_args,
            start_time,
            time_summary_vec,
        )
    }?;

    Ok(())
}

/// Handles the command line arguments and executes the runner.
pub fn run_cli(command_name: String, sub_command: bool) -> Result<CliArgs, CargoMakeError> {
    let global_config = config::load()?;

    let cli_args = cli_parser::parse(&global_config, &command_name, sub_command)?;

    run(&cli_args, &global_config, None)?;
    Ok(cli_args)
}
