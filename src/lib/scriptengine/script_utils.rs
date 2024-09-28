//! # script_utils
//!
//! Helper functions for script invocations.
//!

#[cfg(test)]
#[path = "script_utils_test.rs"]
mod script_utils_test;

use crate::error::CargoMakeError;
use crate::io::create_text_file;
use fsio::file::write_text_file;
use fsio::path::as_path::AsPath;
use md5;
use std::path::PathBuf;

pub(crate) fn create_script_file(
    script_text: &Vec<String>,
    extension: &str,
) -> Result<String, CargoMakeError> {
    let text = script_text.join("\n");

    create_text_file(&text, &extension)
}

pub(crate) fn create_persisted_script_file(
    script_text: &Vec<String>,
    extension: &str,
) -> Result<String, CargoMakeError> {
    match create_persisted_script_file_with_options(script_text, extension, None) {
        Ok(value) => Ok(value),
        Err(_) => {
            let limit = envmnt::get_usize("CARGO_MAKE_SCRIPT_FILE_PATH_MAX_LENGTH", 130);
            create_persisted_script_file_with_options(script_text, extension, Some(limit))
        }
    }
}

fn create_persisted_script_file_with_options(
    script_text: &Vec<String>,
    extension: &str,
    filename_limit: Option<usize>,
) -> Result<String, CargoMakeError> {
    let text = script_text.join("\n");

    let string_bytes = text.as_bytes();
    let bytes = md5::compute(string_bytes);
    let mut file_name = format!("{:x}", bytes);

    let default_target_directory = envmnt::get_or("CARGO_MAKE_CRATE_TARGET_DIRECTORY", "target");
    let directory = envmnt::get_or(
        "CARGO_MAKE_CRATE_CUSTOM_TRIPLE_TARGET_DIRECTORY",
        &default_target_directory,
    );
    let mut file_path_buf = PathBuf::new();
    file_path_buf.push(&directory);
    file_path_buf.push("_cargo_make_temp");
    file_path_buf.push("persisted_scripts");
    file_path_buf.push(&file_name);
    file_path_buf.set_extension(extension);
    let mut file_path_string: String = file_path_buf.to_string_lossy().into_owned();

    if let Some(limit) = filename_limit {
        if file_path_string.len() > limit {
            let reduce_size = file_path_string.len() - limit;
            if reduce_size < file_name.len() {
                file_name = file_name[0..file_name.len() - reduce_size].to_string();

                file_path_buf = PathBuf::new();
                file_path_buf.push(&directory);
                file_path_buf.push("_cargo_make_temp");
                file_path_buf.push("persisted_scripts");
                file_path_buf.push(file_name);
                file_path_buf.set_extension(extension);
                file_path_string = file_path_buf.to_string_lossy().into_owned();
            }
        }
    }

    let file_path = file_path_string.as_path();
    if file_path.exists() {
        Ok(file_path_string)
    } else {
        match write_text_file(&file_path_string, &text) {
            Ok(_) => Ok(file_path_string),
            Err(error) => {
                error!("Unable to create file: {} {:#?}", &file_path_string, &error);
                Err(error.into())
            }
        }
    }
}
