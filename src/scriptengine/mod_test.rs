use super::*;

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
fn invoke_no_runner() {
    let mut task = Task::new();
    task.script = Some(vec!["test".to_string()]);

    let output = invoke(&task);

    assert!(!output);
}

#[test]
fn invoke_no_script() {
    let mut task = Task::new();
    task.script_runner = Some("@rust".to_string());

    let output = invoke(&task);

    assert!(!output);
}

#[test]
fn invoke_unsupported_runner() {
    let mut task = Task::new();
    task.script_runner = Some("@bad".to_string());
    task.script = Some(vec!["test".to_string()]);

    let output = invoke(&task);

    assert!(!output);
}
