use super::*;

#[test]
fn execute_valid() {
    execute(
        &vec!["echo test".to_string()],
        &vec!["test".to_string()],
        true,
    );
}

#[test]
#[should_panic]
fn execute_error() {
    execute(&vec!["exit 1".to_string()], &vec![], true);
}

#[test]
fn execute_error_no_validate() {
    execute(&vec!["exit 1".to_string()], &vec![], false);
}
