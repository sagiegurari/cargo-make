use super::*;
use log;

#[test]
fn merge_maps_both_empty() {
    let mut map1 = HashMap::<String, String>::new();
    let mut map2 = HashMap::<String, String>::new();

    let output = merge_maps(&mut map1, &mut map2);
    assert_eq!(output.len(), 0);
}

#[test]
fn merge_maps_first_empty() {
    let mut map1 = HashMap::<String, String>::new();
    let mut map2 = HashMap::<String, String>::new();

    map2.insert("test".to_string(), "value".to_string());

    let output = merge_maps(&mut map1, &mut map2);
    assert_eq!(output.len(), 1);
    assert_eq!(output.get("test").unwrap(), &"value".to_string());
}

#[test]
fn merge_maps_second_empty() {
    let mut map1 = HashMap::<String, String>::new();
    let mut map2 = HashMap::<String, String>::new();

    map1.insert("test".to_string(), "value".to_string());

    let output = merge_maps(&mut map1, &mut map2);
    assert_eq!(output.len(), 1);
    assert_eq!(output.get("test").unwrap(), &"value".to_string());
}

#[test]
fn merge_maps_both_with_values() {
    let mut map1 = HashMap::<String, String>::new();
    let mut map2 = HashMap::<String, String>::new();

    map1.insert("test1".to_string(), "value1".to_string());
    map2.insert("test21".to_string(), "value21".to_string());
    map2.insert("test22".to_string(), "value22".to_string());

    let output = merge_maps(&mut map1, &mut map2);
    assert_eq!(output.len(), 3);
    assert_eq!(output.get("test1").unwrap(), &"value1".to_string());
    assert_eq!(output.get("test21").unwrap(), &"value21".to_string());
    assert_eq!(output.get("test22").unwrap(), &"value22".to_string());
}

#[test]
fn merge_tasks_both_empty() {
    let mut map1 = HashMap::<String, Task>::new();
    let mut map2 = HashMap::<String, Task>::new();

    let output = merge_tasks(&mut map1, &mut map2);
    assert_eq!(output.len(), 0);
}

#[test]
fn merge_tasks_first_empty() {
    let mut map1 = HashMap::<String, Task>::new();
    let mut map2 = HashMap::<String, Task>::new();

    let mut task = Task::new();
    task.install_crate = Some("my crate".to_string());
    task.command = Some("test".to_string());

    map2.insert("test".to_string(), task);

    let output = merge_tasks(&mut map1, &mut map2);
    assert_eq!(output.len(), 1);
    let task = output.get("test").unwrap();
    assert!(task.disabled.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.install_crate.is_some());
    assert!(task.install_script.is_none());
    assert!(task.command.is_some());
    assert!(task.args.is_none());
    assert!(task.script.is_none());
    assert!(task.dependencies.is_none());
    assert!(task.linux.is_none());
    assert!(task.windows.is_none());
    assert!(task.mac.is_none());
}

#[test]
fn merge_tasks_second_empty() {
    let mut map1 = HashMap::<String, Task>::new();
    let mut map2 = HashMap::<String, Task>::new();

    let mut task = Task::new();
    task.install_crate = Some("my crate".to_string());
    task.command = Some("test".to_string());

    map1.insert("test".to_string(), task);

    let output = merge_tasks(&mut map1, &mut map2);
    assert_eq!(output.len(), 1);
    let task = output.get("test").unwrap();
    assert!(task.disabled.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.install_crate.is_some());
    assert!(task.install_script.is_none());
    assert!(task.command.is_some());
    assert!(task.args.is_none());
    assert!(task.script.is_none());
    assert!(task.dependencies.is_none());
    assert!(task.linux.is_none());
    assert!(task.windows.is_none());
    assert!(task.mac.is_none());
}

#[test]
fn merge_tasks_both_with_values() {
    let mut map1 = HashMap::<String, Task>::new();
    let mut map2 = HashMap::<String, Task>::new();

    let mut task1 = Task::new();
    task1.install_crate = Some("my crate".to_string());
    task1.command = Some("test".to_string());

    map1.insert("test".to_string(), task1);

    let mut task2 = Task::new();
    task2.command = Some("test".to_string());

    map2.insert("test2".to_string(), task2);

    let output = merge_tasks(&mut map1, &mut map2);
    assert_eq!(output.len(), 2);

    let mut task = output.get("test").unwrap();
    assert!(task.disabled.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.install_crate.is_some());
    assert!(task.install_script.is_none());
    assert!(task.command.is_some());
    assert!(task.args.is_none());
    assert!(task.script.is_none());
    assert!(task.dependencies.is_none());
    assert!(task.linux.is_none());
    assert!(task.windows.is_none());
    assert!(task.mac.is_none());

    task = output.get("test2").unwrap();
    assert!(task.disabled.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.install_crate.is_none());
    assert!(task.install_script.is_none());
    assert!(task.command.is_some());
    assert!(task.args.is_none());
    assert!(task.script.is_none());
    assert!(task.dependencies.is_none());
    assert!(task.linux.is_none());
    assert!(task.windows.is_none());
    assert!(task.mac.is_none());
}

#[test]
fn merge_tasks_extend_task() {
    let mut map1 = HashMap::<String, Task>::new();
    let mut map2 = HashMap::<String, Task>::new();

    let mut task1 = Task::new();
    task1.disabled = Some(false);
    task1.install_crate = Some("my crate".to_string());
    task1.command = Some("test1".to_string());

    map1.insert("test".to_string(), task1);

    let mut task2 = Task::new();
    task2.disabled = Some(true);
    task2.command = Some("test2".to_string());

    map2.insert("test".to_string(), task2);

    let output = merge_tasks(&mut map1, &mut map2);
    assert_eq!(output.len(), 1);

    let task = output.get("test").unwrap();
    assert!(task.disabled.is_some());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.install_crate.is_some());
    assert!(task.install_script.is_none());
    assert!(task.command.is_some());
    assert!(task.args.is_none());
    assert!(task.script.is_none());
    assert!(task.dependencies.is_none());
    assert!(task.linux.is_none());
    assert!(task.windows.is_none());
    assert!(task.mac.is_none());

    let task_clone = task.clone();
    assert!(task_clone.disabled.unwrap());
    assert_eq!(task_clone.install_crate.unwrap(), "my crate");
    assert_eq!(task_clone.command.unwrap(), "test2");
}

#[test]
fn load_external_descriptor_no_file() {
    let logger = log::create("error");
    let config = load_external_descriptor(".", "bad_file.toml2", &logger);

    assert!(config.env.is_none());
    assert!(config.tasks.is_none());
}

#[test]
fn load_external_descriptor_simple_file() {
    let logger = log::create("error");
    let config = load_external_descriptor(".", "./examples/alias.toml", &logger);

    assert!(config.env.is_none());
    assert!(config.tasks.is_some());

    let tasks = config.tasks.unwrap();
    let test_task = tasks.get("D2").unwrap();
    let alias = test_task.alias.clone();
    assert_eq!(alias.unwrap(), "D");
}

#[test]
fn load_external_descriptor_extending_file() {
    let logger = log::create("error");
    let config = load_external_descriptor(".", "examples/extending.toml", &logger);

    assert!(config.env.is_some());
    assert!(config.tasks.is_some());

    assert_eq!(config.env.unwrap().len(), 0);

    let tasks = config.tasks.unwrap();
    let mut test_task = tasks.get("D2").unwrap();
    let mut alias = test_task.alias.clone();
    assert_eq!(alias.unwrap(), "D");

    test_task = tasks.get("extended").unwrap();
    alias = test_task.alias.clone();
    assert_eq!(alias.unwrap(), "D2");
}

#[test]
fn load_external_descriptor_extending_file_sub_folder() {
    let logger = log::create("error");
    let config = load_external_descriptor(".", "examples/files/extending.toml", &logger);

    assert!(config.env.is_some());
    assert!(config.tasks.is_some());

    assert_eq!(config.env.unwrap().len(), 0);

    let tasks = config.tasks.unwrap();
    let mut test_task = tasks.get("D2").unwrap();
    let mut alias = test_task.alias.clone();
    assert_eq!(alias.unwrap(), "D");

    test_task = tasks.get("extended").unwrap();
    alias = test_task.alias.clone();
    assert_eq!(alias.unwrap(), "D2");

    test_task = tasks.get("extended2").unwrap();
    alias = test_task.alias.clone();
    assert_eq!(alias.unwrap(), "extended");
}
