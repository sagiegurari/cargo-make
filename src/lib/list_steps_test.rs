use super::*;

use crate::types::{ConfigSection, EnvValue, Task};
use indexmap::IndexMap;

#[test]
fn run_empty() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();
    let tasks = IndexMap::<String, Task>::new();
    let config = Config {
        config: config_section,
        env,
        tasks,
    };

    let count = run(&config, "default");

    assert_eq!(count, 0);
}

#[test]
fn run_all_public() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    tasks.insert("1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    tasks.insert("2".to_string(), task2);

    let config = Config {
        config: config_section,
        env,
        tasks,
    };

    let count = run(&config, "default");

    assert_eq!(count, 2);
}

#[test]
fn run_all_public_markdown() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    tasks.insert("1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    tasks.insert("2".to_string(), task2);

    let config = Config {
        config: config_section,
        env,
        tasks,
    };

    let count = run(&config, "markdown");

    assert_eq!(count, 2);
}

#[test]
fn run_all_private() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    task1.private = Some(true);
    tasks.insert("1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    task2.private = Some(true);
    tasks.insert("2".to_string(), task2);

    let config = Config {
        config: config_section,
        env,
        tasks,
    };

    let count = run(&config, "default");

    assert_eq!(count, 0);
}

#[test]
fn run_mixed() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    task1.private = Some(true);
    tasks.insert("1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    tasks.insert("2".to_string(), task2);

    let config = Config {
        config: config_section,
        env,
        tasks,
    };

    let count = run(&config, "default");

    assert_eq!(count, 1);
}
