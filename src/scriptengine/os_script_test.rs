use super::*;
use crate::test;

#[test]
fn execute_shell() {
    execute(&vec!["exit 0".to_string()], None, &vec![], true);
}

#[test]
fn execute_shell_with_runner() {
    if !test::is_windows_on_travis_ci() {
        execute(
            &vec!["exit 0".to_string()],
            Some(test::get_os_runner()),
            &vec![],
            true,
        );
    }
}

#[test]
#[should_panic]
fn execute_shell_error() {
    execute(&vec!["exit 1".to_string()], None, &vec![], true);
}

#[test]
fn execute_shell_error_no_validate() {
    execute(&vec!["exit 1".to_string()], None, &vec![], false);
}
