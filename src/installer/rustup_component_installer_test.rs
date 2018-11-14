use super::*;

#[test]
fn is_installed_true() {
    let output = is_installed("cargo", "--version");
    assert!(output);
}

#[test]
fn is_installed_false() {
    let output = is_installed("cargo_bad", "--version");
    assert!(!output);
}

#[test]
fn is_installed_non_zero() {
    let output = is_installed("exit", "1");
    assert!(!output);
}

#[test]
fn invoke_rustup_install_fail() {
    let info = InstallRustupComponentInfo {
        rustup_component_name: "unknown_rustup_component_test".to_string(),
        binary: Some("cargo_bad".to_string()),
        test_arg: Some("--help".to_string()),
    };

    let output = invoke_rustup_install(&info);
    assert!(!output);
}

#[test]
fn install_test() {
    let info = InstallRustupComponentInfo {
        rustup_component_name: "unknown_rustup_component_test".to_string(),
        binary: Some("cargo_bad".to_string()),
        test_arg: Some("--help".to_string()),
    };

    let output = install(&info, false);
    assert!(!output);
}
