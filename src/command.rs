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
use run_script;
use run_script::{ScriptError, ScriptOptions};
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
                panic!("Error while executing command, error: {:#?}", error);
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
                panic!("Error while executing command, error: {:#?}", error);
            }

            -1
        }
    }
}

/// Validates the exit code code and if not 0 or unable to validate it, panic.
pub(crate) fn validate_exit_code(code: i32) {
    if code == -1 {
        error!("Error while executing command, unable to extract exit code.");
        panic!("Error while executing command, unable to extract exit code.");
    } else if code != 0 {
        error!("Error while executing command, exit code: {}", code);
        panic!("Error while executing command, exit code: {}", code);
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

/// Runs the requested script text and returns its output.
pub(crate) fn run_script_get_output(
    script_lines: &Vec<String>,
    script_runner: Option<String>,
    cli_arguments: &Vec<String>,
    capture_output: bool,
) -> Result<(i32, String, String), ScriptError> {
    let mut options = ScriptOptions::new();
    options.runner = script_runner.clone();
    options.capture_output = capture_output;
    options.exit_on_error = true;
    options.print_commands = true;

    if is_silent() {
        options.capture_output = true;
        options.print_commands = false;
    }

    run_script::run(script_lines.join("\n").as_str(), cli_arguments, &options)
}

/// Runs the requested script text and panics in case of any script error.
pub(crate) fn run_script(
    script_lines: &Vec<String>,
    script_runner: Option<String>,
    cli_arguments: &Vec<String>,
    validate: bool,
) -> i32 {
    let output = run_script_get_output(&script_lines, script_runner, cli_arguments, false);

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
            for arg in args_vec.iter() {
                command.arg(arg);
            }
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

/// Runs the given task command and if not defined, the task script.
pub(crate) fn run(step: &Step, cli_arguments: &Vec<String>) {
    let validate = !step.config.is_force();

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
        None => {
            match step.config.script {
                Some(ref script) => {
                    run_script(
                        script,
                        step.config.script_runner.clone(),
                        cli_arguments,
                        validate,
                    );
                    ()
                }
                None => debug!("No script defined."),
            };
        }
    };
}
