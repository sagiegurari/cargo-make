//! # crateinfo
//!
//! Loads crate information.
//!

#[cfg(test)]
#[path = "crateinfo_test.rs"]
mod crateinfo_test;

use crate::types::{CrateDependency, CrateInfo};
use cargo_metadata::camino::Utf8PathBuf;
use cargo_metadata::MetadataCommand;
use fsio;
use glob::glob;
use std::env;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

fn expand_glob_members(glob_member: &str) -> Vec<String> {
    let emulation = envmnt::is("CARGO_MAKE_WORKSPACE_EMULATION");

    match glob(glob_member) {
        Ok(entries) => {
            let mut members = vec![];

            for entry in entries {
                match entry {
                    Ok(path) => {
                        let should_add = if emulation {
                            // emulation may be used for non rust projects
                            // so no extra validations
                            true
                        } else {
                            // ensure Cargo.toml is found
                            let mut cargo_path = path.clone();
                            cargo_path.push("Cargo.toml");
                            let exists = cargo_path.exists();

                            exists
                        };

                        if should_add {
                            let mut updated_path = path.to_str().unwrap().to_string();
                            updated_path = updated_path.replace("\\", "/");
                            members.push(updated_path);
                        }
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
            if let Some(ref mut members) = workspace.members {
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

        if crate_info.package.is_some() {
            let members_vec = vec![".".to_string()];
            add_members(crate_info, members_vec);
        }

        remove_excludes(crate_info);
    }
}

/// Loads the crate info based on the Cargo.toml found in the current working directory.
pub(crate) fn load() -> CrateInfo {
    load_from(Path::new("Cargo.toml").to_path_buf())
}

pub(crate) fn load_from(file_path: PathBuf) -> CrateInfo {
    if file_path.exists() {
        debug!("Reading file: {:#?}", &file_path);
        let crate_info_string = match fsio::file::read_text_file(&file_path) {
            Ok(content) => content,
            Err(error) => panic!("Unable to open Cargo.toml, error: {}", error),
        };

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

#[derive(Debug, Deserialize)]
struct CargoConfig {
    build: Option<CargoConfigBuild>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
struct CargoConfigBuild {
    target: Option<RustTarget>,
}

#[derive(Debug, Deserialize)]
#[serde(from = "PathBuf")]
struct RustTarget(PathBuf);

impl RustTarget {
    fn name(&self) -> &str {
        self.0.file_stem().unwrap().to_str().unwrap()
    }
}

impl From<PathBuf> for RustTarget {
    fn from(buf: PathBuf) -> Self {
        Self(buf)
    }
}

impl AsRef<OsStr> for RustTarget {
    fn as_ref(&self) -> &OsStr {
        self.0.as_ref()
    }
}

fn get_cargo_config(home: Option<PathBuf>) -> Option<CargoConfig> {
    let path = env::current_dir().ok()?;

    let config_file = path
        .ancestors()
        .map(|ancestor| ancestor.join(".cargo"))
        .chain(home)
        .map(|config_file| config_file.join("config"))
        .filter_map(|config_file| {
            let config_file_with_extension = config_file.with_extension("toml");

            if config_file.exists() {
                Some(config_file)
            } else if config_file_with_extension.exists() {
                Some(config_file_with_extension)
            } else {
                None
            }
        })
        .next()?;

    let config_file = fsio::file::read_text_file(&config_file).ok()?;
    toml::from_str(&config_file).ok()
}

pub(crate) fn crate_target_triple(
    default_target_triple: Option<String>,
    home: Option<PathBuf>,
) -> Option<String> {
    get_cargo_config(home)
        .and_then(|config| config.build)
        .and_then(|build| build.target)
        .map(|target| target.name().to_string())
        .or(default_target_triple)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct CrateTargetDirs {
    pub(crate) host: Utf8PathBuf,
    pub(crate) custom: Option<Utf8PathBuf>,
}

pub(crate) fn crate_target_dirs(home: Option<PathBuf>) -> CrateTargetDirs {
    let metadata = MetadataCommand::new()
        .exec()
        .map_err(|err| debug!("Unable to extract cargo metadata, error: {}", err))
        .ok();
    let host = metadata
        .map(|metadata| metadata.target_directory)
        .unwrap_or_else(|| "target".into());
    let custom = get_cargo_config(home)
        .and_then(|config| config.build)
        .and_then(|build| build.target)
        .map(|target_triple| host.join(target_triple.name()));
    CrateTargetDirs { host, custom }
}

pub(crate) fn search_workspace_root() -> Option<String> {
    if envmnt::is("CARGO_MAKE_WORKSPACE_EMULATION") {
        search_workspace_root_for_emulation()
    } else {
        search_workspace_root_via_metadata()
    }
}

fn search_workspace_root_for_emulation() -> Option<String> {
    let path_value = envmnt::get_any(
        &vec![
            "CARGO_MAKE_WORKSPACE_EMULATION_ROOT_DIRECTORY",
            "CARGO_MAKE_WORKING_DIRECTORY",
        ],
        "",
    );

    if path_value.is_empty() {
        None
    } else {
        Some(path_value)
    }
}

fn search_workspace_root_via_metadata() -> Option<String> {
    debug!("Getting cargo metadata.");

    MetadataCommand::new()
        .exec()
        .map(|metadata| metadata.workspace_root.to_string())
        .map_err(|err| debug!("Unable to extract cargo metadata, error: {:#?}", err))
        .ok()
}
