//! # script_utils
//!
//! Helper functions for script invocations.
//!

#[cfg(test)]
#[path = "script_utils_test.rs"]
mod script_utils_test;

use crate::io::create_text_file;
use fsio::file::write_text_file;
use fsio::path::as_path::AsPath;
use sha2::{Digest, Sha256};
use std::fmt::Write;
use std::path::PathBuf;

pub(crate) fn create_script_file(script_text: &Vec<String>, extension: &str) -> String {
    let text = script_text.join("\n");

    create_text_file(&text, &extension)
}

pub(crate) fn create_persisted_script_file(script_text: &Vec<String>, extension: &str) -> String {
    let text = script_text.join("\n");

    let string_bytes = text.as_bytes();
    let bytes = Sha256::digest(string_bytes);
    let file_name = bytes_to_hex(&bytes[..]);

    let directory = envmnt::get_or_panic("CARGO_MAKE_CRATE_CUSTOM_TRIPLE_TARGET_DIRECTORY");
    let mut file_path_buf = PathBuf::new();
    file_path_buf.push(&directory);
    file_path_buf.push("_cargo_make_temp");
    file_path_buf.push("persisted_scripts");
    file_path_buf.push(file_name);
    file_path_buf.set_extension(extension);

    let file_path_string: String = file_path_buf.to_string_lossy().into_owned();

    let file_path = file_path_string.as_path();
    if file_path.exists() {
        file_path_string
    } else {
        match write_text_file(&file_path_string, &text) {
            Ok(_) => file_path_string,
            Err(error) => {
                error!("Unable to create file: {} {:#?}", &file_path_string, &error);
                panic!("Unable to create file, error: {}", error);
            }
        }
    }
}

fn bytes_to_hex(bytes: &[u8]) -> String {
    let mut hex_string = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        match write!(hex_string, "{:02X}", byte) {
            Err(error) => {
                error!(
                    "Unable to conver script hash to hex string, error: {}",
                    &error
                );
                panic!(
                    "Unable to conver script hash to hex string, error: {}",
                    &error
                );
            }
            _ => (),
        }
    }

    hex_string
}
