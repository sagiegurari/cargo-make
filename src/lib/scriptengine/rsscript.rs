//! # rsscript
//!
//! Compiles and runs rust code.
//!

#[cfg(test)]
#[path = "./rsscript_test.rs"]
mod rsscript_test;

use crate::command;
use crate::installer::cargo_plugin_installer;
use crate::io::delete_file;
use crate::scriptengine::script_utils::create_script_file;

fn install_crate() {
    // install dependencies
    cargo_plugin_installer::install_crate(&None, "script", "cargo-script", &None, true);
}

fn create_rust_file(rust_script: &Vec<String>) -> String {
    create_script_file(rust_script, "rs")
}

fn run_file(file: &str, cli_arguments: &Vec<String>) -> bool {
    let mut args = vec!["script".to_string(), file.to_string()];
    let mut cli_args = cli_arguments.clone();
    args.append(&mut cli_args);

    let exit_code = command::run_command("cargo", &Some(args), false);
    debug!("Executed rust code, exit code: {}", exit_code);

    exit_code == 0
}

pub(crate) fn execute(rust_script: &Vec<String>, cli_arguments: &Vec<String>, validate: bool) {
    install_crate();

    let file = create_rust_file(rust_script);

    let valid = run_file(&file, &cli_arguments);

    delete_file(&file);

    if validate && !valid {
        error!("Unable to execute rust code.");
    }
}
