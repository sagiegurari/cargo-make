//! # rsscript
//!
//! Compiles and runs rust code.
//!

#[cfg(test)]
#[path = "./rsscript_test.rs"]
mod rsscript_test;

use crate::command;
use crate::installer::cargo_plugin_installer;
use crate::scriptengine::script_utils::{create_script_file, delete_file};

fn install_crate() {
    // install dependencies
    cargo_plugin_installer::install_crate(&None, "script", "cargo-script", &None, true);
}

fn create_rust_file(rust_script: &Vec<String>) -> String {
    create_script_file(rust_script, "rs")
}

fn run_file(file: &str, cli_arguments: &Vec<String>) -> (bool, String) {
    let mut args = vec!["script".to_string(), file.to_string()];
    let mut cli_args = cli_arguments.clone();
    args.append(&mut cli_args);

    let output = command::run_command_get_output("cargo", &Some(args), false);
    let exit_code = command::get_exit_code_from_output(&output, false);
    debug!("Executed rust code, exit code: {}", exit_code);

    let valid = exit_code == 0;

    match output {
        Ok(ref output_struct) => {
            let stdout = String::from_utf8_lossy(&output_struct.stdout).into_owned();
            (valid, stdout)
        }
        _ => (valid, "".to_string()),
    }
}

pub(crate) fn execute(rust_script: &Vec<String>, cli_arguments: &Vec<String>) -> String {
    install_crate();

    let file = create_rust_file(rust_script);

    let output = run_file(&file, &cli_arguments);
    let valid = output.0;

    delete_file(&file);

    if !valid {
        error!("Unable to execute rust code.");
    }

    output.1
}
