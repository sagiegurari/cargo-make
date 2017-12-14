//! # command
//!
//! Runs task commands/scripts.
//!

#[cfg(test)]
#[path = "./command_test.rs"]
mod command_test;

use run_script;
use run_script::{ScriptError, ScriptOptions};
use std::io;
use std::io::Error;
use std::process::{Command, ExitStatus, Output, Stdio};
use types::Step;

/// Returns the exit code (-1 if no exit code found)
pub(crate) fn get_exit_code(
    exit_status: Result<ExitStatus, Error>,
    force: bool,
) -> i32 {
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

pub(crate) fn get_exit_code_from_output(
    output: &io::Result<Output>,
    force: bool,
) -> i32 {
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

/// Runs the requested script text and returns its output.
pub(crate) fn run_script_get_output(
    script_lines: &Vec<String>,
    script_runner: Option<String>,
    capture_output: bool,
) -> Result<(i32, String, String), ScriptError> {
    let mut options = ScriptOptions::new();
    options.runner = script_runner.clone();
    options.capture_output = capture_output;
    options.exit_on_error = true;
    options.print_commands = true;

    run_script::run(script_lines.join("\n").as_str(), &vec![], &options)
}

/// Runs the requested script text and panics in case of any script error.
pub(crate) fn run_script(
    script_lines: &Vec<String>,
    script_runner: Option<String>,
    validate: bool,
) -> i32 {
    let output = run_script_get_output(&script_lines, script_runner, false);

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
pub(crate) fn run_command(
    command_string: &str,
    args: &Option<Vec<String>>,
    validate: bool,
) -> i32 {
    let output = run_command_get_output(&command_string, &args, false);

    let exit_code = get_exit_code_from_output(&output, !validate);

    if validate {
        validate_exit_code(exit_code);
    }

    exit_code
}

/// Runs the given task command and if not defined, the task script.
pub(crate) fn run(step: &Step) {
    let validate = !step.config.is_force();

    match step.config.command {
        Some(ref command_string) => {
            run_command(&command_string, &step.config.args, validate);
        }
        None => {
            match step.config.script {
                Some(ref script) => {
                    run_script(script, step.config.script_runner.clone(), validate);
                    ()
                }
                None => debug!("No script defined."),
            };
        }
    };
}
