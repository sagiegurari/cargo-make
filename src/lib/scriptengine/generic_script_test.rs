use super::*;
use crate::test;

#[test]
fn execute_shell() {
    execute(
        &vec!["exit 0".to_string()],
        test::get_os_runner(),
        test::get_os_extension(),
        None,
        &vec![],
        true,
    );
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
    );
}

#[test]
fn execute_shell_error_no_validate() {
    execute(
        &vec!["exit 1".to_string()],
        test::get_os_runner(),
        test::get_os_extension(),
        None,
        &vec![],
        false,
    );
}

#[test]
fn execute_shell_empty_arguments() {
    execute(
        &vec!["exit 0".to_string()],
        test::get_os_runner(),
        test::get_os_extension(),
        Some(vec![]),
        &vec![],
        true,
    );
}

#[test]
fn execute_shell_cli_arguments() {
    let command = if cfg!(windows) {
        "exit %1".to_string()
    } else {
        "exit $1".to_string()
    };

    execute(
        &vec![command],
        test::get_os_runner(),
        test::get_os_extension(),
        Some(vec![]),
        &vec!["0".to_string()],
        true,
    );
}

#[test]
#[should_panic]
fn execute_shell_cli_arguments_error() {
    let command = if cfg!(windows) {
        "exit %1".to_string()
    } else {
        "exit $1".to_string()
    };

    execute(
        &vec![command],
        test::get_os_runner(),
        test::get_os_extension(),
        Some(vec![]),
        &vec!["1".to_string()],
        true,
    );
}
