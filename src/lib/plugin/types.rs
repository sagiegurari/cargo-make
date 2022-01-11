//! # types
//!
//! Defines the various plugin related types.
//!

#[cfg(test)]
#[path = "types_test.rs"]
mod types_test;

use indexmap::IndexMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds a plugin implementation
pub(crate) struct Plugin {
    /// The plugin script content
    pub(crate) script: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// Holds the entire plugin config and implementation structure
pub struct Plugins {
    /// The plugin name aliases
    pub(crate) aliases: Option<IndexMap<String, String>>,
    /// All plugin definitions
    #[serde(rename = "impl")]
    pub(crate) plugins: IndexMap<String, Plugin>,
}

impl Plugins {
    /// Creates and returns a new instance.
    pub fn new() -> Plugins {
        Default::default()
    }
}
