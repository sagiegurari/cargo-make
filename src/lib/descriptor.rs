//! # descriptor
//!
//! Loads the tasks descriptor.<br>
//! It will first load the default descriptor which is defined in cargo-make internally and
//! afterwards tries to find the external descriptor and load it as well.<br>
//! If an extenal descriptor exists, it will be loaded and extend the default descriptor.
//!

#[cfg(test)]
#[path = "./descriptor_test.rs"]
mod descriptor_test;

use crate::command;
use crate::io;
use crate::types::{
    Config, ConfigSection, EnvFile, EnvFileInfo, EnvValue, Extend, ExternalConfig, ModifyConfig,
    Task,
};
use crate::version;
use envmnt;
use indexmap::IndexMap;
use std::env;
use std::fs::canonicalize;
use std::path::{Path, PathBuf};
use toml;

fn merge_env(
    base: &mut IndexMap<String, EnvValue>,
    extended: &mut IndexMap<String, EnvValue>,
) -> IndexMap<String, EnvValue> {
    let mut merged = IndexMap::<String, EnvValue>::new();

    for (key, value) in base.iter() {
        let key_str = key.to_string();
        let value_clone = value.clone();
        merged.insert(key_str, value_clone);
    }

    for (key, value) in extended.iter() {
        let key_str = key.to_string();

        if !key_str.starts_with("CARGO_MAKE_CURRENT_TASK_") {
            let value_clone = value.clone();

            if merged.contains_key(&key_str) {
                let base_value = merged.swap_remove(&key_str).unwrap();

                match (base_value, value_clone.clone()) {
                    (
                        EnvValue::Profile(ref base_profile_env),
                        EnvValue::Profile(ref extended_profile_env),
                    ) => {
                        let mut base_profile_env_mut = base_profile_env.clone();
                        let mut extended_profile_env_mut = extended_profile_env.clone();

                        let merged_sub_env =
                            merge_env(&mut base_profile_env_mut, &mut extended_profile_env_mut);

                        merged.insert(key_str, EnvValue::Profile(merged_sub_env));
                    }
                    _ => {
                        merged.insert(key_str, value_clone);
                        ()
                    }
                };
            } else {
                merged.insert(key_str, value_clone);
            }
        }
    }

    merged
}

fn merge_env_files(base: &mut Vec<EnvFile>, extended: &mut Vec<EnvFile>) -> Vec<EnvFile> {
    let mut merged: Vec<EnvFile> = vec![];

    for value in extended.iter() {
        merged.push(value.clone());
    }
    for value in base.iter() {
        merged.push(value.clone());
    }

    merged
}

fn merge_tasks(
    base: &mut IndexMap<String, Task>,
    extended: &mut IndexMap<String, Task>,
) -> IndexMap<String, Task> {
    let mut merged = IndexMap::<String, Task>::new();

    for (key, value) in base.iter() {
        let key_str = key.to_string();
        merged.insert(key_str, value.clone());
    }

    for (key, value) in extended.iter() {
        let key_str = key.to_string();
        let mut task = value.clone();

        task = match base.get(key) {
            Some(ref value) => {
                let mut merged_task = Task::new();

                merged_task.extend(value);
                merged_task.extend(&task);

                merged_task
            }
            _ => task,
        };

        merged.insert(key_str, task);
    }

    merged
}

fn add_file_location_info(
    mut external_config: ExternalConfig,
    file_path: &PathBuf,
) -> ExternalConfig {
    let file_path_string = file_path.to_string_lossy().into_owned();
    let base_directory = match file_path.parent() {
        Some(directory) => directory.to_string_lossy().into_owned(),
        None => "".to_string(),
    };

    match external_config.env_files {
        Some(env_files) => {
            let mut modified_env_files = vec![];

            for env_file in env_files {
                match env_file {
                    EnvFile::Path(path) => {
                        let mut info = EnvFileInfo::new(path);
                        info.base_path = Some(base_directory.clone());

                        modified_env_files.push(EnvFile::Info(info));
                    }
                    EnvFile::Info(mut info) => {
                        if info.base_path.is_none() {
                            info.base_path = Some(base_directory.clone());
                        }

                        modified_env_files.push(EnvFile::Info(info));
                    }
                }
            }

            external_config.env_files = Some(modified_env_files);
        }
        None => (),
    };

    let mut tasks_map = IndexMap::new();
    if let Some(tasks) = external_config.tasks.clone() {
        for (task_name, task) in tasks {
            let mut env = match task.env.clone() {
                Some(env) => env,
                None => IndexMap::new(),
            };

            env.insert(
                "CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE".to_string(),
                EnvValue::Value(file_path_string.to_string()),
            );
            env.insert(
                "CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE_DIRECTORY".to_string(),
                EnvValue::Value(base_directory.to_string()),
            );

            let mut updated_task = task.clone();
            updated_task.env = Some(env);
            tasks_map.insert(task_name, updated_task);
        }

        external_config.tasks = Some(tasks_map);
    };

    external_config
}

fn run_load_script(external_config: &ExternalConfig) -> bool {
    match external_config.config {
        Some(ref config) => {
            let load_script = config.get_load_script();

            match load_script {
                Some(ref script) => {
                    debug!("Load script found.");

                    command::run_script_get_exit_code(script, None, &vec![], true);

                    true
                }
                None => {
                    debug!("No load script defined.");
                    false
                }
            }
        }
        None => {
            debug!("No load script defined.");
            false
        }
    }
}

fn merge_external_configs(config: ExternalConfig, parent_config: ExternalConfig) -> ExternalConfig {
    // merge env files
    let mut parent_env_files = match parent_config.env_files {
        Some(env_files) => env_files,
        None => vec![],
    };
    let mut extended_env_files = match config.env_files {
        Some(env_files) => env_files,
        None => vec![],
    };
    let all_env_files = merge_env_files(&mut parent_env_files, &mut extended_env_files);

    // merge env
    let mut parent_env = match parent_config.env {
        Some(env) => env,
        None => IndexMap::new(),
    };
    let mut extended_env = match config.env {
        Some(env) => env,
        None => IndexMap::new(),
    };
    let all_env = merge_env(&mut parent_env, &mut extended_env);

    // merge tasks
    let mut parent_tasks = match parent_config.tasks {
        Some(tasks) => tasks,
        None => IndexMap::new(),
    };
    let mut extended_tasks = match config.tasks {
        Some(tasks) => tasks,
        None => IndexMap::new(),
    };
    let all_tasks = merge_tasks(&mut parent_tasks, &mut extended_tasks);

    let mut config_section = ConfigSection::new();
    if parent_config.config.is_some() {
        let mut config_section_data = parent_config.config.unwrap();
        debug!("Adding parent config section: {:#?}", &config_section_data);
        config_section.extend(&mut config_section_data);
    }
    if config.config.is_some() {
        let mut config_section_data = config.config.unwrap();
        debug!("Adding config section: {:#?}", &config_section_data);
        config_section.extend(&mut config_section_data);
    }

    ExternalConfig {
        extend: None,
        config: Some(config_section),
        env_files: Some(all_env_files),
        env: Some(all_env),
        tasks: Some(all_tasks),
    }
}

fn load_descriptor_extended_makefiles(
    parent_path: &str,
    extend_struct: &Extend,
) -> Result<ExternalConfig, String> {
    match extend_struct {
        Extend::Path(base_file) => load_external_descriptor(parent_path, &base_file, true, false),
        Extend::Options(extend_options) => {
            let force = !extend_options.optional.unwrap_or(false);
            load_external_descriptor(parent_path, &extend_options.path, force, false)
        }
        Extend::List(extend_list) => {
            let mut ordered_list_config = ExternalConfig::new();

            for entry in extend_list.iter() {
                let extend_options = entry.clone();
                let entry_config = load_descriptor_extended_makefiles(
                    parent_path,
                    &Extend::Options(extend_options),
                )?;

                // merge configs
                ordered_list_config = merge_external_configs(entry_config, ordered_list_config);
            }

            Ok(ordered_list_config)
        }
    }
}

/// Ensure the Makefile's min_version, if present, is older than cargo-make's
/// currently running version.
fn check_makefile_min_version(external_descriptor: &str) -> Result<(), String> {
    let value: toml::Value = match toml::from_str(&external_descriptor) {
        Ok(value) => value,
        // If there's an error parsing the file, let the caller function figure
        // it out
        Err(_) => return Ok(()),
    };

    let min_version = value
        .get("config")
        .and_then(|config| config.get("min_version"))
        .and_then(|min_ver| min_ver.as_str());

    if let Some(ref min_version) = min_version {
        if version::is_newer_found(&min_version) {
            return Err(min_version.to_string());
        }
    }

    Ok(())
}

fn load_external_descriptor(
    base_path: &str,
    file_name: &str,
    force: bool,
    set_env: bool,
) -> Result<ExternalConfig, String> {
    debug!(
        "Loading tasks from file: {} base directory: {}",
        &file_name, &base_path
    );

    let file_path = Path::new(base_path).join(file_name);

    if file_path.exists() && file_path.is_file() {
        let absolute_file_path = match canonicalize(&file_path) {
            Ok(result_path) => result_path,
            _ => file_path.clone(),
        };

        if set_env {
            envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &absolute_file_path);
        }

        let external_descriptor = io::read_text_file(&file_path);

        check_makefile_min_version(&external_descriptor)?;

        let mut file_config: ExternalConfig = match toml::from_str(&external_descriptor) {
            Ok(value) => value,
            Err(error) => panic!("Unable to parse external descriptor, {}", error),
        };
        debug!("Loaded external config: {:#?}", &file_config);

        file_config = add_file_location_info(file_config, &absolute_file_path);

        run_load_script(&file_config);

        match file_config.extend {
            Some(ref extend_struct) => {
                let parent_path_buf = Path::new(base_path).join(file_name).join("..");
                let parent_path = file_path
                    .parent()
                    .unwrap_or(&parent_path_buf)
                    .to_str()
                    .unwrap_or(".");
                debug!("External config parent path: {}", &parent_path);

                let base_file_config =
                    load_descriptor_extended_makefiles(&parent_path, extend_struct)?;

                Ok(merge_external_configs(
                    file_config.clone(),
                    base_file_config,
                ))
            }
            None => Ok(file_config),
        }
    } else if force {
        error!("Descriptor file: {:#?} not found.", &file_path);
        panic!("Descriptor file: {:#?} not found.", &file_path);
    } else {
        info!("External file not found or is not a file, skipping.");

        Ok(ExternalConfig::new())
    }
}

pub(crate) fn load_internal_descriptors(
    stable: bool,
    experimental: bool,
    modify_config: Option<ModifyConfig>,
) -> Config {
    debug!("Loading base tasks.");

    let base_descriptor = if stable {
        include_str!("Makefile.stable.toml")
    } else {
        include_str!("Makefile.base.toml")
    };

    let mut base_config: Config = match toml::from_str(base_descriptor) {
        Ok(value) => value,
        Err(error) => panic!("Unable to parse base descriptor, {}", error),
    };
    debug!("Loaded base config: {:#?}", &base_config);

    if experimental {
        debug!("Loading experimental tasks.");
        let experimental_descriptor = include_str!("Makefile.beta.toml");

        let experimental_config: Config = match toml::from_str(experimental_descriptor) {
            Ok(value) => value,
            Err(error) => panic!("Unable to parse experimental descriptor, {}", error),
        };
        debug!("Loaded experimental config: {:#?}", &experimental_config);

        let mut base_tasks = base_config.tasks;
        let mut experimental_tasks = experimental_config.tasks;
        let all_tasks = merge_tasks(&mut base_tasks, &mut experimental_tasks);

        base_config.tasks = all_tasks;
    }

    // reset
    envmnt::set("CARGO_MAKE_CORE_TASK_NAMESPACE", "");
    envmnt::set("CARGO_MAKE_CORE_TASK_NAMESPACE_PREFIX", "");

    match modify_config {
        Some(props) => {
            base_config.apply(&props);

            match props.namespace {
                Some(ref namespace) => {
                    let prefix = props.get_namespace_prefix();

                    envmnt::set("CARGO_MAKE_CORE_TASK_NAMESPACE", &namespace);
                    envmnt::set("CARGO_MAKE_CORE_TASK_NAMESPACE_PREFIX", &prefix);
                }
                None => (),
            };
        }
        None => (),
    };

    base_config
}

/// Loads the tasks descriptor.<br>
/// It will first load the default descriptor which is defined in cargo-make internally and
/// afterwards tries to find the external descriptor and load it as well.<br>
/// If an extenal descriptor exists, it will be loaded and extend the default descriptor.
/// If one of the descriptor requires a newer version of cargo-make, returns an error with the
/// minimum version required by the descriptor.
fn load_descriptors(
    file_name: &str,
    force: bool,
    env_map: Option<Vec<String>>,
    stable: bool,
    experimental: bool,
    modify_core_tasks: Option<ModifyConfig>,
) -> Result<Config, String> {
    let default_config = load_internal_descriptors(stable, experimental, modify_core_tasks);

    let mut external_config: ExternalConfig =
        load_external_descriptor(".", file_name, force, true)?;

    external_config = match env::var("CARGO_MAKE_WORKSPACE_MAKEFILE") {
        Ok(workspace_makefile) => {
            let mut pathbuf = PathBuf::from(workspace_makefile);
            match pathbuf.clone().file_name() {
                Some(workspace_file_name) => match workspace_file_name.to_str() {
                    Some(workspace_file_name_str) => {
                        pathbuf.pop();

                        match pathbuf.to_str() {
                            Some(directory) => {
                                let workspace_config = load_external_descriptor(
                                    directory,
                                    workspace_file_name_str,
                                    false,
                                    false,
                                )?;
                                merge_external_configs(external_config, workspace_config)
                            }
                            _ => external_config,
                        }
                    }
                    _ => external_config,
                },
                _ => external_config,
            }
        }
        _ => external_config,
    };

    let mut external_tasks = match external_config.tasks {
        Some(tasks) => tasks,
        None => IndexMap::new(),
    };
    let mut default_tasks = default_config.tasks;

    let env_files = match external_config.env_files {
        Some(env_files) => env_files,
        None => vec![],
    };

    let mut external_env = match external_config.env {
        Some(env) => env,
        None => IndexMap::new(),
    };
    let mut default_env = default_config.env;

    // merge env
    let mut all_env = merge_env(&mut default_env, &mut external_env);
    all_env = match env_map {
        Some(values) => {
            let mut cli_env = IndexMap::new();

            for env_pair in &values {
                let env_part: Vec<&str> = env_pair.split('=').collect();
                debug!("Checking env pair: {}", &env_pair);

                if env_part.len() == 2 {
                    cli_env.insert(
                        env_part[0].to_string(),
                        EnvValue::Value(env_part[1].to_string()),
                    );
                }
            }

            merge_env(&mut all_env, &mut cli_env)
        }
        None => all_env,
    };

    let all_tasks = merge_tasks(&mut default_tasks, &mut external_tasks);

    let mut config_section = default_config.config.clone();
    config_section.extend(&mut external_config.config.unwrap_or(ConfigSection::new()));

    let config = Config {
        config: config_section,
        env_files,
        env: all_env,
        tasks: all_tasks,
    };

    debug!("Loaded merged config: {:#?}", &config);

    Ok(config)
}

/// Loads the tasks descriptor.<br>
/// It will first load the default descriptor which is defined in cargo-make internally and
/// afterwards tries to find the external descriptor and load it as well.<br>
/// If an extenal descriptor exists, it will be loaded and extend the default descriptor. <br>
/// If one of the descriptor requires a newer version of cargo-make, returns an error with the
/// minimum version required by the descriptor.
pub(crate) fn load(
    file_name: &str,
    force: bool,
    env_map: Option<Vec<String>>,
    experimental: bool,
) -> Result<Config, String> {
    let mut config =
        load_descriptors(&file_name, force, env_map.clone(), true, experimental, None)?;

    if config.config.skip_core_tasks.unwrap_or(false) {
        config = load_descriptors(&file_name, force, env_map.clone(), false, false, None)?;
    } else {
        let modify_core_tasks = config.config.modify_core_tasks.clone();

        match modify_core_tasks {
            Some(modify_config) => {
                if modify_config.is_modifications_defined() {
                    config = load_descriptors(
                        &file_name,
                        force,
                        env_map.clone(),
                        true,
                        experimental,
                        Some(modify_config),
                    )?;
                }
            }
            None => (),
        };
    }

    Ok(config)
}
