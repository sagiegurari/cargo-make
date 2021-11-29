use super::*;

use crate::io;
use crate::types::{ConfigSection, EnvValue, Task};
use indexmap::IndexMap;
use std::path::PathBuf;

#[test]
fn run_empty() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();
    let tasks = IndexMap::<String, Task>::new();
    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
    };

    let count = run(&config, "default", &None, None);

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
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
    };

    let count = run(&config, "default", &None, None);

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
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
    };

    let count = run(&config, "markdown", &None, None);

    assert_eq!(count, 2);
}

#[test]
fn run_all_public_markdown_sub_section() {
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
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
    };

    let count = run(&config, "markdown-sub-section", &None, None);

    assert_eq!(count, 2);
}

#[test]
fn run_all_public_markdown_single_page() {
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
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
    };

    let count = run(&config, "markdown-single-page", &None, None);

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
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
    };

    let count = run(&config, "default", &None, None);

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
    let mut task3 = Task::new();
    task3.description = Some("3".to_string());
    task3.deprecated = Some(DeprecationInfo::Boolean(true));
    tasks.insert("3".to_string(), task3);
    let mut task4 = Task::new();
    task4.description = Some("4".to_string());
    task4.deprecated = Some(DeprecationInfo::Message("test".to_string()));
    tasks.insert("4".to_string(), task4);

    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
    };

    let count = run(&config, "default", &None, None);

    assert_eq!(count, 3);
}

#[test]
fn run_write_to_file() {
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
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
    };

    let file = "./target/_temp/tasklist.md";
    let count = run(
        &config,
        "markdown-single-page",
        &Some(file.to_string()),
        None,
    );

    assert_eq!(count, 2);

    let mut path = PathBuf::new();
    path.push(&file);

    let text = io::read_text_file(&path);
    io::delete_file(&file);

    assert!(text.contains("# Task List"));
}

#[test]
fn run_category_public() {
    let config_section = ConfigSection::new();
    let env = IndexMap::<String, EnvValue>::new();

    let mut tasks = IndexMap::<String, Task>::new();
    let mut task1 = Task::new();
    task1.description = Some("1".to_string());
    task1.category = Some("TestCategory1".to_string());
    tasks.insert("1".to_string(), task1);
    let mut task2 = Task::new();
    task2.description = Some("2".to_string());
    task2.category = Some("TestCategory1".to_string());
    tasks.insert("2".to_string(), task2);
    let mut task3 = Task::new();
    task3.description = Some("3".to_string());
    task3.category = Some("TestCategory2".to_string());
    tasks.insert("3".to_string(), task3);

    let config = Config {
        config: config_section,
        env_files: vec![],
        env,
        env_scripts: vec![],
        tasks,
    };

    let count = run(&config, "default", &None, Some("TestCategory1".to_owned()));

    assert_eq!(count, 2);
}
