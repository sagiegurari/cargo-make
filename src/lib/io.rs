//! # io
//!
//! IO helper functions
//!

#[cfg(test)]
#[path = "io_test.rs"]
mod io_test;

use fsio::file::modify_file;
use fsio::path as fsio_path;
use fsio::path::from_path::FromPath;
use glob::glob;
use ignore::WalkBuilder;
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::path::PathBuf;

pub(crate) fn create_text_file(text: &str, extension: &str) -> String {
    let file_path = fsio_path::get_temporary_file_path(extension);

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
    let file_path = fsio_path::get_temporary_file_path(extension);

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

pub(crate) fn get_path_list(
    glob_pattern: &str,
    include_files: bool,
    include_dirs: bool,
    ignore_type: Option<String>,
) -> Vec<String> {
    let mut path_list = vec![];
    match glob(glob_pattern) {
        Ok(paths) => {
            for entry in paths {
                match entry {
                    Ok(path) => {
                        if (include_dirs && path.is_dir()) || (include_files && path.is_file()) {
                            let mut value_string: String = FromPath::from_path(&path);
                            value_string = value_string.replace("\\", "/");
                            if !value_string.starts_with("./") {
                                value_string.insert_str(0, "./");
                            }

                            path_list.push(value_string);
                        }
                    }
                    Err(error) => {
                        error!(
                            "Error while iterating over path entries of glob: {}, error: {:#?}",
                            glob_pattern, error
                        );
                        return vec![];
                    }
                }
            }
        }
        Err(error) => {
            error!(
                "Error while running glob: {}, error: {:#?}",
                glob_pattern, error
            );
            return vec![];
        }
    }

    if !path_list.is_empty() {
        if let Some(ignore_type_value) = ignore_type {
            let mut included_paths = HashSet::new();

            match ignore_type_value.as_str() {
                "git" => {
                    for entry in WalkBuilder::new("./")
                        .hidden(true)
                        .parents(true)
                        .git_ignore(true)
                        .git_exclude(true)
                        .build()
                    {
                        match entry {
                            Ok(path) => {
                                let mut value_string: String = FromPath::from_path(&path.path());
                                value_string = value_string.replace("\\", "/");
                                included_paths.insert(value_string);
                            }
                            Err(error) => error!(
                                "Error while running git ignore path checks, error: {:#?}",
                                error
                            ),
                        }
                    }
                }
                _ => error!("Unsupported ignore type: {}", &ignore_type_value),
            };

            if included_paths.is_empty() {
                path_list.clear();
            } else {
                let org_path_list = path_list;
                path_list = vec![];
                for path in org_path_list {
                    if included_paths.contains(&path) {
                        path_list.push(path);
                    }
                }
            }
        }
    }

    path_list
}

pub(crate) fn canonicalize_to_string(path_string: &str) -> String {
    #[cfg(not(windows))]
    {
        fsio_path::canonicalize_or(path_string, path_string)
    }
    #[cfg(windows)]
    {
        match dunce::canonicalize(path_string) {
            Ok(value) => FromPath::from_path(&value),
            Err(_) => path_string.to_string(),
        }
    }
}
