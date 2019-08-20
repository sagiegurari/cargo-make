use super::*;

use envmnt;

#[test]
#[should_panic]
fn split_invoke_empty() {
    invoke(&vec![]);
}

#[test]
#[should_panic]
fn split_invoke_invalid_too_many_args() {
    invoke(&vec!["TEST".to_string(), "1".to_string(), "2".to_string()]);
}

#[test]
#[should_panic]
fn split_invoke_invalid_split_by_big() {
    invoke(&vec!["TEST".to_string(), "ab".to_string()]);
}

#[test]
#[should_panic]
fn split_invoke_invalid_split_by_empty() {
    invoke(&vec!["TEST".to_string(), "".to_string()]);
}

#[test]
fn split_invoke_exists_splitted_comma() {
    envmnt::set("TEST_SPLIT_VALUE", "1,2,3,4");

    let output = invoke(&vec!["TEST_SPLIT_VALUE".to_string(), ",".to_string()]);

    assert_eq!(output, vec!["1", "2", "3", "4"]);
}

#[test]
fn split_invoke_exists_splitted_space() {
    envmnt::set("TEST_SPLIT_VALUE", "1 2 3 4");

    let output = invoke(&vec!["TEST_SPLIT_VALUE".to_string(), " ".to_string()]);

    assert_eq!(output, vec!["1", "2", "3", "4"]);
}

#[test]
fn split_invoke_exists_not_splitted() {
    envmnt::set("TEST_SPLIT_VALUE", "1,2,3,4");

    let output = invoke(&vec!["TEST_SPLIT_VALUE".to_string(), "|".to_string()]);

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
