use super::*;

use std::env;

#[test]
#[should_panic]
fn invoke_empty() {
    invoke(&vec![]);
}

#[test]
#[should_panic]
fn invoke_invalid_split_by_big() {
    invoke(&vec!["TEST".to_string(), "ab".to_string()]);
}

#[test]
#[should_panic]
fn invoke_invalid_split_by_empty() {
    invoke(&vec!["TEST".to_string(), "".to_string()]);
}

#[test]
fn invoke_exists_splitted_comma() {
    env::set_var("TEST_SPLIT_VALUE", "1,2,3,4");

    let output = invoke(&vec!["TEST_SPLIT_VALUE".to_string(), ",".to_string()]);

    assert_eq!(output, vec!["1", "2", "3", "4"]);
}

#[test]
fn invoke_exists_splitted_space() {
    env::set_var("TEST_SPLIT_VALUE", "1 2 3 4");

    let output = invoke(&vec!["TEST_SPLIT_VALUE".to_string(), " ".to_string()]);

    assert_eq!(output, vec!["1", "2", "3", "4"]);
}

#[test]
fn invoke_exists_not_splitted() {
    env::set_var("TEST_SPLIT_VALUE", "1,2,3,4");

    let output = invoke(&vec!["TEST_SPLIT_VALUE".to_string(), "|".to_string()]);

    assert_eq!(output, vec!["1,2,3,4"]);
}

#[test]
fn invoke_not_exists() {
    let output = invoke(&vec![
        "TEST_SPLIT_VALUE_NOT_EXISTS".to_string(),
        ",".to_string(),
    ]);

    let expected: Vec<String> = vec![];
    assert_eq!(output, expected);
}
