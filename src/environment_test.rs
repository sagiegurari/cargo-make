use super::*;

use log;
use std::collections::HashMap;
use std::env;

#[test]
fn setup_env_empty() {
    let logger = log::create("error");
    let config = Config { env: HashMap::new(), tasks: HashMap::new() };

    setup(&logger, &config, "setup_env_empty1");

    let mut value = env::var("CARGO_MAKE_TASK");
    assert_eq!(value.unwrap(), "setup_env_empty1");

    setup(&logger, &config, "setup_env_empty2");

    value = env::var("CARGO_MAKE_TASK");
    assert_eq!(value.unwrap(), "setup_env_empty2");
}

#[test]
fn set_env_values() {
    let logger = log::create("error");
    let mut config = Config { env: HashMap::new(), tasks: HashMap::new() };
    config.env.insert("MY_ENV_KEY".to_string(), "MY_ENV_VALUE".to_string());
    config.env.insert("MY_ENV_KEY2".to_string(), "MY_ENV_VALUE2".to_string());

    assert_eq!(env::var("MY_ENV_KEY").unwrap_or("NONE".to_string()), "NONE".to_string());
    assert_eq!(env::var("MY_ENV_KEY2").unwrap_or("NONE".to_string()), "NONE".to_string());

    setup(&logger, &config, "set_env_values");

    assert_eq!(env::var("MY_ENV_KEY").unwrap(), "MY_ENV_VALUE");
    assert_eq!(env::var("MY_ENV_KEY2").unwrap(), "MY_ENV_VALUE2");
}
