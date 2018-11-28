//! # runrs
//!
//! Rust task runner and build tool.<br>
//! The runrs executable works the same as cargo-make except that it is not invoked
//! as a cargo sub command.
//!

extern crate cli;

#[cfg(test)]
#[path = "./runrs_test.rs"]
mod runrs_test;

fn main() {
    cli::run_cli("runrs".to_string(), false);
}
