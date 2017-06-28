use super::*;
use log;
use std::io::ErrorKind;
use types::Task;

#[test]
#[should_panic]
fn validate_exit_code_error() {
    validate_exit_code(Err(Error::new(ErrorKind::Other, "test")));
}

#[test]
fn run_no_command() {
    let logger = log::create("error");
    let task = Task {
        install_crate: None,
        command: None,
        args: None,
        disabled: None,
        alias: None,
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        install_script: None,
        script: None,
        dependencies: None,
        linux: None,
        windows: None,
        mac: None
    };
    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
fn run_command() {
    let logger = log::create("error");
    let task = Task {
        command: Some("echo".to_string()),
        args: Some(vec!["1".to_string()]),
        install_crate: None,
        disabled: None,
        alias: None,
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        install_script: None,
        script: None,
        dependencies: None,
        linux: None,
        windows: None,
        mac: None
    };
    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
fn run_script() {
    let logger = log::create("error");
    let task = Task {
        script: Some(vec!["echo 1".to_string()]),
        command: None,
        install_crate: None,
        args: None,
        disabled: None,
        alias: None,
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        install_script: None,
        dependencies: None,
        linux: None,
        windows: None,
        mac: None
    };
    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}
