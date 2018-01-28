use super::*;
use std::path::PathBuf;
use std::env;

#[test]
fn load_from_path_exists() {
    let path = PathBuf::from("examples/.cargo-make");
    let global_config = load_from_path(path);

    assert!(global_config.file_name.is_some());
    assert_eq!(global_config.log_level.unwrap(), "error".to_string());
    assert_eq!(
        global_config.default_task_name.unwrap(),
        "build".to_string()
    );
    assert_eq!(
        global_config.update_check_minimum_interval.unwrap(),
        "daily".to_string()
    );
}

#[test]
fn load_from_path_not_exists() {
    let path = PathBuf::from("examples2/.cargo-make");
    let global_config = load_from_path(path);

    assert!(global_config.file_name.is_none());
    assert!(global_config.log_level.is_none());
    assert!(global_config.default_task_name.is_none());
    assert!(global_config.update_check_minimum_interval.is_none());
}

#[test]
fn load_with_cargo_home() {
    let path = env::current_dir().unwrap();
    let directory = path.join("examples/.cargo-make");
    env::set_var("CARGO_MAKE_HOME", directory.to_str().unwrap());
    let global_config = load();

    assert!(global_config.file_name.is_some());
    assert_eq!(global_config.log_level.unwrap(), "error".to_string());
    assert_eq!(
        global_config.default_task_name.unwrap(),
        "build".to_string()
    );
    assert_eq!(
        global_config.update_check_minimum_interval.unwrap(),
        "daily".to_string()
    );
}

#[test]
fn load_without_cargo_home() {
    env::remove_var("CARGO_MAKE_HOME");
    load();
}
