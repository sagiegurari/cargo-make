//! # rsscript
//!
//! Compiles and runs rust code.
//!

#[cfg(test)]
#[path = "./rsscript_test.rs"]
mod rsscript_test;

use command;
use installer;
use rand::{Rng, thread_rng};
use std::env;
use std::fs::{File, create_dir_all, remove_file};
use std::io::prelude::*;

fn install_crate() {
    // install dependencies
    installer::install_crate("script", "cargo-script", &None, true);
}

fn create_rust_file(rust_script: &Vec<String>) -> String {
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
    file_path.set_extension("rs");

    let file_path_str = &file_path.to_str().unwrap_or("???");

    debug!("Creating temporary rust file: {}", &file_path_str);

    let mut file = match File::create(&file_path) {
        Err(error) => {
            error!("Unable to create rust file: {} {:#?}", &file_path_str, &error);
            panic!("Unable to create rust file, error: {}", error);
        }
        Ok(file) => file,
    };

    let text = rust_script.join("\n");

    match file.write_all(text.as_bytes()) {
        Err(error) => {
            error!("Unable to write to rust file: {} {:#?}", &file_path_str, &error);
            panic!("Unable to write to rust file, error: {}", error);
        }
        Ok(_) => debug!("Written rust file text:\n{}", &text),
    }

    file_path_str.to_string()
}

fn run_file(file: &str) -> bool {
    let exit_code = command::run_command("cargo", &Some(vec!["script".to_string(), file.to_string()]), false);
    debug!("Executed rust code, exit code: {}", exit_code);

    exit_code == 0
}

pub fn execute(rust_script: &Vec<String>) {
    install_crate();

    let file = create_rust_file(rust_script);

    let valid = run_file(&file);

    match remove_file(&file) {
        Ok(_) => debug!("Temporary file deleted: {}", &file),
        Err(error) => debug!("Unable to delete temporary file: {} {:#?}", &file, error),
    };

    if !valid {
        error!("Unable to execute rust code.");
    }
}
