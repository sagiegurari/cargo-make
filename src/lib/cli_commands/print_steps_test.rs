use super::*;
use crate::types::{ConfigSection, ExecutionPlan, Step, Task};
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

    print(&config, "test", "default", false, None);
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

    print(&config, "test", "default", false, None);
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

    print(&config, "test", "default", false, Some("test".to_string()));
}

#[test]
fn print_default_valid() {
    let step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };
    let steps = vec![step];
    let execution_plan = ExecutionPlan { steps };

    print_default(&execution_plan);
}

#[test]
fn print_short_description_valid() {
    let step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };
    let steps = vec![step];
    let execution_plan = ExecutionPlan { steps };

    print_short_description(&execution_plan);
}
