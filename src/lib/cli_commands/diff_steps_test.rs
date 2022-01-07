use super::*;
use crate::types::{ConfigSection, Task};
use indexmap::IndexMap;

#[test]
fn run_same() {
    let mut config1 = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config1.tasks.insert("init".to_string(), Task::new());
    config1.tasks.insert("end".to_string(), Task::new());
    config1.tasks.insert("test".to_string(), Task::new());

    let config2 = config1.clone();

    run(&config1, &config2, "test", &CliArgs::new());
}

#[test]
fn run_different() {
    let mut config1 = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let mut task = Task::new();
    task.description = Some("test".to_string());

    config1.tasks.insert("init".to_string(), Task::new());
    config1.tasks.insert("end".to_string(), Task::new());
    config1.tasks.insert("test".to_string(), task);

    let mut config2 = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config2.tasks.insert("init".to_string(), Task::new());
    config2.tasks.insert("end".to_string(), Task::new());
    config2.tasks.insert("test".to_string(), Task::new());

    run(&config1, &config2, "test", &CliArgs::new());
}

#[test]
fn run_different_with_skip() {
    let mut config1 = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let mut task = Task::new();
    task.description = Some("test".to_string());

    config1.tasks.insert("init".to_string(), Task::new());
    config1.tasks.insert("end".to_string(), Task::new());
    config1.tasks.insert("test".to_string(), task);

    let mut config2 = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config2.tasks.insert("init".to_string(), Task::new());
    config2.tasks.insert("end".to_string(), Task::new());
    config2.tasks.insert("test".to_string(), Task::new());

    let mut cli_args = CliArgs::new();
    cli_args.skip_tasks_pattern = Some("test".to_string());

    run(&config1, &config2, "test", &cli_args);
}

#[test]
#[should_panic]
fn run_missing_task_in_first_config() {
    let mut config1 = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config1.tasks.insert("init".to_string(), Task::new());
    config1.tasks.insert("end".to_string(), Task::new());

    let mut config2 = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config2.tasks.insert("init".to_string(), Task::new());
    config2.tasks.insert("end".to_string(), Task::new());
    config2.tasks.insert("test".to_string(), Task::new());

    run(&config1, &config2, "test", &CliArgs::new());
}

#[test]
#[should_panic]
fn run_missing_task_in_second_config() {
    let mut config1 = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config1.tasks.insert("init".to_string(), Task::new());
    config1.tasks.insert("end".to_string(), Task::new());
    config1.tasks.insert("test".to_string(), Task::new());

    let mut config2 = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config2.tasks.insert("init".to_string(), Task::new());
    config2.tasks.insert("end".to_string(), Task::new());

    run(&config1, &config2, "test", &CliArgs::new());
}
