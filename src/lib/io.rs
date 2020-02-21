//! # io
//!
//! IO helper functions
//!

#[cfg(test)]
#[path = "./io_test.rs"]
mod io_test;

use fsio::file::modify_file;
use fsio::path::get_temporary_file_path;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub(crate) fn create_text_file(text: &str, extension: &str) -> String {
    let file_path = get_temporary_file_path(extension);

    match fsio::file::write_text_file(&file_path, text) {
        Ok(_) => file_path,
        Err(error) => {
            error!("Unable to create file: {} {:#?}", &file_path, &error);
            panic!("Unable to create file, error: {}", error);
        }
    }
}

pub(crate) fn create_file(
    write_content: &Fn(&mut File) -> io::Result<()>,
    extension: &str,
) -> String {
    let file_path = get_temporary_file_path(extension);

    match modify_file(&file_path, write_content, false) {
        Ok(_) => file_path,
        Err(error) => {
            error!("Unable to write to file: {} {:#?}", &file_path, &error);
            panic!("Unable to write to file, error: {}", error);
        }
    }
}

pub(crate) fn delete_file(file: &str) {
    match fsio::file::delete(file) {
        Ok(_) => debug!("File deleted: {}", &file),
        Err(error) => debug!("Unable to delete file: {} {:#?}", &file, error),
    }
}

pub(crate) fn write_text_file(file_path: &str, text: &str) -> bool {
    match fsio::file::write_text_file(file_path, text) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub(crate) fn read_text_file(file_path: &PathBuf) -> String {
    debug!("Opening file: {:#?}", &file_path);

    match fsio::file::read_text_file(file_path) {
        Ok(content) => content,
        Err(error) => panic!("Unable to read file: {:?} error: {:#?}", file_path, error),
    }
}
