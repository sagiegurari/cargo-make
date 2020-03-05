//! # makefiles
//!
//! Provides access to internal makefiles.
//!

pub(crate) static BASE: &str = include_str!("base.toml");
pub(crate) static STABLE: &str = concat!(
    include_str!("stable.toml"),
    include_str!("git.toml"),
    include_str!("github.toml"),
    include_str!("rust.toml"),
    include_str!("rust-coverage.toml")
);
pub(crate) static BETA: &str = include_str!("beta.toml");
