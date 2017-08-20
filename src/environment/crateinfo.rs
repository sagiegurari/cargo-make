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
use types::{CrateDependency, CrateInfo, Workspace};

fn get_members_from_dependencies(crate_info: &CrateInfo) -> Vec<String> {
    let mut members = vec![];

    match crate_info.dependencies {
        Some(ref dependencies) => {
            for value in dependencies.values() {
                match *value {
                    CrateDependency::Info(ref info) => {
                        match info.get("path") {
                            Some(ref path) => {
                                if path.starts_with("./") {
                                    let member_path = path.chars().skip(2).take(path.len() - 2).collect();
                                    members.push(member_path);
                                }
                            }
                            None => (),
                        }
                    }
                    _ => (),
                };
            }
        }
        None => (),
    };

    members
}

fn load_workspace_members(crate_info: &mut CrateInfo) {
    let dependencies = get_members_from_dependencies(&crate_info);

    add_members(crate_info, dependencies);
}

fn add_members(
    crate_info: &mut CrateInfo,
    new_members: Vec<String>,
) {
    if new_members.len() > 0 {
        match crate_info.workspace {
            Some(ref mut workspace) => {
                match workspace.members {
                    Some(ref mut members) => {
                        for new_member in new_members.iter() {
                            let member_string = new_member.to_string();

                            match members.iter().position(|member| *member == member_string) {
                                None => members.push(member_string),
                                _ => (),
                            }
                        }
                    }
                    None => workspace.members = Some(new_members),
                }
            }
            None => crate_info.workspace = Some(Workspace { members: Some(new_members) }),
        }
    }
}

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

        let mut crate_info: CrateInfo = match toml::from_str(&crate_info_string) {
            Ok(value) => value,
            Err(error) => panic!("Unable to parse Cargo.toml, {}", error),
        };

        load_workspace_members(&mut crate_info);

        logger.verbose("Loaded Cargo.toml:", &[], Some(&crate_info));

        crate_info
    } else {
        CrateInfo::new()
    }
}
