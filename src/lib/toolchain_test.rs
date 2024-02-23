use super::*;
use crate::types::ToolchainBoundedSpecifier;

fn get_test_env_toolchain() -> ToolchainSpecifier {
    let channel = envmnt::get_or_panic("CARGO_MAKE_RUST_CHANNEL");
    let version = envmnt::get_or_panic("CARGO_MAKE_RUST_VERSION");

    ToolchainSpecifier::Bounded(ToolchainBoundedSpecifier {
        channel,
        min_version: version,
    })
}

#[test]
#[should_panic]
fn wrap_command_invalid_toolchain() {
    wrap_command(&"invalid-chain".into(), "true", &None);
}

#[test]
#[should_panic]
fn wrap_command_unreachable_version() {
    let toolchain = ToolchainSpecifier::Bounded(ToolchainBoundedSpecifier {
        channel: envmnt::get_or_panic("CARGO_MAKE_RUST_CHANNEL"),
        min_version: "9999.9.9".to_string(), // If we ever reach this version, add another 9
    });
    wrap_command(&toolchain, "true", &None);
}

#[test]
fn wrap_command_empty_toolchain() {
    let output = wrap_command(&"".into(), "mycommand", &Some(vec!["arg1".to_string()]));

    assert_eq!(output.command, "mycommand".to_string());

    let args = output.args.unwrap();
    assert_eq!(args.len(), 1);
    assert_eq!(args[0], "arg1".to_string());
}

#[test]
fn wrap_command_none_args() {
    let toolchain = get_test_env_toolchain();
    let output = wrap_command(&toolchain, "true", &None);

    assert_eq!(output.command, "rustup".to_string());

    let args = output.args.unwrap();
    assert_eq!(args.len(), 3);
    assert_eq!(args[0], "run".to_string());
    assert_eq!(args[1], toolchain.channel());
    assert_eq!(args[2], "true".to_string());
}

#[test]
fn wrap_command_empty_args() {
    let toolchain = get_test_env_toolchain();
    let output = wrap_command(&toolchain, "true", &Some(vec![]));

    assert_eq!(output.command, "rustup".to_string());

    let args = output.args.unwrap();
    assert_eq!(args.len(), 3);
    assert_eq!(args[0], "run".to_string());
    assert_eq!(args[1], toolchain.channel());
    assert_eq!(args[2], "true".to_string());
}

#[test]
fn wrap_command_with_args() {
    let toolchain = get_test_env_toolchain();
    let output = wrap_command(
        &toolchain,
        "true",
        &Some(vec!["echo".to_string(), "test".to_string()]),
    );

    assert_eq!(output.command, "rustup".to_string());

    let args = output.args.unwrap();
    assert_eq!(args.len(), 5);
    assert_eq!(args[0], "run".to_string());
    assert_eq!(args[1], toolchain.channel());
    assert_eq!(args[2], "true".to_string());
    assert_eq!(args[3], "echo".to_string());
    assert_eq!(args[4], "test".to_string());
}

#[test]
fn wrap_command_with_args_and_simple_variable_toolchain() {
    let env_toolchain = get_test_env_toolchain();
    let toolchain = ToolchainSpecifier::Bounded(ToolchainBoundedSpecifier {
        channel: "${CARGO_MAKE_RUST_CHANNEL}".to_string(),
        min_version: "${CARGO_MAKE_RUST_VERSION}".to_string(),
    });
    let output = wrap_command(
        &toolchain,
        "true",
        &Some(vec!["echo".to_string(), "test".to_string()]),
    );

    assert_eq!(output.command, "rustup".to_string());

    let args = output.args.unwrap();
    assert_eq!(args.len(), 5);
    assert_eq!(args[0], "run".to_string());
    assert_eq!(args[1], env_toolchain.channel());
    assert_eq!(args[2], "true".to_string());
    assert_eq!(args[3], "echo".to_string());
    assert_eq!(args[4], "test".to_string());
}

#[test]
fn get_cargo_binary_path_valid() {
    let toolchain = get_test_env_toolchain();
    let binary_path = get_cargo_binary_path(&toolchain);

    assert!(binary_path.is_some());
    let binary_path_value = binary_path.unwrap();
    assert!(binary_path_value.find("bin").is_some());
    assert!(binary_path_value.find("cargo").is_some());
}

#[test]
#[ignore]
fn remove_rust_env_vars_existed() {
    envmnt::set("RUSTC", "bad1");
    envmnt::set("RUSTDOC", "bad2");
    envmnt::set("RUSTFLAGS", "bad3");

    assert!(envmnt::exists("RUSTC"));
    assert!(envmnt::exists("RUSTDOC"));
    assert!(envmnt::exists("RUSTFLAGS"));

    remove_rust_env_vars();

    assert!(!envmnt::exists("RUSTC"));
    assert!(!envmnt::exists("RUSTDOC"));
    assert!(!envmnt::exists("RUSTFLAGS"));
}
