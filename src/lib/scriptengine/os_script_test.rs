use super::*;
use crate::test;

#[test]
fn execute_shell() {
    execute(&vec!["exit 0".to_string()], None, &vec![], true);
}

#[test]
fn execute_shell_with_runner() {
    let valid = execute(
        &vec!["exit 0".to_string()],
        Some(test::get_os_runner()),
        &vec![],
        true,
    );
    assert!(valid);
}

#[test]
#[should_panic]
fn execute_shell_error() {
    execute(&vec!["exit 1".to_string()], None, &vec![], true);
}

#[test]
fn execute_shell_error_no_validate() {
    let valid = execute(&vec!["exit 1".to_string()], None, &vec![], false);
    assert!(!valid);
}
