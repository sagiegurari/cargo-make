//! # log
//!
//! Initializes the global logger.
//!

#[cfg(test)]
#[path = "./logger_test.rs"]
mod logger_test;

use fern;
use log::{LogLevel, LogLevelFilter};
use std::io::stdout;

#[cfg(test)]
fn exit(code: i32) {
    panic!(code);
}

#[cfg(not(test))]
use std::process::exit;

#[derive(Debug, PartialEq)]
/// The log levels
pub enum Level {
    VERBOSE,
    INFO,
    ERROR
}

fn get_level(level_name: &str) -> Level {
    let mut level = Level::INFO;

    if level_name == "verbose" {
        level = Level::VERBOSE;
    } else if level_name == "error" {
        level = Level::ERROR;
    }

    level
}

/// Initializes the global logger.
///
/// # Arguments
///
/// * `level_name` - The log level name ('verbose', 'info', 'error')
/// ```
pub fn init(level_name: &str) {
    let level = get_level(level_name);

    let log_level = match level {
        Level::VERBOSE => LogLevelFilter::Trace,
        Level::INFO => LogLevelFilter::Info,
        Level::ERROR => LogLevelFilter::Error,
    };

    let result = fern::Dispatch::new()
        .format(|out, message, record| {
            let name = env!("CARGO_PKG_NAME");
            let record_level = record.level();
            out.finish(format_args!("[{}] {} - {}", &name, record_level, message));

            if record_level == LogLevel::Error {
                warn!("Build Failed.");

                exit(0);
            }
        })
        .level(log_level)
        .chain(stdout())
        .apply();

    if result.is_err() {
        println!("Unable to setup logger.");
    }
}
