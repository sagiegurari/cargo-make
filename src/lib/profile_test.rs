use super::*;

use std::env;

#[test]
fn get_not_defined() {
    env::remove_var("CARGO_MAKE_PROFILE");
    let output = get();
    assert_eq!(output, "development".to_string());
}

#[test]
fn get_defined() {
    env::set_var("CARGO_MAKE_PROFILE", "TEST123");
    let output = get();
    assert_eq!(output, "TEST123".to_string());
}

#[test]
fn set_empty() {
    env::remove_var("CARGO_MAKE_PROFILE");
    let mut output = set("");
    assert_eq!(output, "development".to_string());
    output = get();
    assert_eq!(output, "development".to_string());
    output = env::var("CARGO_MAKE_PROFILE").unwrap();
    assert_eq!(output, "development".to_string());
}

#[test]
fn set_spaces() {
    env::remove_var("CARGO_MAKE_PROFILE");
    let mut output = set("   ");
    assert_eq!(output, "development".to_string());
    output = get();
    assert_eq!(output, "development".to_string());
    output = env::var("CARGO_MAKE_PROFILE").unwrap();
    assert_eq!(output, "development".to_string());
}

#[test]
fn set_mixed() {
    env::remove_var("CARGO_MAKE_PROFILE");
    let mut output = set("   SOME profile NAME  ");
    assert_eq!(output, "some profile name".to_string());
    output = get();
    assert_eq!(output, "some profile name".to_string());
    output = env::var("CARGO_MAKE_PROFILE").unwrap();
    assert_eq!(output, "some profile name".to_string());
}
