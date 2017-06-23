
extern crate clap;
extern crate rand;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use clap::{App, Arg, SubCommand};

mod types;
mod log;
mod descriptor;
mod command;
mod installer;
mod runner;

fn main() {
    let name = "make";
    let version = env!("CARGO_PKG_VERSION");
    let author = env!("CARGO_PKG_AUTHORS");
    let description = env!("CARGO_PKG_DESCRIPTION");

    let default_toml = "Makefile.toml";

    let mut build_file_arg = "-b, --buildFile=[FILE] 'Build toml file containing the build descriptor (default: ".to_string();
    build_file_arg.push_str(&default_toml);
    build_file_arg.push_str(" if exists)'");

    let matches = App::new("cargo")
        .subcommand(
            SubCommand::with_name(name)
                .version(version)
                .author(author)
                .about(description)
                .arg_from_usage(&build_file_arg)
                .arg_from_usage("-t, --task=[TASK NAME] 'The task name to execute (default: default)'")
                .arg(Arg::from_usage("-l, --loglevel=[LOG LEVEL] 'The log level (default: info)'").possible_values(&["verbose", "info", "error"]))
        )
        .get_matches();

    let (build_file, task, log_level) = if let Some(cmd_matches) = (cmd_matches.value_of("buildFile").unwrap_or(&default_toml), cmd_matches.value_of("task").unwrap_or("default"), cmd_matches.value_of("loglevel").unwrap_or("info"));

    let logger = log::create(log_level);

    logger.info::<()>("Using Build File: ", &[build_file], None);
    logger.info::<()>("Task: ", &[task], None);

    let config = descriptor::load(&build_file, &logger);

    runner::run(&logger, &config, &task);
}
