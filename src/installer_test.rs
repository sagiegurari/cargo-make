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
    let task = Task::new();

    install(&logger, &task);
}

#[test]
fn install_crate_already_installed() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.install_crate = Some("test".to_string());
    task.command = Some("cargo".to_string());
    task.args = Some(vec!["test".to_string()]);

    install(&logger, &task);
}

#[test]
#[should_panic]
fn install_crate_missing_cargo_command() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.install_crate = Some("test".to_string());
    task.command = Some("cargo".to_string());

    install(&logger, &task);
}

#[test]
fn install_script_ok() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.install_script = Some(vec!["exit 0".to_string()]);

    install(&logger, &task);
}


#[test]
fn install_script_error_force() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.force = Some(true);
    task.install_script = Some(vec!["exit 1".to_string()]);

    install(&logger, &task);
}
