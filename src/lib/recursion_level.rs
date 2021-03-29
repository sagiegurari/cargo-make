//! # recursion_level
//!
//! Utility functions to keep track of the recursion of cargo-make calls.
//!

#[cfg(test)]
#[path = "recursion_level_test.rs"]
mod recursion_level_test;

use envmnt;

static RECURSION_ENV_VAR_NAME: &str = "CARGO_MAKE_INTERNAL_RECURSION_LEVEL";

pub(crate) fn get() -> u32 {
    envmnt::get_u32(RECURSION_ENV_VAR_NAME, 0)
}

pub(crate) fn is_top() -> bool {
    get() == 0
}

pub(crate) fn increment() {
    if envmnt::exists(RECURSION_ENV_VAR_NAME) {
        envmnt::increment(RECURSION_ENV_VAR_NAME);
    } else {
        envmnt::set_u32(RECURSION_ENV_VAR_NAME, 0);
    }
}
