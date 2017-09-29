//! # command
//!
//! Runs task commands/scripts.
//!

#[cfg(test)]
#[path = "./command_test.rs"]
mod command_test;

use rand::{Rng, thread_rng};
use std::env;
use std::env::current_dir;
use std::fs::{File, create_dir_all, remove_file};
use std::io;
use std::io::Error;
use std::io::prelude::*;
use std::process::{Command, ExitStatus, Output, Stdio};
use types::Step;

/// Returns the exit code (-1 if no exit code found)
pub fn get_exit_code(
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

pub fn get_exit_code_from_output(
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
pub fn validate_exit_code(code: i32) {
    if code == -1 {
        error!("Error while executing command, unable to extract exit code.");
    } else if code != 0 {
        error!("Error while executing command, exit code: {}", code);
    }
}

fn create_script(script_lines: &Vec<String>) -> String {
    let cwd_holder = match current_dir() {
        Ok(value) => value,
        Err(error) => {
            error!("Unable to get current working directory {:#?}", &error);
            panic!("Unable to get current working directory, error: {}", error);
        }
    };

    let cwd = match cwd_holder.to_str() {
        Some(cwd_str) => cwd_str.clone(),
        None => {
            error!("Unable to get current working directory.");
            panic!("Unable to get current working directory");
        }
    };

    // get local copy
    let mut mut_script_lines = script_lines.clone();

    // create cd command
    let mut cd_command = "cd ".to_string();
    cd_command.push_str(cwd);

    // check if first line is shebang line
    let mut insert_index = if mut_script_lines[0].starts_with("#!") {
        1
    } else {
        0
    };

    if !cfg!(windows) {
        mut_script_lines.insert(insert_index, "set -xe".to_string());
        insert_index = insert_index + 1;
    }
    mut_script_lines.insert(insert_index, cd_command);

    mut_script_lines.push("\n".to_string());

    mut_script_lines.join("\n")
}

/// Runs the requested script text and returns its output.
pub fn run_script_get_output(
    script_lines: &Vec<String>,
    script_runner: Option<String>,
    capture_output: bool,
) -> io::Result<Output> {
    let name = env!("CARGO_PKG_NAME");
    let file_name: String = thread_rng().gen_ascii_chars().take(10).collect();

    let mut file_path = env::temp_dir();
    file_path.push(name);

    // create parent directory
    match create_dir_all(&file_path) {
        Ok(_) => debug!("Created temporary directory."),
        Err(error) => debug!("Unable to create temporary directory: {} {:#?}", &file_path.to_str().unwrap_or("???"), error),
    };

    file_path.push(file_name);
    if cfg!(windows) {
        file_path.set_extension("bat");
    } else {
        file_path.set_extension("sh");
    };

    let file_path_str = &file_path.to_str().unwrap_or("???");

    debug!("Creating temporary script file: {}", &file_path_str);

    let mut file = match File::create(&file_path) {
        Err(error) => {
            error!("Unable to create script file: {} {:#?}", &file_path_str, &error);
            panic!("Unable to create script file, error: {}", error);
        }
        Ok(file) => file,
    };

    let text = create_script(&script_lines);

    match file.write_all(text.as_bytes()) {
        Err(error) => {
            error!("Unable to write to script file: {} {:#?}", &file_path_str, &error);
            panic!("Unable to write to script file, error: {}", error);
        }
        Ok(_) => debug!("Written script file text:\n{}", &text),
    }

    let command = match script_runner {
        Some(ref value) => value,
        None => {
            if cfg!(windows) {
                "cmd.exe"
            } else {
                "sh"
            }
        }
    };

    let args_vector = if cfg!(windows) {
        vec!["/C".to_string(), file_path_str.to_string()]
    } else {
        vec![file_path_str.to_string()]
    };

    let args = Some(args_vector);

    let output = run_command_get_output(&command, &args, capture_output);

    match remove_file(&file_path_str) {
        Ok(_) => debug!("Temporary file deleted: {}", &file_path_str),
        Err(error) => debug!("Unable to delete temporary file: {} {:#?}", &file_path_str, error),
    };

    output
}

/// Runs the requested script text and panics in case of any script error.
pub fn run_script(
    script_lines: &Vec<String>,
    script_runner: Option<String>,
    validate: bool,
) -> i32 {
    let output = run_script_get_output(&script_lines, script_runner, false);

    let exit_code = get_exit_code_from_output(&output, !validate);

    if validate {
        validate_exit_code(exit_code);
    }

    exit_code
}

/// Runs the requested command and return its output.
pub fn run_command_get_output(
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
pub fn run_command(
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
pub fn run(step: &Step) {
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
