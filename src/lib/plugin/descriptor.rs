//! # descriptor
//!
//! Handles the plugin descriptor section logic.
//!

#[cfg(test)]
#[path = "descriptor_test.rs"]
mod descriptor_test;

use crate::plugin::types::{Plugin, Plugins};
use indexmap::IndexMap;

fn merge_aliases(
    base: &IndexMap<String, String>,
    extended: &IndexMap<String, String>,
) -> IndexMap<String, String> {
    let mut target = base.clone();
    target.extend(
        extended
            .into_iter()
            .map(|(key, value)| (key.clone(), value.clone())),
    );

    target
}

fn merge_plugins_map(
    base: &IndexMap<String, Plugin>,
    extended: &IndexMap<String, Plugin>,
) -> IndexMap<String, Plugin> {
    let mut target = base.clone();
    target.extend(
        extended
            .into_iter()
            .map(|(key, value)| (key.clone(), value.clone())),
    );

    target
}

fn merge_plugins_config_impl(base: Plugins, extended: Plugins) -> Plugins {
    let aliases = match base.aliases {
        Some(base_aliases) => match extended.aliases {
            Some(ref extended_aliases) => Some(merge_aliases(&base_aliases, extended_aliases)),
            None => Some(base_aliases),
        },
        None => extended.aliases,
    };

    let plugins = merge_plugins_map(&base.plugins, &extended.plugins);

    Plugins { aliases, plugins }
}

pub(crate) fn merge_plugins_config(
    base: Option<Plugins>,
    extended: Option<Plugins>,
) -> Option<Plugins> {
    match base {
        Some(base_plugins) => match extended {
            Some(extended_plugins) => {
                let plugins = merge_plugins_config_impl(base_plugins, extended_plugins);
                Some(plugins)
            }
            None => Some(base_plugins),
        },
        None => extended,
    }
}
