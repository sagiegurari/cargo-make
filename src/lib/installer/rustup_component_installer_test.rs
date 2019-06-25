use super::*;
use crate::test;
use crate::types::TestArg;

#[test]
fn is_installed_true() {
    let output = is_installed(&None, "cargo", &["--version".to_string()]);
    assert!(output);
}

#[test]
fn is_installed_false() {
    let output = is_installed(&None, "cargo_bad", &["--version".to_string()]);
    assert!(!output);
}

#[test]
fn is_installed_non_zero() {
    let output = is_installed(&None, "exit", &["1".to_string()]);
    assert!(!output);
}

#[test]
fn is_installed_with_toolchain_true() {
    if test::is_not_rust_stable() {
        let toolchain = test::get_toolchain();

        let output = is_installed(&Some(toolchain), "cargo", &["--version".to_string()]);
        assert!(output);
    }
}

#[test]
fn is_installed_with_toolchain_false() {
    if test::is_not_rust_stable() {
        let toolchain = test::get_toolchain();

        let output = is_installed(&Some(toolchain), "cargo_bad", &["--version".to_string()]);
        assert!(!output);
    }
}

#[test]
fn is_installed_with_toolchain_non_zero() {
    let toolchain = test::get_toolchain();

    let output = is_installed(&Some(toolchain), "exit", &["1".to_string()]);
    assert!(!output);
}

#[test]
fn invoke_rustup_install_fail() {
    let info = InstallRustupComponentInfo {
        rustup_component_name: "unknown_rustup_component_test".to_string(),
        binary: Some("cargo_bad".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };

    let output = invoke_rustup_install(&None, &info);
    assert!(!output);
}

#[test]
fn invoke_rustup_install_with_toolchain_fail() {
    let toolchain = test::get_toolchain();

    let info = InstallRustupComponentInfo {
        rustup_component_name: "unknown_rustup_component_test".to_string(),
        binary: Some("cargo_bad".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };

    let output = invoke_rustup_install(&Some(toolchain), &info);
    assert!(!output);
}

#[test]
fn install_test() {
    let info = InstallRustupComponentInfo {
        rustup_component_name: "unknown_rustup_component_test".to_string(),
        binary: Some("cargo_bad".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };

    let output = install(&None, &info, false);
    assert!(!output);
}

#[test]
fn install_with_toolchain_test() {
    let toolchain = test::get_toolchain();

    let info = InstallRustupComponentInfo {
        rustup_component_name: "unknown_rustup_component_test".to_string(),
        binary: Some("cargo_bad".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };

    let output = install(&Some(toolchain), &info, false);
    assert!(!output);
}
