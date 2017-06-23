//! # log
//!
//! Implements a simple output log
//!

use std::fmt::Debug;

pub enum Level {
    VERBOSE,
    INFO,
    ERROR
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

pub struct Log {
    pub level: Level
}

impl Log {
    pub fn verbose<T: Debug>(
        self: &Log,
        message: &str,
        args: &[&str],
        obj: Option<T>,
    ) {
        match self.level {
            Level::VERBOSE => print_message("verbose", message, args, obj),
            _ => (),
        }
    }

    pub fn info<T: Debug>(
        self: &Log,
        message: &str,
        args: &[&str],
        obj: Option<T>,
    ) {
        match self.level {
            Level::ERROR => (),
            _ => print_message("info", message, args, obj),
        }
    }

    pub fn error<T: Debug>(
        self: &Log,
        message: &str,
        args: &[&str],
        obj: Option<T>,
    ) {
        print_message("error", message, args, obj);
        panic!("Existing...");
    }
}

pub fn create(name: &str) -> Log {
    let mut level = Level::INFO;

    if name == "verbose" {
        level = Level::VERBOSE;
    } else if name == "error" {
        level = Level::ERROR;
    }

    Log { level }
}
