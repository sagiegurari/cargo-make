use super::*;
use std::io::ErrorKind;
use types::Task;

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
