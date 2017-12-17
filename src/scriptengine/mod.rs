//! # scriptengine
//!
//! Facade for all different non OS scripts.
//!

mod rsscript;
mod shell_to_batch;

#[cfg(test)]
#[path = "./mod_test.rs"]
mod mod_test;

use types::Task;

#[derive(Debug, Clone, PartialEq)]
/// The currently supported engine types
enum EngineType {
    /// Rust language
    Rust,
    /// shell to windows batch conversion
    Shell2Batch,
    /// Unsupported type
    Unsupported,
}

fn get_engine_type(task: &Task) -> EngineType {
    match task.script_runner {
        Some(ref script_runner) => match task.script {
            None => EngineType::Unsupported,
            _ => {
                debug!("Checking script runner: {}", script_runner);

                if script_runner == "@rust" {
                    debug!("Rust script detected.");
                    EngineType::Rust
                } else if script_runner == "@shell" {
                    debug!("Shell to batch detected.");
                    EngineType::Shell2Batch
                } else {
                    EngineType::Unsupported
                }
            }
        },
        None => EngineType::Unsupported,
    }
}

pub(crate) fn invoke(task: &Task) -> bool {
    let engine_type = get_engine_type(&task);

    match engine_type {
        EngineType::Rust => {
            let script = task.script.as_ref().unwrap();
            rsscript::execute(script);

            true
        }
        EngineType::Shell2Batch => {
            let script = task.script.as_ref().unwrap();
            shell_to_batch::execute(script);

            true
        }
        EngineType::Unsupported => false,
    }
}
