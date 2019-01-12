//! # shell_to_batch
//!
//! Converts shell scripts to windows batch and invokes them.
//!

#[cfg(test)]
#[path = "./shell_to_batch_test.rs"]
mod shell_to_batch_test;

use crate::command;
use shell2batch;

pub(crate) fn execute_and_update_env(
    name: &str,
    script: &Vec<String>,
    cli_arguments: &Vec<String>,
) {
    let task_name = Some(name.to_string());

    if cfg!(windows) {
        let shell_script = script.join("\n");
        let windows_batch = shell2batch::convert(&shell_script);

        let windows_script_lines = windows_batch
            .split("\n")
            .map(|string| string.to_string())
            .collect();

        command::run_script_and_update_env(
            task_name,
            &windows_script_lines,
            None,
            cli_arguments,
            true,
        );
    } else {
        command::run_script_and_update_env(task_name, script, None, cli_arguments, true);
    };
}
