//! # makefiles
//!
//! Provides access to internal makefiles.
//!

pub(crate) static BASE: &str = include_str!("Makefile.base.toml");
pub(crate) static STABLE: &str = include_str!("Makefile.stable.toml");
pub(crate) static BETA: &str = include_str!("Makefile.beta.toml");
