use super::*;

#[test]
fn is_crate_installed_true() {
    let output = is_crate_installed("test");
    assert!(output);
}

#[test]
fn is_crate_installed_false() {
    let output = is_crate_installed("badbadbad");
    assert!(!output);
}

#[test]
fn install_empty() {
    let task = Task::new();

    install(&task);
}

#[test]
fn install_crate_already_installed() {
    let mut task = Task::new();
    task.install_crate = Some("test".to_string());
    task.command = Some("cargo".to_string());
    task.args = Some(vec!["test".to_string()]);

    install(&task);
}

#[test]
#[should_panic]
fn install_crate_missing_cargo_command() {
    let mut task = Task::new();
    task.install_crate = Some("test".to_string());
    task.command = Some("cargo".to_string());

    install(&task);
}

#[test]
fn install_crate_auto_detect_already_installed() {
    let mut task = Task::new();
    task.command = Some("cargo".to_string());
    task.args = Some(vec!["test".to_string()]);

    install(&task);
}

#[test]
#[should_panic]
fn install_crate_auto_detect_unable_to_install() {
    let mut task = Task::new();
    task.command = Some("cargo".to_string());
    task.args = Some(vec!["badbadbad".to_string()]);

    install(&task);
}

#[test]
fn get_install_crate_args_no_args() {
    let all_args = get_install_crate_args("test123", &None);

    assert_eq!(all_args.len(), 2);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "test123");
}

#[test]
fn get_install_crate_args_empty_args() {
    let all_args = get_install_crate_args("test123", &Some(vec![]));

    assert_eq!(all_args.len(), 2);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "test123");
}

#[test]
fn get_install_crate_args_with_args() {
    let all_args = get_install_crate_args(
        "test123",
        &Some(vec!["arg1".to_string(), "arg2".to_string()]),
    );

    assert_eq!(all_args.len(), 4);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "arg1");
    assert_eq!(all_args[2], "arg2");
    assert_eq!(all_args[3], "test123");
}

#[test]
fn install_script_ok() {
    let mut task = Task::new();
    task.install_script = Some(vec!["exit 0".to_string()]);

    install(&task);
}

#[test]
#[should_panic]
fn install_script_error() {
    let mut task = Task::new();
    task.install_script = Some(vec!["exit 1".to_string()]);

    install(&task);
}

#[test]
fn install_script_error_force() {
    let mut task = Task::new();
    task.force = Some(true);
    task.install_script = Some(vec!["exit 1".to_string()]);

    install(&task);
}
