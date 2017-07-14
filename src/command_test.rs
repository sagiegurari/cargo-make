use super::*;
use log;
use std::env::current_dir;
use std::io::ErrorKind;
use types::Task;

#[test]
fn create_script_no_shebang() {
    let logger = log::create("error");

    let script_lines = vec!["echo test".to_string()];
    let cwd = current_dir().unwrap();
    let mut expected_script = "".to_string();
    if !cfg!(windows) {
        expected_script.push_str("set -e\n");
    }
    expected_script.push_str("cd ");
    expected_script.push_str(cwd.to_str().unwrap());
    expected_script.push_str("\necho test\n\n");

    let script = create_script(&logger, &script_lines);

    assert_eq!(script, expected_script);
}

#[test]
fn create_script_with_shebang() {
    let logger = log::create("error");

    let script_lines = vec!["#!/bin/bash".to_string(), "echo test".to_string()];
    let cwd = current_dir().unwrap();
    let mut expected_script = "#!/bin/bash\n".to_string();
    if !cfg!(windows) {
        expected_script.push_str("set -e\n");
    }
    expected_script.push_str("cd ");
    expected_script.push_str(cwd.to_str().unwrap());
    expected_script.push_str("\necho test\n\n");

    let script = create_script(&logger, &script_lines);

    assert_eq!(script, expected_script);
}

#[test]
#[should_panic]
fn validate_exit_code_error() {
    let logger = log::create("error");
    validate_exit_code(Err(Error::new(ErrorKind::Other, "test")), &logger);
}

#[test]
fn run_no_command() {
    let logger = log::create("error");
    let task = Task::new();

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
fn run_command() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.command = Some("echo".to_string());

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
#[should_panic]
fn run_command_error() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.command = Some("badbadbad".to_string());

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
fn run_command_error_force() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.force = Some(true);
    task.command = Some("badbadbad".to_string());

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
fn run_script() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.script = Some(vec!["echo 1".to_string()]);

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
#[should_panic]
fn run_script_error() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.script = Some(vec!["exit 1".to_string()]);

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
fn run_script_error_force() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.force = Some(true);
    task.script = Some(vec!["exit 1".to_string()]);

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}

#[test]
#[cfg(target_os = "linux")]
fn run_script_custom_runner() {
    let logger = log::create("error");
    let mut task = Task::new();
    task.script = Some(vec!["echo test".to_string()]);
    task.script_runner = Some("bash".to_string());

    let step = Step { name: "test".to_string(), config: task };

    run(&logger, &step);
}
