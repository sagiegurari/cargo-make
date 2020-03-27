//! # recursion_level
//!
//! Utility functions to keep track of the recursion of cargo-make calls.
//!

#[cfg(test)]
#[path = "./recursion_level_test.rs"]
mod recursion_level_test;

use envmnt;

static RECURSION_ENV_VAR_NAME: &str = "CARGO_MAKE_INTERNAL_RECURSION";

pub(crate) fn recursion_level() -> u32 {
    envmnt::get_or(RECURSION_ENV_VAR_NAME, "0").parse().expect("failed to retrieve env var")
}

pub(crate) fn is_first_level() -> bool {
    recursion_level() == 0
}

pub(crate) fn increase_level() {
    envmnt::set(
        RECURSION_ENV_VAR_NAME,
        if envmnt::exists(RECURSION_ENV_VAR_NAME) { (recursion_level() + 1).to_string() }
        else { "0".to_string() }
    )
}
