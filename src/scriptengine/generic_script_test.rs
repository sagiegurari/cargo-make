use super::*;
use crate::test;

#[test]
fn execute_shell() {
    execute(
        &vec!["exit 0".to_string()],
        test::get_os_runner(),
        test::get_os_extension(),
    );
}

#[test]
fn execute_shell_hello() {
    execute(
        &vec!["echo hello".to_string()],
        test::get_os_runner(),
        test::get_os_extension(),
    );
}

#[cfg(target_os = "linux")]
#[test]
fn execute_shebang_bash_hello() {
    let script_test = vec!["#!/usr/bin/env bash".to_string(), "echo hello".to_string()];
    let extension = "sh".to_string();
    let runner = "usr/bin/env bash".to_string();
    execute(&script_test, runner, extension);
}

#[test]
#[should_panic]
fn execute_shell_error() {
    execute(
        &vec!["exit 1".to_string()],
        test::get_os_runner(),
        test::get_os_extension(),
    );
}
