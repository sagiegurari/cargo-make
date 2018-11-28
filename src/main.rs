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
//! See [contributing guide](https://github.com/sagiegurari/simple_redis/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/simple_redis/blob/master/LICENSE) open source license.
//!

extern crate cli;

#[cfg(test)]
#[path = "./main_test.rs"]
mod main_test;

fn main() {
    cli::run_cli("make".to_string(), true);
}
