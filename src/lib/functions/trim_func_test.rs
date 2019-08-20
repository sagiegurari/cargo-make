use super::*;

use envmnt;

#[test]
#[should_panic]
fn trim_invoke_empty() {
    invoke(&vec![]);
}

#[test]
#[should_panic]
fn trim_invoke_invalid_too_many_args() {
    invoke(&vec!["TEST".to_string(), "1".to_string(), "2".to_string()]);
}

#[test]
#[should_panic]
fn trim_invoke_invalid_trim_type() {
    invoke(&vec!["TEST".to_string(), "bad".to_string()]);
}

#[test]
fn trim_invoke_exists_with_value() {
    envmnt::set("TEST_TRIM_VALID", "abc");

    let output = invoke(&vec!["TEST_TRIM_VALID".to_string()]);

    assert_eq!(output, vec!["abc"]);
}

#[test]
fn trim_invoke_exists_empty() {
    envmnt::set("TEST_TRIM_EMPTY", "");

    let output = invoke(&vec!["TEST_TRIM_EMPTY".to_string()]);

    assert_eq!(output.len(), 0);
}

#[test]
fn trim_invoke_not_exists() {
    let output = invoke(&vec!["TEST_TRIM_NOT_EXISTS".to_string()]);

    assert_eq!(output.len(), 0);
}

#[test]
fn trim_invoke_all_spaces() {
    envmnt::set("TEST_TRIM_ALL_SPACES", "");

    let output = invoke(&vec!["TEST_TRIM_ALL_SPACES".to_string()]);

    assert_eq!(output.len(), 0);
}

#[test]
fn trim_invoke_partial_spaces() {
    envmnt::set("TEST_TRIM_ALL_PARTIAL_SPACES", "   123   123   ");

    let output = invoke(&vec!["TEST_TRIM_ALL_PARTIAL_SPACES".to_string()]);

    assert_eq!(output, vec!["123   123"]);
}

#[test]
fn trim_invoke_trim_start() {
    envmnt::set("TEST_TRIM_ALL_PARTIAL_SPACES", "   123   ");

    let output = invoke(&vec![
        "TEST_TRIM_ALL_PARTIAL_SPACES".to_string(),
        "start".to_string(),
    ]);

    assert_eq!(output, vec!["123   "]);
}

#[test]
fn trim_invoke_trim_end() {
    envmnt::set("TEST_TRIM_ALL_PARTIAL_SPACES", "   123   ");

    let output = invoke(&vec![
        "TEST_TRIM_ALL_PARTIAL_SPACES".to_string(),
        "end".to_string(),
    ]);

    assert_eq!(output, vec!["   123"]);
}
