use super::*;
use std::path::PathBuf;
use std::env;

#[test]
fn load_from_path_exists() {
    let path = PathBuf::from("examples/.cargo-make");
    let storage_data = load_from_path(path);

    assert!(storage_data.file_name.is_some());
    assert_eq!(storage_data.last_update_check.unwrap(), 1000u64);
}

#[test]
fn load_from_path_not_exists() {
    let path = PathBuf::from("examples2/.cargo-make");
    let storage_data = load_from_path(path);

    assert!(storage_data.file_name.is_none());
    assert!(storage_data.last_update_check.is_none());
}

#[test]
fn load_with_cargo_home() {
    let path = env::current_dir().unwrap();
    let directory = path.join("examples/.cargo-make");
    env::set_var("CARGO_MAKE_HOME", directory.to_str().unwrap());
    let storage_data = load();

    assert!(storage_data.file_name.is_some());
    assert_eq!(storage_data.last_update_check.unwrap(), 1000u64);
}

#[test]
fn load_without_cargo_home() {
    env::remove_var("CARGO_MAKE_HOME");
    load();
}
