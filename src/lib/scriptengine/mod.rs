//! # scriptengine
//!
//! Facade for all different non OS scripts.
//!

pub(crate) mod duck_script;
pub(crate) mod generic_script;
mod os_script;
mod rsscript;
pub(crate) mod script_utils;
mod shebang_script;
mod shell_to_batch;

#[cfg(test)]
#[path = "mod_test.rs"]
mod mod_test;

use crate::environment;
use crate::error::CargoMakeError;
use crate::io;
use crate::toolchain;
use crate::types::{FlowInfo, FlowState, ScriptValue, Task};
use std::cell::RefCell;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
/// The currently supported engine types
pub(crate) enum EngineType {
    /// OS native script
    OS,
    /// Duckscript script runner
    Duckscript,
    /// Rust language
    Rust,
    /// shell to windows batch conversion
    Shell2Batch,
    /// Generic script runner
    Generic,
    /// Shebang script runner
    Shebang,
    /// Unsupported type
    Unsupported,
}

pub(crate) fn get_script_text(script: &ScriptValue) -> Result<Vec<String>, CargoMakeError> {
    match script {
        ScriptValue::SingleLine(text) => Ok(vec![text.clone()]),
        ScriptValue::Text(text) => Ok(text.clone()),
        ScriptValue::File(info) => {
            let mut file_path_string = String::new();
            if !info.absolute_path.unwrap_or(false) {
                file_path_string.push_str("${CARGO_MAKE_WORKING_DIRECTORY}/");
            }
            file_path_string.push_str(&info.file);

            // expand env
            let expanded_value = environment::expand_value(&file_path_string);

            let mut file_path = PathBuf::new();
            file_path.push(expanded_value);

            let script_text = io::read_text_file(&file_path)?;
            let lines: Vec<&str> = script_text.split('\n').collect();

            let mut script_lines: Vec<String> = vec![];

            for line in lines.iter() {
                script_lines.push(line.to_string());
            }

            Ok(script_lines)
        }
        ScriptValue::Sections(sections) => {
            let mut script_lines = vec![];

            if let Some(ref text) = sections.pre {
                script_lines.push(text.to_string());
            }
            if let Some(ref text) = sections.main {
                script_lines.push(text.to_string());
            }
            if let Some(ref text) = sections.post {
                script_lines.push(text.to_string());
            }

            Ok(script_lines)
        }
    }
}

fn get_internal_runner(script_runner: &str) -> EngineType {
    if script_runner == "@duckscript" {
        debug!("Duckscript detected.");
        EngineType::Duckscript
    } else if script_runner == "@rust" {
        debug!("Rust script detected.");
        EngineType::Rust
    } else if script_runner == "@shell" {
        debug!("Shell to batch detected.");
        EngineType::Shell2Batch
    } else {
        EngineType::Unsupported
    }
}

pub(crate) fn get_engine_type(
    script: &ScriptValue,
    script_runner: &Option<String>,
    script_extension: &Option<String>,
) -> Result<EngineType, CargoMakeError> {
    match script_runner {
        Some(ref runner) => {
            debug!("Checking script runner: {}", runner);

            let engine_type = get_internal_runner(runner);

            match engine_type {
                EngineType::Unsupported => {
                    if script_extension.is_some() {
                        // if both script runner and extension is defined, we use generic script runner
                        debug!("Generic script detected.");
                        Ok(EngineType::Generic)
                    } else {
                        // use default OS extension with custom runner
                        debug!("OS script with custom runner detected.");
                        Ok(EngineType::OS)
                    }
                }
                _ => Ok(engine_type),
            }
        }
        None => {
            // if no runner specified, try to extract it from script content
            let script_text = get_script_text(&script)?;

            let shebang = shebang_script::get_shebang(&script_text);
            match shebang.runner {
                Some(script_runner) => {
                    if shebang.arguments.is_none() {
                        let engine_type = get_internal_runner(&script_runner);

                        match engine_type {
                            EngineType::Unsupported => {
                                debug!("Shebang line does not point to an internal engine, using normal shebang script runner.");
                                Ok(EngineType::Shebang)
                            }
                            _ => Ok(engine_type),
                        }
                    } else {
                        Ok(EngineType::Shebang)
                    }
                }
                None => Ok(EngineType::OS),
            }
        }
    }
}

pub(crate) fn invoke(
    task: &Task,
    flow_info: &FlowInfo,
    flow_state: Rc<RefCell<FlowState>>,
) -> Result<bool, CargoMakeError> {
    match task.script {
        Some(ref script) => {
            let validate = !task.should_ignore_errors();

            // setup toolchain environment
            let (reset_env, original_cargo) = match task.toolchain {
                Some(ref toolchain) => match toolchain::get_cargo_binary_path(toolchain) {
                    Some(cargo_binary) => (true, envmnt::get_set("CARGO", cargo_binary)),
                    None => (false, None),
                },
                None => (false, None),
            };

            let output = invoke_script_in_flow_context(
                script,
                task.script_runner.clone(),
                task.script_runner_args.clone(),
                task.script_extension.clone(),
                validate,
                Some(flow_info),
                Some(flow_state),
            );

            // reset toolchain environment
            if reset_env {
                if let Some(value) = original_cargo {
                    envmnt::set("CARGO", value)
                }
            }

            output
        }
        None => Ok(false),
    }
}

pub(crate) fn invoke_script_in_flow_context(
    script: &ScriptValue,
    script_runner: Option<String>,
    script_runner_args: Option<Vec<String>>,
    script_extension: Option<String>,
    validate: bool,
    flow_info: Option<&FlowInfo>,
    flow_state: Option<Rc<RefCell<FlowState>>>,
) -> Result<bool, CargoMakeError> {
    let cli_arguments = match flow_info {
        Some(info) => match info.cli_arguments {
            Some(ref args) => args.clone(),
            None => vec![],
        },
        None => vec![],
    };

    invoke_script(
        script,
        script_runner,
        script_runner_args,
        script_extension,
        validate,
        flow_info,
        flow_state,
        &cli_arguments,
    )
}

pub(crate) fn invoke_script_pre_flow(
    script: &ScriptValue,
    script_runner: Option<String>,
    script_runner_args: Option<Vec<String>>,
    script_extension: Option<String>,
    validate: bool,
    cli_arguments: &Vec<String>,
) -> Result<bool, CargoMakeError> {
    invoke_script(
        script,
        script_runner,
        script_runner_args,
        script_extension,
        validate,
        None,
        None,
        cli_arguments,
    )
}

fn invoke_script(
    script: &ScriptValue,
    script_runner: Option<String>,
    script_runner_args: Option<Vec<String>>,
    script_extension: Option<String>,
    validate: bool,
    flow_info: Option<&FlowInfo>,
    flow_state: Option<Rc<RefCell<FlowState>>>,
    cli_arguments: &Vec<String>,
) -> Result<bool, CargoMakeError> {
    let expanded_script_runner = match script_runner {
        Some(ref value) => Some(environment::expand_value(value)),
        None => None,
    };
    let engine_type = get_engine_type(script, &expanded_script_runner, &script_extension)?;

    match engine_type {
        EngineType::OS => {
            let script_text = get_script_text(script)?;
            os_script::execute(
                &script_text,
                expanded_script_runner,
                cli_arguments,
                validate,
            )
        }
        EngineType::Duckscript => {
            let script_text = get_script_text(script)?;
            duck_script::execute(&script_text, cli_arguments, flow_info, flow_state, validate)
        }
        EngineType::Rust => {
            let script_text = get_script_text(script)?;
            rsscript::execute(
                &script_text,
                script_runner_args.clone(),
                cli_arguments,
                validate,
            )
        }
        EngineType::Shell2Batch => {
            let script_text = get_script_text(script)?;
            shell_to_batch::execute(&script_text, cli_arguments, validate)
        }
        EngineType::Generic => {
            let script_text = get_script_text(script)?;
            let extension = script_extension.clone().unwrap();
            generic_script::execute(
                &script_text,
                expanded_script_runner.unwrap(),
                extension,
                script_runner_args.clone(),
                cli_arguments,
                validate,
            )
        }
        EngineType::Shebang => {
            let script_text = get_script_text(script)?;
            let extension = script_extension.clone();
            shebang_script::execute(&script_text, &extension, cli_arguments, validate)
        }
        EngineType::Unsupported => Ok(false),
    }
}
