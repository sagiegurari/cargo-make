//! # crateinfo
//!
//! Loads crate information.
//!

#[cfg(test)]
#[path = "./crateinfo_test.rs"]
mod crateinfo_test;

use log::Logger;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;
use types::CrateInfo;

/// Loads the crate info based on the Cargo.toml found in the current working directory.
///
/// # Arguments
///
/// * `logger` - Logger instance
pub fn load(logger: &Logger) -> CrateInfo {
    // load crate info
    let file_path = Path::new("Cargo.toml");

    if file_path.exists() {
        logger.verbose("Opening file:", &[], Some(&file_path));
        let mut file = match File::open(&file_path) {
            Ok(value) => value,
            Err(error) => panic!("Unable to open Cargo.toml, error: {}", error),
        };
        let mut crate_info_string = String::new();
        file.read_to_string(&mut crate_info_string).unwrap();

        let crate_info: CrateInfo = match toml::from_str(&crate_info_string) {
            Ok(value) => value,
            Err(error) => panic!("Unable to parse Cargo.toml, {}", error),
        };

        logger.verbose("Loaded Cargo.toml:", &[], Some(&crate_info));

        crate_info
    } else {
        CrateInfo::new()
    }
}
