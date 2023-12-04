//! # profile
//!
//! Profile related utility functions
//!

#[cfg(test)]
#[path = "profile_test.rs"]
mod profile_test;

use envmnt;

static PROFILE_ENV_KEY: &str = "CARGO_MAKE_PROFILE";
static DEFAULT_PROFILE_ENV_KEY: &str = "CARGO_MAKE_DEFAULT_PROFILE";
static ADDITIONAL_PROFILES_ENV_KEY: &str = "CARGO_MAKE_ADDITIONAL_PROFILES";
static DEFAULT_PROFILE: &str = "development";

fn normalize_profile(profile: &str) -> String {
    let profile_normalized = profile.to_lowercase();
    profile_normalized.trim().to_string()
}

fn normalize_additional_profiles(profiles: &Vec<String>) -> Vec<String> {
    let mut nomralized_profiles = vec![];

    for profile in profiles {
        let profile_normalized = normalize_profile(&profile);

        if profile_normalized.len() > 0 {
            nomralized_profiles.push(profile_normalized);
        }
    }

    nomralized_profiles
}

pub(crate) fn get() -> String {
    envmnt::get_or(PROFILE_ENV_KEY, &default_profile())
}

pub(crate) fn default_profile() -> String {
    envmnt::get_or(DEFAULT_PROFILE_ENV_KEY, DEFAULT_PROFILE)
}

pub(crate) fn set(profile: &str) -> String {
    let mut profile_normalized = normalize_profile(&profile);

    if profile_normalized.len() == 0 {
        profile_normalized = DEFAULT_PROFILE.to_string();
    }

    envmnt::set(PROFILE_ENV_KEY, &profile_normalized);

    get()
}

pub(crate) fn set_additional(profiles: &Vec<String>) {
    let nomralized_profiles = normalize_additional_profiles(&profiles);

    envmnt::set_list(ADDITIONAL_PROFILES_ENV_KEY, &nomralized_profiles);
}
