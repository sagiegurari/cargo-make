use super::*;
use std::path::PathBuf;

#[test]
fn load_from_path_exists() {
    let path = PathBuf::from("examples");
    let global_config = load_from_path(path);

    assert_eq!(global_config.log_level.unwrap(), "error".to_string());
    assert_eq!(
        global_config.default_task_name.unwrap(),
        "build".to_string()
    );
}

#[test]
fn load_from_path_not_exists() {
    let path = PathBuf::from("examples2");
    let global_config = load_from_path(path);

    assert!(global_config.log_level.is_none());
    assert!(global_config.default_task_name.is_none());
}

#[test]
fn load_test() {
    load();
}
