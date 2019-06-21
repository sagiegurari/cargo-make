use super::*;
use crate::test;
use crate::types::TestArg;

#[test]
fn invoke_rustup_install_none() {
    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "test".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: None,
    };

    let output = invoke_rustup_install(&None, &info);
    assert!(!output);
}

#[test]
fn invoke_rustup_install_fail() {
    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "test".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("unknown_rustup_component_test".to_string()),
    };

    let output = invoke_rustup_install(&None, &info);
    assert!(!output);
}

#[test]
fn invoke_rustup_install_with_toolchain_none() {
    let toolchain = test::get_toolchain();

    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "test".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: None,
    };

    let output = invoke_rustup_install(&Some(toolchain), &info);
    assert!(!output);
}

#[test]
fn invoke_rustup_install_with_toolchain_fail() {
    let toolchain = test::get_toolchain();

    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "test".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("unknown_rustup_component_test".to_string()),
    };

    let output = invoke_rustup_install(&Some(toolchain), &info);
    assert!(!output);
}

#[test]
fn invoke_cargo_install_test() {
    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "cargo_bad".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("unknown_rustup_component_test".to_string()),
    };

    invoke_cargo_install(&None, &info, &None, false);
}

#[test]
fn invoke_cargo_install_with_toolchain_test() {
    let toolchain = test::get_toolchain();

    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "cargo_bad".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("unknown_rustup_component_test".to_string()),
    };

    invoke_cargo_install(&Some(toolchain), &info, &None, false);
}

#[test]
fn install_test_test() {
    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "cargo_bad".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("unknown_rustup_component_test".to_string()),
    };

    install(&None, &info, &None, false);
}

#[test]
fn install_test_with_toolchain_test() {
    let toolchain = test::get_toolchain();

    let info = InstallCrateInfo {
        crate_name: "bad_crate_name".to_string(),
        binary: "cargo_bad".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("unknown_rustup_component_test".to_string()),
    };

    install(&Some(toolchain), &info, &None, false);
}
