//! # command
//!
//! Runs task commands/scripts.
//!

#[cfg(test)]
#[path = "./command_test.rs"]
mod command_test;

use crate::logger;
use crate::toolchain;
use crate::types::{CommandSpec, Step};
use envmnt;
use run_script;
use run_script::{IoOptions, ScriptError, ScriptOptions};
use std::io;
use std::io::Error;
use std::process::{Command, ExitStatus, Output, Stdio};

/// Returns the exit code (-1 if no exit code found)
pub(crate) fn get_exit_code(exit_status: Result<ExitStatus, Error>, force: bool) -> i32 {
    match exit_status {
        Ok(code) => {
            if !code.success() {
                match code.code() {
                    Some(value) => value,
                    None => -1,
                }
            } else {
                0
            }
        }
        Err(error) => {
            if !force {
                error!("Error while executing command, error: {:#?}", error);
            }

            -1
        }
    }
}

pub(crate) fn get_exit_code_from_output(output: &io::Result<Output>, force: bool) -> i32 {
    match output {
        &Ok(ref output_struct) => get_exit_code(Ok(output_struct.status), force),
        &Err(ref error) => {
            if !force {
                error!("Error while executing command, error: {:#?}", error);
            }

            -1
        }
    }
}

/// Validates the exit code code and if not 0 or unable to validate it, panic.
pub(crate) fn validate_exit_code(code: i32) {
    if code == -1 {
        error!("Error while executing command, unable to extract exit code.");
    } else if code != 0 {
        error!("Error while executing command, exit code: {}", code);
    }
}

fn is_silent() -> bool {
    let log_level = logger::get_log_level();
    is_silent_for_level(log_level)
}

fn is_silent_for_level(log_level: String) -> bool {
    let level = logger::get_level(&log_level);

    match level {
        logger::LogLevel::ERROR => true,
        _ => false,
    }
}

fn should_print_commands_by_default() -> bool {
    let log_level = logger::get_log_level();

    if should_print_commands_for_level(log_level) {
        true
    } else {
        // if log level defaults to not printing the script commands
        // we also check if we are running in a CI env.
        // Users will not see the commands while CI builds will have the commands printed out.
        envmnt::is("CARGO_MAKE_CI")
    }
}

fn should_print_commands_for_level(log_level: String) -> bool {
    let level = logger::get_level(&log_level);

    match level {
        logger::LogLevel::VERBOSE => true,
        _ => false,
    }
}

/// Runs the requested script text and returns its output.
pub(crate) fn run_script_get_output(
    script_lines: &Vec<String>,
    script_runner: Option<String>,
    cli_arguments: &Vec<String>,
    capture_output: bool,
    print_commands: Option<bool>,
) -> Result<(i32, String, String), ScriptError> {
    let mut options = ScriptOptions::new();
    options.runner = script_runner.clone();
    options.output_redirection = if capture_output {
        IoOptions::Pipe
    } else {
        IoOptions::Inherit
    };
    options.exit_on_error = true;
    options.print_commands = match print_commands {
        Some(bool_value) => bool_value,
        None => should_print_commands_by_default(),
    };

    if is_silent() {
        options.output_redirection = IoOptions::Pipe;
        options.print_commands = false;
    } else if !capture_output && envmnt::is("CARGO_MAKE_SCRIPT_FORCE_PIPE_STDIN") {
        options.input_redirection = IoOptions::Pipe;
    }

    run_script::run(script_lines.join("\n").as_str(), cli_arguments, &options)
}

/// Runs the requested script text and panics in case of any script error.
pub(crate) fn run_script_get_exit_code(
    script_lines: &Vec<String>,
    script_runner: Option<String>,
    cli_arguments: &Vec<String>,
    validate: bool,
) -> i32 {
    let output = run_script_get_output(&script_lines, script_runner, cli_arguments, false, None);

    let exit_code = match output {
        Ok(output_struct) => output_struct.0,
        _ => -1,
    };

    if validate {
        validate_exit_code(exit_code);
    }

    exit_code
}

/// Runs the requested command and return its output.
pub(crate) fn run_command_get_output(
    command_string: &str,
    args: &Option<Vec<String>>,
    capture_output: bool,
) -> io::Result<Output> {
    debug!("Execute Command: {}", &command_string);
    let mut command = Command::new(&command_string);

    match *args {
        Some(ref args_vec) => {
            command.args(args_vec);
        }
        None => debug!("No command args defined."),
    };

    command.stdin(Stdio::inherit());
    if !capture_output {
        command.stdout(Stdio::inherit()).stderr(Stdio::inherit());
    }
    info!("Execute Command: {:#?}", &command);

    let output = command.output();
    debug!("Output: {:#?}", &output);

    output
}

/// Runs the requested command and panics in case of any error.
pub(crate) fn run_command(command_string: &str, args: &Option<Vec<String>>, validate: bool) -> i32 {
    let output = run_command_get_output(&command_string, &args, false);

    let exit_code = get_exit_code_from_output(&output, !validate);

    if validate {
        validate_exit_code(exit_code);
    }

    exit_code
}

/// Runs the given task command.
pub(crate) fn run(step: &Step) {
    let validate = !step.config.should_ignore_errors();

    match step.config.command {
        Some(ref command_string) => {
            let command_spec = match step.config.toolchain {
                Some(ref toolchain) => {
                    toolchain::wrap_command(&toolchain, &command_string, &step.config.args)
                }
                None => CommandSpec {
                    command: command_string.to_string(),
                    args: step.config.args.clone(),
                },
            };

            run_command(&command_spec.command, &command_spec.args, validate);
        }
        None => debug!("No command defined."),
    };
}
