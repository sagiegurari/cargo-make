use super::*;
use log;

#[test]
fn is_crate_installed_true() {
    let logger = log::create("error");
    let output = is_crate_installed(&logger, "test");
    assert!(output);
}

#[test]
fn is_crate_installed_false() {
    let logger = log::create("error");
    let output = is_crate_installed(&logger, "badbadbad");
    assert!(!output);
}

#[test]
fn install_empty() {
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

    install(&logger, &task);
}

#[test]
fn install_crate_already_installed() {
    let logger = log::create("error");
    let task = Task {
        install_crate: Some("test".to_string()),
        command: Some("cargo".to_string()),
        args: Some(vec!["test".to_string()]),
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

    install(&logger, &task);
}

#[test]
#[should_panic]
fn install_crate_missing_cargo_command() {
    let logger = log::create("error");
    let task = Task {
        install_crate: Some("test".to_string()),
        command: Some("cargo".to_string()),
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

    install(&logger, &task);
}

#[test]
fn install_script_ok() {
    let logger = log::create("error");
    let task = Task {
        install_script: Some(vec!["exit 0".to_string()]),
        install_crate: None,
        command: None,
        args: None,
        disabled: None,
        alias: None,
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        script: None,
        dependencies: None,
        linux: None,
        windows: None,
        mac: None
    };

    install(&logger, &task);
}
