use super::*;
use std::collections::HashMap;

#[test]
fn load_from_file_no_file() {
    let tasks = load_from_file("./badfile.toml");

    assert!(tasks.is_empty());
}

#[test]
fn load_from_file_parse_error() {
    let tasks = load_from_file("./src/lib/test/cargo/invalid_config.toml");

    assert!(tasks.is_empty());
}

#[test]
fn load_from_file_no_alias_data() {
    let tasks = load_from_file("./Cargo.toml");

    assert!(tasks.is_empty());
}

#[test]
fn load_from_file_aliases_found() {
    let tasks = load_from_file("./src/lib/test/cargo/config.toml");

    assert_eq!(tasks.len(), 4);

    let mut map = HashMap::new();
    for pair in &tasks {
        map.insert(pair.0.clone(), pair.1.clone());
    }

    let mut task = map.get("b2").unwrap();
    assert_eq!(task.args.clone().unwrap(), vec!["b2"]);
    task = map.get("c2").unwrap();
    assert_eq!(task.args.clone().unwrap(), vec!["c2"]);
    task = map.get("t2").unwrap();
    assert_eq!(task.args.clone().unwrap(), vec!["t2"]);
    task = map.get("test_specific").unwrap();
    assert_eq!(task.args.clone().unwrap(), vec!["test_specific"]);
}
