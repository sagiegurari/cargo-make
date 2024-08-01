use super::*;
use crate::descriptor;
use crate::types::{ConfigSection, DependencyIdentifier, PlatformOverrideTask, TaskWatchOptions};

#[test]
fn get_actual_task_name_not_found() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let name = get_actual_task_name(&config, "test");

    assert_eq!(
        format!("{:?}", name.err().unwrap()),
        format!(
            "{:?}",
            CargoMakeError::NotFound(String::from("Task \"test\" not found"))
        )
    );
}

#[test]
fn get_actual_task_name_no_alias() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("test".to_string(), Task::new());

    let name = get_actual_task_name(&config, "test");

    assert_eq!(name.unwrap(), "test");
}

#[test]
fn get_actual_task_name_alias() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let mut task = Task::new();
    task.alias = Some("test2".to_string());
    config.tasks.insert("test".to_string(), task);

    config.tasks.insert("test2".to_string(), Task::new());

    let name = get_actual_task_name(&config, "test");

    assert_eq!(name.unwrap(), "test2");
}

#[test]
#[should_panic]
fn get_actual_task_name_alias_self_referential() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let mut task = Task::new();
    task.alias = Some("rec".to_string());
    config.tasks.insert("rec".to_string(), task);

    get_actual_task_name(&config, "rec").unwrap();
}

#[test]
#[should_panic]
fn get_actual_task_name_alias_circular() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let mut task_a = Task::new();
    let mut task_b = Task::new();

    task_a.alias = Some("rec-mut-b".to_string());
    task_b.alias = Some("rec-mut-a".to_string());

    config.tasks.insert("rec-mut-a".to_string(), task_a);
    config.tasks.insert("rec-mut-b".to_string(), task_b);

    get_actual_task_name(&config, "rec-mut-a").unwrap();
}

#[test]
fn get_actual_task_name_platform_alias() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let mut task = Task::new();
    if cfg!(windows) {
        task.windows_alias = Some("test2".to_string());
    } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        task.mac_alias = Some("test2".to_string());
    } else {
        task.linux_alias = Some("test2".to_string());
    };

    config.tasks.insert("test".to_string(), task);

    config.tasks.insert("test2".to_string(), Task::new());

    let name = get_actual_task_name(&config, "test");

    assert_eq!(name.unwrap(), "test2");
}

#[test]
fn get_workspace_members_config_not_defined_or_empty() {
    let members = get_workspace_members_config("".to_string());

    assert_eq!(members.len(), 0);
}

#[test]
fn get_workspace_members_config_single() {
    let members = get_workspace_members_config("test".to_string());

    assert_eq!(members.len(), 1);
    assert!(members.contains(&"test".to_string()));
}

#[test]
fn get_workspace_members_config_multiple() {
    let members = get_workspace_members_config("test1;test2;test3".to_string());

    assert_eq!(members.len(), 3);
    assert!(members.contains(&"test1".to_string()));
    assert!(members.contains(&"test2".to_string()));
    assert!(members.contains(&"test3".to_string()));
}

#[test]
fn create_workspace_task_no_members() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    let task = create_workspace_task(&crate_info, "some_task");

    assert!(task.script.is_some());
    let script = match task.script.unwrap() {
        ScriptValue::Text(value) => value.join("\n"),
        _ => panic!("Invalid script value type."),
    };
    assert_eq!(script, "".to_string());
    assert!(task.env.is_none());
}

#[test]
#[ignore]
fn create_workspace_task_with_members() {
    let mut crate_info = CrateInfo::new();
    let members = vec![
        "member1".to_string(),
        "member2".to_string(),
        "dir1/member3".to_string(),
    ];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    envmnt::remove("CARGO_MAKE_USE_WORKSPACE_PROFILE");

    let task = create_workspace_task(&crate_info, "some_task");

    let mut expected_script = r#"workspace_directory = pwd
cd ./member1
exec --fail-on-error cargo make --disable-check-for-updates --allow-private --no-on-error --loglevel=LEVEL_NAME --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=member1 --profile PROFILE_NAME -- some_task
cd ${workspace_directory}
cd ./member2
exec --fail-on-error cargo make --disable-check-for-updates --allow-private --no-on-error --loglevel=LEVEL_NAME --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=member2 --profile PROFILE_NAME -- some_task
cd ${workspace_directory}
cd ./dir1/member3
exec --fail-on-error cargo make --disable-check-for-updates --allow-private --no-on-error --loglevel=LEVEL_NAME --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=member3 --profile PROFILE_NAME -- some_task
cd ${workspace_directory}"#
        .to_string();

    let log_level = logger::get_log_level();
    expected_script = str::replace(&expected_script, "LEVEL_NAME", &log_level);

    let profile_name = profile::get();
    expected_script = str::replace(&expected_script, "PROFILE_NAME", &profile_name);

    assert!(task.script.is_some());
    let script = match task.script.unwrap() {
        ScriptValue::Text(value) => value.join("\n"),
        _ => panic!("Invalid script value type."),
    };
    assert_eq!(script, expected_script);
    assert!(task.env.is_none());
}

#[test]
#[ignore]
fn create_workspace_task_with_members_no_workspace_profile() {
    let mut crate_info = CrateInfo::new();
    let members = vec![
        "member1".to_string(),
        "member2".to_string(),
        "dir1/member3".to_string(),
    ];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    envmnt::set_bool("CARGO_MAKE_USE_WORKSPACE_PROFILE", false);

    let task = create_workspace_task(&crate_info, "some_task");

    let mut expected_script = r#"workspace_directory = pwd
cd ./member1
exec --fail-on-error cargo make --disable-check-for-updates --allow-private --no-on-error --loglevel=LEVEL_NAME --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=member1 --profile development -- some_task
cd ${workspace_directory}
cd ./member2
exec --fail-on-error cargo make --disable-check-for-updates --allow-private --no-on-error --loglevel=LEVEL_NAME --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=member2 --profile development -- some_task
cd ${workspace_directory}
cd ./dir1/member3
exec --fail-on-error cargo make --disable-check-for-updates --allow-private --no-on-error --loglevel=LEVEL_NAME --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=member3 --profile development -- some_task
cd ${workspace_directory}"#
        .to_string();

    let log_level = logger::get_log_level();
    expected_script = str::replace(&expected_script, "LEVEL_NAME", &log_level);

    assert!(task.script.is_some());
    let script = match task.script.unwrap() {
        ScriptValue::Text(value) => value.join("\n"),
        _ => panic!("Invalid script value type."),
    };
    assert_eq!(script, expected_script);
    assert!(task.env.is_none());
}

#[test]
#[ignore]
fn create_workspace_task_with_members_and_arguments() {
    let mut crate_info = CrateInfo::new();
    let members = vec![
        "member1".to_string(),
        "member2".to_string(),
        "dir1/member3".to_string(),
    ];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    envmnt::remove("CARGO_MAKE_USE_WORKSPACE_PROFILE");

    envmnt::set_list(
        "CARGO_MAKE_TASK_ARGS",
        &vec!["arg1".to_string(), "arg2".to_string()],
    );

    let task = create_workspace_task(&crate_info, "some_task");

    envmnt::remove("CARGO_MAKE_TASK_ARGS");

    let mut expected_script = r#"workspace_directory = pwd
cd ./member1
exec --fail-on-error cargo make --disable-check-for-updates --allow-private --no-on-error --loglevel=LEVEL_NAME --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=member1 --profile PROFILE_NAME -- some_task arg1 arg2
cd ${workspace_directory}
cd ./member2
exec --fail-on-error cargo make --disable-check-for-updates --allow-private --no-on-error --loglevel=LEVEL_NAME --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=member2 --profile PROFILE_NAME -- some_task arg1 arg2
cd ${workspace_directory}
cd ./dir1/member3
exec --fail-on-error cargo make --disable-check-for-updates --allow-private --no-on-error --loglevel=LEVEL_NAME --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=member3 --profile PROFILE_NAME -- some_task arg1 arg2
cd ${workspace_directory}"#
        .to_string();

    let log_level = logger::get_log_level();
    expected_script = str::replace(&expected_script, "LEVEL_NAME", &log_level);

    let profile_name = profile::get();
    expected_script = str::replace(&expected_script, "PROFILE_NAME", &profile_name);

    assert!(task.script.is_some());
    let script = match task.script.unwrap() {
        ScriptValue::Text(value) => value.join("\n"),
        _ => panic!("Invalid script value type."),
    };
    assert_eq!(script, expected_script);
    assert!(task.env.is_none());
}

#[test]
#[ignore]
fn create_workspace_task_with_included_members() {
    let mut crate_info = CrateInfo::new();
    let members = vec![
        "member1".to_string(),
        "member2".to_string(),
        "dir1/member3".to_string(),
        "dir1/member4".to_string(),
    ];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    envmnt::set_list(
        "CARGO_MAKE_WORKSPACE_INCLUDE_MEMBERS",
        &vec![
            "member1".to_string(),
            "member2".to_string(),
            "dir1/member3".to_string(),
        ],
    );

    profile::set(&profile::default_profile());

    let task = create_workspace_task(&crate_info, "some_task");

    envmnt::remove("CARGO_MAKE_WORKSPACE_INCLUDE_MEMBERS");

    let mut expected_script = r#"workspace_directory = pwd
cd ./member1
exec --fail-on-error cargo make --disable-check-for-updates --allow-private --no-on-error --loglevel=LEVEL_NAME --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=member1 --profile development -- some_task
cd ${workspace_directory}
cd ./member2
exec --fail-on-error cargo make --disable-check-for-updates --allow-private --no-on-error --loglevel=LEVEL_NAME --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=member2 --profile development -- some_task
cd ${workspace_directory}
cd ./dir1/member3
exec --fail-on-error cargo make --disable-check-for-updates --allow-private --no-on-error --loglevel=LEVEL_NAME --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=member3 --profile development -- some_task
cd ${workspace_directory}"#.to_string();

    let log_level = logger::get_log_level();
    expected_script = str::replace(&expected_script, "LEVEL_NAME", &log_level);

    assert!(task.script.is_some());
    let script = match task.script.unwrap() {
        ScriptValue::Text(value) => value.join("\n"),
        _ => panic!("Invalid script value type."),
    };
    assert_eq!(script, expected_script);
    assert!(task.env.is_none());
}

#[test]
#[ignore]
fn create_workspace_task_with_included_and_skipped_members() {
    let mut crate_info = CrateInfo::new();
    let members = vec![
        "member1".to_string(),
        "member2".to_string(),
        "dir1/member3".to_string(),
    ];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    envmnt::set_list(
        "CARGO_MAKE_WORKSPACE_INCLUDE_MEMBERS",
        &vec!["member1".to_string(), "member2".to_string()],
    );

    envmnt::set_list(
        "CARGO_MAKE_WORKSPACE_SKIP_MEMBERS",
        &vec!["member2".to_string(), "dir1/member3".to_string()],
    );

    profile::set(&profile::default_profile());

    let task = create_workspace_task(&crate_info, "some_task");

    envmnt::remove("CARGO_MAKE_WORKSPACE_INCLUDE_MEMBERS");
    envmnt::remove("CARGO_MAKE_WORKSPACE_SKIP_MEMBERS");

    let mut expected_script = r#"workspace_directory = pwd
cd ./member1
exec --fail-on-error cargo make --disable-check-for-updates --allow-private --no-on-error --loglevel=LEVEL_NAME --env CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER=member1 --profile development -- some_task
cd ${workspace_directory}"#
        .to_string();

    let log_level = logger::get_log_level();
    expected_script = str::replace(&expected_script, "LEVEL_NAME", &log_level);

    assert!(task.script.is_some());
    let script = match task.script.unwrap() {
        ScriptValue::Text(value) => value.join("\n"),
        _ => panic!("Invalid script value type."),
    };
    assert_eq!(script, expected_script);
    assert!(task.env.is_none());
}

#[test]
#[ignore]
fn create_workspace_task_extend_workspace_makefile() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    envmnt::set("CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE", "true");
    let task = create_workspace_task(&crate_info, "some_task");
    envmnt::set("CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE", "false");

    assert!(task.script.is_some());
    let script = match task.script.unwrap() {
        ScriptValue::Text(value) => value.join("\n"),
        _ => panic!("Invalid script value type."),
    };
    assert_eq!(script, "".to_string());
    assert!(task.env.is_some());
    assert!(task
        .env
        .unwrap()
        .get("CARGO_MAKE_WORKSPACE_MAKEFILE")
        .is_some());
}

#[test]
fn is_workspace_flow_true_default() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    let task = Task::new();

    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", false, &crate_info, false);

    assert!(workspace_flow);
}

#[test]
fn is_workspace_flow_false_in_config() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    let task = Task::new();

    let mut config_section = ConfigSection::new();
    config_section.default_to_workspace = Some(false);

    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", false, &crate_info, false);

    assert!(!workspace_flow);
}

#[test]
fn is_workspace_flow_true_in_config() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    let task = Task::new();

    let mut config_section = ConfigSection::new();
    config_section.default_to_workspace = Some(true);

    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", false, &crate_info, false);

    assert!(workspace_flow);
}

#[test]
fn is_workspace_flow_true_in_task() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    let mut task = Task::new();
    task.workspace = Some(true);

    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", false, &crate_info, false);

    assert!(workspace_flow);
}

#[test]
fn is_workspace_flow_default_false_in_task_and_sub_flow() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    let task = Task::new();

    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", false, &crate_info, true);

    assert!(!workspace_flow);
}

#[test]
fn is_workspace_flow_true_in_task_and_sub_flow() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    let mut task = Task::new();
    task.workspace = Some(true);

    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", false, &crate_info, true);

    assert!(workspace_flow);
}

#[test]
fn is_workspace_flow_false_in_task_and_sub_flow() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    let mut task = Task::new();
    task.workspace = Some(false);

    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", false, &crate_info, true);

    assert!(!workspace_flow);
}

#[test]
fn is_workspace_flow_task_not_defined() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let workspace_flow = is_workspace_flow(&config, "notfound", false, &crate_info, false);

    assert!(workspace_flow);
}

#[test]
fn is_workspace_flow_no_workspace() {
    let crate_info = CrateInfo::new();

    let mut task = Task::new();
    task.workspace = Some(true);

    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", false, &crate_info, false);

    assert!(!workspace_flow);
}

#[test]
fn is_workspace_flow_disabled_via_cli() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    let mut task = Task::new();
    task.workspace = Some(true);

    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", true, &crate_info, false);

    assert!(!workspace_flow);
}

#[test]
fn is_workspace_flow_disabled_via_task() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
        dependencies: None,
        package: None,
    });

    let mut task = Task::new();
    task.workspace = Some(false);

    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", false, &crate_info, false);

    assert!(!workspace_flow);
}

#[test]
fn create_single() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let task = Task::new();

    config.tasks.insert("test".to_string(), task);

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();
    assert_eq!(execution_plan.steps.len(), 3);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "test");
    assert_eq!(execution_plan.steps[2].name, "end");
}

#[test]
fn create_single_disabled() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.disabled = Some(true);

    config.tasks.insert("test".to_string(), task);

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();
    assert_eq!(execution_plan.steps.len(), 2);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "end");
}

#[test]
#[should_panic]
fn create_single_private() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.private = Some(true);

    config.tasks.insert("test-private".to_string(), task);

    ExecutionPlanBuilder::new(&config, "test-private")
        .build()
        .unwrap();
}

#[test]
fn create_single_allow_private() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.private = Some(true);

    config.tasks.insert("test-private".to_string(), task);

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        ..ExecutionPlanBuilder::new(&config, "test-private")
    }
    .build()
    .unwrap();
    assert_eq!(execution_plan.steps.len(), 3);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "test-private");
    assert_eq!(execution_plan.steps[2].name, "end");
}

#[test]
fn create_with_dependencies() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.dependencies = Some(vec!["task_dependency".into()]);

    let task_dependency = Task::new();

    config.tasks.insert("test".to_string(), task);
    config
        .tasks
        .insert("task_dependency".to_string(), task_dependency);

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();
    assert_eq!(execution_plan.steps.len(), 4);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "task_dependency");
    assert_eq!(execution_plan.steps[2].name, "test");
    assert_eq!(execution_plan.steps[3].name, "end");
}

#[test]
fn create_with_foreign_dependencies_directory() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.dependencies = Some(vec![DependencyIdentifier::Definition(TaskIdentifier {
        name: "task_dependency".to_string(),
        path: Some("./examples/workspace".to_string()),
    })]);

    let task_dependency = Task::new();

    config.tasks.insert("test".to_string(), task);
    config
        .tasks
        .insert("task_dependency".to_string(), task_dependency);

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();

    assert_eq!(execution_plan.steps.len(), 4);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "task_dependency_proxy");
    assert_eq!(execution_plan.steps[2].name, "test");
    assert_eq!(execution_plan.steps[3].name, "end");

    let task_config = execution_plan.steps[1].config.clone();
    assert_eq!(task_config.cwd, Some("./examples/workspace".to_string()));
    assert_eq!(task_config.args.unwrap()[7], "Makefile.toml");
}

#[test]
fn create_with_foreign_dependencies_filename() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.dependencies = Some(vec![DependencyIdentifier::Definition(TaskIdentifier {
        name: "task_dependency".to_string(),
        path: Some("Cargo.toml".to_string()),
    })]);

    let task_dependency = Task::new();

    config.tasks.insert("test".to_string(), task);
    config
        .tasks
        .insert("task_dependency".to_string(), task_dependency);

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();

    assert_eq!(execution_plan.steps.len(), 4);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "task_dependency_proxy");
    assert_eq!(execution_plan.steps[2].name, "test");
    assert_eq!(execution_plan.steps[3].name, "end");

    let task_config = execution_plan.steps[1].config.clone();
    assert_eq!(task_config.cwd, None);
    assert_eq!(task_config.args.unwrap()[7], "Cargo.toml");
}

#[test]
fn create_with_foreign_dependencies_file_and_directory() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.dependencies = Some(vec![DependencyIdentifier::Definition(TaskIdentifier {
        name: "task_dependency".to_string(),
        path: Some("./examples/cross-file.toml".to_string()),
    })]);

    let task_dependency = Task::new();

    config.tasks.insert("test".to_string(), task);
    config
        .tasks
        .insert("task_dependency".to_string(), task_dependency);

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();

    assert_eq!(execution_plan.steps.len(), 4);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "task_dependency_proxy");
    assert_eq!(execution_plan.steps[2].name, "test");
    assert_eq!(execution_plan.steps[3].name, "end");

    let task_config = execution_plan.steps[1].config.clone();
    assert_eq!(task_config.cwd, Some("./examples".to_string()));
    assert_eq!(task_config.args.unwrap()[7], "cross-file.toml");
}

#[test]
fn create_with_dependencies_sub_flow() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.dependencies = Some(vec!["task_dependency".into()]);

    let task_dependency = Task::new();

    config.tasks.insert("test".to_string(), task);
    config
        .tasks
        .insert("task_dependency".to_string(), task_dependency);

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        sub_flow: true,
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();
    assert_eq!(execution_plan.steps.len(), 2);
    assert_eq!(execution_plan.steps[0].name, "task_dependency");
    assert_eq!(execution_plan.steps[1].name, "test");
}

#[test]
fn create_disabled_task_with_dependencies() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.disabled = Some(true);
    task.dependencies = Some(vec!["task_dependency".into()]);

    let task_dependency = Task::new();

    config.tasks.insert("test".to_string(), task);
    config
        .tasks
        .insert("task_dependency".to_string(), task_dependency);

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();
    assert_eq!(execution_plan.steps.len(), 2);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "end");
}

#[test]
fn create_with_dependencies_disabled() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.dependencies = Some(vec!["task_dependency".into()]);

    let mut task_dependency = Task::new();
    task_dependency.disabled = Some(true);

    config.tasks.insert("test".to_string(), task);
    config
        .tasks
        .insert("task_dependency".to_string(), task_dependency);

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();
    assert_eq!(execution_plan.steps.len(), 3);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "test");
    assert_eq!(execution_plan.steps[2].name, "end");
}

#[test]
fn create_platform_disabled() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let mut task = Task::new();
    task.linux = Some(PlatformOverrideTask {
        clear: Some(true),
        disabled: Some(true),
        private: Some(false),
        deprecated: None,
        extend: None,
        plugin: None,
        watch: Some(TaskWatchOptions::Boolean(false)),
        condition: None,
        condition_script: None,
        condition_script_runner_args: None,
        install_crate: None,
        install_crate_args: None,
        command: None,
        ignore_errors: None,
        force: None,
        env_files: None,
        env: None,
        cwd: None,
        install_script: None,
        args: None,
        script: None,
        script_runner: None,
        script_runner_args: None,
        script_extension: None,
        run_task: None,
        dependencies: None,
        toolchain: None,
    });
    task.windows = Some(PlatformOverrideTask {
        clear: Some(true),
        disabled: Some(true),
        private: Some(false),
        deprecated: None,
        extend: None,
        plugin: None,
        watch: Some(TaskWatchOptions::Boolean(false)),
        condition: None,
        condition_script: None,
        condition_script_runner_args: None,
        install_crate: None,
        install_crate_args: None,
        command: None,
        ignore_errors: None,
        force: None,
        env_files: None,
        env: None,
        cwd: None,
        install_script: None,
        args: None,
        script: None,
        script_runner: None,
        script_runner_args: None,
        script_extension: None,
        run_task: None,
        dependencies: None,
        toolchain: None,
    });
    task.mac = Some(PlatformOverrideTask {
        clear: Some(true),
        disabled: Some(true),
        private: Some(false),
        deprecated: None,
        extend: None,
        plugin: None,
        watch: Some(TaskWatchOptions::Boolean(false)),
        condition: None,
        condition_script: None,
        condition_script_runner_args: None,
        install_crate: None,
        install_crate_args: None,
        command: None,
        ignore_errors: None,
        force: None,
        env_files: None,
        env: None,
        cwd: None,
        install_script: None,
        args: None,
        script: None,
        script_runner: None,
        script_runner_args: None,
        script_extension: None,
        run_task: None,
        dependencies: None,
        toolchain: None,
    });

    config.tasks.insert("test".to_string(), task);

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();
    assert_eq!(execution_plan.steps.len(), 0);
}

#[test]
fn create_with_dependencies_and_skip_filter() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.dependencies = Some(vec!["task_dependency".into(), "filtered2".into()]);

    let mut task_dependency = Task::new();
    task_dependency.dependencies = Some(vec!["filtered2".into()]);

    config.tasks.insert("test".to_string(), task);
    config
        .tasks
        .insert("task_dependency".to_string(), task_dependency);

    let skip_filter = Regex::new("filtered.*").unwrap();

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        skip_tasks_pattern: Some(&skip_filter),
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();
    assert_eq!(execution_plan.steps.len(), 4);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "task_dependency");
    assert_eq!(execution_plan.steps[2].name, "test");
    assert_eq!(execution_plan.steps[3].name, "end");
}

#[test]
#[ignore]
fn create_workspace() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let task = Task::new();

    config.tasks.insert("test".to_string(), task);

    env::set_current_dir("./examples/workspace").unwrap();
    let crateinfo = environment::crateinfo::load().unwrap();
    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        crate_info: Some(&crateinfo),
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();
    env::set_current_dir("../../").unwrap();
    assert_eq!(execution_plan.steps.len(), 1);
    assert_eq!(execution_plan.steps[0].name, "workspace");
}

#[test]
#[ignore]
fn create_noworkspace() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let task = Task::new();

    config.tasks.insert("test".to_string(), task);

    env::set_current_dir("./examples/workspace").unwrap();
    let crateinfo = environment::crateinfo::load().unwrap();
    let execution_plan = ExecutionPlanBuilder {
        disable_workspace: true,
        allow_private: true,
        crate_info: Some(&crateinfo),
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();
    env::set_current_dir("../../").unwrap();
    assert_eq!(execution_plan.steps.len(), 1);
    assert_eq!(execution_plan.steps[0].name, "test");
}

#[test]
#[ignore]
fn create_task_extends_empty_env_bug_verification() {
    let config = descriptor::load(
        "./src/lib/test/makefiles/task_extend.toml",
        true,
        None,
        false,
    )
    .unwrap();

    let execution_plan = ExecutionPlanBuilder {
        disable_workspace: true,
        ..ExecutionPlanBuilder::new(&config, "task2")
    }
    .build()
    .unwrap();

    assert_eq!(execution_plan.steps.len(), 3);

    let step = execution_plan.steps[1].clone();
    assert_eq!(step.name, "task2");

    let env = step.config.env.unwrap();
    match env.get("Foo").unwrap() {
        EnvValue::Value(value) => assert_eq!(value, "foo"),
        _ => panic!("Invalid env type"),
    }
}

#[test]
fn should_skip_workspace_member_empty() {
    let skipped_members = HashSet::new();

    let skip = should_skip_workspace_member("member", &skipped_members);

    assert!(!skip);
}

#[test]
fn should_skip_workspace_member_not_found_string() {
    let mut skipped_members = HashSet::new();
    skipped_members.insert("test1".to_string());
    skipped_members.insert("test2".to_string());
    skipped_members.insert("test3".to_string());

    let skip = should_skip_workspace_member("member", &skipped_members);

    assert!(!skip);
}

#[test]
fn should_skip_workspace_member_found_string() {
    let mut skipped_members = HashSet::new();
    skipped_members.insert("test1".to_string());
    skipped_members.insert("test2".to_string());
    skipped_members.insert("member".to_string());
    skipped_members.insert("test3".to_string());

    let skip = should_skip_workspace_member("member", &skipped_members);

    assert!(skip);
}

#[test]
fn should_skip_workspace_member_not_found_glob() {
    let mut skipped_members = HashSet::new();
    skipped_members.insert("test1".to_string());
    skipped_members.insert("test2".to_string());
    skipped_members.insert("test3".to_string());
    skipped_members.insert("test/*".to_string());

    let skip = should_skip_workspace_member("test1/member", &skipped_members);

    assert!(!skip);
}

#[test]
fn should_skip_workspace_member_found_glob() {
    let mut skipped_members = HashSet::new();
    skipped_members.insert("test1".to_string());
    skipped_members.insert("test2".to_string());
    skipped_members.insert("test3".to_string());
    skipped_members.insert("members/*".to_string());

    let skip = should_skip_workspace_member("members/test", &skipped_members);

    assert!(skip);
}

#[test]
fn get_normalized_task_multi_extend() {
    let mut task1 = Task::new();
    task1.category = Some("1".to_string());
    task1.description = Some("1".to_string());
    task1.command = Some("echo".to_string());
    task1.args = Some(vec!["1".to_string()]);

    let platform_task = PlatformOverrideTask {
        clear: None,
        disabled: None,
        private: None,
        deprecated: None,
        extend: None,
        plugin: None,
        watch: None,
        condition: None,
        condition_script: None,
        condition_script_runner_args: None,
        install_crate: None,
        install_crate_args: None,
        command: None,
        ignore_errors: None,
        force: Some(true),
        env_files: None,
        env: None,
        cwd: None,
        install_script: None,
        args: None,
        script: None,
        script_runner: None,
        script_runner_args: None,
        script_extension: None,
        run_task: None,
        dependencies: None,
        toolchain: None,
    };

    let mut task2 = Task::new();
    task2.extend = Some("1".to_string());
    task2.category = Some("2".to_string());
    task2.args = Some(vec!["2".to_string()]);
    task2.linux = Some(platform_task.clone());
    task2.mac = Some(platform_task.clone());
    task2.windows = Some(platform_task.clone());

    let mut task3 = Task::new();
    task3.extend = Some("2".to_string());
    task3.args = Some(vec!["3".to_string()]);

    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    config.tasks.insert("1".to_string(), task1);
    config.tasks.insert("2".to_string(), task2);
    config.tasks.insert("3".to_string(), task3);

    let task = get_normalized_task(&config, "3", true).unwrap();

    assert_eq!(task.category.unwrap(), "2");
    assert_eq!(task.description.unwrap(), "1");
    assert_eq!(task.command.unwrap(), "echo");
    assert_eq!(task.args.unwrap(), vec!["3".to_string()]);
    assert_eq!(task.extend.unwrap(), "2");
    assert!(task.force.unwrap());
}

#[test]
fn get_normalized_task_simple() {
    let mut task1 = Task::new();
    task1.category = Some("1".to_string());
    task1.description = Some("1".to_string());
    task1.command = Some("echo".to_string());
    task1.args = Some(vec!["1".to_string()]);

    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    config.tasks.insert("1".to_string(), task1);

    let task = get_normalized_task(&config, "1", true).unwrap();

    assert_eq!(task.category.unwrap(), "1");
    assert_eq!(task.description.unwrap(), "1");
    assert_eq!(task.command.unwrap(), "echo");
    assert_eq!(task.args.unwrap(), vec!["1".to_string()]);
}

#[test]
fn respect_skip_init_end_tasks() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let task = Task::new();

    config.tasks.insert("test".to_string(), task);

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        skip_init_end_tasks: true,
        ..ExecutionPlanBuilder::new(&config, "test")
    }
    .build()
    .unwrap();
    assert_eq!(execution_plan.steps.len(), 1);
    assert_eq!(execution_plan.steps[0].name, "test");
}

#[test]
fn respect_before_each_and_after_each_tasks() {
    let config = Config {
        config: ConfigSection {
            init_task: None,
            before_each_task: Some("before_each".to_string()),
            after_each_task: Some("after_each".to_string()),
            end_task: None,
            ..ConfigSection::default()
        },
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: indexmap::indexmap! {
            "before_each".to_string() => Task::new(),
            "test2".to_string() => Task {
                dependencies: Some(vec![DependencyIdentifier::Definition(TaskIdentifier {
                    name: "test1".to_string(),
                    path: Some("./examples/workspace".to_string()),
                })]),
                ..Task::default()
            },
            "test1".to_string() => Task {
                dependencies: Some(vec![DependencyIdentifier::Definition(TaskIdentifier {
                    name: "test0".to_string(),
                    path: Some("./examples/workspace".to_string()),
                })]),
                ..Task::default()
            },
            "test0".to_string() => Task::new(),
            "after_each".to_string() => Task::new(),
        },
        plugins: None,
    };

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        skip_init_end_tasks: true,
        ..ExecutionPlanBuilder::new(&config, "test2")
    }
    .build()
    .unwrap();
    assert_eq!(execution_plan.steps.len(), 5);
    assert_eq!(
        execution_plan
            .steps
            .iter()
            .map(|step| step.name.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "test1_proxy",
            "after_each",
            "before_each",
            "test2",
            "after_each"
        ]
    );
}

#[test]
fn respect_init_and_end_and_before_each_and_after_each_tasks() {
    let config = Config {
        config: ConfigSection {
            init_task: Some("init".to_string()),
            before_each_task: Some("before_each".to_string()),
            after_each_task: Some("after_each".to_string()),
            end_task: Some("end".to_string()),
            ..ConfigSection::default()
        },
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: indexmap::indexmap! {
            "init".to_string() => Task::new(),
            "before_each".to_string() => Task::new(),
            "test2".to_string() => Task {
                dependencies: Some(vec![DependencyIdentifier::Definition(TaskIdentifier {
                    name: "test1".to_string(),
                    path: Some("./examples/workspace".to_string()),
                })]),
                ..Task::default()
            },
            "test1".to_string() => Task {
                dependencies: Some(vec![DependencyIdentifier::Definition(TaskIdentifier {
                    name: "test0".to_string(),
                    path: Some("./examples/workspace".to_string()),
                })]),
                ..Task::default()
            },
            "test0".to_string() => Task::new(),
            "after_each".to_string() => Task::new(),
            "end".to_string() => Task::new(),
        },
        plugins: None,
    };

    let execution_plan = ExecutionPlanBuilder {
        allow_private: true,
        skip_init_end_tasks: true,
        ..ExecutionPlanBuilder::new(&config, "test2")
    }
    .build()
    .unwrap();
    assert_eq!(execution_plan.steps.len(), 5);
    assert_eq!(
        execution_plan
            .steps
            .iter()
            .map(|step| step.name.as_str())
            .collect::<Vec<&str>>(),
        vec![
            "test1_proxy",
            "after_each",
            "before_each",
            "test2",
            "after_each"
        ]
    );
}
