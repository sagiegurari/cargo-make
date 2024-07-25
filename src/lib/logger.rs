//! # log
//!
//! Initializes the global logger.
//!

#[cfg(test)]
#[path = "logger_test.rs"]
mod logger_test;

use crate::recursion_level;
use crate::types::FlowInfo;
use colored::{ColoredString, Colorize};
use log::{Level, LevelFilter};
use std::io::stdout;
use std::process::exit;

#[derive(Debug, PartialEq)]
/// The log levels
pub(crate) enum LogLevel {
    VERBOSE,
    INFO,
    ERROR,
    OFF,
}

/// The logger options used to initialize the logger
pub struct LoggerOptions {
    /// Name to prefix each log message: "[name] %s"
    pub name: String,
    /// The logger level name (verbose, info, error, off)
    pub level: String,
    /// True to printout colorful output
    pub color: bool,
}

pub(crate) fn get_level(level_name: &str) -> LogLevel {
    match level_name {
        "verbose" => LogLevel::VERBOSE,
        "error" => LogLevel::ERROR,
        "off" => LogLevel::OFF,
        _ => LogLevel::INFO,
    }
}

/// Returns the current logger level name
pub(crate) fn get_log_level() -> String {
    if envmnt::is_equal("CARGO_MAKE_LOG_LEVEL", "off") {
        "off"
    } else if log_enabled!(Level::Debug) {
        "verbose"
    } else if log_enabled!(Level::Info) {
        "info"
    } else {
        "error"
    }
    .to_string()
}

fn get_name_for_filter(filter: &LevelFilter) -> String {
    let level = match filter {
        LevelFilter::Off => return "off".to_string(),
        LevelFilter::Debug => Level::Debug,
        LevelFilter::Warn => Level::Warn,
        LevelFilter::Error => Level::Error,
        _ => Level::Info,
    };

    get_name_for_level(&level)
}

fn get_name_for_level(level: &Level) -> String {
    match level {
        Level::Debug => "verbose".to_string(),
        Level::Warn => "warn".to_string(),
        Level::Error => "error".to_string(),
        _ => "info".to_string(),
    }
}

fn get_formatted_name(name: &str, use_color: bool) -> ColoredString {
    if use_color {
        name.bold()
    } else {
        name.normal()
    }
}

fn get_formatted_log_level(level: &Level, use_color: bool) -> ColoredString {
    let mut level_name = get_name_for_level(&level);
    level_name = level_name.to_uppercase();

    if use_color {
        let fmt_value = match level {
            Level::Debug => level_name.cyan(),
            Level::Info => level_name.green(),
            Level::Warn => level_name.yellow(),
            Level::Error => level_name.red(),
            _ => level_name.normal(),
        };

        fmt_value.bold()
    } else {
        level_name.normal()
    }
}

pub(crate) fn should_reduce_output(flow_info: &FlowInfo) -> bool {
    match flow_info.config.config.reduce_output {
        Some(value) => value,
        None => !envmnt::is("CARGO_MAKE_CI"),
    }
}

/// Initializes the global logger.
///
/// # Arguments
///
/// * `level_name` - The log level name ('verbose', 'info', 'error', 'off')
pub(crate) fn init(options: &LoggerOptions) {
    let level_name = &options.level;
    let color = options.color;

    let level = get_level(level_name);

    let log_level = match level {
        LogLevel::VERBOSE => LevelFilter::Debug,
        LogLevel::INFO => LevelFilter::Info,
        LogLevel::ERROR => LevelFilter::Error,
        LogLevel::OFF => LevelFilter::Off,
    };
    let level_name_value = get_name_for_filter(&log_level);

    envmnt::set("CARGO_MAKE_LOG_LEVEL", &level_name_value);
    envmnt::set_bool("CARGO_MAKE_DISABLE_COLOR", !color);

    let recursion_lvl = recursion_level::get();
    let recursion_level_log = if recursion_lvl == 0 {
        "".to_string()
    } else {
        format!("[{}]", recursion_lvl)
    };

    let name_fmt = get_formatted_name(&options.name, color);

    let result = fern::Dispatch::new()
        .format(move |out, message, record| {
            let record_level = record.level();

            if cfg!(test) {
                if record_level == LevelFilter::Error {
                    panic!("test error flow: {}", message);
                }
            }

            let record_level_fmt = get_formatted_log_level(&record_level, color);

            out.finish(format_args!(
                "[{}]{} {} - {}",
                &name_fmt, &recursion_level_log, &record_level_fmt, &message
            ));

            if record_level == Level::Error {
                warn!("Build Failed.");

                exit(1);
            }
        })
        .level(log_level)
        .chain(stdout())
        .apply();

    if result.is_err() {
        println!("Unable to setup logger.");
    }
}
