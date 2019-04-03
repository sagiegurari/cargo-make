use super::*;

use std::env;

#[test]
#[should_panic]
fn invoke_empty() {
    invoke(&vec![]);
}

#[test]
#[should_panic]
fn invoke_invalid_too_many_args() {
    invoke(&vec!["TEST".to_string(), "1".to_string()]);
}

#[test]
fn invoke_exists_with_value() {
    env::set_var("TEST_REMOVE_EMPTY_VALID", "abc");

    let output = invoke(&vec!["TEST_REMOVE_EMPTY_VALID".to_string()]);

    assert_eq!(output, vec!["abc"]);
}

#[test]
fn invoke_exists_empty() {
    env::set_var("TEST_REMOVE_EMPTY_EMPTY", "");

    let output = invoke(&vec!["TEST_REMOVE_EMPTY_EMPTY".to_string()]);

    assert_eq!(output.len(), 0);
}

#[test]
fn invoke_not_exists() {
    let output = invoke(&vec!["TEST_REMOVE_EMPTY_NOT_EXISTS".to_string()]);

    assert_eq!(output.len(), 0);
}
