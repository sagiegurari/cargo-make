use super::*;
use crate::test;
use std::env;

#[test]
fn get_engine_type_no_runner() {
    let mut task = Task::new();
    task.script = Some(vec!["test".to_string()]);

    let output = get_engine_type(&task);

    assert_eq!(output, EngineType::Unsupported);
}

#[test]
fn get_engine_type_no_script() {
    let mut task = Task::new();
    task.script_runner = Some("@rust".to_string());

    let output = get_engine_type(&task);

    assert_eq!(output, EngineType::Unsupported);
}

#[test]
fn get_engine_type_unsupported_runner() {
    let mut task = Task::new();
    task.script_runner = Some("@bad".to_string());
    task.script = Some(vec!["test".to_string()]);

    let output = get_engine_type(&task);

    assert_eq!(output, EngineType::Unsupported);
}

#[test]
fn get_engine_type_rust() {
    let mut task = Task::new();
    task.script_runner = Some("@rust".to_string());
    task.script = Some(vec!["test".to_string()]);

    let output = get_engine_type(&task);

    assert_eq!(output, EngineType::Rust);
}

#[test]
fn get_engine_type_shell_to_batch() {
    let mut task = Task::new();
    task.script_runner = Some("@shell".to_string());
    task.script = Some(vec!["test".to_string()]);

    let output = get_engine_type(&task);

    assert_eq!(output, EngineType::Shell2Batch);
}

#[test]
fn get_engine_type_generic() {
    let mut task = Task::new();
    task.script_runner = Some("test1".to_string());
    task.script_extension = Some("test2".to_string());
    task.script = Some(vec!["test".to_string()]);

    let output = get_engine_type(&task);

    assert_eq!(output, EngineType::Generic);
}

#[test]
fn invoke_no_runner() {
    let mut task = Task::new();
    task.script = Some(vec!["test".to_string()]);

    let output = invoke("test", &task, &vec![]);

    assert!(!output);
}

#[test]
fn invoke_no_script() {
    let mut task = Task::new();
    task.script_runner = Some("@rust".to_string());

    let output = invoke("test", &task, &vec![]);

    assert!(!output);
}

#[test]
fn invoke_unsupported_runner() {
    let mut task = Task::new();
    task.script_runner = Some("@bad".to_string());
    task.script = Some(vec!["test".to_string()]);

    let output = invoke("test", &task, &vec![]);

    assert!(!output);
}

#[test]
fn invoke_rust_runner() {
    if test::should_test(false) {
        env::set_var(
            "CARGO_MAKE_TASK_TEST_RUST_SCRIPT_OUTPUT_ENV_OUTPUT",
            "EMPTY",
        );
        assert_eq!(
            env::var("CARGO_MAKE_TASK_TEST_RUST_SCRIPT_OUTPUT_ENV_OUTPUT").unwrap(),
            "EMPTY"
        );

        let mut task = Task::new();
        task.script_runner = Some("@rust".to_string());
        task.script = Some(vec!["fn main() {println!(\"test\");}".to_string()]);

        let output = invoke("test_rust_script_output_env", &task, &vec![]);

        assert!(output);

        assert!(env::var("CARGO_MAKE_TASK_TEST_RUST_SCRIPT_OUTPUT_ENV_OUTPUT").unwrap() != "EMPTY");
        assert_eq!(
            env::var("CARGO_MAKE_TASK_TEST_RUST_SCRIPT_OUTPUT_ENV_OUTPUT").unwrap(),
            env::var("CARGO_MAKE_TASK_OUTPUT_PREV").unwrap()
        );
    }
}

#[test]
#[should_panic]
fn invoke_rust_runner_error() {
    if test::should_test(true) {
        let mut task = Task::new();
        task.script_runner = Some("@rust".to_string());
        task.script = Some(vec!["fn main() {bad!(\"test\");}".to_string()]);

        let output = invoke("test", &task, &vec![]);

        assert!(output);
    }
}

#[test]
fn invoke_shell_to_batch_runner() {
    env::set_var(
        "CARGO_MAKE_TASK_TEST_SHELL2BATCH_SCRIPT_OUTPUT_ENV_OUTPUT",
        "EMPTY",
    );
    assert_eq!(
        env::var("CARGO_MAKE_TASK_TEST_SHELL2BATCH_SCRIPT_OUTPUT_ENV_OUTPUT").unwrap(),
        "EMPTY"
    );

    let mut task = Task::new();
    task.script_runner = Some("@shell".to_string());
    task.script = Some(vec!["echo test".to_string()]);

    let output = invoke("test_shell2batch_script_output_env", &task, &vec![]);

    assert!(output);

    assert!(
        env::var("CARGO_MAKE_TASK_TEST_SHELL2BATCH_SCRIPT_OUTPUT_ENV_OUTPUT").unwrap() != "EMPTY"
    );
    assert_eq!(
        env::var("CARGO_MAKE_TASK_TEST_SHELL2BATCH_SCRIPT_OUTPUT_ENV_OUTPUT").unwrap(),
        env::var("CARGO_MAKE_TASK_OUTPUT_PREV").unwrap()
    );
}

#[test]
#[should_panic]
fn invoke_shell_to_batch_runner_error() {
    let mut task = Task::new();
    task.script_runner = Some("@shell".to_string());
    task.script = Some(vec!["exit 1".to_string()]);

    let output = invoke("test", &task, &vec![]);

    assert!(output);
}

#[test]
fn invoke_generic_runner() {
    env::set_var(
        "CARGO_MAKE_TASK_TEST_GENERIC_SCRIPT_OUTPUT_ENV_OUTPUT",
        "EMPTY",
    );
    assert_eq!(
        env::var("CARGO_MAKE_TASK_TEST_GENERIC_SCRIPT_OUTPUT_ENV_OUTPUT").unwrap(),
        "EMPTY"
    );

    let mut task = Task::new();
    task.script_runner = Some(test::get_os_runner());
    task.script_extension = Some(test::get_os_extension());
    task.script = Some(vec!["echo test".to_string()]);

    let output = invoke("test_generic_script_output_env", &task, &vec![]);

    assert!(output);

    assert!(env::var("CARGO_MAKE_TASK_TEST_GENERIC_SCRIPT_OUTPUT_ENV_OUTPUT").unwrap() != "EMPTY");
    assert_eq!(
        env::var("CARGO_MAKE_TASK_TEST_GENERIC_SCRIPT_OUTPUT_ENV_OUTPUT").unwrap(),
        env::var("CARGO_MAKE_TASK_OUTPUT_PREV").unwrap()
    );
}

#[test]
#[should_panic]
fn invoke_generic_runner_error() {
    let mut task = Task::new();
    task.script_runner = Some(test::get_os_runner());
    task.script_extension = Some(test::get_os_extension());
    task.script = Some(vec!["exit 1".to_string()]);

    let output = invoke("test", &task, &vec![]);

    assert!(output);
}
