use super::*;
use crate::test;

#[test]
fn execute_shell() {
    let valid = execute(
        &vec!["exit 0".to_string()],
        test::get_os_runner(),
        test::get_os_extension(),
        None,
        &vec![],
        true,
    )
    .unwrap();
    assert!(valid);
}

#[test]
#[should_panic]
fn execute_shell_error() {
    execute(
        &vec!["exit 1".to_string()],
        test::get_os_runner(),
        test::get_os_extension(),
        None,
        &vec![],
        true,
    )
    .unwrap();
}

#[test]
fn execute_shell_error_no_validate() {
    let valid = execute(
        &vec!["exit 1".to_string()],
        test::get_os_runner(),
        test::get_os_extension(),
        None,
        &vec![],
        false,
    )
    .unwrap();
    assert!(!valid);
}

#[test]
fn execute_shell_empty_arguments() {
    let valid = execute(
        &vec!["exit 0".to_string()],
        test::get_os_runner(),
        test::get_os_extension(),
        Some(vec![]),
        &vec![],
        true,
    )
    .unwrap();
    assert!(valid);
}

#[test]
#[cfg(target_os = "linux")]
fn execute_shell_cli_arguments() {
    let valid = execute(
        &vec!["exit $1".to_string()],
        test::get_os_runner(),
        test::get_os_extension(),
        Some(vec![]),
        &vec!["0".to_string()],
        true,
    )
    .unwrap();
    assert!(valid);
}

#[test]
#[should_panic]
#[cfg(target_os = "linux")]
fn execute_shell_cli_arguments_error() {
    execute(
        &vec!["exit $1".to_string()],
        test::get_os_runner(),
        test::get_os_extension(),
        Some(vec![]),
        &vec!["1".to_string()],
        true,
    )
    .unwrap();
}
