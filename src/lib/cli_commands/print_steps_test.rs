use super::*;
use crate::types::{ConfigSection, Step, Task};
use indexmap::IndexMap;

#[test]
fn get_format_type_default() {
    let output = get_format_type("default");
    assert_eq!(output, PrintFormat::Default);
}

#[test]
fn get_format_type_unknown() {
    let output = get_format_type("test123");
    assert_eq!(output, PrintFormat::Default);
}

#[test]
fn get_format_type_short_description() {
    let output = get_format_type("short-description");
    assert_eq!(output, PrintFormat::ShortDescription);
}

#[test]
fn print_default_format() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());
    config.tasks.insert("test".to_string(), Task::new());

    print(
        &mut std::io::stdout(),
        &config,
        "test",
        "default",
        false,
        &None,
        &CrateInfo::new(),
        false,
    )
    .expect("print should succeed");
}

#[test]
#[should_panic]
fn print_task_not_found() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    print(
        &mut std::io::stdout(),
        &config,
        "test",
        "default",
        false,
        &None,
        &CrateInfo::new(),
        false,
    )
    .expect("print should succeed");
}

#[test]
fn print_task_not_found_but_skipped() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    print(
        &mut std::io::stdout(),
        &config,
        "test",
        "default",
        false,
        &Some("test".to_string()),
        &CrateInfo::new(),
        false,
    )
    .expect("print should succeed");
}

#[test]
fn print_default_valid() {
    let step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };
    let steps = vec![step];
    let execution_plan = ExecutionPlan { steps };

    print_default(&mut std::io::stdout(), &execution_plan).expect("print should succeed");
}

#[test]
fn print_short_description_valid() {
    let step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };
    let steps = vec![step];
    let execution_plan = ExecutionPlan { steps };

    print_short_description(&mut std::io::stdout(), &execution_plan).expect("print should succeed");
}

#[test]
fn print_skip_init_end_tasks() {
    // Use a unique string, so that we are certain it shouldn't appear in the output.
    let init_task_name = "init_5ec3_5b28_7b73_dcee";
    let end_task_name = "end_3afa_4ede_b49a_1767";

    let tasks = IndexMap::from([
        (init_task_name.to_string(), Task::new()),
        (end_task_name.to_string(), Task::new()),
        ("entry".to_string(), Task::new()),
    ]);

    // Test with only init_task enabled.
    let config = Config {
        config: ConfigSection {
            init_task: Some(init_task_name.to_string()),
            end_task: Some(end_task_name.to_string()),
            ..ConfigSection::new()
        },
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };

    let mut output_bytes = Vec::<u8>::new();
    print(
        &mut output_bytes,
        &config,
        "entry",
        "default",
        false,
        &None,
        &CrateInfo::new(),
        true,
    )
    .expect("print should succeed");
    let output = std::str::from_utf8(&output_bytes).expect("output must be valid UTF-8 strings");
    assert!(
        !output.contains(init_task_name),
        "output {} shouldn't contain the init task name {}",
        output,
        init_task_name
    );
    assert!(
        !output.contains(end_task_name),
        "output {} shouldn't contain the end task name {}",
        output,
        end_task_name
    );
}
