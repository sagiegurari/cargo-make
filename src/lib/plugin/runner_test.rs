use super::*;
use crate::plugin::types::Plugins;
use crate::test::create_empty_flow_info;
use crate::types::{ConfigSection, Task};

#[test]
fn get_plugin_name_recursive_empty() {
    let aliases = IndexMap::new();

    let output = get_plugin_name_recursive(&aliases, "test", &mut vec![]);

    assert_eq!(output, "test");
}

#[test]
fn get_plugin_name_recursive_not_found() {
    let mut aliases = IndexMap::new();
    aliases.insert("a".to_string(), "b".to_string());

    let output = get_plugin_name_recursive(&aliases, "test", &mut vec![]);

    assert_eq!(output, "test");
}

#[test]
fn get_plugin_name_recursive_found() {
    let mut aliases = IndexMap::new();
    aliases.insert("test".to_string(), "test1".to_string());
    aliases.insert("test1".to_string(), "test2".to_string());
    aliases.insert("test2".to_string(), "test3".to_string());

    let output = get_plugin_name_recursive(&aliases, "test", &mut vec![]);

    assert_eq!(output, "test3");
}

#[test]
#[should_panic]
fn get_plugin_name_recursive_endless_loop() {
    let mut aliases = IndexMap::new();
    aliases.insert("test".to_string(), "test1".to_string());
    aliases.insert("test1".to_string(), "test2".to_string());
    aliases.insert("test2".to_string(), "test1".to_string());

    let output = get_plugin_name_recursive(&aliases, "test", &mut vec![]);

    assert_eq!(output, "test3");
}

#[test]
fn get_plugin_no_plugins_config() {
    let output = get_plugin(
        &Config {
            config: ConfigSection::new(),
            env_files: vec![],
            env: IndexMap::new(),
            env_scripts: vec![],
            tasks: IndexMap::new(),
            plugins: None,
        },
        "test",
    );

    assert!(output.is_none());
}

#[test]
fn get_plugin_not_found() {
    let output = get_plugin(
        &Config {
            config: ConfigSection::new(),
            env_files: vec![],
            env: IndexMap::new(),
            env_scripts: vec![],
            tasks: IndexMap::new(),
            plugins: Some(Plugins {
                aliases: None,
                plugins: IndexMap::new(),
            }),
        },
        "test",
    );

    assert!(output.is_none());
}

#[test]
fn get_plugin_found() {
    let mut plugins = IndexMap::new();
    plugins.insert(
        "test".to_string(),
        Plugin {
            script: "test script".to_string(),
        },
    );

    let output = get_plugin(
        &Config {
            config: ConfigSection::new(),
            env_files: vec![],
            env: IndexMap::new(),
            env_scripts: vec![],
            tasks: IndexMap::new(),
            plugins: Some(Plugins {
                aliases: None,
                plugins,
            }),
        },
        "test",
    );

    assert!(output.is_some());
    let (name, plugin) = output.unwrap();
    assert_eq!(name, "test");
    assert_eq!(plugin.script, "test script");
}

#[test]
fn get_plugin_found_with_alias() {
    let mut plugins = IndexMap::new();
    plugins.insert(
        "test2".to_string(),
        Plugin {
            script: "test script".to_string(),
        },
    );
    let mut aliases = IndexMap::new();
    aliases.insert("test".to_string(), "test2".to_string());

    let output = get_plugin(
        &Config {
            config: ConfigSection::new(),
            env_files: vec![],
            env: IndexMap::new(),
            env_scripts: vec![],
            tasks: IndexMap::new(),
            plugins: Some(Plugins {
                aliases: Some(aliases),
                plugins,
            }),
        },
        "test",
    );

    assert!(output.is_some());
    let (name, plugin) = output.unwrap();
    assert_eq!(name, "test2");
    assert_eq!(plugin.script, "test script");
}

#[test]
fn run_task_with_plugin_disabled() {
    let mut plugins = IndexMap::new();
    plugins.insert(
        "test2".to_string(),
        Plugin {
            script: "test script".to_string(),
        },
    );
    let mut aliases = IndexMap::new();
    aliases.insert("test_plugin".to_string(), "test2".to_string());

    let mut task = Task::new();
    task.plugin = Some("test_plugin".to_string());

    let mut flow_info = create_empty_flow_info();
    flow_info
        .config
        .tasks
        .insert("test".to_string(), task.clone());
    flow_info.config.plugins = Some(Plugins {
        aliases: Some(aliases),
        plugins,
    });

    let done = run_task(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &Step {
            name: "test".to_string(),
            config: task,
        },
        &RunTaskOptions {
            plugins_enabled: false,
        },
    );

    assert!(!done);
}

#[test]
#[should_panic]
fn run_task_with_no_plugin_config() {
    let mut task = Task::new();
    task.plugin = Some("test_plugin".to_string());

    let mut flow_info = create_empty_flow_info();
    flow_info
        .config
        .tasks
        .insert("test".to_string(), task.clone());

    let done = run_task(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &Step {
            name: "test".to_string(),
            config: task,
        },
        &RunTaskOptions {
            plugins_enabled: true,
        },
    );

    assert!(!done);
}

#[test]
fn run_task_no_plugin_value() {
    let mut plugins = IndexMap::new();
    plugins.insert(
        "test2".to_string(),
        Plugin {
            script: "test script".to_string(),
        },
    );
    let mut aliases = IndexMap::new();
    aliases.insert("test_plugin".to_string(), "test2".to_string());

    let task = Task::new();

    let mut flow_info = create_empty_flow_info();
    flow_info
        .config
        .tasks
        .insert("test".to_string(), task.clone());
    flow_info.config.plugins = Some(Plugins {
        aliases: Some(aliases),
        plugins,
    });

    let done = run_task(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &Step {
            name: "test".to_string(),
            config: task,
        },
        &RunTaskOptions {
            plugins_enabled: true,
        },
    );

    assert!(!done);
}

#[test]
#[should_panic]
fn run_task_with_plugin_not_found() {
    let mut plugins = IndexMap::new();
    plugins.insert(
        "test2".to_string(),
        Plugin {
            script: "test script".to_string(),
        },
    );
    let mut aliases = IndexMap::new();
    aliases.insert("test_plugin".to_string(), "test2".to_string());

    let mut task = Task::new();
    task.plugin = Some("not_found".to_string());

    let mut flow_info = create_empty_flow_info();
    flow_info
        .config
        .tasks
        .insert("test".to_string(), task.clone());
    flow_info.config.plugins = Some(Plugins {
        aliases: Some(aliases),
        plugins,
    });

    run_task(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &Step {
            name: "test".to_string(),
            config: task,
        },
        &RunTaskOptions {
            plugins_enabled: true,
        },
    );
}

#[test]
#[ignore]
#[should_panic]
fn run_task_invoked_with_error() {
    let mut plugins = IndexMap::new();
    plugins.insert(
        "test2".to_string(),
        Plugin {
            script: "assert_eq ${task.plugin.name} wrongvalue".to_string(),
        },
    );
    let mut aliases = IndexMap::new();
    aliases.insert("test_plugin".to_string(), "test2".to_string());

    let mut task = Task::new();
    task.plugin = Some("test_plugin".to_string());

    let mut flow_info = create_empty_flow_info();
    flow_info
        .config
        .tasks
        .insert("test".to_string(), task.clone());
    flow_info.config.plugins = Some(Plugins {
        aliases: Some(aliases),
        plugins,
    });

    run_task(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &Step {
            name: "test".to_string(),
            config: task,
        },
        &RunTaskOptions {
            plugins_enabled: true,
        },
    );
}

#[test]
#[ignore]
fn run_task_invoked() {
    let mut plugins = IndexMap::new();
    plugins.insert(
        "test2".to_string(),
        Plugin {
            script: r#"
                assert_eq ${flow.task.name} test
                assert_eq ${task.name} test
                assert_eq ${task.plugin.name} test_plugin
                assert_eq ${plugin.impl.name} test2
                set_env PLUGIN_RUNNER_RUN_TASK_INVOKED done"#
                .to_string(),
        },
    );
    let mut aliases = IndexMap::new();
    aliases.insert("test_plugin".to_string(), "test2".to_string());

    let mut task = Task::new();
    task.plugin = Some("test_plugin".to_string());

    let mut flow_info = create_empty_flow_info();
    flow_info
        .config
        .tasks
        .insert("test".to_string(), task.clone());
    flow_info.config.plugins = Some(Plugins {
        aliases: Some(aliases),
        plugins,
    });

    assert!(!envmnt::is_equal("PLUGIN_RUNNER_RUN_TASK_INVOKED", "done"));

    let done = run_task(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &Step {
            name: "test".to_string(),
            config: task,
        },
        &RunTaskOptions {
            plugins_enabled: true,
        },
    );

    assert!(done);
    assert!(envmnt::is_equal("PLUGIN_RUNNER_RUN_TASK_INVOKED", "done"));
}
