use super::*;

#[test]
fn get_shebang_empty_vec() {
    let output = get_shebang(&vec![]);

    assert!(output.runner.is_none());
}

#[test]
fn get_shebang_not_shebang_line() {
    let output = get_shebang(&vec!["test".to_string()]);

    assert!(output.runner.is_none());
}

#[test]
fn get_shebang_empty_shebang_line() {
    let output = get_shebang(&vec!["#!".to_string()]);

    assert!(output.runner.is_none());
}

#[test]
fn get_shebang_space_shebang_line() {
    let output = get_shebang(&vec!["#!   ".to_string()]);

    assert!(output.runner.is_none());
}

#[test]
fn get_shebang_single_command() {
    let output = get_shebang(&vec!["#! test  ".to_string()]);

    assert!(output.runner.is_some());
    assert_eq!(output.runner.unwrap(), "test");
    assert!(output.arguments.is_none());
}

#[test]
fn get_shebang_command_and_args() {
    let output = get_shebang(&vec!["#! test 1  2   3 ".to_string()]);

    assert!(output.runner.is_some());
    assert_eq!(output.runner.unwrap(), "test");
    assert!(output.arguments.is_some());

    let args = output.arguments.unwrap();
    assert_eq!(args.len(), 3);
    assert_eq!(args[0], "1");
    assert_eq!(args[1], "2");
    assert_eq!(args[2], "3");
}

#[test]
fn get_shebang_second_line() {
    let output = get_shebang(&vec!["test".to_string(), "#! test 1  2   3 ".to_string()]);

    assert!(output.runner.is_none());
}

#[test]
fn get_shebang_command_and_args_multi_line() {
    let output = get_shebang(&vec!["#! test 1  2   3 ".to_string(), "test".to_string()]);

    assert!(output.runner.is_some());
    assert_eq!(output.runner.unwrap(), "test");
    assert!(output.arguments.is_some());

    let args = output.arguments.unwrap();
    assert_eq!(args.len(), 3);
    assert_eq!(args[0], "1");
    assert_eq!(args[1], "2");
    assert_eq!(args[2], "3");
}

#[test]
#[cfg(target_os = "linux")]
fn execute_sh() {
    execute(
        &vec!["#! sh".to_string(), "exit $1".to_string()],
        &None,
        &vec!["0".to_string()],
        true,
    );
}

#[test]
#[should_panic]
#[cfg(target_os = "linux")]
fn execute_sh_error() {
    execute(
        &vec!["#! sh".to_string(), "exit $1".to_string()],
        &None,
        &vec!["1".to_string()],
        true,
    );
}
