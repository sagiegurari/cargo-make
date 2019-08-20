use super::*;

use envmnt;

#[test]
#[should_panic]
fn decode_invoke_empty() {
    invoke(&vec![]);
}

#[test]
fn decode_invoke_only_source_not_found() {
    envmnt::remove("TEST_DECODE_ONLY_SOURCE_NOT_DEFINED");

    let output = invoke(&vec!["TEST_DECODE_ONLY_SOURCE_NOT_DEFINED".to_string()]);

    assert_eq!(output.len(), 0);
}

#[test]
fn decode_invoke_only_source_found_empty() {
    envmnt::set("TEST_DECODE_ONLY_SOURCE_DEFINED_EMPTY", "");

    let output = invoke(&vec!["TEST_DECODE_ONLY_SOURCE_DEFINED_EMPTY".to_string()]);

    assert_eq!(output.len(), 0);
}

#[test]
fn decode_invoke_only_source_found_value() {
    envmnt::set("TEST_DECODE_ONLY_SOURCE_DEFINED_VALUE", "test");

    let output = invoke(&vec!["TEST_DECODE_ONLY_SOURCE_DEFINED_VALUE".to_string()]);

    assert_eq!(output, vec!["test"]);
}

#[test]
fn decode_invoke_only_default_empty() {
    envmnt::set("TEST_DECODE_ONLY_DEFAULT_EMPTY", "test");

    let output = invoke(&vec![
        "TEST_DECODE_ONLY_DEFAULT_EMPTY".to_string(),
        "".to_string(),
    ]);

    assert_eq!(output.len(), 0);
}

#[test]
fn decode_invoke_only_default_value() {
    envmnt::set("TEST_DECODE_ONLY_DEFAULT_VALUE", "test");

    let output = invoke(&vec![
        "TEST_DECODE_ONLY_DEFAULT_VALUE".to_string(),
        "default".to_string(),
    ]);

    assert_eq!(output, vec!["default"]);
}

#[test]
fn decode_invoke_only_default_eval_value() {
    envmnt::set("TEST_DECODE_ONLY_DEFAULT_EVAL_VALUE", "test");
    envmnt::set("TEST_DECODE_ONLY_DEFAULT_EVAL_VALUE_RESULT", "result");

    let output = invoke(&vec![
        "TEST_DECODE_ONLY_DEFAULT_EVAL_VALUE".to_string(),
        "${TEST_DECODE_ONLY_DEFAULT_EVAL_VALUE_RESULT}-test".to_string(),
    ]);

    assert_eq!(output, vec!["result-test"]);
}

#[test]
fn decode_invoke_mappings_not_found_use_source() {
    envmnt::set("TEST_DECODE_MAPPINGS_NOT_FOUND_USE_SOURCE", "source");

    let output = invoke(&vec![
        "TEST_DECODE_MAPPINGS_NOT_FOUND_USE_SOURCE".to_string(),
        "key1".to_string(),
        "value1".to_string(),
        "key2".to_string(),
        "value2".to_string(),
    ]);

    assert_eq!(output, vec!["source"]);
}

#[test]
fn decode_invoke_mappings_not_found_use_default() {
    envmnt::set("TEST_DECODE_MAPPINGS_NOT_FOUND_USE_DEFAULT", "source");

    let output = invoke(&vec![
        "TEST_DECODE_MAPPINGS_NOT_FOUND_USE_DEFAULT".to_string(),
        "key1".to_string(),
        "value1".to_string(),
        "key2".to_string(),
        "value2".to_string(),
        "default".to_string(),
    ]);

    assert_eq!(output, vec!["default"]);
}

#[test]
fn decode_invoke_mappings_found_no_default() {
    envmnt::set("TEST_DECODE_MAPPINGS_FOUND_NO_DEFAULT", "key2");

    let output = invoke(&vec![
        "TEST_DECODE_MAPPINGS_FOUND_NO_DEFAULT".to_string(),
        "key1".to_string(),
        "value1".to_string(),
        "key2".to_string(),
        "value2".to_string(),
    ]);

    assert_eq!(output, vec!["value2"]);
}

#[test]
fn decode_invoke_mappings_found_with_default() {
    envmnt::set("TEST_DECODE_MAPPINGS_FOUND_WITH_DEFAULT", "key2");

    let output = invoke(&vec![
        "TEST_DECODE_MAPPINGS_FOUND_WITH_DEFAULT".to_string(),
        "key1".to_string(),
        "value1".to_string(),
        "key2".to_string(),
        "value2".to_string(),
        "default".to_string(),
    ]);

    assert_eq!(output, vec!["value2"]);
}

#[test]
fn decode_invoke_mappings_found_eval_output() {
    envmnt::set("TEST_DECODE_MAPPINGS_FOUND_EVAL_OUTPUT", "key2");
    envmnt::set("TEST_DECODE_MAPPINGS_FOUND_EVAL_OUTPUT_VALUE", "value2");

    let output = invoke(&vec![
        "TEST_DECODE_MAPPINGS_FOUND_EVAL_OUTPUT".to_string(),
        "key1".to_string(),
        "value1".to_string(),
        "key2".to_string(),
        "${TEST_DECODE_MAPPINGS_FOUND_EVAL_OUTPUT_VALUE}-output".to_string(),
        "default".to_string(),
    ]);

    assert_eq!(output, vec!["value2-output"]);
}
