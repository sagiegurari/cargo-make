//! # makers
//!
//! Rust task runner and build tool.<br>
//! The makers executable works the same as cargo-make except that it is not invoked
//! as a cargo sub command.
//!
//! # Contributing
//! See [contributing guide](https://github.com/sagiegurari/cargo-make/blob/master/.github/CONTRIBUTING.md)
//!
//! # License
//! Developed by Sagie Gur-Ari and licensed under the
//! [Apache 2](https://github.com/sagiegurari/cargo-make/blob/master/LICENSE) open source license.
//!

use cli::error::SuccessOrCargoMakeError;

#[cfg(test)]
#[path = "makers_test.rs"]
mod makers_test;

fn get_name() -> String {
    "makers".to_string()
}

fn main() -> SuccessOrCargoMakeError<std::process::ExitCode> {
    #[cfg(windows)]
    let _ = nu_ansi_term::enable_ansi_support();
    let name = get_name();
    match cli::run_cli(name, false) {
        Ok(_) => Ok(std::process::ExitCode::SUCCESS).into(),
        Err(e) => SuccessOrCargoMakeError::Err(e),
    }
}
