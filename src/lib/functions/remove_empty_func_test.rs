use super::*;

use envmnt;

#[test]
#[should_panic]
fn remote_empty_invoke_empty() {
    invoke(&vec![]);
}

#[test]
#[should_panic]
fn remote_empty_invoke_invalid_too_many_args() {
    invoke(&vec!["TEST".to_string(), "1".to_string()]);
}

#[test]
fn remote_empty_invoke_exists_with_value() {
    envmnt::set("TEST_REMOVE_EMPTY_VALID", "abc");

    let output = invoke(&vec!["TEST_REMOVE_EMPTY_VALID".to_string()]);

    assert_eq!(output, vec!["abc"]);
}

#[test]
fn remote_empty_invoke_exists_empty() {
    envmnt::set("TEST_REMOVE_EMPTY_EMPTY", "");

    let output = invoke(&vec!["TEST_REMOVE_EMPTY_EMPTY".to_string()]);

    assert_eq!(output.len(), 0);
}

#[test]
fn remote_empty_invoke_not_exists() {
    let output = invoke(&vec!["TEST_REMOVE_EMPTY_NOT_EXISTS".to_string()]);

    assert_eq!(output.len(), 0);
}
