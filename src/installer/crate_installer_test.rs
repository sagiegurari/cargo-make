use super::*;

#[test]
fn is_crate_installed_true() {
    let output = is_crate_installed("cargo", "--version");
    assert!(output);
}

#[test]
fn is_crate_installed_false() {
    let output = is_crate_installed("cargo_bad", "--version");
    assert!(!output);
}

#[test]
fn is_crate_installed_non_zero() {
    let output = is_crate_installed("exit", "1");
    assert!(!output);
}

#[test]
fn invoke_rustup_install_none() {
    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "test".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: None,
    };

    let output = invoke_rustup_install(&info);
    assert!(!output);
}

#[test]
fn invoke_rustup_install_fail() {
    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "test".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("unknown_rustup_component_test".to_string()),
    };

    let output = invoke_rustup_install(&info);
    assert!(!output);
}

#[test]
fn invoke_cargo_install_test() {
    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "cargo_bad".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("unknown_rustup_component_test".to_string()),
    };

    invoke_cargo_install(&info, &None, false);
}

#[test]
fn install_crate_test() {
    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "cargo_bad".to_string(),
        test_arg: "--help".to_string(),
        rustup_component_name: Some("unknown_rustup_component_test".to_string()),
    };

    install_crate(&info, &None, false);
}
