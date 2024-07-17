use super::*;

#[test]
fn execute_valid() {
    let valid = execute(
        &vec!["echo test".to_string()],
        &vec!["test".to_string()],
        true,
    )
    .unwrap();

    assert!(valid);
}

#[test]
#[should_panic]
fn execute_error() {
    execute(&vec!["exit 1".to_string()], &vec![], true).unwrap();
}

#[test]
fn execute_error_no_validate() {
    let valid = execute(&vec!["exit 1".to_string()], &vec![], false).unwrap();
    assert!(!valid);
}
