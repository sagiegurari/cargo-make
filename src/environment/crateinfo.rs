//! # crateinfo
//!
//! Loads crate information.
//!

#[cfg(test)]
#[path = "./crateinfo_test.rs"]
mod crateinfo_test;

use crate::types::{CrateDependency, CrateInfo};
use glob::glob;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

fn expand_glob_members(glob_member: &str) -> Vec<String> {
    match glob(glob_member) {
        Ok(entries) => {
            let mut members = vec![];

            for entry in entries {
                match entry {
                    Ok(path) => {
                        let mut updated_path = path.to_str().unwrap().to_string();
                        updated_path = updated_path.replace("\\", "/");
                        members.push(updated_path);
                    }
                    _ => (),
                };
            }

            members
        }
        _ => vec![],
    }
}

fn normalize_members(crate_info: &mut CrateInfo) {
    match crate_info.workspace {
        Some(ref mut workspace) => {
            match workspace.members {
                Some(ref mut members) => {
                    let existing_members = members.clone();

                    let mut index = 0;
                    for member in existing_members.iter() {
                        // glob
                        if member.contains("*") {
                            let mut expanded_members = expand_glob_members(&member);

                            members.remove(index);
                            members.append(&mut expanded_members);
                        } else {
                            index = index + 1;
                        }
                    }
                }
                None => (),
            };
        }
        None => (), //not a workspace
    }
}

fn get_members_from_dependencies(crate_info: &CrateInfo) -> Vec<String> {
    let mut members = vec![];

    match crate_info.dependencies {
        Some(ref dependencies) => {
            for value in dependencies.values() {
                match *value {
                    CrateDependency::Info(ref info) => match info.path {
                        Some(ref path) => {
                            if path.starts_with("./") {
                                let member_path =
                                    path.chars().skip(2).take(path.len() - 2).collect();
                                members.push(member_path);
                            }
                        }
                        None => (),
                    },
                    _ => (),
                };
            }
        }
        None => (),
    };

    members
}

fn add_members(crate_info: &mut CrateInfo, new_members: Vec<String>) {
    if new_members.len() > 0 {
        match crate_info.workspace {
            Some(ref mut workspace) => match workspace.members {
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
            },
            None => (), //not a workspace
        }
    }
}

fn remove_excludes(crate_info: &mut CrateInfo) -> bool {
    let mut removed = false;

    match crate_info.workspace {
        Some(ref mut workspace) => match workspace.exclude {
            Some(ref excludes) => match workspace.members {
                Some(ref mut members) => {
                    for exclude in excludes.iter() {
                        let exclude_string = exclude.to_string();

                        let result = members.iter().position(|member| *member == exclude_string);
                        match result {
                            Some(index) => {
                                members.remove(index);

                                removed = true;

                                ()
                            }
                            None => (),
                        };
                    }
                }
                None => (),
            },
            None => (),
        },
        None => (), //not a workspace
    };

    removed
}

fn load_workspace_members(crate_info: &mut CrateInfo) {
    if crate_info.workspace.is_some() {
        normalize_members(crate_info);

        let dependencies = get_members_from_dependencies(&crate_info);

        add_members(crate_info, dependencies);

        remove_excludes(crate_info);
    }
}

/// Loads the crate info based on the Cargo.toml found in the current working directory.
pub(crate) fn load() -> CrateInfo {
    // load crate info
    let file_path = Path::new("Cargo.toml");

    if file_path.exists() {
        debug!("Opening file: {:#?}", &file_path);
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

        debug!("Loaded Cargo.toml: {:#?}", &crate_info);

        crate_info
    } else {
        CrateInfo::new()
    }
}
