use super::*;
use log;
use std::collections::HashMap;
use types::Task;

#[test]
fn set_env_empty() {
    let logger = log::create("error");
    let config = Config { env: HashMap::new(), tasks: HashMap::new() };

    set_env(&logger, &config);
}

#[test]
fn set_env_values() {
    let logger = log::create("error");
    let mut config = Config { env: HashMap::new(), tasks: HashMap::new() };
    config.env.insert("MY_ENV_KEY".to_string(), "MY_ENV_VALUE".to_string());

    assert_eq!(env::var("MY_ENV_KEY").unwrap_or("NONE".to_string()), "NONE".to_string());

    set_env(&logger, &config);

    assert_eq!(env::var("MY_ENV_KEY").unwrap(), "MY_ENV_VALUE");
}

#[test]
#[should_panic]
fn get_task_name_not_found() {
    let logger = log::create("error");
    let config = Config { env: HashMap::new(), tasks: HashMap::new() };

    get_task_name(&logger, &config, "test");
}

#[test]
fn get_task_name_no_alias() {
    let logger = log::create("error");
    let mut config = Config { env: HashMap::new(), tasks: HashMap::new() };

    config.tasks.insert("test".to_string(), Task::new());

    let name = get_task_name(&logger, &config, "test");

    assert_eq!(name, "test");
}

#[test]
fn get_task_name_alias() {
    let logger = log::create("error");
    let mut config = Config { env: HashMap::new(), tasks: HashMap::new() };

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
    let mut config = Config { env: HashMap::new(), tasks: HashMap::new() };

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
