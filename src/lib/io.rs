//! # io
//!
//! IO helper functions
//!

#[cfg(test)]
#[path = "./io_test.rs"]
mod io_test;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::fs::{create_dir_all, remove_file, File};
use std::io::prelude::*;
use std::path::PathBuf;
use std::{env, iter};

pub(crate) fn create_text_file(text: &str, extension: &str) -> String {
    let write_content =
        move |file: &mut File, file_path: &str| match file.write_all(text.as_bytes()) {
            Err(error) => {
                error!("Unable to write to file: {} {:#?}", &file_path, &error);
                panic!("Unable to write to file, error: {}", error);
            }
            Ok(_) => debug!("Written file text:\n{}", &text),
        };

    create_file(&write_content, &extension)
}

pub(crate) fn create_file(write_content: &Fn(&mut File, &str), extension: &str) -> String {
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

    debug!("Creating temporary file: {}", &file_path_str);

    let mut file = match File::create(&file_path) {
        Err(error) => {
            error!("Unable to create file: {} {:#?}", &file_path_str, &error);
            panic!("Unable to create file, error: {}", error);
        }
        Ok(file) => file,
    };

    write_content(&mut file, &file_path_str);

    match file.sync_all() {
        Ok(_) => debug!("File Synched."),
        Err(error) => debug!("Error Synching File: {:#?}", error),
    };

    file_path_str.to_string()
}

pub(crate) fn delete_file(file: &str) {
    match remove_file(&file) {
        Ok(_) => debug!("Temporary file deleted: {}", &file),
        Err(error) => debug!("Unable to delete temporary file: {} {:#?}", &file, error),
    };
}

pub(crate) fn read_text_file(file_path: &PathBuf) -> String {
    debug!("Opening file: {:#?}", &file_path);
    let mut file = match File::open(&file_path) {
        Ok(value) => value,
        Err(error) => panic!("Unable to open file: {:#?} error: {}", file_path, error),
    };

    let mut content = String::new();
    file.read_to_string(&mut content).unwrap();

    content
}
