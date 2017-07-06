//! # env
//!
//! Sets up the env vars before running the tasks.
//!

#[cfg(test)]
#[path = "./environment_test.rs"]
mod environment_test;

use log::Logger;
use std::env;
use types::Config;

/// Updates the env for the current execution based on the descriptor.
fn set_env(
    logger: &Logger,
    config: &Config,
) {
    logger.info::<()>("Setting Up Env.", &[], None);

    for (key, value) in &config.env {
        logger.verbose::<()>("Setting env: ", &[&key, "=", &value], None);
        env::set_var(&key, &value);
    }
}

/// Sets up the env before the tasks execution.
pub fn setup(
    logger: &Logger,
    config: &Config,
    task: &str,
) {
    set_env(logger, config);

    env::set_var("CARGO_MAKE_TASK", &task);
}
