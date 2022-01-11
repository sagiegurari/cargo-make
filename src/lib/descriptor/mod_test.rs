use super::*;

use crate::environment;
use crate::environment::setup_cwd;
use crate::types::{ExtendOptions, InstallCrate, ScriptValue};

#[test]
fn merge_env_both_empty() {
    let mut map1 = IndexMap::<String, EnvValue>::new();
    let mut map2 = IndexMap::<String, EnvValue>::new();

    let output = merge_env(&mut map1, &mut map2);
    assert_eq!(output.len(), 0);
}

#[test]
fn merge_env_first_empty() {
    let mut map1 = IndexMap::<String, EnvValue>::new();
    let mut map2 = IndexMap::<String, EnvValue>::new();

    map2.insert("test".to_string(), EnvValue::Value("value".to_string()));

    let output = merge_env(&mut map1, &mut map2);
    assert_eq!(output.len(), 1);
    let value = output.get("test").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value".to_string()),
        _ => panic!("wrong value type"),
    };
}

#[test]
fn merge_env_second_empty() {
    let mut map1 = IndexMap::<String, EnvValue>::new();
    let mut map2 = IndexMap::<String, EnvValue>::new();

    map1.insert("test".to_string(), EnvValue::Value("value".to_string()));

    let output = merge_env(&mut map1, &mut map2);
    assert_eq!(output.len(), 1);
    let value = output.get("test").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value".to_string()),
        _ => panic!("wrong value type"),
    };
}

#[test]
fn merge_env_both_with_values() {
    let mut map1 = IndexMap::<String, EnvValue>::new();
    let mut map2 = IndexMap::<String, EnvValue>::new();

    map1.insert("test1".to_string(), EnvValue::Value("value1".to_string()));
    map1.insert("test21".to_string(), EnvValue::Value("value11".to_string()));
    map2.insert("test21".to_string(), EnvValue::Value("value21".to_string()));
    map2.insert("test22".to_string(), EnvValue::Value("value22".to_string()));

    let output = merge_env(&mut map1, &mut map2);
    assert_eq!(output.len(), 3);
    let mut value = output.get("test1").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value1".to_string()),
        _ => panic!("wrong value type"),
    };
    value = output.get("test21").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value21".to_string()),
        _ => panic!("wrong value type"),
    };
    value = output.get("test22").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value22".to_string()),
        _ => panic!("wrong value type"),
    };
}

#[test]
fn merge_env_both_with_sub_envs() {
    let mut map1 = IndexMap::<String, EnvValue>::new();
    let mut map2 = IndexMap::<String, EnvValue>::new();

    map1.insert("test1".to_string(), EnvValue::Value("value1".to_string()));
    map1.insert("test21".to_string(), EnvValue::Value("value11".to_string()));
    map2.insert("test21".to_string(), EnvValue::Value("value21".to_string()));
    map2.insert("test22".to_string(), EnvValue::Value("value22".to_string()));

    let mut base_profile = IndexMap::<String, EnvValue>::new();
    let mut extended_profile = IndexMap::<String, EnvValue>::new();
    let mut extended_profile2 = IndexMap::<String, EnvValue>::new();

    base_profile.insert("base1".to_string(), EnvValue::Value("base1".to_string()));
    base_profile.insert("base2".to_string(), EnvValue::Value("base2".to_string()));
    extended_profile.insert(
        "base1".to_string(),
        EnvValue::Value("extended1".to_string()),
    );
    extended_profile.insert(
        "extended2".to_string(),
        EnvValue::Value("extended2".to_string()),
    );

    extended_profile2.insert("test".to_string(), EnvValue::Value("test1".to_string()));

    map1.insert("myprofile".to_string(), EnvValue::Profile(base_profile));
    map2.insert("myprofile".to_string(), EnvValue::Profile(extended_profile));
    map2.insert(
        "myprofile2".to_string(),
        EnvValue::Profile(extended_profile2),
    );

    let output = merge_env(&mut map1, &mut map2);
    assert_eq!(output.len(), 5);
    let mut value = output.get("test1").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value1".to_string()),
        _ => panic!("wrong value type"),
    };
    value = output.get("test21").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value21".to_string()),
        _ => panic!("wrong value type"),
    };
    value = output.get("test22").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value22".to_string()),
        _ => panic!("wrong value type"),
    };
    let mut sub_env_type = output.get("myprofile").unwrap();
    match sub_env_type {
        EnvValue::Profile(sub_env) => {
            assert_eq!(sub_env.len(), 3);
            value = sub_env.get("base1").unwrap();
            match value {
                &EnvValue::Value(ref value_string) => {
                    assert_eq!(value_string, &"extended1".to_string())
                }
                _ => panic!("wrong value type"),
            };
            value = sub_env.get("base2").unwrap();
            match value {
                &EnvValue::Value(ref value_string) => {
                    assert_eq!(value_string, &"base2".to_string())
                }
                _ => panic!("wrong value type"),
            };
            value = sub_env.get("extended2").unwrap();
            match value {
                &EnvValue::Value(ref value_string) => {
                    assert_eq!(value_string, &"extended2".to_string())
                }
                _ => panic!("wrong value type"),
            };
        }
        _ => panic!("wrong value type"),
    }
    sub_env_type = output.get("myprofile2").unwrap();
    match sub_env_type {
        EnvValue::Profile(sub_env) => {
            assert_eq!(sub_env.len(), 1);
            value = sub_env.get("test").unwrap();
            match value {
                &EnvValue::Value(ref value_string) => {
                    assert_eq!(value_string, &"test1".to_string())
                }
                _ => panic!("wrong value type"),
            };
        }
        _ => panic!("wrong value type"),
    }
}

#[test]
fn merge_env_both_skip_current_task_env() {
    let mut map1 = IndexMap::<String, EnvValue>::new();
    let mut map2 = IndexMap::<String, EnvValue>::new();

    map1.insert("test1".to_string(), EnvValue::Value("value1".to_string()));
    map1.insert(
        "CARGO_MAKE_CURRENT_TASK_TEST".to_string(),
        EnvValue::Value("test1".to_string()),
    );
    map2.insert("test1".to_string(), EnvValue::Value("value2".to_string()));
    map2.insert(
        "CARGO_MAKE_CURRENT_TASK_TEST".to_string(),
        EnvValue::Value("test2".to_string()),
    );

    let output = merge_env(&mut map1, &mut map2);
    assert_eq!(output.len(), 2);
    let mut value = output.get("test1").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"value2".to_string()),
        _ => panic!("wrong value type"),
    };
    value = output.get("CARGO_MAKE_CURRENT_TASK_TEST").unwrap();
    match value {
        &EnvValue::Value(ref value_string) => assert_eq!(value_string, &"test1".to_string()),
        _ => panic!("wrong value type"),
    };
}

#[test]
fn merge_tasks_both_empty() {
    let mut map1 = IndexMap::<String, Task>::new();
    let mut map2 = IndexMap::<String, Task>::new();

    let output = merge_tasks(&mut map1, &mut map2, false);
    assert_eq!(output.len(), 0);
}

#[test]
fn merge_tasks_first_empty() {
    let mut map1 = IndexMap::<String, Task>::new();
    let mut map2 = IndexMap::<String, Task>::new();

    let mut task = Task::new();
    task.install_crate = Some(InstallCrate::Value("my crate".to_string()));
    task.command = Some("test".to_string());

    map2.insert("test".to_string(), task);

    let output = merge_tasks(&mut map1, &mut map2, false);
    assert_eq!(output.len(), 1);
    let task = output.get("test").unwrap();
    assert!(task.disabled.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.install_crate.is_some());
    assert!(task.install_script.is_none());
    assert!(task.command.is_some());
    assert!(task.args.is_none());
    assert!(task.script.is_none());
    assert!(task.dependencies.is_none());
    assert!(task.linux.is_none());
    assert!(task.windows.is_none());
    assert!(task.mac.is_none());
}

#[test]
fn merge_tasks_second_empty() {
    let mut map1 = IndexMap::<String, Task>::new();
    let mut map2 = IndexMap::<String, Task>::new();

    let mut task = Task::new();
    task.install_crate = Some(InstallCrate::Value("my crate".to_string()));
    task.command = Some("test".to_string());

    map1.insert("test".to_string(), task);

    let output = merge_tasks(&mut map1, &mut map2, false);
    assert_eq!(output.len(), 1);
    let task = output.get("test").unwrap();
    assert!(task.disabled.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.install_crate.is_some());
    assert!(task.install_script.is_none());
    assert!(task.command.is_some());
    assert!(task.args.is_none());
    assert!(task.script.is_none());
    assert!(task.dependencies.is_none());
    assert!(task.linux.is_none());
    assert!(task.windows.is_none());
    assert!(task.mac.is_none());
}

#[test]
fn merge_tasks_both_with_values() {
    let mut map1 = IndexMap::<String, Task>::new();
    let mut map2 = IndexMap::<String, Task>::new();

    let mut task1 = Task::new();
    task1.install_crate = Some(InstallCrate::Value("my crate".to_string()));
    task1.command = Some("test".to_string());

    map1.insert("test".to_string(), task1);

    let mut task2 = Task::new();
    task2.command = Some("test".to_string());

    map2.insert("test2".to_string(), task2);

    let output = merge_tasks(&mut map1, &mut map2, false);
    assert_eq!(output.len(), 2);

    let mut task = output.get("test").unwrap();
    assert!(task.disabled.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.install_crate.is_some());
    assert!(task.install_script.is_none());
    assert!(task.command.is_some());
    assert!(task.args.is_none());
    assert!(task.script.is_none());
    assert!(task.dependencies.is_none());
    assert!(task.linux.is_none());
    assert!(task.windows.is_none());
    assert!(task.mac.is_none());

    task = output.get("test2").unwrap();
    assert!(task.disabled.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.install_crate.is_none());
    assert!(task.install_script.is_none());
    assert!(task.command.is_some());
    assert!(task.args.is_none());
    assert!(task.script.is_none());
    assert!(task.dependencies.is_none());
    assert!(task.linux.is_none());
    assert!(task.windows.is_none());
    assert!(task.mac.is_none());
}

#[test]
fn merge_tasks_extend_task() {
    let mut map1 = IndexMap::<String, Task>::new();
    let mut map2 = IndexMap::<String, Task>::new();

    let mut task1 = Task::new();
    task1.disabled = Some(false);
    task1.install_crate = Some(InstallCrate::Value("my crate".to_string()));
    task1.command = Some("test1".to_string());

    map1.insert("test".to_string(), task1);

    let mut task2 = Task::new();
    task2.disabled = Some(true);
    task2.command = Some("test2".to_string());

    map2.insert("test".to_string(), task2);

    let output = merge_tasks(&mut map1, &mut map2, false);
    assert_eq!(output.len(), 1);

    let task = output.get("test").unwrap();
    assert!(task.disabled.is_some());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.install_crate.is_some());
    assert!(task.install_script.is_none());
    assert!(task.command.is_some());
    assert!(task.args.is_none());
    assert!(task.script.is_none());
    assert!(task.dependencies.is_none());
    assert!(task.linux.is_none());
    assert!(task.windows.is_none());
    assert!(task.mac.is_none());

    let task_clone = task.clone();
    assert!(task_clone.disabled.unwrap());
    assert_eq!(
        task_clone.install_crate.unwrap(),
        InstallCrate::Value("my crate".to_string())
    );
    assert_eq!(task_clone.command.unwrap(), "test2");
}

#[test]
#[ignore]
fn load_descriptors_load_workspace_makefile() {
    envmnt::set(
        "CARGO_MAKE_WORKSPACE_MAKEFILE",
        "./examples/workspace/Makefile.toml",
    );
    let config = load_descriptors("./bad/bad.toml", false, None, false, false, None).unwrap();
    envmnt::remove("CARGO_MAKE_WORKSPACE_MAKEFILE");

    let task = config.tasks.get("workspace-echo");
    assert!(task.is_some());
}

#[test]
#[ignore]
fn load_descriptors_load_workspace_makefile_no_exists() {
    envmnt::set(
        "CARGO_MAKE_WORKSPACE_MAKEFILE",
        "./examples/workspace/Makefile2.toml",
    );
    let config = load_descriptors("./bad/bad.toml", false, None, false, false, None).unwrap();
    envmnt::remove("CARGO_MAKE_WORKSPACE_MAKEFILE");

    let task = config.tasks.get("workspace-echo");
    assert!(task.is_none());
}

#[test]
#[ignore]
fn load_descriptors_no_load_workspace_makefile() {
    envmnt::remove("CARGO_MAKE_WORKSPACE_MAKEFILE");
    let config = load_descriptors("./bad/bad.toml", false, None, false, false, None).unwrap();

    let task = config.tasks.get("workspace-echo");
    assert!(task.is_none());
}

#[test]
#[ignore]
fn load_env_and_plugins_extended_override() {
    let toml_file = "./src/lib/test/makefiles/env-extended.toml";

    envmnt::remove_all(&vec!["IF_UNDEFINED", "COMPOSITE_OF_MAPPED"]);
    let config = load(toml_file, true, None, false).unwrap();
    environment::set_env_for_config(config.env, None, false);

    assert!(envmnt::is_equal("IF_UNDEFINED", "EXTENDED"));
    assert!(envmnt::is_equal("COMPOSITE_OF_MAPPED", "VALUE: EXTENDED"));

    envmnt::remove_all(&vec!["IF_UNDEFINED", "COMPOSITE_OF_MAPPED"]);

    let plugins_wrapper = config.plugins.unwrap();
    let aliases = plugins_wrapper.aliases.unwrap();
    assert_eq!(aliases.len(), 3);
    assert_eq!(aliases.get("old").unwrap(), "new");
    assert_eq!(aliases.get("new").unwrap(), "newer2");
    assert_eq!(aliases.get("extended").unwrap(), "test");
    assert_eq!(plugins_wrapper.plugins.len(), 3);
    assert_eq!(
        plugins_wrapper.plugins.get("base1").unwrap().script,
        "extended1 test"
    );
    assert_eq!(
        plugins_wrapper.plugins.get("base2").unwrap().script,
        "base2 test"
    );
    assert_eq!(
        plugins_wrapper.plugins.get("extended").unwrap().script,
        "extended test"
    );
}

#[test]
#[ignore]
fn load_env_override_and_plugins() {
    let toml_file = "./src/lib/test/makefiles/env.toml";

    envmnt::remove_all(&vec!["IF_UNDEFINED", "COMPOSITE_OF_MAPPED"]);
    let mut config = load(toml_file, true, None, false).unwrap();
    let mut env = IndexMap::<String, EnvValue>::new();
    env.insert(
        "IF_UNDEFINED".to_string(),
        config.env.get(&"IF_UNDEFINED".to_string()).unwrap().clone(),
    );
    env.insert(
        "COMPOSITE_OF_MAPPED".to_string(),
        config
            .env
            .get(&"COMPOSITE_OF_MAPPED".to_string())
            .unwrap()
            .clone(),
    );
    environment::set_env_for_config(env, None, false);

    assert!(envmnt::is_equal("IF_UNDEFINED", "defined_in_makefile"));
    assert!(envmnt::is_equal(
        "COMPOSITE_OF_MAPPED",
        "VALUE: defined_in_makefile"
    ));

    config = load(
        toml_file,
        true,
        Some(vec!["IF_UNDEFINED=test".to_string()]),
        false,
    )
    .unwrap();
    env = IndexMap::<String, EnvValue>::new();
    env.insert(
        "IF_UNDEFINED".to_string(),
        config.env.get(&"IF_UNDEFINED".to_string()).unwrap().clone(),
    );
    env.insert(
        "COMPOSITE_OF_MAPPED".to_string(),
        config
            .env
            .get(&"COMPOSITE_OF_MAPPED".to_string())
            .unwrap()
            .clone(),
    );
    environment::set_env_for_config(env, None, false);

    assert!(envmnt::is_equal("IF_UNDEFINED", "test"));
    assert!(envmnt::is_equal("COMPOSITE_OF_MAPPED", "VALUE: test"));

    envmnt::remove_all(&vec!["IF_UNDEFINED", "COMPOSITE_OF_MAPPED"]);

    let plugins_wrapper = config.plugins.unwrap();
    let aliases = plugins_wrapper.aliases.unwrap();
    assert_eq!(aliases.len(), 2);
    assert_eq!(aliases.get("old").unwrap(), "new");
    assert_eq!(aliases.get("new").unwrap(), "newer");
    assert_eq!(plugins_wrapper.plugins.len(), 2);
    assert_eq!(
        plugins_wrapper.plugins.get("base1").unwrap().script,
        "base1 test"
    );
    assert_eq!(
        plugins_wrapper.plugins.get("base2").unwrap().script,
        "base2 test"
    );
}

#[test]
#[ignore]
fn load_no_stable() {
    let config = load("./examples/skip_core_tasks.toml", true, None, false).unwrap();

    assert!(config.env.get(&"RUST_BACKTRACE".to_string()).is_none());

    assert!(config.env_scripts.is_empty());
    assert!(config.env_files.is_empty());

    let mut task = config.tasks.get("empty");
    assert!(task.is_some());
    task = config.tasks.get("default");
    assert!(task.is_some());
    task = config.tasks.get("init");
    assert!(task.is_some());
    task = config.tasks.get("end");
    assert!(task.is_some());
    task = config.tasks.get("wait");
    assert!(task.is_none());
    assert_eq!(config.config.init_task.unwrap(), "init");
    assert_eq!(config.config.end_task.unwrap(), "end");
}

#[test]
#[ignore]
fn load_with_stable() {
    let config = load("./examples/simple-example.toml", true, None, false).unwrap();

    assert!(config.env.get(&"RUST_BACKTRACE".to_string()).is_some());

    assert!(config.env_scripts.is_empty());
    assert!(config.env_files.is_empty());

    let mut task = config.tasks.get("empty");
    assert!(task.is_some());
    task = config.tasks.get("default");
    assert!(task.is_some());
    task = config.tasks.get("init");
    assert!(task.is_some());
    task = config.tasks.get("end");
    assert!(task.is_some());
    task = config.tasks.get("wait");
    assert!(task.is_some());
    assert_eq!(config.config.init_task.unwrap(), "init");
    assert_eq!(config.config.end_task.unwrap(), "end");
    assert!(config.plugins.is_none());
}

#[test]
#[ignore]
fn load_with_modify() {
    let config = load("./examples/modify_core_tasks.toml", true, None, false).unwrap();

    assert!(config.env.get(&"RUST_BACKTRACE".to_string()).is_some());

    let mut task = config.tasks.get("empty");
    assert!(task.is_none());
    task = config.tasks.get("default::empty");
    assert!(task.is_some());
    assert!(task.clone().unwrap().private.unwrap());
    task = config.tasks.get("default::init");
    assert!(task.is_some());
}

#[test]
#[should_panic]
fn load_not_found() {
    load("./examples/not-found.toml", true, None, false).unwrap();
}

#[test]
fn load_internal_descriptors_no_stable() {
    let config = load_internal_descriptors(false, false, None);

    let mut task = config.tasks.get("empty");
    assert!(task.is_some());
    task = config.tasks.get("wait");
    assert!(task.is_none());
}

#[test]
fn load_internal_descriptors_with_stable() {
    let config = load_internal_descriptors(true, false, None);

    let mut task = config.tasks.get("empty");
    assert!(task.is_some());
    task = config.tasks.get("wait");
    assert!(task.is_some());
}

#[test]
fn load_internal_descriptors_no_experimental() {
    let config = load_internal_descriptors(true, false, None);

    let mut task = config.tasks.get("ci-flow");
    assert!(task.is_some());
    task = config.tasks.get("coverage-lcov");
    assert!(task.is_none());
}

#[test]
fn load_internal_descriptors_with_experimental() {
    let config = load_internal_descriptors(true, true, None);

    let mut task = config.tasks.get("ci-flow");
    assert!(task.is_some());
    task = config.tasks.get("coverage-lcov");
    assert!(task.is_some());
}

#[test]
fn load_internal_descriptors_modify_empty() {
    let config = load_internal_descriptors(
        true,
        false,
        Some(ModifyConfig {
            private: None,
            namespace: None,
        }),
    );

    let mut task = config.tasks.get("empty");
    assert!(task.is_some());
    assert!(task.unwrap().private.is_none());
    task = config.tasks.get("init");
    assert!(task.is_some());
    assert!(task.unwrap().private.is_none());
}

#[test]
fn load_internal_descriptors_modify_private() {
    let config = load_internal_descriptors(
        true,
        false,
        Some(ModifyConfig {
            private: Some(true),
            namespace: None,
        }),
    );

    let mut task = config.tasks.get("empty");
    assert!(task.is_some());
    assert!(task.unwrap().private.unwrap());
    task = config.tasks.get("init");
    assert!(task.is_some());
    assert!(task.unwrap().private.unwrap());
}

#[test]
fn load_internal_descriptors_modify_namespace() {
    let config = load_internal_descriptors(
        true,
        false,
        Some(ModifyConfig {
            private: None,
            namespace: Some("default".to_string()),
        }),
    );

    let mut task = config.tasks.get("empty");
    assert!(task.is_none());
    task = config.tasks.get("default::empty");
    assert!(task.is_some());
    assert!(task.unwrap().private.is_none());
    task = config.tasks.get("init");
    assert!(task.is_none());
    task = config.tasks.get("default::init");
    assert!(task.is_some());
    assert!(task.unwrap().private.is_none());
}

#[test]
#[ignore]
fn load_external_descriptor_no_file() {
    let config = load_external_descriptor(".", "bad_file.toml2", false, false).unwrap();

    assert!(config.config.is_none());
    assert!(config.env.is_none());
    assert!(config.tasks.is_none());
}

#[test]
#[should_panic]
fn load_external_descriptor_no_file_force() {
    load_external_descriptor(".", "bad_file.toml2", true, false).unwrap();
}

#[test]
#[should_panic]
fn load_external_descriptor_extended_not_found_force() {
    load_external_descriptor(".", "./examples/extends_not_found.toml", true, false).unwrap();
}

#[test]
#[ignore]
fn load_external_descriptor_simple_file() {
    let config = load_external_descriptor(".", "./examples/alias.toml", true, false).unwrap();

    assert!(config.config.is_none());
    assert!(config.env.is_none());
    assert!(config.tasks.is_some());

    let tasks = config.tasks.unwrap();
    let test_task = tasks.get("D2").unwrap();
    let alias = test_task.alias.clone();
    assert_eq!(alias.unwrap(), "D");
}

#[test]
#[ignore]
fn load_external_descriptor_extending_file() {
    let config = load_external_descriptor(".", "examples/extending.toml", true, false).unwrap();

    assert!(config.config.is_some());
    assert!(config.env.is_some());
    assert!(config.tasks.is_some());

    assert_eq!(config.env.unwrap().len(), 0);

    let tasks = config.tasks.unwrap();
    let mut test_task = tasks.get("D2").unwrap();
    let mut alias = test_task.alias.clone();
    assert_eq!(alias.unwrap(), "D");

    test_task = tasks.get("extended").unwrap();
    alias = test_task.alias.clone();
    assert_eq!(alias.unwrap(), "D2");
}

#[test]
#[ignore]
fn load_external_descriptor_extending_file_sub_folder() {
    let config =
        load_external_descriptor(".", "examples/files/extending.toml", true, false).unwrap();

    assert!(config.config.is_some());
    assert!(config.env.is_some());
    assert!(config.tasks.is_some());

    let config_section = config.config.unwrap();
    assert!(config_section.init_task.is_some());
    assert!(config_section.end_task.is_none());
    assert_eq!(config_section.init_task.unwrap(), "test_init");

    assert_eq!(config.env.unwrap().len(), 0);

    let tasks = config.tasks.unwrap();
    let mut test_task = tasks.get("D2").unwrap();
    let mut alias = test_task.alias.clone();
    assert_eq!(alias.unwrap(), "D");

    test_task = tasks.get("extended").unwrap();
    alias = test_task.alias.clone();
    assert_eq!(alias.unwrap(), "D2");

    test_task = tasks.get("extended2").unwrap();
    alias = test_task.alias.clone();
    assert_eq!(alias.unwrap(), "extended");
}

#[test]
#[ignore]
fn load_external_descriptor_set_env() {
    envmnt::set("CARGO_MAKE_MAKEFILE_PATH", "EMPTY");
    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_MAKEFILE_PATH"), "EMPTY");

    load_external_descriptor(".", "./examples/alias.toml", true, true).unwrap();

    assert!(envmnt::get_or_panic("CARGO_MAKE_MAKEFILE_PATH").ends_with("alias.toml"));
}

#[test]
fn load_external_descriptor_min_version_broken_makefile_nopanic() {
    // Ensure that we can properly get the min_version of a descriptor that
    // doesn't match our internal data-structures.
    assert_eq!(
        load_external_descriptor(
            ".",
            "src/lib/test/makefiles/broken_makefile_minversion.toml",
            false,
            false
        )
        .err(),
        Some("999.999.999".into())
    );
}

#[test]
#[should_panic]
fn load_external_descriptor_broken_makefile_panic() {
    // Ensure the descriptor we used in the test above is properly broken when
    // it doesn't have a min_version. This is to ensure that the previous test
    // is working properly - min_version is optional.
    load_external_descriptor(
        ".",
        "src/lib/test/makefiles/broken_makefile.toml",
        false,
        false,
    )
    .unwrap();
}

#[test]
fn run_load_script_no_config_section() {
    let external_config = ExternalConfig::new();

    let invoked = run_load_script(&external_config);
    assert!(!invoked);
}

#[test]
fn run_load_script_no_load_script() {
    let mut external_config = ExternalConfig::new();
    external_config.config = Some(ConfigSection::new());

    let invoked = run_load_script(&external_config);
    assert!(!invoked);
}

#[test]
fn run_load_script_valid_load_script() {
    let mut config = ConfigSection::new();
    config.load_script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));

    let mut external_config = ExternalConfig::new();
    external_config.config = Some(config);

    let invoked = run_load_script(&external_config);
    assert!(invoked);
}

#[test]
#[should_panic]
fn run_load_script_invalid_load_script() {
    let mut config = ConfigSection::new();
    config.load_script = Some(ScriptValue::Text(vec!["exit 1".to_string()]));

    let mut external_config = ExternalConfig::new();
    external_config.config = Some(config);

    run_load_script(&external_config);
}

#[test]
fn run_load_script_valid_load_script_duckscript() {
    envmnt::remove("run_load_script_valid_load_script_duckscript");
    assert!(!envmnt::exists(
        "run_load_script_valid_load_script_duckscript"
    ));

    let mut config = ConfigSection::new();
    config.load_script = Some(ScriptValue::Text(vec![r#"#!@duckscript
    set_env run_load_script_valid_load_script_duckscript true
    "#
    .to_string()]));

    let mut external_config = ExternalConfig::new();
    external_config.config = Some(config);

    let invoked = run_load_script(&external_config);
    assert!(invoked);

    assert!(envmnt::exists(
        "run_load_script_valid_load_script_duckscript"
    ));
    assert!(envmnt::is_or(
        "run_load_script_valid_load_script_duckscript",
        false
    ));
}

#[test]
#[ignore]
fn load_descriptor_extended_makefiles_path_exists() {
    let parent_path = envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY");
    let descriptor = load_descriptor_extended_makefiles(
        &parent_path,
        &Extend::Path("src/lib/test/makefiles/test1.toml".to_string()),
    )
    .unwrap();

    let tasks = descriptor.tasks.unwrap();
    assert!(tasks.contains_key("test1"));
}

#[test]
#[ignore]
#[should_panic]
fn load_descriptor_extended_makefiles_path_not_exists() {
    let parent_path = envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY");
    load_descriptor_extended_makefiles(
        &parent_path,
        &Extend::Path("src/lib/test/makefiles/bad.toml".to_string()),
    )
    .unwrap();
}

#[test]
#[ignore]
fn load_descriptor_extended_makefiles_options_exists() {
    let parent_path = envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY");
    let descriptor = load_descriptor_extended_makefiles(
        &parent_path,
        &Extend::Options(ExtendOptions {
            path: "src/lib/test/makefiles/test1.toml".to_string(),
            optional: None,
        }),
    )
    .unwrap();

    let tasks = descriptor.tasks.unwrap();
    assert!(tasks.contains_key("test1"));
}

#[test]
#[ignore]
#[should_panic]
fn load_descriptor_extended_makefiles_options_not_exists() {
    let parent_path = envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY");
    load_descriptor_extended_makefiles(
        &parent_path,
        &Extend::Options(ExtendOptions {
            path: "src/lib/test/makefiles/bad.toml".to_string(),
            optional: None,
        }),
    )
    .unwrap();
}

#[test]
#[ignore]
fn load_descriptor_extended_makefiles_options_exists_optional() {
    let parent_path = envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY");
    let descriptor = load_descriptor_extended_makefiles(
        &parent_path,
        &Extend::Options(ExtendOptions {
            path: "src/lib/test/makefiles/test1.toml".to_string(),
            optional: Some(true),
        }),
    )
    .unwrap();

    let tasks = descriptor.tasks.unwrap();
    assert!(tasks.contains_key("test1"));
}

#[test]
#[ignore]
fn load_descriptor_extended_makefiles_options_exists_not_optional() {
    let parent_path = envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY");
    let descriptor = load_descriptor_extended_makefiles(
        &parent_path,
        &Extend::Options(ExtendOptions {
            path: "src/lib/test/makefiles/test1.toml".to_string(),
            optional: Some(false),
        }),
    )
    .unwrap();

    let tasks = descriptor.tasks.unwrap();
    assert!(tasks.contains_key("test1"));
}

#[test]
#[ignore]
#[should_panic]
fn load_descriptor_extended_makefiles_options_not_exists_optional() {
    let parent_path = envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY");
    let descriptor = load_descriptor_extended_makefiles(
        &parent_path,
        &Extend::Options(ExtendOptions {
            path: "src/lib/test/makefiles/bad.toml".to_string(),
            optional: Some(true),
        }),
    )
    .unwrap();

    let tasks = descriptor.tasks.unwrap();
    assert!(!tasks.contains_key("test1"));
}

#[test]
#[ignore]
#[should_panic]
fn load_descriptor_extended_makefiles_options_not_exists_not_optional() {
    let parent_path = envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY");
    load_descriptor_extended_makefiles(
        &parent_path,
        &Extend::Options(ExtendOptions {
            path: "src/lib/test/makefiles/bad.toml".to_string(),
            optional: Some(false),
        }),
    )
    .unwrap();
}

#[test]
#[ignore]
fn load_descriptor_extended_makefiles_list_exists() {
    let parent_path = envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY");
    let list = vec![
        ExtendOptions {
            path: "src/lib/test/makefiles/test1.toml".to_string(),
            optional: Some(false),
        },
        ExtendOptions {
            path: "src/lib/test/makefiles/test2.toml".to_string(),
            optional: Some(false),
        },
    ];
    let descriptor = load_descriptor_extended_makefiles(&parent_path, &Extend::List(list)).unwrap();

    let tasks = descriptor.tasks.unwrap();
    assert!(tasks.contains_key("test1"));
    assert!(tasks.contains_key("test2"));
}

#[test]
#[ignore]
#[should_panic]
fn load_descriptor_extended_makefiles_list_not_exists() {
    let parent_path = envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY");
    let list = vec![
        ExtendOptions {
            path: "src/lib/test/makefiles/test1.toml".to_string(),
            optional: Some(false),
        },
        ExtendOptions {
            path: "src/lib/test/makefiles/bad.toml".to_string(),
            optional: Some(false),
        },
    ];
    load_descriptor_extended_makefiles(&parent_path, &Extend::List(list)).unwrap();
}

#[test]
#[ignore]
fn load_descriptor_extended_makefiles_list_exists_optional() {
    let parent_path = envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY");
    let list = vec![
        ExtendOptions {
            path: "src/lib/test/makefiles/test1.toml".to_string(),
            optional: Some(false),
        },
        ExtendOptions {
            path: "src/lib/test/makefiles/bad.toml".to_string(),
            optional: Some(true),
        },
    ];
    let descriptor = load_descriptor_extended_makefiles(&parent_path, &Extend::List(list)).unwrap();

    let tasks = descriptor.tasks.unwrap();
    assert!(tasks.contains_key("test1"));
    assert!(!tasks.contains_key("test2"));
}

#[test]
fn check_makefile_min_version_empty() {
    let toml_string = "";
    let result = check_makefile_min_version(toml_string);

    assert!(result.is_ok());
}

#[test]
fn check_makefile_min_version_invalid_format() {
    let toml_string = "123";
    let result = check_makefile_min_version(toml_string);

    assert!(result.is_ok());
}

#[test]
fn check_makefile_min_version_no_config() {
    let toml_string = "test = true";
    let result = check_makefile_min_version(toml_string);

    assert!(result.is_ok());
}

#[test]
fn check_makefile_min_version_no_min_version() {
    let toml_string = r#"
    [config]
    test = true
    "#;
    let result = check_makefile_min_version(toml_string);

    assert!(result.is_ok());
}

#[test]
fn check_makefile_min_version_smaller_min_version() {
    let toml_string = r#"
    [config]
    min_version = "0.0.1"
    "#;
    let result = check_makefile_min_version(toml_string);

    assert!(result.is_ok());
}

#[test]
fn check_makefile_min_version_bigger_min_version() {
    let toml_string = r#"
    [config]
    min_version = "999.999.999"
    "#;
    let result = check_makefile_min_version(toml_string);

    assert!(result.is_err());
    assert_eq!(result.err().unwrap(), "999.999.999");
}

#[test]
fn check_makefile_min_version_same_min_version() {
    let mut toml_string = r#"
    [config]
    min_version = ""#;
    let mut buffer = toml_string.to_string();
    buffer.push_str(env!("CARGO_PKG_VERSION"));
    buffer.push_str("\"");
    toml_string = buffer.as_str();
    let result = check_makefile_min_version(toml_string);

    assert!(result.is_ok());
}

#[test]
fn load_cargo_aliases_no_file() {
    let mut config = load_internal_descriptors(false, false, None);
    let count = config.tasks.len();

    load_cargo_aliases(&mut config);

    assert_eq!(count, config.tasks.len());
}

#[test]
#[ignore]
fn load_cargo_aliases_found() {
    let mut config = load_internal_descriptors(false, false, None);
    let count = config.tasks.len();

    setup_cwd(Some("src/lib/test/workspace1/member1"));
    load_cargo_aliases(&mut config);
    setup_cwd(Some("../../../../.."));

    assert_eq!(count, config.tasks.len());
}
