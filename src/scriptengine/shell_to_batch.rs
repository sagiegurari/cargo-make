//! # shell_to_batch
//!
//! Converts shell scripts to windows batch and invokes them.
//!

#[cfg(test)]
#[path = "./shell_to_batch_test.rs"]
mod shell_to_batch_test;

use command;
use shell2batch;

pub(crate) fn execute(script: &Vec<String>) {
    if cfg!(windows) {
        let shell_script = script.join("\n");
        let windows_batch = shell2batch::convert(&shell_script);

        let windows_script_lines = windows_batch.split("\n").map(|string| string.to_string()).collect();

        command::run_script(&windows_script_lines, None, true);
    } else {
        command::run_script(script, None, true);
    };
}
