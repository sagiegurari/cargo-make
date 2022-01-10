use super::*;
use crate::plugin::types::Plugins;
use crate::test::create_empty_flow_info;
use crate::types::{
    ConfigSection, EnvFile, EnvValue, InstallCrate, RunTaskInfo, ScriptValue, Task, TaskCondition,
    TaskWatchOptions,
};

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
fn run_task_invoked_valid() {
    let mut plugins = IndexMap::new();
    plugins.insert(
        "test2".to_string(),
        Plugin {
            script: r#"
                assert_eq ${flow.task.name} test
                arg = array_get ${flow.cli.args} 0
                assert_eq ${arg} arg1
                arg = array_get ${flow.cli.args} 1
                assert_eq ${arg} arg2

                assert_eq ${plugin.impl.name} test2

                found = is_defined task.as_json
                assert ${found}
                assert ${task.has_condition}
                assert ${task.has_env}
                assert ${task.has_install_instructions}
                assert ${task.has_command}
                assert ${task.has_script}
                assert ${task.has_run_task}
                assert ${task.has_dependencies}
                assert ${task.has_toolchain_specifier}

                assert_eq ${task.name} test
                assert_eq ${task.description} description
                assert_eq ${task.category} category
                assert ${task.disabled}
                assert ${task.private}
                assert ${task.deprecated}
                assert ${task.workspace}
                assert_eq ${task.plugin.name} test_plugin
                assert ${task.watch}
                assert ${task.ignore_errors}
                assert_eq ${task.cwd} cwd
                assert_eq ${task.command} test_command
                arg = array_get ${task.args} 0
                assert_eq ${arg} a1
                arg = array_get ${task.args} 1
                assert_eq ${arg} a2
                assert_eq ${task.script_runner} sh2
                arg = array_get ${task.script_runner_args} 0
                assert_eq ${arg} sr_a1
                arg = array_get ${task.script_runner_args} 1
                assert_eq ${arg} sr_a2
                assert_eq ${task.script_extension} ext2

                set_env PLUGIN_RUNNER_RUN_TASK_INVOKED done
            "#
            .to_string(),
        },
    );
    plugins.insert(
        "empty".to_string(),
        Plugin {
            script: r#"
                assert_eq ${flow.task.name} test
                empty = array_is_empty ${flow.cli.args}
                assert "${empty}"

                assert_eq ${plugin.impl.name} empty

                found = is_defined task.as_json
                assert ${found}
                assert_false ${task.has_condition}
                assert_false ${task.has_env}
                assert_false ${task.has_install_instructions}
                assert_false ${task.has_command}
                assert_false ${task.has_script}
                assert_false ${task.has_run_task}
                assert_false ${task.has_dependencies}
                assert_false ${task.has_toolchain_specifier}

                assert_eq ${task.name} test
                assert_eq ${task.description} ""
                assert_eq ${task.category} ""
                assert_false ${task.disabled}
                assert_false ${task.private}
                assert_false ${task.deprecated}
                assert_false ${task.workspace}
                assert_eq ${task.plugin.name} empty
                assert_false ${task.watch}
                assert_false ${task.ignore_errors}
                assert_eq ${task.cwd} ""
                assert_eq ${task.command} ""
                empty = array_is_empty ${task.args}
                assert "${empty}"
                assert_eq ${task.script_runner} ""
                empty = array_is_empty ${task.script_runner_args}
                assert "${empty}"
                assert_eq ${task.script_extension} ""

                set_env PLUGIN_RUNNER_RUN_TASK_INVOKED2 done
            "#
            .to_string(),
        },
    );
    let mut aliases = IndexMap::new();
    aliases.insert("test_plugin".to_string(), "test2".to_string());

    let mut env = IndexMap::new();
    env.insert("test".to_string(), EnvValue::Value("value".to_string()));
    let mut task = Task {
        clear: Some(false),
        install_crate: Some(InstallCrate::Value("my crate2".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("test_command".to_string()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(true),
        plugin: Some("test_plugin".to_string()),
        disabled: Some(true),
        private: Some(true),
        deprecated: Some(DeprecationInfo::Boolean(true)),
        extend: Some("extended".to_string()),
        watch: Some(TaskWatchOptions::Boolean(true)),
        condition: Some(TaskCondition {
            fail_message: None,
            profiles: Some(vec!["development".to_string()]),
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        ignore_errors: Some(true),
        force: Some(true),
        env_files: Some(vec![EnvFile::Path("extended".to_string())]),
        env: Some(env.clone()),
        cwd: Some("cwd".to_string()),
        alias: Some("alias2".to_string()),
        linux_alias: Some("linux".to_string()),
        windows_alias: Some("windows".to_string()),
        mac_alias: Some("mac".to_string()),
        install_script: Some(ScriptValue::Text(vec!["i1".to_string(), "i2".to_string()])),
        args: Some(vec!["a1".to_string(), "a2".to_string()]),
        script: Some(ScriptValue::Text(vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
        ])),
        script_runner: Some("sh2".to_string()),
        script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
        script_extension: Some("ext2".to_string()),
        run_task: Some(RunTaskInfo::Name("task2".to_string())),
        dependencies: Some(vec!["A".into()]),
        toolchain: Some("toolchain".into()),
        linux: None,
        windows: None,
        mac: None,
    };

    let mut flow_info = create_empty_flow_info();
    flow_info.cli_arguments = Some(vec!["arg1".to_string(), "arg2".to_string()]);
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

    task = Task::new();
    task.plugin = Some("empty".to_string());

    flow_info.cli_arguments = None;
    flow_info
        .config
        .tasks
        .insert("test".to_string(), task.clone());

    assert!(!envmnt::is_equal("PLUGIN_RUNNER_RUN_TASK_INVOKED2", "done"));

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
    assert!(envmnt::is_equal("PLUGIN_RUNNER_RUN_TASK_INVOKED2", "done"));
}
