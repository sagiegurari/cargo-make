//! # scriptengine
//!
//! Facade for all different non OS scripts.
//!

mod duck_script;
pub(crate) mod generic_script;
mod os_script;
mod rsscript;
pub(crate) mod script_utils;
mod shebang_script;
mod shell_to_batch;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use crate::environment;
use crate::io;
use crate::types::{FlowInfo, ScriptValue, Task};
use std::path::PathBuf;

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

pub(crate) fn get_script_text(script: &ScriptValue) -> Vec<String> {
    match script {
        ScriptValue::Text(text) => text.clone(),
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

            let script_text = io::read_text_file(&file_path);
            let lines: Vec<&str> = script_text.split('\n').collect();

            let mut script_lines: Vec<String> = vec![];

            for line in lines.iter() {
                script_lines.push(line.to_string());
            }

            script_lines
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
) -> EngineType {
    match script_runner {
        Some(ref runner) => {
            debug!("Checking script runner: {}", runner);

            let engine_type = get_internal_runner(runner);

            match engine_type {
                EngineType::Unsupported => {
                    if script_extension.is_some() {
                        // if both script runner and extension is defined, we use generic script runner
                        debug!("Generic script detected.");
                        EngineType::Generic
                    } else {
                        // use default OS extension with custom runner
                        debug!("OS script with custom runner detected.");
                        EngineType::OS
                    }
                }
                _ => engine_type,
            }
        }
        None => {
            // if no runner specified, try to extract it from script content
            let script_text = get_script_text(&script);

            let shebang = shebang_script::get_shebang(&script_text);
            match shebang.runner {
                Some(script_runner) => {
                    if shebang.arguments.is_none() {
                        let engine_type = get_internal_runner(&script_runner);

                        match engine_type {
                            EngineType::Unsupported => {
                                debug!("Shebang line does not point to an internal engine, using normal shebang script runner.");
                                EngineType::Shebang
                            }
                            _ => engine_type,
                        }
                    } else {
                        EngineType::Shebang
                    }
                }
                None => EngineType::OS,
            }
        }
    }
}

pub(crate) fn invoke(task: &Task, flow_info: &FlowInfo) -> bool {
    match task.script {
        Some(ref script) => {
            let validate = !task.should_ignore_errors();

            invoke_script_in_flow_context(
                script,
                task.script_runner.clone(),
                task.script_runner_args.clone(),
                task.script_extension.clone(),
                validate,
                Some(flow_info),
            )
        }
        None => false,
    }
}

pub(crate) fn invoke_script_in_flow_context(
    script: &ScriptValue,
    script_runner: Option<String>,
    script_runner_args: Option<Vec<String>>,
    script_extension: Option<String>,
    validate: bool,
    flow_info: Option<&FlowInfo>,
) -> bool {
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
) -> bool {
    invoke_script(
        script,
        script_runner,
        script_runner_args,
        script_extension,
        validate,
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
    cli_arguments: &Vec<String>,
) -> bool {
    let engine_type = get_engine_type(script, &script_runner, &script_extension);

    match engine_type {
        EngineType::OS => {
            let script_text = get_script_text(script);
            os_script::execute(&script_text, script_runner, cli_arguments, validate);

            true
        }
        EngineType::Duckscript => {
            let script_text = get_script_text(script);
            duck_script::execute(&script_text, cli_arguments, flow_info, validate);

            true
        }
        EngineType::Rust => {
            let script_text = get_script_text(script);
            rsscript::execute(&script_text, cli_arguments, validate);

            true
        }
        EngineType::Shell2Batch => {
            let script_text = get_script_text(script);
            shell_to_batch::execute(&script_text, cli_arguments, validate);

            true
        }
        EngineType::Generic => {
            let script_text = get_script_text(script);
            let extension = script_extension.clone().unwrap();
            generic_script::execute(
                &script_text,
                script_runner.unwrap(),
                extension,
                script_runner_args.clone(),
                cli_arguments,
                validate,
            );

            true
        }
        EngineType::Shebang => {
            let script_text = get_script_text(script);
            let extension = script_extension.clone();
            shebang_script::execute(&script_text, &extension, cli_arguments, validate);

            true
        }
        EngineType::Unsupported => false,
    }
}
