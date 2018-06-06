//! # rsscript
//!
//! Compiles and runs rust code.
//!

#[cfg(test)]
#[path = "./rsscript_test.rs"]
mod rsscript_test;

use command;
use installer;
use scriptengine::script_utils::{create_script_file, delete_file};

fn install_crate() {
    // install dependencies
    installer::install_crate("script", "cargo-script", &None, true);
}

fn create_rust_file(rust_script: &Vec<String>) -> String {
    create_script_file(rust_script, "rs")
}

fn run_file(file: &str) -> bool {
    let exit_code = command::run_command(
        "cargo",
        &Some(vec!["script".to_string(), file.to_string()]),
        false,
    );
    debug!("Executed rust code, exit code: {}", exit_code);

    exit_code == 0
}

pub(crate) fn execute(rust_script: &Vec<String>) {
    install_crate();

    let file = create_rust_file(rust_script);

    let valid = run_file(&file);

    delete_file(&file);

    if !valid {
        error!("Unable to execute rust code.");
    }
}
