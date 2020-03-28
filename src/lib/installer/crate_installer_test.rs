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
        min_version: None,
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
        min_version: None,
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
        min_version: None,
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
        min_version: None,
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
        min_version: None,
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
        min_version: None,
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
        min_version: None,
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
        min_version: None,
    };

    install(&Some(toolchain), &info, &None, false);
}

#[test]
fn install_already_installed_crate_only() {
    let info = InstallCrateInfo {
        crate_name: "cargo-make".to_string(),
        binary: "cargo".to_string(),
        test_arg: TestArg {
            inner: vec!["make".to_string(), "--version".to_string()],
        },
        rustup_component_name: None,
        min_version: None,
    };

    install(&None, &info, &None, false);
}

#[test]
fn install_already_installed_crate_only_min_version_equal() {
    let version = crate_version_check::get_crate_version("cargo-make").unwrap();
    let mut version_string = String::new();
    version_string.push_str(&version.major.to_string());
    version_string.push_str(".");
    version_string.push_str(&version.minor.to_string());
    version_string.push_str(".");
    version_string.push_str(&version.patch.to_string());

    let info = InstallCrateInfo {
        crate_name: "cargo-make".to_string(),
        binary: "cargo".to_string(),
        test_arg: TestArg {
            inner: vec!["make".to_string(), "--version".to_string()],
        },
        rustup_component_name: None,
        min_version: Some(version_string),
    };

    install(&None, &info, &None, false);
}

#[test]
fn install_already_installed_crate_only_min_version_smaller() {
    let mut version = crate_version_check::get_crate_version("cargo-make").unwrap();
    if version.patch > 0 {
        version.patch = version.patch - 1;
    } else if version.minor > 0 {
        version.minor = version.minor - 1;
    } else if version.major > 0 {
        version.major = version.major - 1;
    }

    let mut version_string = String::new();
    version_string.push_str(&version.major.to_string());
    version_string.push_str(".");
    version_string.push_str(&version.minor.to_string());
    version_string.push_str(".");
    version_string.push_str(&version.patch.to_string());

    let info = InstallCrateInfo {
        crate_name: "cargo-make".to_string(),
        binary: "cargo".to_string(),
        test_arg: TestArg {
            inner: vec!["make".to_string(), "--version".to_string()],
        },
        rustup_component_name: None,
        min_version: Some(version_string),
    };

    install(&None, &info, &None, false);
}

#[test]
fn is_crate_only_info_with_rustup_component_name() {
    let info = InstallCrateInfo {
        crate_name: "crate_name".to_string(),
        binary: "test".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("component".to_string()),
        min_version: None,
    };

    let crate_only_info = is_crate_only_info(&info);

    assert!(!crate_only_info);
}

#[test]
fn is_crate_only_info_without_rustup_component_name() {
    let info = InstallCrateInfo {
        crate_name: "crate_name".to_string(),
        binary: "test".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: None,
        min_version: None,
    };

    let crate_only_info = is_crate_only_info(&info);

    assert!(crate_only_info);
}
