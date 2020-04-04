use super::*;

use crate::test;
use envmnt;

#[test]
#[should_panic]
fn getat_invoke_empty() {
    invoke(&vec![]);
}

#[test]
#[should_panic]
fn getat_invoke_invalid_too_many_args() {
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
fn getat_invoke_invalid_getat_by_big() {
    test::on_test_startup();
    invoke(&vec!["TEST".to_string(), "ab".to_string(), "0".to_string()]);
}

#[test]
#[should_panic]
fn getat_invoke_invalid_getat_by_empty() {
    invoke(&vec!["TEST".to_string(), "".to_string(), "0".to_string()]);
}

#[test]
fn getat_invoke_exists_splitted_comma() {
    envmnt::set("TEST_GETAT_VALUE_COMMA", "1,2,3,4");

    let output = invoke(&vec![
        "TEST_GETAT_VALUE_COMMA".to_string(),
        ",".to_string(),
        "0".to_string(),
    ]);

    assert_eq!(output, vec!["1"]);
}

#[test]
fn getat_invoke_exists_splitted_space() {
    envmnt::set("TEST_GETAT_VALUE_SPACE", "1 2 3 4");

    let output = invoke(&vec![
        "TEST_GETAT_VALUE_SPACE".to_string(),
        " ".to_string(),
        "0".to_string(),
    ]);

    assert_eq!(output, vec!["1"]);
}

#[test]
fn getat_invoke_exists_not_splitted() {
    envmnt::set("TEST_GETAT_VALUE_NOT_GETATTED", "1,2,3,4");

    let output = invoke(&vec![
        "TEST_GETAT_VALUE_NOT_GETATTED".to_string(),
        "|".to_string(),
        "0".to_string(),
    ]);

    assert_eq!(output, vec!["1,2,3,4"]);
}

#[test]
fn getat_invoke_not_exists() {
    let output = invoke(&vec![
        "TEST_GETAT_VALUE_NOT_EXISTS".to_string(),
        ",".to_string(),
        "0".to_string(),
    ]);

    let expected: Vec<String> = vec![];
    assert_eq!(output, expected);
}

#[test]
fn getat_invoke_exists_splitted_middle() {
    envmnt::set("TEST_GETAT_VALUE_MIDDLE", "1,2,3,4");

    let output = invoke(&vec![
        "TEST_GETAT_VALUE_MIDDLE".to_string(),
        ",".to_string(),
        "2".to_string(),
    ]);

    assert_eq!(output, vec!["3"]);
}

#[test]
fn getat_invoke_exists_splitted_out_of_bounds() {
    envmnt::set("TEST_GETAT_VALUE_OUT_OF_BOUNDS", "1,2,3,4");

    let output = invoke(&vec![
        "TEST_GETAT_VALUE_OUT_OF_BOUNDS".to_string(),
        ",".to_string(),
        "20".to_string(),
    ]);

    let expected: Vec<String> = vec![];
    assert_eq!(output, expected);
}
