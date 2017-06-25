//! # log
//!
//! Implements a simple output logger.
//!

use std::fmt::Debug;

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

fn print_message<T: Debug>(
    level: &str,
    message: &str,
    args: &[&str],
    obj: Option<T>,
) {
    let name = env!("CARGO_PKG_NAME");
    print!("[{}] {} - {}", name, level, message);

    for arg in args {
        print!("{}", arg);
    }

    match obj {
        Some(value) => println!("{:#?}", value),
        _ => print!("\n"),
    }
}

/// A very basic and simple logger used internally
pub struct Logger {
    level: Level
}

impl Logger {
    /// Returns true if verbose logging is enabled
    pub fn is_verbose_enabled(self: &Logger) -> bool {
        match self.level {
            Level::VERBOSE => true,
            _ => false,
        }
    }

    /// Returns true if info logging is enabled
    pub fn is_info_enabled(self: &Logger) -> bool {
        match self.level {
            Level::ERROR => false,
            _ => true,
        }
    }

    /// Logs the provided info only if the current log level is verbose
    pub fn verbose<T: Debug>(
        self: &Logger,
        message: &str,
        args: &[&str],
        obj: Option<T>,
    ) {
        if self.is_verbose_enabled() {
            print_message("verbose", message, args, obj);
        }
    }

    /// Logs the provided info only if the current log level is verbose or info
    pub fn info<T: Debug>(
        self: &Logger,
        message: &str,
        args: &[&str],
        obj: Option<T>,
    ) {
        if self.is_info_enabled() {
            print_message("info", message, args, obj);
        }
    }

    /// Logs the provided info and panics
    pub fn error<T: Debug>(
        self: &Logger,
        message: &str,
        args: &[&str],
        obj: Option<T>,
    ) {
        print_message("error", message, args, obj);

        panic!("Existing...");
    }
}

/// Constructs a new logger and returns it.
///
/// # Arguments
///
/// * `level_name` - The log level name ('verbose', 'info', 'error')
/// * `panic_on_error` - True if to exit in case of any error
/// ```
pub fn create(level_name: &str) -> Logger {
    let level = get_level(level_name);

    Logger { level }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_level_verbose() {
        let level = get_level("verbose");
        assert_eq!(level, Level::VERBOSE);
    }

    #[test]
    fn get_level_info() {
        let level = get_level("info");
        assert_eq!(level, Level::INFO);
    }

    #[test]
    fn get_level_error() {
        let level = get_level("error");
        assert_eq!(level, Level::ERROR);
    }

    #[test]
    fn get_level_other() {
        let level = get_level("test123");
        assert_eq!(level, Level::INFO);
    }

    #[test]
    fn create_verbose() {
        let logger = create("verbose");

        assert!(logger.is_verbose_enabled());
        assert!(logger.is_info_enabled());

        logger.verbose::<()>("test", &["test"], None);
    }

    #[test]
    fn create_info() {
        let logger = create("info");

        assert!(!logger.is_verbose_enabled());
        assert!(logger.is_info_enabled());

        logger.info::<()>("test", &["test"], None);
    }

    #[test]
    #[should_panic]
    fn create_error() {
        let logger = create("error");

        assert!(!logger.is_verbose_enabled());
        assert!(!logger.is_info_enabled());

        logger.error::<()>("test", &["test"], None);
    }
}
