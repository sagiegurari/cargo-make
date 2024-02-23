use super::*;
use crate::test;
use crate::types::Task;

#[test]
#[should_panic]
fn validate_exit_code_unable_to_fetch() {
    validate_exit_code(-1);
}

#[test]
#[should_panic]
fn validate_exit_code_not_zero() {
    validate_exit_code(1);
}

#[test]
fn validate_exit_code_zero() {
    validate_exit_code(0);
}

#[test]
#[should_panic]
fn get_exit_code_error() {
    test::on_test_startup();

    get_exit_code(Err(Error::new(ErrorKind::Other, "test")), false);
}

#[test]
fn is_silent_for_level_error() {
    let silent = is_silent_for_level("error".to_string());
    assert!(silent);
}

#[test]
fn is_silent_for_level_info() {
    let silent = is_silent_for_level("info".to_string());
    assert!(!silent);
}

#[test]
fn is_silent_for_level_verbose() {
    let silent = is_silent_for_level("verbose".to_string());
    assert!(!silent);
}

#[test]
fn is_silent_for_level_other() {
    let silent = is_silent_for_level("test".to_string());
    assert!(!silent);
}

#[test]
fn should_print_commands_for_level_error() {
    let print_commands = should_print_commands_for_level("error".to_string());
    assert!(!print_commands)
}

#[test]
fn should_print_commands_for_level_info() {
    let print_commands = should_print_commands_for_level("info".to_string());
    assert!(!print_commands)
}

#[test]
fn should_print_commands_for_level_verbose() {
    let print_commands = should_print_commands_for_level("verbose".to_string());
    assert!(print_commands);
}

#[test]
fn should_print_commands_for_level_other() {
    let print_commands = should_print_commands_for_level("test".to_string());
    assert!(!print_commands)
}

#[test]
fn run_no_command() {
    let task = Task::new();

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step);
}

#[test]
fn run_command() {
    let mut task = Task::new();
    task.command = Some("echo".to_string());
    task.args = Some(vec!["test".to_string()]);

    let step = Step {
        name: "test_command_output_env".to_string(),
        config: task,
    };

    run(&step);
}

#[test]
fn run_command_for_toolchain() {
    if test::is_not_rust_stable() {
        let toolchain = test::get_toolchain();

        let mut task = Task::new();
        task.command = Some("echo".to_string());
        task.args = Some(vec!["test".to_string()]);
        task.toolchain = Some(toolchain.into());

        let step = Step {
            name: "test".to_string(),
            config: task,
        };

        run(&step);
    }
}

#[test]
#[should_panic]
fn run_command_error() {
    test::on_test_startup();

    let mut task = Task::new();
    task.command = Some("badbadbad".to_string());

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step);
}

#[test]
fn run_command_error_ignore_errors() {
    let mut task = Task::new();
    task.ignore_errors = Some(true);
    task.command = Some("badbadbad".to_string());

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step);
}

#[test]
fn run_script_get_exit_code_valid() {
    run_script_get_exit_code(&vec!["echo 1".to_string()], None, &vec![], true);
}

#[test]
#[should_panic]
fn run_script_get_exit_code_error() {
    run_script_get_exit_code(&vec!["exit 1".to_string()], None, &vec![], true);
}

#[test]
fn run_script_get_exit_code_error_force() {
    run_script_get_exit_code(&vec!["exit 1".to_string()], None, &vec![], false);
}

#[test]
#[cfg(target_os = "linux")]
fn run_script_get_exit_code_custom_runner() {
    run_script_get_exit_code(
        &vec!["echo test".to_string()],
        Some("bash".to_string()),
        &vec![],
        true,
    );
}

#[test]
#[cfg(target_os = "linux")]
fn run_script_get_exit_code_cli_args_valid() {
    run_script_get_exit_code(
        &vec!["exit $1".to_string()],
        None,
        &vec!["0".to_string()],
        true,
    );
}

#[test]
#[should_panic]
#[cfg(target_os = "linux")]
fn run_script_get_exit_code_cli_args_error() {
    run_script_get_exit_code(
        &vec!["exit $1".to_string()],
        None,
        &vec!["1".to_string()],
        true,
    );
}
