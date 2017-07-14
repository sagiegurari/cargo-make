use super::*;
use log;
use std::collections::HashMap;
use types::{ConfigSection, PlatformOverrideTask, Task};

#[test]
#[should_panic]
fn get_task_name_not_found() {
    let logger = log::create("error");
    let config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };

    get_task_name(&logger, &config, "test");
}

#[test]
fn get_task_name_no_alias() {
    let logger = log::create("error");
    let mut config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };

    config.tasks.insert("test".to_string(), Task::new());

    let name = get_task_name(&logger, &config, "test");

    assert_eq!(name, "test");
}

#[test]
fn get_task_name_alias() {
    let logger = log::create("error");
    let mut config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };

    let mut task = Task::new();
    task.alias = Some("test2".to_string());
    config.tasks.insert("test".to_string(), task);

    config.tasks.insert("test2".to_string(), Task::new());

    let name = get_task_name(&logger, &config, "test");

    assert_eq!(name, "test2");
}

#[test]
fn get_task_name_platform_alias() {
    let logger = log::create("error");
    let mut config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };

    let mut task = Task::new();
    if cfg!(windows) {
        task.windows_alias = Some("test2".to_string());
    } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        task.mac_alias = Some("test2".to_string());
    } else {
        task.linux_alias = Some("test2".to_string());
    };

    config.tasks.insert("test".to_string(), task);

    config.tasks.insert("test2".to_string(), Task::new());

    let name = get_task_name(&logger, &config, "test");

    assert_eq!(name, "test2");
}

#[test]
fn create_execution_plan_single() {
    let logger = log::create("error");
    let mut config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };

    let task = Task::new();

    config.tasks.insert("test".to_string(), task);

    let execution_plan = create_execution_plan(&logger, &config, "test");
    assert_eq!(execution_plan.steps.len(), 1);
}

#[test]
fn create_execution_plan_single_disabled() {
    let logger = log::create("error");
    let mut config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };

    let mut task = Task::new();
    task.disabled = Some(true);

    config.tasks.insert("test".to_string(), task);

    let execution_plan = create_execution_plan(&logger, &config, "test");
    assert_eq!(execution_plan.steps.len(), 0);
}

#[test]
fn create_execution_plan_platform_disabled() {
    let logger = log::create("error");
    let mut config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };

    let mut task = Task::new();
    task.linux = Some(PlatformOverrideTask {
        clear: Some(true),
        disabled: Some(true),
        install_crate: None,
        command: None,
        force: None,
        install_script: None,
        args: None,
        script: None,
        script_runner: None,
        dependencies: None
    });
    task.windows = Some(PlatformOverrideTask {
        clear: Some(true),
        disabled: Some(true),
        install_crate: None,
        command: None,
        force: None,
        install_script: None,
        args: None,
        script: None,
        script_runner: None,
        dependencies: None
    });
    task.mac = Some(PlatformOverrideTask {
        clear: Some(true),
        disabled: Some(true),
        install_crate: None,
        command: None,
        force: None,
        install_script: None,
        args: None,
        script: None,
        script_runner: None,
        dependencies: None
    });

    config.tasks.insert("test".to_string(), task);

    let execution_plan = create_execution_plan(&logger, &config, "test");
    assert_eq!(execution_plan.steps.len(), 0);
}
