//! # script_utils
//!
//! Helper functions for script invocations.
//!

#[cfg(test)]
#[path = "./script_utils_test.rs"]
mod script_utils_test;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::{create_dir_all, remove_file, File};
use std::io::prelude::*;
use std::{env, iter};

pub(crate) fn create_script_file(script_text: &Vec<String>, extension: &str) -> String {
    let name = env!("CARGO_PKG_NAME");
    let mut rng = thread_rng();
    let file_name: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .take(10)
        .collect();

    let mut file_path = env::temp_dir();
    file_path.push(name);

    // create parent directory
    match create_dir_all(&file_path) {
        Ok(_) => debug!("Created temporary directory."),
        Err(error) => debug!(
            "Unable to create temporary directory: {} {:#?}",
            &file_path.to_str().unwrap_or("???"),
            error
        ),
    };

    file_path.push(file_name);
    file_path.set_extension(extension);

    let file_path_str = &file_path.to_str().unwrap_or("???");

    debug!("Creating temporary script file: {}", &file_path_str);

    let mut file = match File::create(&file_path) {
        Err(error) => {
            error!(
                "Unable to create script file: {} {:#?}",
                &file_path_str, &error
            );
            panic!("Unable to create script file, error: {}", error);
        }
        Ok(file) => file,
    };

    let text = script_text.join("\n");

    match file.write_all(text.as_bytes()) {
        Err(error) => {
            error!(
                "Unable to write to script file: {} {:#?}",
                &file_path_str, &error
            );
            panic!("Unable to write to script file, error: {}", error);
        }
        Ok(_) => debug!("Written script file text:\n{}", &text),
    }

    file_path_str.to_string()
}

pub(crate) fn delete_file(file: &str) {
    match remove_file(&file) {
        Ok(_) => debug!("Temporary file deleted: {}", &file),
        Err(error) => debug!("Unable to delete temporary file: {} {:#?}", &file, error),
    };
}
