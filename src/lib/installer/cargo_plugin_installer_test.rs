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
    let all_args = get_install_crate_args("test123", false, &None, &None);

    assert_eq!(all_args.len(), 2);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "test123");
}

#[test]
fn get_install_crate_args_no_args_force() {
    let all_args = get_install_crate_args("test123", true, &None, &None);

    assert_eq!(all_args.len(), 3);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "--force");
    assert_eq!(all_args[2], "test123");
}

#[test]
fn get_install_crate_args_empty_args() {
    let all_args = get_install_crate_args("test123", false, &Some(vec![]), &None);

    assert_eq!(all_args.len(), 2);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "test123");
}

#[test]
fn get_install_crate_args_empty_args_force() {
    let all_args = get_install_crate_args("test123", true, &Some(vec![]), &None);

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
        &None,
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
        &None,
    );

    assert_eq!(all_args.len(), 5);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "--force");
    assert_eq!(all_args[2], "arg1");
    assert_eq!(all_args[3], "arg2");
    assert_eq!(all_args[4], "test123");
}

#[test]
fn get_install_crate_args_without_crate_name() {
    let all_args = get_install_crate_args(
        "test123",
        false,
        &Some(vec!["--git".to_string(), "arg2".to_string()]),
        &None,
    );

    assert_eq!(all_args.len(), 3);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "--git");
    assert_eq!(all_args[2], "arg2");
}

#[test]
#[ignore]
fn get_install_crate_args_locked() {
    envmnt::set_bool("CARGO_MAKE_CRATE_INSTALLATION_LOCKED", true);
    let all_args = get_install_crate_args(
        "test123",
        false,
        &Some(vec!["arg1".to_string(), "arg2".to_string()]),
        &Some("1.2.3".to_string()),
    );
    envmnt::remove("CARGO_MAKE_CRATE_INSTALLATION_LOCKED");

    assert_eq!(all_args.len(), 7);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "arg1");
    assert_eq!(all_args[2], "arg2");
    assert_eq!(all_args[3], "--locked");
    assert_eq!(all_args[4], "--version");
    assert_eq!(all_args[5], "1.2.3");
    assert_eq!(all_args[6], "test123");
}

#[test]
#[ignore]
fn get_install_crate_args_no_version_locked() {
    envmnt::set_bool("CARGO_MAKE_CRATE_INSTALLATION_LOCKED", true);
    let all_args = get_install_crate_args(
        "test123",
        false,
        &Some(vec!["arg1".to_string(), "arg2".to_string()]),
        &None,
    );
    envmnt::remove("CARGO_MAKE_CRATE_INSTALLATION_LOCKED");

    assert_eq!(all_args.len(), 4);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "arg1");
    assert_eq!(all_args[2], "arg2");
    assert_eq!(all_args[3], "test123");
}

#[test]
#[ignore]
fn get_install_crate_args_without_crate_name_locked() {
    envmnt::set_bool("CARGO_MAKE_CRATE_INSTALLATION_LOCKED", true);
    let all_args = get_install_crate_args(
        "test123",
        false,
        &Some(vec!["--git".to_string(), "arg2".to_string()]),
        &Some("1.2.3".to_string()),
    );
    envmnt::remove("CARGO_MAKE_CRATE_INSTALLATION_LOCKED");

    assert_eq!(all_args.len(), 3);
    assert_eq!(all_args[0], "install");
    assert_eq!(all_args[1], "--git");
    assert_eq!(all_args[2], "arg2");
}

#[test]
fn should_skip_crate_name_none() {
    let output = should_skip_crate_name(&None);

    assert!(!output);
}

#[test]
fn should_skip_crate_name_empty() {
    let output = should_skip_crate_name(&Some(vec![]));

    assert!(!output);
}

#[test]
fn should_skip_crate_name_false() {
    let output = should_skip_crate_name(&Some(vec!["arg1".to_string(), "arg2".to_string()]));

    assert!(!output);
}

#[test]
fn should_skip_crate_name_git() {
    let output = should_skip_crate_name(&Some(vec!["--git".to_string(), "arg2".to_string()]));

    assert!(output);
}

#[test]
fn install_crate_already_installed_test() {
    install_crate(&None, "test", "bad", &None, true, &None);
}

#[test]
fn install_crate_already_installed_cargo_make() {
    install_crate(&None, "make", "cargo-make", &None, true, &None);
}

#[test]
#[ignore]
fn install_crate_already_installed_min_version_equal() {
    if test::is_local_or_travis_ci() {
        let version = crate_version_check::get_crate_version("cargo-make").unwrap();
        let mut version_string = String::new();
        version_string.push_str(&version.major.to_string());
        version_string.push_str(".");
        version_string.push_str(&version.minor.to_string());
        version_string.push_str(".");
        version_string.push_str(&version.patch.to_string());

        install_crate(
            &None,
            "make",
            "cargo-make",
            &None,
            true,
            &Some(version_string),
        );
    }
}

#[test]
#[ignore]
fn install_crate_already_installed_min_version_smaller() {
    if test::is_local_or_travis_ci() {
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

        install_crate(
            &None,
            "make",
            "cargo-make",
            &None,
            true,
            &Some(version_string),
        );
    }
}
