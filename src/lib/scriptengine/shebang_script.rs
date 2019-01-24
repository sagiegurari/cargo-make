//! # shebang_script
//!
//! Runs scripts using the executable defined in the shebang line.
//!

#[cfg(test)]
#[path = "./shebang_script_test.rs"]
mod shebang_script_test;

use crate::scriptengine::generic_script;

#[derive(Debug, Clone)]
/// Holds flow information
struct Shebang {
    /// The script runner
    runner: Option<String>,
    /// additional arguments
    arguments: Option<Vec<String>>,
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

fn get_shebang(script_text: &Vec<String>) -> Shebang {
    match script_text.first() {
        Some(line) => {
            if line.starts_with("#!") {
                let mut lines = line.split("\n");
                match lines.next() {
                    Some(first_line) => {
                        let mut shebang_line = first_line.replace("#!", "");
                        shebang_line = shebang_line.trim().to_string();

                        if shebang_line.len() > 0 {
                            let mut values = shebang_line.split_whitespace();
                            let runner = match values.next() {
                                Some(value) => Some(value.trim().to_string()),
                                _ => None,
                            };

                            let mut argument_values = vec![];
                            for arg in values {
                                argument_values.push(arg.trim().to_string());
                            }

                            let arguments = if argument_values.len() > 0 {
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

pub(crate) fn execute(
    script_text: &Vec<String>,
    extension: &Option<String>,
    cli_arguments: &Vec<String>,
    validate: bool,
) {
    let shebang = get_shebang(&script_text);

    match shebang.runner {
        Some(runner) => {
            let extension_str = match extension {
                Some(value) => value,
                None => "sh",
            };

            generic_script::execute(
                &script_text,
                runner,
                extension_str.to_string(),
                shebang.arguments,
                &cli_arguments,
                validate,
            );
        }
        None => {
            if validate {
                error!("Unable to execute script using shebang.");
            }
        }
    };
}

pub(crate) fn is_shebang_exists(script_text: &Vec<String>) -> bool {
    let shebang = get_shebang(&script_text);

    shebang.runner.is_some()
}
