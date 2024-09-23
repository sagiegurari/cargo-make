//! # shebang_script
//!
//! Runs scripts using the executable defined in the shebang line.
//!

#[cfg(test)]
#[path = "shebang_script_test.rs"]
mod shebang_script_test;

use std::path::Path;

use crate::error::CargoMakeError;
use crate::scriptengine::generic_script;

#[cfg(target_os = "windows")]
const DEFAULT_EXTENSION: &'static str = "cmd.exe";
#[cfg(not(target_os = "windows"))]
const DEFAULT_EXTENSION: &'static str = "sh";

#[derive(Debug, Clone)]
/// Holds flow information
pub(crate) struct Shebang {
    /// The script runner
    pub(crate) runner: Option<String>,
    /// additional arguments
    pub(crate) arguments: Option<Vec<String>>,
}

impl Shebang {
    /// Creates and returns a new instance.
    fn new() -> Shebang {
        Shebang {
            runner: None,
            arguments: None,
        }
    }
}

pub(crate) fn get_shebang(script_text: &Vec<String>) -> Shebang {
    match script_text.first() {
        Some(line) => {
            let trimmed_line = line.trim();

            if trimmed_line.starts_with("#!") {
                let mut lines = trimmed_line.split("\n");
                match lines.next() {
                    Some(first_line) => {
                        let mut shebang_line = first_line.replace("#!", "");
                        shebang_line = shebang_line.trim().to_string();

                        if !shebang_line.is_empty() {
                            let mut values = shebang_line.split_whitespace();
                            let runner = match values.next() {
                                Some(value) => Some(value.trim().to_string()),
                                _ => None,
                            };

                            let mut argument_values = vec![];
                            for arg in values {
                                argument_values.push(arg.trim().to_string());
                            }

                            let arguments = if !argument_values.is_empty() {
                                Some(argument_values)
                            } else {
                                None
                            };

                            Shebang { runner, arguments }
                        } else {
                            Shebang::new()
                        }
                    }
                    _ => Shebang::new(),
                }
            } else {
                Shebang::new()
            }
        }
        None => Shebang::new(),
    }
}

fn get_extension_for_runner(runner: &str) -> String {
    let runner_no_extension = match Path::new(runner).file_stem() {
        Some(value) => value,
        None => return DEFAULT_EXTENSION.to_string(),
    };

    let extension = if runner_no_extension == "python" {
        "py"
    } else if runner_no_extension == "perl" {
        "pl"
    } else if runner_no_extension == "node" {
        "js"
    } else if runner_no_extension == "powershell" || runner_no_extension == "pwsh" {
        "ps1"
    } else {
        DEFAULT_EXTENSION
    };

    extension.to_string()
}

pub(crate) fn execute(
    script_text: &Vec<String>,
    extension: &Option<String>,
    cli_arguments: &Vec<String>,
    validate: bool,
) -> Result<bool, CargoMakeError> {
    let shebang = get_shebang(&script_text);

    match shebang.runner {
        Some(runner) => {
            let extension_str = match extension {
                Some(value) => value.to_string(),
                None => get_extension_for_runner(&runner),
            };

            generic_script::execute(
                &script_text,
                runner,
                extension_str,
                shebang.arguments,
                &cli_arguments,
                validate,
            )
        }
        None => {
            if validate {
                error!("Unable to execute script using shebang.");
            }

            Ok(false)
        }
    }
}
