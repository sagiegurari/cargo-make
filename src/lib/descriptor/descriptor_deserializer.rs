//! # descriptor_deserializer
//!
//! Deserializes and validates the configs.

#[cfg(test)]
#[path = "./descriptor_deserializer_test.rs"]
mod descriptor_deserializer_test;

use crate::types::{Config, ExternalConfig};
use serde_ignored;
use toml;

pub(crate) fn load_config(descriptor_string: &str, validate: bool) -> Config {
    let config: Config = if validate {
        let deserializer = &mut toml::de::Deserializer::new(descriptor_string);

        match serde_ignored::deserialize(deserializer, |path| {
            error!("Found unknown key: {}", path);
        }) {
            Ok(value) => value,
            Err(error) => {
                error!("Unable to parse internal descriptor: {}", error);
                panic!("Unable to parse internal descriptor: {}", error);
            }
        }
    } else {
        match toml::from_str(descriptor_string) {
            Ok(value) => value,
            Err(error) => {
                error!("Unable to parse internal descriptor: {}", error);
                panic!("Unable to parse internal descriptor: {}", error);
            }
        }
    };

    config
}

pub(crate) fn load_external_config(descriptor_string: &str, file: &str) -> ExternalConfig {
    let deserializer = &mut toml::de::Deserializer::new(descriptor_string);

    let config: ExternalConfig = match serde_ignored::deserialize(deserializer, |path| {
        warn!("Found unknown key: {} in file: {}", path, file);
    }) {
        Ok(value) => value,
        Err(error) => {
            error!("Unable to parse external file: {:#?}, {}", &file, error);
            panic!("Unable to parse external file: {:#?}, {}", &file, error);
        }
    };

    config
}
