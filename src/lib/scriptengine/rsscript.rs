//! # rsscript
//!
//! Compiles and runs rust code.
//!

#[cfg(test)]
#[path = "rsscript_test.rs"]
mod rsscript_test;

use crate::command;
use crate::error::CargoMakeError;
use crate::installer::{cargo_plugin_installer, crate_installer};
use crate::scriptengine::script_utils::create_persisted_script_file;
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
        _ => ScriptRunner::RustScript,
    }
}

fn install_crate(provider: &ScriptRunner) -> Result<(), CargoMakeError> {
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
                force: None,
            };

            crate_installer::install(&None, &info, &None, false)?;
        }
        ScriptRunner::CargoScript => cargo_plugin_installer::install_crate(
            &None,
            Some("script"),
            "cargo-script",
            &None,
            true,
            &None,
            &None,
            &None,
        )?,
        ScriptRunner::CargoPlay => cargo_plugin_installer::install_crate(
            &None,
            Some("play"),
            "cargo-play",
            &None,
            true,
            &None,
            &None,
            &None,
        )?,
    };
    Ok(())
}

fn create_rust_file(rust_script: &Vec<String>) -> Result<String, CargoMakeError> {
    create_persisted_script_file(rust_script, "rs")
}

fn run_file(
    file: &str,
    runner_arguments: Option<Vec<String>>,
    cli_arguments: &Vec<String>,
    provider: &ScriptRunner,
) -> Result<bool, CargoMakeError> {
    let (use_cargo, command) = match provider {
        ScriptRunner::RustScript => (false, "rust-script"),
        ScriptRunner::CargoScript => (true, "script"),
        ScriptRunner::CargoPlay => (true, "play"),
    };

    let mut args = vec![];
    if use_cargo {
        args.push(command.to_string());
    }
    if let Some(mut runner_args) = runner_arguments {
        args.append(&mut runner_args);
    }
    args.push(file.to_string());
    let mut cli_args = cli_arguments.clone();
    args.append(&mut cli_args);

    let exit_code = if use_cargo {
        command::run_command("cargo", &Some(args), false)
    } else {
        command::run_command(command, &Some(args), false)
    }?;
    debug!("Executed rust code, exit code: {}", exit_code);

    Ok(exit_code == 0)
}

pub(crate) fn execute(
    rust_script: &Vec<String>,
    runner_arguments: Option<Vec<String>>,
    cli_arguments: &Vec<String>,
    validate: bool,
) -> Result<bool, CargoMakeError> {
    let provider = get_script_runner();

    install_crate(&provider)?;

    let file = create_rust_file(rust_script)?;

    let valid = run_file(&file, runner_arguments, &cli_arguments, &provider)?;

    if validate && !valid {
        error!("Unable to execute rust code.");
    }

    Ok(valid)
}
