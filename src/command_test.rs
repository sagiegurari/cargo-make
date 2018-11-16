use super::*;
use crate::test;
use crate::types::Task;
use std::io::ErrorKind;

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
fn is_silent_for_level_debug() {
    let silent = is_silent_for_level("debug".to_string());
    assert!(!silent);
}

#[test]
fn is_silent_for_level_other() {
    let silent = is_silent_for_level("test".to_string());
    assert!(!silent);
}

#[test]
fn run_no_command() {
    let task = Task::new();

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step, &vec![]);
}

#[test]
fn run_command() {
    let mut task = Task::new();
    task.command = Some("echo".to_string());
    task.args = Some(vec!["test".to_string()]);

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step, &vec![]);
}

#[test]
fn run_command_for_toolchain() {
    let toolchain = test::get_toolchain();

    let mut task = Task::new();
    task.command = Some("echo".to_string());
    task.args = Some(vec!["test".to_string()]);
    task.toolchain = Some(toolchain.to_string());

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step, &vec![]);
}

#[test]
#[should_panic]
fn run_command_error() {
    let mut task = Task::new();
    task.command = Some("badbadbad".to_string());

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step, &vec![]);
}

#[test]
fn run_command_error_force() {
    let mut task = Task::new();
    task.force = Some(true);
    task.command = Some("badbadbad".to_string());

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step, &vec![]);
}

#[test]
fn run_script() {
    let mut task = Task::new();
    task.script = Some(vec!["echo 1".to_string()]);

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step, &vec![]);
}

#[test]
#[should_panic]
fn run_script_error() {
    let mut task = Task::new();
    task.script = Some(vec!["exit 1".to_string()]);

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step, &vec![]);
}

#[test]
fn run_script_error_force() {
    let mut task = Task::new();
    task.force = Some(true);
    task.script = Some(vec!["exit 1".to_string()]);

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step, &vec![]);
}

#[test]
#[cfg(target_os = "linux")]
fn run_script_custom_runner() {
    let mut task = Task::new();
    task.script = Some(vec!["echo test".to_string()]);
    task.script_runner = Some("bash".to_string());

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step, &vec![]);
}

#[test]
#[cfg(target_os = "linux")]
fn run_script_cli_args_valid() {
    let mut task = Task::new();
    task.script = Some(vec!["exit $1".to_string()]);

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step, &vec!["0".to_string()]);
}

#[test]
#[should_panic]
#[cfg(target_os = "linux")]
fn run_script_cli_args_error() {
    let mut task = Task::new();
    task.script = Some(vec!["exit $1".to_string()]);

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run(&step, &vec!["1".to_string()]);
}
