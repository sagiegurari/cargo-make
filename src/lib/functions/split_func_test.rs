use super::*;

use crate::test;

#[test]
#[should_panic]
fn split_invoke_empty() {
    invoke(&vec![]);
}

#[test]
#[should_panic]
fn split_invoke_invalid_too_many_args() {
    test::on_test_startup();
    invoke(&vec![
        "TEST".to_string(),
        "1".to_string(),
        "2".to_string(),
        "3".to_string(),
    ]);
}

#[test]
#[should_panic]
fn split_invoke_invalid_split_by_big() {
    test::on_test_startup();
    invoke(&vec!["TEST".to_string(), "ab".to_string()]);
}

#[test]
#[should_panic]
fn split_invoke_invalid_split_by_empty() {
    invoke(&vec!["TEST".to_string(), "".to_string()]);
}

#[test]
fn split_invoke_exists_splitted_comma() {
    envmnt::set("TEST_SPLIT_VALUE_COMMA", "1,2,3,4");

    let output = invoke(&vec!["TEST_SPLIT_VALUE_COMMA".to_string(), ",".to_string()]);

    assert_eq!(output, vec!["1", "2", "3", "4"]);
}

#[test]
fn split_invoke_exists_splitted_space() {
    envmnt::set("TEST_SPLIT_VALUE_SPACE", "1 2 3 4");

    let output = invoke(&vec!["TEST_SPLIT_VALUE_SPACE".to_string(), " ".to_string()]);

    assert_eq!(output, vec!["1", "2", "3", "4"]);
}

#[test]
fn split_invoke_exists_splitted_with_empty_value() {
    envmnt::set("TEST_SPLIT_VALUE_WITH_EMPTY_VALUE", "1;2;3;;4");

    let mut output = invoke(&vec![
        "TEST_SPLIT_VALUE_WITH_EMPTY_VALUE".to_string(),
        ";".to_string(),
    ]);

    assert_eq!(output, vec!["1", "2", "3", "", "4"]);

    output = invoke(&vec![
        "TEST_SPLIT_VALUE_WITH_EMPTY_VALUE".to_string(),
        ";".to_string(),
        "default".to_string(),
    ]);

    assert_eq!(output, vec!["1", "2", "3", "", "4"]);
}

#[test]
fn split_invoke_exists_splitted_with_empty_value_removed() {
    envmnt::set("TEST_SPLIT_VALUE_WITH_EMPTY_VALUE_REMOVED", "1;2;3;;4");

    let output = invoke(&vec![
        "TEST_SPLIT_VALUE_WITH_EMPTY_VALUE_REMOVED".to_string(),
        ";".to_string(),
        "remove-empty".to_string(),
    ]);

    assert_eq!(output, vec!["1", "2", "3", "4"]);
}

#[test]
fn split_invoke_exists_not_splitted() {
    envmnt::set("TEST_SPLIT_VALUE_NOT_SPLITTED", "1,2,3,4");

    let output = invoke(&vec![
        "TEST_SPLIT_VALUE_NOT_SPLITTED".to_string(),
        "|".to_string(),
    ]);

    assert_eq!(output, vec!["1,2,3,4"]);
}

#[test]
fn split_invoke_not_exists() {
    let output = invoke(&vec![
        "TEST_SPLIT_VALUE_NOT_EXISTS".to_string(),
        ",".to_string(),
    ]);

    let expected: Vec<String> = vec![];
    assert_eq!(output, expected);
}
