use super::*;
use crate::test;
use crate::types::{InstallCrateInfo, InstallRustupComponentInfo, TestArg};
use envmnt;

#[test]
fn get_cargo_plugin_info_from_command_no_command() {
    let task = Task::new();

    let value = get_cargo_plugin_info_from_command(&task);

    assert!(value.is_none());
}

#[test]
fn get_cargo_plugin_info_from_command_not_cargo_command() {
    let mut task = Task::new();
    task.command = Some("echo".to_string());
    task.args = Some(vec!["test".to_string()]);

    let value = get_cargo_plugin_info_from_command(&task);

    assert!(value.is_none());
}

#[test]
fn get_cargo_plugin_info_from_command_no_args() {
    let mut task = Task::new();
    task.command = Some("cargo".to_string());
    task.args = None;

    let value = get_cargo_plugin_info_from_command(&task);

    assert!(value.is_none());
}

#[test]
fn get_cargo_plugin_info_from_command_empty_args() {
    let mut task = Task::new();
    task.command = Some("cargo".to_string());
    task.args = Some(vec![]);

    let value = get_cargo_plugin_info_from_command(&task);

    assert!(value.is_none());
}

#[test]
fn get_cargo_plugin_info_from_command_valid() {
    let mut task = Task::new();
    task.command = Some("cargo".to_string());
    task.args = Some(vec!["test".to_string()]);

    let value = get_cargo_plugin_info_from_command(&task);

    let (command, crate_name) = value.unwrap();

    assert_eq!(command, "test");
    assert_eq!(crate_name, "cargo-test");
}

#[test]
fn install_empty() {
    let task = Task::new();

    install(&task, &test::create_empty_flow_info());
}

#[test]
fn install_crate_already_installed() {
    let mut task = Task::new();
    task.install_crate = Some(InstallCrate::Value("test".to_string()));
    task.command = Some("cargo".to_string());
    task.args = Some(vec!["test".to_string()]);

    install(&task, &test::create_empty_flow_info());
}

#[test]
#[should_panic]
fn install_crate_missing_cargo_command() {
    let mut task = Task::new();
    task.install_crate = Some(InstallCrate::Value("test".to_string()));
    task.command = Some("cargo".to_string());

    install(&task, &test::create_empty_flow_info());
}

#[test]
fn install_crate_auto_detect_already_installed() {
    let mut task = Task::new();
    task.command = Some("cargo".to_string());
    task.args = Some(vec!["test".to_string()]);

    install(&task, &test::create_empty_flow_info());
}

#[test]
#[should_panic]
fn install_crate_auto_detect_unable_to_install() {
    let mut task = Task::new();
    task.command = Some("cargo".to_string());
    task.args = Some(vec!["badbadbad".to_string()]);

    install(&task, &test::create_empty_flow_info());
}

#[test]
fn install_rustup_via_crate_info() {
    let info = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "cargo".to_string(),
        test_arg: TestArg {
            inner: vec!["--version".to_string()],
        },
        rustup_component_name: None,
        min_version: None,
    };

    let mut task = Task::new();
    task.command = Some("test".to_string());
    task.install_crate = Some(InstallCrate::CrateInfo(info));

    install(&task, &test::create_empty_flow_info());
}

#[test]
fn install_rustup_via_rustup_info() {
    let info = InstallRustupComponentInfo {
        rustup_component_name: "test".to_string(),
        binary: Some("cargo".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--version".to_string()],
        }),
    };

    let mut task = Task::new();
    task.command = Some("test".to_string());
    task.install_crate = Some(InstallCrate::RustupComponentInfo(info));

    install(&task, &test::create_empty_flow_info());
}

#[test]
fn install_script_ok() {
    let mut task = Task::new();
    task.install_script = Some(vec!["exit 0".to_string()]);

    install(&task, &test::create_empty_flow_info());
}

#[test]
#[should_panic]
fn install_script_error() {
    let mut task = Task::new();
    task.install_script = Some(vec!["exit 1".to_string()]);

    install(&task, &test::create_empty_flow_info());
}

#[test]
fn install_script_duckscript() {
    envmnt::remove("install_script_duckscript");
    assert!(!envmnt::exists("install_script_duckscript"));

    let mut task = Task::new();
    task.install_script = Some(vec![r#"#!@duckscript
    set_env install_script_duckscript true
    "#
    .to_string()]);

    install(&task, &test::create_empty_flow_info());

    assert!(envmnt::exists("install_script_duckscript"));
    assert!(envmnt::is_or("install_script_duckscript", false));
}

#[test]
fn install_script_error_ignore_errors() {
    let mut task = Task::new();
    task.ignore_errors = Some(true);
    task.install_script = Some(vec!["exit 1".to_string()]);

    install(&task, &test::create_empty_flow_info());
}
