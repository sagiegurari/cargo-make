//! # makers
//!
//! Rust task runner and build tool.<br>
//! The makers executable works the same as cargo-make except that it is not invoked
//! as a cargo sub command.
//!

extern crate cli;

#[cfg(test)]
#[path = "./makers_test.rs"]
mod makers_test;

fn get_name() -> String {
    return "makers".to_string();
}

fn main() {
    let name = get_name();
    cli::run_cli(name, false);
}
