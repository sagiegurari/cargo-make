//! # descriptor
//!
//! Loads the tasks descriptor.<br>
//! It will first load the default descriptor which is defined in cargo-make
//! internally and afterwards tries to find the external descriptor and load it
//! as well.<br> If an external descriptor exists, it will be loaded and extend
//! the default descriptor.

#[cfg(test)]
#[path = "mod_test.rs"]
mod mod_test;

mod cargo_alias;
pub(crate) mod descriptor_deserializer;
mod env;
mod makefiles;

use std::path::{Path, PathBuf};

use fsio::path::as_path::AsPath;
use fsio::path::from_path::FromPath;
use indexmap::IndexMap;

use crate::descriptor::env::{merge_env, merge_env_files, merge_env_scripts};
use crate::error::CargoMakeError;
use crate::plugin::descriptor::merge_plugins_config;
use crate::types::{
    Config, ConfigSection, EnvFile, EnvFileInfo, EnvValue, Extend, ExternalConfig, ModifyConfig,
    Task,
};
use crate::{io, scriptengine, version};

fn merge_tasks(
    base: &mut IndexMap<String, Task>,
    extended: &mut IndexMap<String, Task>,
    merge_task_env: bool,
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

                if merge_task_env && value.env.is_some() && task.env.is_some() {
                    let extended_env = task.env.clone().unwrap();
                    let clear = task.clear.clone().unwrap_or(false);
                    if extended_env.len() == 2
                        && extended_env.contains_key("CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE")
                        && extended_env
                            .contains_key("CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE_DIRECTORY")
                        && !clear
                    {
                        let base_env = value.env.clone().unwrap();
                        merged_task.env = Some(base_env);
                    }
                }

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
    file_path_string: &str,
) -> ExternalConfig {
    let file_path = file_path_string.as_path();
    let base_directory = match file_path.parent() {
        Some(directory) => FromPath::from_path(directory),
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

fn run_load_script(external_config: &ExternalConfig) -> Result<bool, CargoMakeError> {
    match external_config.config {
        Some(ref config) => {
            let load_script = config.get_load_script();

            match load_script {
                Some(ref script) => {
                    debug!("Load script found.");

                    scriptengine::invoke_script_pre_flow(script, None, None, None, true, &vec![])?;

                    Ok(true)
                }
                None => {
                    debug!("No load script defined.");
                    Ok(false)
                }
            }
        }
        None => {
            debug!("No load script defined.");
            Ok(false)
        }
    }
}

fn merge_external_configs(
    config: ExternalConfig,
    parent_config: ExternalConfig,
) -> Result<ExternalConfig, CargoMakeError> {
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
    let all_env = merge_env(&mut parent_env, &mut extended_env)?;

    // merge env scripts
    let mut parent_env_scripts = match parent_config.env_scripts {
        Some(env_scripts) => env_scripts,
        None => vec![],
    };
    let mut extended_env_scripts = match config.env_scripts {
        Some(env_scripts) => env_scripts,
        None => vec![],
    };
    let all_env_scripts = merge_env_scripts(&mut parent_env_scripts, &mut extended_env_scripts);

    // merge tasks
    let mut parent_tasks = match parent_config.tasks {
        Some(tasks) => tasks,
        None => IndexMap::new(),
    };
    let mut extended_tasks = match config.tasks {
        Some(tasks) => tasks,
        None => IndexMap::new(),
    };
    let all_tasks = merge_tasks(&mut parent_tasks, &mut extended_tasks, false);

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

    let plugins = merge_plugins_config(parent_config.plugins, config.plugins);

    let config = ExternalConfig {
        extend: None,
        config: Some(config_section),
        env_files: Some(all_env_files),
        env: Some(all_env),
        env_scripts: Some(all_env_scripts),
        tasks: Some(all_tasks),
        plugins,
    };

    Ok(config)
}

fn load_descriptor_extended_makefiles(
    parent_path: &str,
    extend_struct: &Extend,
) -> Result<ExternalConfig, CargoMakeError> {
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
                ordered_list_config = merge_external_configs(entry_config, ordered_list_config)?;
            }

            Ok(ordered_list_config)
        }
    }
}

/// Ensure the Makefile's min_version, if present, is older than cargo-make's
/// currently running version.
fn check_makefile_min_version(external_descriptor: &str) -> Result<(), CargoMakeError> {
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
            return Err(CargoMakeError::VersionTooOld(min_version.to_string()));
        }
    }

    Ok(())
}

fn load_external_descriptor(
    base_path: &str,
    file_name: &str,
    force: bool,
    set_env: bool,
) -> Result<ExternalConfig, CargoMakeError> {
    debug!(
        "Loading tasks from file: {} base directory: {}",
        &file_name, &base_path
    );

    let file_path = Path::new(base_path).join(file_name);

    if file_path.exists() && file_path.is_file() {
        let file_path_string: String = FromPath::from_path(&file_path);
        let absolute_file_path = io::canonicalize_to_string(&file_path_string);

        if set_env {
            envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &absolute_file_path);
        }

        let external_descriptor = io::read_text_file(&file_path)?;

        check_makefile_min_version(&external_descriptor)?;

        let mut file_config =
            descriptor_deserializer::load_external_config(&external_descriptor, &file_path_string)?;
        debug!("Loaded external config: {:#?}", &file_config);

        file_config = add_file_location_info(file_config, &absolute_file_path);

        run_load_script(&file_config)?;

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

                merge_external_configs(file_config.clone(), base_file_config)
            }
            None => Ok(file_config),
        }
    } else if force {
        error!("Descriptor file: {:#?} not found.", &file_path);
        Err(CargoMakeError::NotFound(format!(
            "Descriptor file: {:#?} not found.",
            &file_path
        )))
    } else {
        debug!("External file not found or is not a file, skipping.");

        Ok(ExternalConfig::new())
    }
}

pub(crate) fn load_internal_descriptors(
    stable: bool,
    experimental: bool,
    modify_config: Option<ModifyConfig>,
) -> Result<Config, CargoMakeError> {
    debug!("Loading base tasks.");

    let base_descriptor = if stable {
        makefiles::STABLE
    } else {
        makefiles::BASE
    };

    let mut base_config = descriptor_deserializer::load_config(&base_descriptor, false)?;
    debug!("Loaded base config: {:#?}", &base_config);

    if experimental {
        debug!("Loading experimental tasks.");
        let experimental_descriptor = makefiles::BETA;

        let experimental_config =
            descriptor_deserializer::load_config(&experimental_descriptor, false)?;
        debug!("Loaded experimental config: {:#?}", &experimental_config);

        let mut base_tasks = base_config.tasks;
        let mut experimental_tasks = experimental_config.tasks;
        let all_tasks = merge_tasks(&mut base_tasks, &mut experimental_tasks, false);

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

    Ok(base_config)
}

fn merge_base_config_and_external_config(
    base_config: Config,
    external_config: ExternalConfig,
    env_map: Option<Vec<String>>,
    late_merge: bool,
) -> Result<Config, CargoMakeError> {
    let mut external_tasks = match external_config.tasks {
        Some(tasks) => tasks,
        None => IndexMap::new(),
    };
    let mut base_tasks = base_config.tasks;

    let env_files = match external_config.env_files {
        Some(env_files) => env_files,
        None => vec![],
    };

    let env_scripts = match external_config.env_scripts {
        Some(env_scripts) => env_scripts,
        None => vec![],
    };

    let mut external_env = match external_config.env {
        Some(env) => env,
        None => IndexMap::new(),
    };
    let mut base_env = base_config.env;

    // merge env
    let mut all_env = merge_env(&mut base_env, &mut external_env)?;
    all_env = match env_map {
        Some(values) => {
            let mut cli_env = IndexMap::new();

            for env_pair in &values {
                debug!("Checking env pair: {}", &env_pair);
                let env_parts: Option<(&str, &str)> = split_once(env_pair, '=');

                if let Some((key, value)) = env_parts {
                    cli_env.insert(key.to_string(), EnvValue::Value(value.to_string()));
                }
            }

            // cli env should be ordered first and not overwritten
            // to enable composite/mapped env vars in makefiles to work correctly
            for (key, value) in all_env.iter() {
                let key_str = key.to_string();

                if !cli_env.contains_key(&key_str) {
                    cli_env.insert(key_str, value.clone());
                }
            }
            cli_env
        }
        None => all_env,
    };

    let all_tasks = merge_tasks(&mut base_tasks, &mut external_tasks, late_merge);

    let mut config_section = base_config.config.clone();
    config_section.extend(&mut external_config.config.unwrap_or(ConfigSection::new()));

    let plugins = merge_plugins_config(base_config.plugins, external_config.plugins);

    let config = Config {
        config: config_section,
        env_files,
        env: all_env,
        env_scripts,
        tasks: all_tasks,
        plugins,
    };

    Ok(config)
}

fn split_once(value: &str, delimiter: char) -> Option<(&str, &str)> {
    let mut parts = value.splitn(2, delimiter);
    let part1 = parts.next()?;
    let part2 = parts.next()?;
    Some((part1, part2))
}

/// Loads the tasks descriptor.<br>
/// It will first load the default descriptor which is defined in cargo-make
/// internally and afterwards tries to find the external descriptor and load it
/// as well.<br> If an external descriptor exists, it will be loaded and extend
/// the default descriptor. If one of the descriptor requires a newer version of
/// cargo-make, returns an error with the minimum version required by the
/// descriptor.
fn load_descriptors(
    file_name: &str,
    force: bool,
    env_map: Option<Vec<String>>,
    stable: bool,
    experimental: bool,
    modify_core_tasks: Option<ModifyConfig>,
) -> Result<Config, CargoMakeError> {
    let default_config = load_internal_descriptors(stable, experimental, modify_core_tasks)?;

    let mut external_config = load_external_descriptor(".", file_name, force, true)?;

    external_config = match std::env::var("CARGO_MAKE_WORKSPACE_MAKEFILE") {
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
                                merge_external_configs(external_config, workspace_config)?
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

    let config =
        merge_base_config_and_external_config(default_config, external_config, env_map, false)?;

    debug!("Loaded merged config: {:#?}", &config);

    Ok(config)
}

fn load_cargo_aliases(config: &mut Config) -> Result<(), CargoMakeError> {
    if let Some(load_cargo_aliases) = config.config.load_cargo_aliases {
        if load_cargo_aliases {
            let alias_tasks = cargo_alias::load()?;
            for (name, task) in alias_tasks {
                match config.tasks.get(&name) {
                    None => {
                        debug!("Creating cargo alias task: {}", &name);
                        config.tasks.insert(name, task);
                    }
                    Some(_) => debug!("Ignoring cargo alias task: {}", &name),
                }
            }
        }
    }
    Ok(())
}

/// Loads the tasks descriptor.<br>
/// It will first load the default descriptor which is defined in cargo-make
/// internally and afterwards tries to find the external descriptor and load it
/// as well.<br> If an external descriptor exists, it will be loaded and extend
/// the default descriptor. <br> If one of the descriptor requires a newer
/// version of cargo-make, returns an error with the minimum version required by
/// the descriptor.
pub fn load(
    file_name: &str,
    force: bool,
    env_map: Option<Vec<String>>,
    experimental: bool,
) -> Result<Config, CargoMakeError> {
    // load extended descriptor only
    let mut config = load_descriptors(&file_name, force, env_map.clone(), false, false, None)?;

    // need to load core tasks as well
    if !config.config.skip_core_tasks.unwrap_or(false) {
        let modify_core_tasks = config.config.modify_core_tasks.clone();

        match modify_core_tasks {
            Some(modify_config) => {
                if modify_config.is_modifications_defined() {
                    // reload everything with core modifications
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
            None => {
                let core_config = load_internal_descriptors(true, experimental, modify_core_tasks)?;
                let external_config = ExternalConfig {
                    extend: None,
                    config: Some(config.config),
                    env_files: Some(config.env_files),
                    env: Some(config.env),
                    env_scripts: Some(config.env_scripts),
                    tasks: Some(config.tasks),
                    plugins: config.plugins,
                };

                config = merge_base_config_and_external_config(
                    core_config,
                    external_config,
                    env_map.clone(),
                    true,
                )?;
            }
        };
    }

    load_cargo_aliases(&mut config)?;

    if let Some(unstable_features) = &config.config.unstable_features {
        for feature in unstable_features {
            config
                .env
                .insert(feature.to_env_name(), EnvValue::Boolean(true));
        }
    }

    for (name, _) in config.tasks.iter() {
        match name.as_str() {
            "before_each" => config.config.before_each_task = Some(name.to_string()),
            "after_each" => config.config.after_each_task = Some(name.to_string()),
            _ => {}
        }
    }

    Ok(config)
}
