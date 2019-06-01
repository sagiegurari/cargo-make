use super::*;
use envmnt;

#[test]
fn normalize_profile_same() {
    let output = normalize_profile("test");

    assert_eq!(output, "test");
}

#[test]
fn normalize_profile_mixed_case() {
    let output = normalize_profile("tEst");

    assert_eq!(output, "test");
}

#[test]
fn normalize_profile_spaces() {
    let output = normalize_profile("  test  ");

    assert_eq!(output, "test");
}

#[test]
fn normalize_profile_case_and_spaces() {
    let output = normalize_profile("  tEst  ");

    assert_eq!(output, "test");
}

#[test]
fn normalize_additional_profiles_empty() {
    let output = normalize_additional_profiles(&vec![]);

    assert_eq!(output, "");
}

#[test]
fn normalize_additional_profiles_single() {
    let output = normalize_additional_profiles(&vec!["  TEst  ".to_string()]);

    assert_eq!(output, "test");
}

#[test]
fn normalize_additional_profiles_multiple() {
    let output =
        normalize_additional_profiles(&vec!["  TEst  ".to_string(), "  test2  ".to_string()]);

    assert_eq!(output, "test;test2");
}

#[test]
fn get_not_defined() {
    envmnt::remove("CARGO_MAKE_PROFILE");
    let output = get();
    assert_eq!(output, "development".to_string());
}

#[test]
fn get_defined() {
    envmnt::set("CARGO_MAKE_PROFILE", "TEST123");
    let output = get();
    assert_eq!(output, "TEST123".to_string());
}

#[test]
fn set_empty() {
    envmnt::remove("CARGO_MAKE_PROFILE");
    let mut output = set("");
    assert_eq!(output, "development".to_string());
    output = get();
    assert_eq!(output, "development".to_string());
    output = envmnt::get_or_panic("CARGO_MAKE_PROFILE");
    assert_eq!(output, "development".to_string());
}

#[test]
fn set_spaces() {
    envmnt::remove("CARGO_MAKE_PROFILE");
    let mut output = set("   ");
    assert_eq!(output, "development".to_string());
    output = get();
    assert_eq!(output, "development".to_string());
    output = envmnt::get_or_panic("CARGO_MAKE_PROFILE");
    assert_eq!(output, "development".to_string());
}

#[test]
fn set_mixed() {
    envmnt::remove("CARGO_MAKE_PROFILE");
    let mut output = set("   SOME profile NAME  ");
    assert_eq!(output, "some profile name".to_string());
    output = get();
    assert_eq!(output, "some profile name".to_string());
    output = envmnt::get_or_panic("CARGO_MAKE_PROFILE");
    assert_eq!(output, "some profile name".to_string());
}

#[test]
fn set_additional_multiple() {
    envmnt::remove("CARGO_MAKE_ADDITIONAL_PROFILES");
    set_additional(&vec!["  TEst1  ".to_string(), "  test2  ".to_string()]);
    let output = envmnt::get_or_panic("CARGO_MAKE_ADDITIONAL_PROFILES");
    assert_eq!(output, "test1;test2".to_string());
}
