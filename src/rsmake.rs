//! # rsmake
//!
//! Rust task runner and build tool.<br>
//! The rsmake executable works the same as cargo-make except that it is not invoked
//! as a cargo sub command.
//!

extern crate cli;

#[cfg(test)]
#[path = "./rsmake_test.rs"]
mod rsmake_test;

fn get_name() -> String {
    return "rsmake".to_string();
}

fn main() {
    let name = get_name();
    cli::run_cli(name, false);
}
