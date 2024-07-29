#![deny(
    future_incompatible,
    keyword_idents,
    let_underscore,
    nonstandard_style,
    unused
)]
#![warn(unknown_lints)]

//! # cargo-make
//!
//! Rust task runner and build tool.<br>
//! The cargo-make task runner enables to define and configure sets of tasks and run them as a flow.<br>
//! A task is a command or a script to execute.<br>
//! Tasks can have dependencies which are also tasks that will be executed before the task itself.<br>
//! With a simple toml based configuration file, you can define a multi platform build script that can run build, test,
//! documentation generation, bench tests execution, security validations and more by running a single command.
//!
//! ## Installation
//! In order to install, just run the following command
//!
//! ```sh
//! cargo install cargo-make
//! ```
//!
//! This will install cargo-make in your ~/.cargo/bin.<br>
//! Make sure to add ~/.cargo/bin directory to your PATH variable.
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/cargo-make/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/cargo-make/blob/master/LICENSE) open source license.
//!

// Dependencies used in the binary `makers`:
use crate::error::CargoMakeError;
use crate::types::CliArgs;
#[cfg(windows)]
use nu_ansi_term as _;

#[macro_use]
extern crate log;
#[macro_use]
extern crate serde_derive;

#[cfg(test)]
mod test;

// make types public for docs
pub mod types;

mod cache;
pub mod cli;
pub mod cli_commands;
pub mod cli_parser;
mod command;
mod condition;
pub mod config;
mod descriptor;
mod environment;
pub mod error;
mod execution_plan;
mod functions;
mod installer;
mod io;
mod legacy;
pub mod logger;
mod plugin;
mod profile;
mod proxy_task;
mod recursion_level;
pub mod runner;
mod scriptengine;
mod storage;
mod time_summary;
mod toolchain;
mod version;

/// Handles the command line arguments and executes the runner.
pub fn run_cli(command_name: String, sub_command: bool) -> Result<CliArgs, CargoMakeError> {
    cli::run_cli(command_name, sub_command)
}
