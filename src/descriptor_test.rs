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

    map2.insert(
        "test".to_string(),
        Task {
            disabled: None,
            alias: None,
            linux_alias: None,
            windows_alias: None,
            mac_alias: None,
            install_crate: Some("my crate".to_string()),
            install_script: None,
            command: Some("test".to_string()),
            args: None,
            script: None,
            dependencies: None,
            linux: None,
            windows: None,
            mac: None
        }
    );

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

    map1.insert(
        "test".to_string(),
        Task {
            disabled: None,
            alias: None,
            linux_alias: None,
            windows_alias: None,
            mac_alias: None,
            install_crate: Some("my crate".to_string()),
            install_script: None,
            command: Some("test".to_string()),
            args: None,
            script: None,
            dependencies: None,
            linux: None,
            windows: None,
            mac: None
        }
    );

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

    map1.insert(
        "test".to_string(),
        Task {
            disabled: None,
            alias: None,
            linux_alias: None,
            windows_alias: None,
            mac_alias: None,
            install_crate: Some("my crate".to_string()),
            install_script: None,
            command: Some("test".to_string()),
            args: None,
            script: None,
            dependencies: None,
            linux: None,
            windows: None,
            mac: None
        }
    );

    map2.insert(
        "test2".to_string(),
        Task {
            disabled: None,
            alias: None,
            linux_alias: None,
            windows_alias: None,
            mac_alias: None,
            install_crate: None,
            install_script: None,
            command: Some("test".to_string()),
            args: None,
            script: None,
            dependencies: None,
            linux: None,
            windows: None,
            mac: None
        }
    );

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

    map1.insert(
        "test".to_string(),
        Task {
            disabled: None,
            alias: None,
            linux_alias: None,
            windows_alias: None,
            mac_alias: None,
            install_crate: Some("my crate".to_string()),
            install_script: None,
            command: Some("test1".to_string()),
            args: None,
            script: None,
            dependencies: None,
            linux: None,
            windows: None,
            mac: None
        }
    );

    map2.insert(
        "test".to_string(),
        Task {
            disabled: Some(true),
            alias: None,
            linux_alias: None,
            windows_alias: None,
            mac_alias: None,
            install_crate: None,
            install_script: None,
            command: Some("test2".to_string()),
            args: None,
            script: None,
            dependencies: None,
            linux: None,
            windows: None,
            mac: None
        }
    );

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
    let config = load_external_descriptor("bad_file.toml2", &logger);

    assert!(config.env.is_none());
    assert!(config.tasks.is_none());
}
