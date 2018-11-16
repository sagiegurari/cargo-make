use super::*;
use crate::test;

#[test]
fn is_crate_installed_true() {
    let output = is_crate_installed(&None, "test");
    assert!(output);
}

#[test]
fn is_crate_installed_false() {
    let output = is_crate_installed(&None, "badbadbad");
    assert!(!output);
}

#[test]
fn is_crate_installed_with_toolchain_true() {
    if test::is_not_rust_stable() {
        let toolchain = test::get_toolchain();

        let output = is_crate_installed(&Some(toolchain), "test");
        assert!(output);
    }
}

#[test]
fn is_crate_installed_with_toolchain_false() {
    if test::is_not_rust_stable() {
        let toolchain = test::get_toolchain();

        let output = is_crate_installed(&Some(toolchain), "badbadbad");
        assert!(!output);
    }
}

#[test]
fn get_install_crate_args_no_args() {
    let all_args = get_install_crate_args("test123", false, &None);

    assert_eq!(all_args.len(), 2);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "test123");
}

#[test]
fn get_install_crate_args_no_args_force() {
    let all_args = get_install_crate_args("test123", true, &None);

    assert_eq!(all_args.len(), 3);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "--force");
    assert_eq!(all_args[2], "test123");
}

#[test]
fn get_install_crate_args_empty_args() {
    let all_args = get_install_crate_args("test123", false, &Some(vec![]));

    assert_eq!(all_args.len(), 2);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "test123");
}

#[test]
fn get_install_crate_args_empty_args_force() {
    let all_args = get_install_crate_args("test123", true, &Some(vec![]));

    assert_eq!(all_args.len(), 3);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "--force");
    assert_eq!(all_args[2], "test123");
}

#[test]
fn get_install_crate_args_with_args() {
    let all_args = get_install_crate_args(
        "test123",
        false,
        &Some(vec!["arg1".to_string(), "arg2".to_string()]),
    );

    assert_eq!(all_args.len(), 4);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "arg1");
    assert_eq!(all_args[2], "arg2");
    assert_eq!(all_args[3], "test123");
}

#[test]
fn get_install_crate_args_with_args_force() {
    let all_args = get_install_crate_args(
        "test123",
        true,
        &Some(vec!["arg1".to_string(), "arg2".to_string()]),
    );

    assert_eq!(all_args.len(), 5);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "--force");
    assert_eq!(all_args[2], "arg1");
    assert_eq!(all_args[3], "arg2");
    assert_eq!(all_args[4], "test123");
}
