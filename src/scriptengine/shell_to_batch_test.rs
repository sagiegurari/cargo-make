use super::*;

#[test]
fn execute_valid() {
    execute(&vec!["echo test".to_string()], &vec!["test".to_string()]);
}

#[test]
#[should_panic]
fn execute_error() {
    execute(&vec!["exit 1".to_string()], &vec![]);
}
