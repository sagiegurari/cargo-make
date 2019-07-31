//! # rsscript
//!
//! Compiles and runs rust code.
//!

#[cfg(test)]
#[path = "./rsscript_test.rs"]
mod rsscript_test;

use crate::command;
use crate::installer::cargo_plugin_installer;
use crate::io::delete_file;
use crate::scriptengine::script_utils::create_script_file;

#[derive(PartialEq, Debug)]
enum ScriptRunner {
    CargoScript,
    CargoPlay,
}

fn get_script_runner() -> ScriptRunner {
    let provider = envmnt::get_or("CARGO_MAKE_RUST_SCRIPT_PROVIDER", "cargo-script");

    match provider.as_str() {
        "cargo-script" => ScriptRunner::CargoScript,
        "cargo-play" => ScriptRunner::CargoPlay,
        _ => ScriptRunner::CargoScript,
    }
}

fn install_crate(provider: &ScriptRunner) {
    // install dependencies
    match provider {
        ScriptRunner::CargoScript => cargo_plugin_installer::install_crate(
            &None,
            "script",
            "cargo-script",
            &None,
            true,
            &None,
        ),
        ScriptRunner::CargoPlay => {
            cargo_plugin_installer::install_crate(&None, "play", "cargo-play", &None, true, &None)
        }
    };
}

fn create_rust_file(rust_script: &Vec<String>) -> String {
    create_script_file(rust_script, "rs")
}

fn run_file(file: &str, cli_arguments: &Vec<String>, provider: &ScriptRunner) -> bool {
    let command = match provider {
        ScriptRunner::CargoScript => "script",
        ScriptRunner::CargoPlay => "play",
    };

    let mut args = vec![command.to_string(), file.to_string()];
    let mut cli_args = cli_arguments.clone();
    args.append(&mut cli_args);

    let exit_code = command::run_command("cargo", &Some(args), false);
    debug!("Executed rust code, exit code: {}", exit_code);

    exit_code == 0
}

pub(crate) fn execute(rust_script: &Vec<String>, cli_arguments: &Vec<String>, validate: bool) {
    let provider = get_script_runner();

    install_crate(&provider);

    let file = create_rust_file(rust_script);

    let valid = run_file(&file, &cli_arguments, &provider);

    delete_file(&file);

    if validate && !valid {
        error!("Unable to execute rust code.");
    }
}
