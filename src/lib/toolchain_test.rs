use super::*;

#[test]
fn wrap_command_none_args() {
    let output = wrap_command("mychain", "testcommand", &None);

    assert_eq!(output.command, "rustup".to_string());

    let args = output.args.unwrap();
    assert_eq!(args.len(), 3);
    assert_eq!(args[0], "run".to_string());
    assert_eq!(args[1], "mychain".to_string());
    assert_eq!(args[2], "testcommand".to_string());
}

#[test]
fn wrap_command_empty_args() {
    let output = wrap_command("mychain", "testcommand", &Some(vec![]));

    assert_eq!(output.command, "rustup".to_string());

    let args = output.args.unwrap();
    assert_eq!(args.len(), 3);
    assert_eq!(args[0], "run".to_string());
    assert_eq!(args[1], "mychain".to_string());
    assert_eq!(args[2], "testcommand".to_string());
}

#[test]
fn wrap_command_with_args() {
    let output = wrap_command(
        "mychain",
        "testcommand",
        &Some(vec!["echo".to_string(), "test".to_string()]),
    );

    assert_eq!(output.command, "rustup".to_string());

    let args = output.args.unwrap();
    assert_eq!(args.len(), 5);
    assert_eq!(args[0], "run".to_string());
    assert_eq!(args[1], "mychain".to_string());
    assert_eq!(args[2], "testcommand".to_string());
    assert_eq!(args[3], "echo".to_string());
    assert_eq!(args[4], "test".to_string());
}
