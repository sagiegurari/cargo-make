//! # profile
//!
//! Profile related utility functions
//!

#[cfg(test)]
#[path = "./profile_test.rs"]
mod profile_test;

use crate::environment;
use envmnt;

static PROFILE_ENV_KEY: &str = "CARGO_MAKE_PROFILE";
pub(crate) static DEFAULT_PROFILE: &str = "development";

pub(crate) fn get() -> String {
    environment::get_env(PROFILE_ENV_KEY, DEFAULT_PROFILE)
}

pub(crate) fn set(profile: &str) -> String {
    let mut profile_normalized = profile.to_lowercase();
    profile_normalized = profile_normalized.trim().to_string();

    if profile_normalized.len() == 0 {
        profile_normalized = DEFAULT_PROFILE.to_string();
    }

    envmnt::set(PROFILE_ENV_KEY, &profile_normalized);

    get()
}
