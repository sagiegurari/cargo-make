use super::*;
use crate::test;
use envmnt;

#[test]
#[should_panic]
fn wrap_command_invalid_toolchain() {
    if test::is_not_rust_stable() {
        wrap_command("invalid-chain", "true", &None);
    }
}

#[test]
fn wrap_command_none_args() {
    let channel = envmnt::get_or_panic("CARGO_MAKE_RUST_CHANNEL");
    let output = wrap_command(&channel, "true", &None);

    assert_eq!(output.command, "rustup".to_string());

    let args = output.args.unwrap();
    assert_eq!(args.len(), 3);
    assert_eq!(args[0], "run".to_string());
    assert_eq!(args[1], channel);
    assert_eq!(args[2], "true".to_string());
}

#[test]
fn wrap_command_empty_args() {
    let channel = envmnt::get_or_panic("CARGO_MAKE_RUST_CHANNEL");
    let output = wrap_command(&channel, "true", &Some(vec![]));

    assert_eq!(output.command, "rustup".to_string());

    let args = output.args.unwrap();
    assert_eq!(args.len(), 3);
    assert_eq!(args[0], "run".to_string());
    assert_eq!(args[1], channel);
    assert_eq!(args[2], "true".to_string());
}

#[test]
fn wrap_command_with_args() {
    let channel = envmnt::get_or_panic("CARGO_MAKE_RUST_CHANNEL");
    let output = wrap_command(
        &channel,
        "true",
        &Some(vec!["echo".to_string(), "test".to_string()]),
    );

    assert_eq!(output.command, "rustup".to_string());

    let args = output.args.unwrap();
    assert_eq!(args.len(), 5);
    assert_eq!(args[0], "run".to_string());
    assert_eq!(args[1], channel);
    assert_eq!(args[2], "true".to_string());
    assert_eq!(args[3], "echo".to_string());
    assert_eq!(args[4], "test".to_string());
}
