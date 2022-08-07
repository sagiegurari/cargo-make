//! # rsscript
//!
//! Compiles and runs rust code.
//!

#[cfg(test)]
#[path = "rsscript_test.rs"]
mod rsscript_test;

use crate::command;
use crate::installer::{cargo_plugin_installer, crate_installer};
use crate::io::delete_file;
use crate::scriptengine::script_utils::create_script_file;
use crate::types::{InstallCrateInfo, TestArg};

#[derive(PartialEq, Debug)]
enum ScriptRunner {
    RustScript,
    CargoScript,
    CargoPlay,
}

fn get_script_runner() -> ScriptRunner {
    let provider = envmnt::get_or("CARGO_MAKE_RUST_SCRIPT_PROVIDER", "rust-script");

    match provider.as_str() {
        "rust-script" => ScriptRunner::RustScript,
        "cargo-script" => ScriptRunner::CargoScript,
        "cargo-play" => ScriptRunner::CargoPlay,
        _ => ScriptRunner::CargoScript,
    }
}

fn install_crate(provider: &ScriptRunner) {
    // install dependencies
    match provider {
        ScriptRunner::RustScript => {
            let info = InstallCrateInfo {
                crate_name: "rust-script".to_string(),
                rustup_component_name: None,
                binary: "rust-script".to_string(),
                test_arg: TestArg {
                    inner: vec!["--version".to_string()],
                },
                min_version: None,
                version: None,
                install_command: None,
            };

            // due to fornwall/rust-script/issues/42
            let rust_script_install_args = vec!["--version".to_string(), "0.7.0".to_string()];

            crate_installer::install(&None, &info, &Some(rust_script_install_args), false);
        }
        ScriptRunner::CargoScript => cargo_plugin_installer::install_crate(
            &None,
            "script",
            "cargo-script",
            &None,
            true,
            &None,
            &None,
            &None,
        ),
        ScriptRunner::CargoPlay => cargo_plugin_installer::install_crate(
            &None,
            "play",
            "cargo-play",
            &None,
            true,
            &None,
            &None,
            &None,
        ),
    };
}

fn create_rust_file(rust_script: &Vec<String>) -> String {
    create_script_file(rust_script, "rs")
}

fn run_file(file: &str, cli_arguments: &Vec<String>, provider: &ScriptRunner) -> bool {
    let (use_cargo, command) = match provider {
        ScriptRunner::RustScript => (false, "rust-script"),
        ScriptRunner::CargoScript => (true, "script"),
        ScriptRunner::CargoPlay => (true, "play"),
    };

    let mut args = vec![];
    if use_cargo {
        args.push(command.to_string());
    }
    args.push(file.to_string());
    let mut cli_args = cli_arguments.clone();
    args.append(&mut cli_args);

    let exit_code = if use_cargo {
        command::run_command("cargo", &Some(args), false)
    } else {
        command::run_command(command, &Some(args), false)
    };
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
