use super::*;
use crate::plugin::runner;
use crate::plugin::types::{Plugin, Plugins};
use crate::test::create_empty_flow_info;
use crate::types::{EnvValue, ScriptValue, Task};
use indexmap::IndexMap;

#[test]
fn run_valid() {
    let mut plugins = IndexMap::new();
    plugins.insert(
        "test".to_string(),
        Plugin {
            script: r#"
                value = get_env cm_plugin_run_task_test_valid_env
                assert_eq ${value} ""
                value = get_env cm_plugin_run_task_test_valid_script
                assert_eq ${value} ""

                cm_plugin_run_task

                value = get_env cm_plugin_run_task_test_valid_env
                assert_eq ${value} 1
                value = get_env cm_plugin_run_task_test_valid_script
                assert_eq ${value} yes

                set_env cm_plugin_run_task_test_valid_plugin done
            "#
            .to_string(),
        },
    );

    let mut task = Task::new();
    task.plugin = Some("test".to_string());
    task.script_runner = Some("@duckscript".to_string());
    task.script = Some(ScriptValue::SingleLine(
        r#"
            set_env cm_plugin_run_task_test_valid_script yes
        "#
        .to_string(),
    ));
    let mut env = IndexMap::new();
    env.insert(
        "cm_plugin_run_task_test_valid_env".to_string(),
        EnvValue::Value("1".to_string()),
    );
    task.env = Some(env);

    let mut flow_info = create_empty_flow_info();
    flow_info
        .config
        .tasks
        .insert("test".to_string(), task.clone());
    flow_info.config.plugins = Some(Plugins {
        aliases: None,
        plugins,
    });

    assert!(!envmnt::exists("cm_plugin_run_task_test_valid_env"));
    assert!(!envmnt::exists("cm_plugin_run_task_test_valid_script"));
    assert!(!envmnt::exists("cm_plugin_run_task_test_valid_plugin"));

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
    assert!(envmnt::is_equal("cm_plugin_run_task_test_valid_env", "1"));
    assert!(envmnt::is_equal(
        "cm_plugin_run_task_test_valid_script",
        "yes"
    ));
    assert!(envmnt::is_equal(
        "cm_plugin_run_task_test_valid_plugin",
        "done"
    ));
}
