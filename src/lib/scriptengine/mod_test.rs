use super::*;
use crate::test;
use crate::types::{FileScriptValue, ScriptSections};

#[test]
fn get_script_text_single_line() {
    let output = get_script_text(&ScriptValue::SingleLine("test".to_string()))
        .unwrap()
        .join("\n");

    assert_eq!(output, "test");
}

#[test]
fn get_script_text_vector() {
    let output = get_script_text(&ScriptValue::Text(vec![
        "line 1".to_string(),
        "line 2".to_string(),
    ]))
    .unwrap()
    .join("\n");

    assert_eq!(output, "line 1\nline 2");
}

#[test]
fn get_script_text_file() {
    let file_info = FileScriptValue {
        file: "src/lib/test/test_files/text_file.txt".to_string(),
        absolute_path: None,
    };
    let output = get_script_text(&ScriptValue::File(file_info))
        .unwrap()
        .join("\n")
        .replace("\r", "");

    assert_eq!(output, "text 1\ntext 2");
}

#[test]
fn get_script_text_file_relative() {
    let file_info = FileScriptValue {
        file: "src/lib/test/test_files/text_file.txt".to_string(),
        absolute_path: Some(false),
    };
    let output = get_script_text(&ScriptValue::File(file_info))
        .unwrap()
        .join("\n")
        .replace("\r", "");

    assert_eq!(output, "text 1\ntext 2");
}

#[test]
fn get_script_text_file_absolute() {
    let file_info = FileScriptValue {
        file: "${CARGO_MAKE_WORKING_DIRECTORY}/src/lib/test/test_files/text_file.txt".to_string(),
        absolute_path: Some(true),
    };
    let output = get_script_text(&ScriptValue::File(file_info))
        .unwrap()
        .join("\n")
        .replace("\r", "");

    assert_eq!(output, "text 1\ntext 2");
}

#[test]
fn get_script_text_script_content_sections() {
    let output = get_script_text(&ScriptValue::Sections(ScriptSections {
        pre: Some("pre".to_string()),
        main: Some("main".to_string()),
        post: Some("post".to_string()),
    }))
    .unwrap()
    .join("\n");

    assert_eq!(output, "pre\nmain\npost");
}

#[test]
fn get_script_text_script_content_sections_empty() {
    let output = get_script_text(&ScriptValue::Sections(ScriptSections {
        pre: None,
        main: None,
        post: None,
    }))
    .unwrap();

    assert!(output.is_empty());
}

#[test]
fn get_engine_type_no_runner() {
    let output =
        get_engine_type(&ScriptValue::Text(vec!["test".to_string()]), &None, &None).unwrap();

    assert_eq!(output, EngineType::OS);
}

#[test]
fn get_engine_type_runner_no_extension() {
    let output = get_engine_type(
        &ScriptValue::Text(vec!["test".to_string()]),
        &Some("@bad".to_string()),
        &None,
    )
    .unwrap();

    assert_eq!(output, EngineType::OS);
}

#[test]
fn get_engine_type_duckscript() {
    let output = get_engine_type(
        &ScriptValue::Text(vec!["test".to_string()]),
        &Some("@duckscript".to_string()),
        &None,
    )
    .unwrap();

    assert_eq!(output, EngineType::Duckscript);
}

#[test]
fn get_engine_type_rust() {
    let output = get_engine_type(
        &ScriptValue::Text(vec!["test".to_string()]),
        &Some("@rust".to_string()),
        &None,
    )
    .unwrap();

    assert_eq!(output, EngineType::Rust);
}

#[test]
fn get_engine_type_shell_to_batch() {
    let output = get_engine_type(
        &ScriptValue::Text(vec!["test".to_string()]),
        &Some("@shell".to_string()),
        &None,
    )
    .unwrap();

    assert_eq!(output, EngineType::Shell2Batch);
}

#[test]
fn get_engine_type_generic() {
    let output = get_engine_type(
        &ScriptValue::Text(vec!["test".to_string()]),
        &Some("test1".to_string()),
        &Some("test2".to_string()),
    )
    .unwrap();

    assert_eq!(output, EngineType::Generic);
}

#[test]
fn get_engine_type_shebang() {
    let output =
        get_engine_type(&ScriptValue::Text(vec!["#!bash".to_string()]), &None, &None).unwrap();

    assert_eq!(output, EngineType::Shebang);
}

#[test]
fn get_engine_type_duckscript_from_shebang() {
    let output = get_engine_type(
        &ScriptValue::Text(vec!["#!@duckscript".to_string()]),
        &None,
        &None,
    )
    .unwrap();

    assert_eq!(output, EngineType::Duckscript);
}

#[test]
fn get_engine_type_shell_to_batch_from_shebang() {
    let output = get_engine_type(
        &ScriptValue::Text(vec!["#!@shell".to_string()]),
        &None,
        &None,
    )
    .unwrap();

    assert_eq!(output, EngineType::Shell2Batch);
}

#[test]
fn get_engine_type_rust_from_shebang() {
    let output = get_engine_type(
        &ScriptValue::Text(vec!["#!@rust".to_string()]),
        &None,
        &None,
    )
    .unwrap();

    assert_eq!(output, EngineType::Rust);
}

#[test]
fn invoke_no_runner() {
    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let output = invoke(
        &task,
        &test::create_empty_flow_info(),
        Rc::new(RefCell::new(FlowState::new())),
    )
    .unwrap();

    assert!(output);
}

#[test]
fn invoke_no_script_no_runner() {
    let task = Task::new();

    let output = invoke(
        &task,
        &test::create_empty_flow_info(),
        Rc::new(RefCell::new(FlowState::new())),
    )
    .unwrap();

    assert!(!output);
}

#[test]
fn invoke_no_script() {
    let mut task = Task::new();
    task.script_runner = Some("@rust".to_string());

    let output = invoke(
        &task,
        &test::create_empty_flow_info(),
        Rc::new(RefCell::new(FlowState::new())),
    )
    .unwrap();

    assert!(!output);
}

#[test]
fn invoke_os_runner() {
    let mut task = Task::new();
    task.script_runner = Some(test::get_os_runner());
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let output = invoke(
        &task,
        &test::create_empty_flow_info(),
        Rc::new(RefCell::new(FlowState::new())),
    )
    .unwrap();

    assert!(output);
}

#[test]
fn invoke_duckscript_runner() {
    if test::should_test(false) {
        let mut task = Task::new();
        task.script_runner = Some("@duckscript".to_string());
        task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

        let output = invoke(
            &task,
            &test::create_empty_flow_info(),
            Rc::new(RefCell::new(FlowState::new())),
        )
        .unwrap();

        assert!(output);
    }
}

#[test]
#[should_panic]
fn invoke_duckscript_runner_error() {
    if test::should_test(true) {
        let mut task = Task::new();
        task.script_runner = Some("@duckscript".to_string());
        task.script = Some(ScriptValue::Text(vec!["function test".to_string()]));

        let output = invoke(
            &task,
            &test::create_empty_flow_info(),
            Rc::new(RefCell::new(FlowState::new())),
        )
        .unwrap();

        assert!(output);
    }
}

#[test]
fn invoke_rust_runner() {
    if test::should_test(false) {
        let mut task = Task::new();
        task.script_runner = Some("@rust".to_string());
        task.script = Some(ScriptValue::Text(vec![
            "fn main() {println!(\"test\");}".to_string()
        ]));

        let output = invoke(
            &task,
            &test::create_empty_flow_info(),
            Rc::new(RefCell::new(FlowState::new())),
        )
        .unwrap();

        assert!(output);
    }
}

#[test]
#[should_panic]
fn invoke_rust_runner_error() {
    if test::should_test(true) {
        let mut task = Task::new();
        task.script_runner = Some("@rust".to_string());
        task.script = Some(ScriptValue::Text(vec![
            "fn main() {bad!(\"test\");}".to_string()
        ]));

        let output = invoke(
            &task,
            &test::create_empty_flow_info(),
            Rc::new(RefCell::new(FlowState::new())),
        )
        .unwrap();

        assert!(output);
    }
}

#[test]
fn invoke_shell_to_batch_runner() {
    let mut task = Task::new();
    task.script_runner = Some("@shell".to_string());
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let output = invoke(
        &task,
        &test::create_empty_flow_info(),
        Rc::new(RefCell::new(FlowState::new())),
    )
    .unwrap();

    assert!(output);
}

#[test]
#[should_panic]
fn invoke_shell_to_batch_runner_error() {
    let mut task = Task::new();
    task.script_runner = Some("@shell".to_string());
    task.script = Some(ScriptValue::Text(vec!["exit 1".to_string()]));

    let output = invoke(
        &task,
        &test::create_empty_flow_info(),
        Rc::new(RefCell::new(FlowState::new())),
    )
    .unwrap();

    assert!(output);
}

#[test]
fn invoke_generic_runner() {
    let mut task = Task::new();
    task.script_runner = Some(test::get_os_runner());
    task.script_extension = Some(test::get_os_extension());
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let output = invoke(
        &task,
        &test::create_empty_flow_info(),
        Rc::new(RefCell::new(FlowState::new())),
    )
    .unwrap();

    assert!(output);
}

#[test]
#[should_panic]
fn invoke_generic_runner_error() {
    let mut task = Task::new();
    task.script_runner = Some(test::get_os_runner());
    task.script_extension = Some(test::get_os_extension());
    task.script = Some(ScriptValue::Text(vec!["exit 1".to_string()]));

    let output = invoke(
        &task,
        &test::create_empty_flow_info(),
        Rc::new(RefCell::new(FlowState::new())),
    )
    .unwrap();

    assert!(output);
}

#[test]
fn invoke_shell_to_batch_runner_with_env_expansion() {
    envmnt::set("invoke_shell_to_batch_runner_with_env_expansion", "@shell");
    let mut task = Task::new();
    task.script_runner = Some("${invoke_shell_to_batch_runner_with_env_expansion}".to_string());
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let output = invoke(
        &task,
        &test::create_empty_flow_info(),
        Rc::new(RefCell::new(FlowState::new())),
    )
    .unwrap();

    assert!(output);
}

#[test]
#[should_panic]
fn invoke_shell_to_batch_runner_with_env_expansion_no_env() {
    let mut task = Task::new();
    task.script_runner =
        Some("${invoke_shell_to_batch_runner_with_env_expansion_no_env}".to_string());
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let output = invoke(
        &task,
        &test::create_empty_flow_info(),
        Rc::new(RefCell::new(FlowState::new())),
    )
    .unwrap();

    assert!(output);
}
