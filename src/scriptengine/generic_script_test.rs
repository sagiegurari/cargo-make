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

#[test]
#[should_panic]
fn execute_shell_error() {
    execute(
        &vec!["exit 1".to_string()],
        test::get_os_runner(),
        test::get_os_extension(),
    );
}
