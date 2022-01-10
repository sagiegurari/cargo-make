use super::*;
use crate::plugin::runner;
use crate::plugin::types::{Plugin, Plugins};
use crate::test::create_empty_flow_info;
use crate::types::{FlowState, RunTaskOptions, Task, TaskCondition};
use indexmap::IndexMap;
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn run_valid() {
    let mut plugins = IndexMap::new();
    plugins.insert(
        "test".to_string(),
        Plugin {
            script: r#"
                value = get_env cm_plugin_check_task_condition_test_valid_env
                assert_eq ${value} ""
                valid = cm_plugin_check_task_condition
                assert_false "${valid}"

                set_env cm_plugin_check_task_condition_test_valid_env 1
                valid = cm_plugin_check_task_condition
                assert "${valid}"

                set_env cm_plugin_check_task_condition_test_valid_plugin done
            "#
            .to_string(),
        },
    );

    let mut task = Task::new();
    task.plugin = Some("test".to_string());
    task.condition = Some(TaskCondition {
        fail_message: None,
        profiles: None,
        platforms: None,
        channels: None,
        env_set: Some(vec![
            "cm_plugin_check_task_condition_test_valid_env".to_string()
        ]),
        env_not_set: None,
        env_true: None,
        env_false: None,
        env: None,
        env_contains: None,
        rust_version: None,
        files_exist: None,
        files_not_exist: None,
    });

    let mut flow_info = create_empty_flow_info();
    flow_info
        .config
        .tasks
        .insert("test".to_string(), task.clone());
    flow_info.config.plugins = Some(Plugins {
        aliases: None,
        plugins,
    });

    assert!(!envmnt::exists(
        "cm_plugin_check_task_condition_test_valid_plugin"
    ));

    let done = runner::run_task(
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
    assert!(envmnt::is_equal(
        "cm_plugin_check_task_condition_test_valid_plugin",
        "done"
    ));
}
