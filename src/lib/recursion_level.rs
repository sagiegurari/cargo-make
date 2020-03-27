//! # recursion_level
//!
//! Utility functions to keep track of the recursion of cargo-make calls.
//!

#[cfg(test)]
#[path = "./recursion_level_test.rs"]
mod recursion_level_test;

use envmnt;

static RECURSION_ENV_VAR_NAME: &str = "CARGO_MAKE_INTERNAL_RECURSION_LEVEL";

pub(crate) fn get() -> u32 {
    let level = envmnt::get_or(RECURSION_ENV_VAR_NAME, "0");

    match level.parse() {
        Ok(value) => value,
        Err(error) => {
            debug!(
                "Failed to retrieve the recursion level environment variable, error: {}",
                error
            );
            0
        }
    }
}

pub(crate) fn is_top() -> bool {
    get() == 0
}

pub(crate) fn increment() {
    let level = if envmnt::exists(RECURSION_ENV_VAR_NAME) {
        (get() + 1).to_string()
    } else {
        "0".to_string()
    };

    envmnt::set(RECURSION_ENV_VAR_NAME, level)
}
