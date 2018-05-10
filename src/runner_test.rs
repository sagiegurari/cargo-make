use super::*;
use rust_info::types::RustInfo;
use indexmap::IndexMap;
use std::env;
use types::{ConfigSection, CrateInfo, EnvInfo, EnvValue, FlowInfo, GitInfo, PlatformOverrideTask,
            Step, Task, Workspace};

#[test]
#[should_panic]
fn get_task_name_not_found() {
    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    get_task_name(&config, "test");
}

#[test]
fn get_task_name_no_alias() {
    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    config.tasks.insert("test".to_string(), Task::new());

    let name = get_task_name(&config, "test");

    assert_eq!(name, "test");
}

#[test]
fn get_task_name_alias() {
    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    let mut task = Task::new();
    task.alias = Some("test2".to_string());
    config.tasks.insert("test".to_string(), task);

    config.tasks.insert("test2".to_string(), Task::new());

    let name = get_task_name(&config, "test");

    assert_eq!(name, "test2");
}

#[test]
fn get_task_name_platform_alias() {
    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
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

    let name = get_task_name(&config, "test");

    assert_eq!(name, "test2");
}

#[test]
fn create_execution_plan_single() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let task = Task::new();

    config.tasks.insert("test".to_string(), task);

    let execution_plan = create_execution_plan(&config, "test", false);
    assert_eq!(execution_plan.steps.len(), 3);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "test");
    assert_eq!(execution_plan.steps[2].name, "end");
}

#[test]
fn create_execution_plan_single_disabled() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.disabled = Some(true);

    config.tasks.insert("test".to_string(), task);

    let execution_plan = create_execution_plan(&config, "test", false);
    assert_eq!(execution_plan.steps.len(), 2);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "end");
}

#[test]
fn create_execution_plan_with_dependencies() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.dependencies = Some(vec!["task_dependency".to_string()]);

    let task_dependency = Task::new();

    config.tasks.insert("test".to_string(), task);
    config
        .tasks
        .insert("task_dependency".to_string(), task_dependency);

    let execution_plan = create_execution_plan(&config, "test", false);
    assert_eq!(execution_plan.steps.len(), 4);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "task_dependency");
    assert_eq!(execution_plan.steps[2].name, "test");
    assert_eq!(execution_plan.steps[3].name, "end");
}

#[test]
fn create_execution_plan_disabled_task_with_dependencies() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.disabled = Some(true);
    task.dependencies = Some(vec!["task_dependency".to_string()]);

    let task_dependency = Task::new();

    config.tasks.insert("test".to_string(), task);
    config
        .tasks
        .insert("task_dependency".to_string(), task_dependency);

    let execution_plan = create_execution_plan(&config, "test", false);
    assert_eq!(execution_plan.steps.len(), 2);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "end");
}

#[test]
fn create_execution_plan_with_dependencies_disabled() {
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    let mut config = Config {
        config: config_section,
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    config.tasks.insert("init".to_string(), Task::new());
    config.tasks.insert("end".to_string(), Task::new());

    let mut task = Task::new();
    task.dependencies = Some(vec!["task_dependency".to_string()]);

    let mut task_dependency = Task::new();
    task_dependency.disabled = Some(true);

    config.tasks.insert("test".to_string(), task);
    config
        .tasks
        .insert("task_dependency".to_string(), task_dependency);

    let execution_plan = create_execution_plan(&config, "test", false);
    assert_eq!(execution_plan.steps.len(), 3);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "test");
    assert_eq!(execution_plan.steps[2].name, "end");
}

#[test]
fn create_execution_plan_platform_disabled() {
    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    let mut task = Task::new();
    task.linux = Some(PlatformOverrideTask {
        clear: Some(true),
        disabled: Some(true),
        condition: None,
        condition_script: None,
        install_crate: None,
        install_crate_args: None,
        command: None,
        force: None,
        env: None,
        cwd: None,
        install_script: None,
        args: None,
        script: None,
        script_runner: None,
        run_task: None,
        dependencies: None,
    });
    task.windows = Some(PlatformOverrideTask {
        clear: Some(true),
        disabled: Some(true),
        condition: None,
        condition_script: None,
        install_crate: None,
        install_crate_args: None,
        command: None,
        force: None,
        env: None,
        cwd: None,
        install_script: None,
        args: None,
        script: None,
        script_runner: None,
        run_task: None,
        dependencies: None,
    });
    task.mac = Some(PlatformOverrideTask {
        clear: Some(true),
        disabled: Some(true),
        condition: None,
        condition_script: None,
        install_crate: None,
        install_crate_args: None,
        command: None,
        force: None,
        env: None,
        cwd: None,
        install_script: None,
        args: None,
        script: None,
        script_runner: None,
        run_task: None,
        dependencies: None,
    });

    config.tasks.insert("test".to_string(), task);

    let execution_plan = create_execution_plan(&config, "test", false);
    assert_eq!(execution_plan.steps.len(), 0);
}

#[test]
fn create_execution_plan_workspace() {
    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    let task = Task::new();

    config.tasks.insert("test".to_string(), task);

    env::set_current_dir("./examples/workspace").unwrap();
    let execution_plan = create_execution_plan(&config, "test", false);
    env::set_current_dir("../../").unwrap();
    assert_eq!(execution_plan.steps.len(), 1);
    assert_eq!(execution_plan.steps[0].name, "workspace");
}

#[test]
fn create_execution_plan_noworkspace() {
    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    let task = Task::new();

    config.tasks.insert("test".to_string(), task);

    env::set_current_dir("./examples/workspace").unwrap();
    let execution_plan = create_execution_plan(&config, "test", true);
    env::set_current_dir("../../").unwrap();
    assert_eq!(execution_plan.steps.len(), 1);
    assert_eq!(execution_plan.steps[0].name, "test");
}

#[test]
fn create_workspace_task_no_members() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
    });

    let task = create_workspace_task(crate_info, "some_task");

    assert!(task.script.is_some());
    let script = task.script.unwrap();
    assert_eq!(script.join("\n"), "".to_string());
    assert!(task.env.is_none());
}

#[test]
#[cfg(target_os = "linux")]
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
    });

    let task = create_workspace_task(crate_info, "some_task");

    let mut expected_script = r#"cd ./member1
cargo make --disable-check-for-updates --loglevel=LEVEL_NAME some_task
cd -
cd ./member2
cargo make --disable-check-for-updates --loglevel=LEVEL_NAME some_task
cd -
cd ./dir1/member3
cargo make --disable-check-for-updates --loglevel=LEVEL_NAME some_task
cd -"#.to_string();

    let log_level = logger::get_log_level();
    expected_script = str::replace(&expected_script, "LEVEL_NAME", &log_level);

    assert!(task.script.is_some());
    let script = task.script.unwrap();
    assert_eq!(script.join("\n"), expected_script);
    assert!(task.env.is_none());
}

#[test]
fn create_workspace_task_extend_workspace_makefile() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
    });

    env::set_var("CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE", "true");
    let task = create_workspace_task(crate_info, "some_task");
    env::set_var("CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE", "false");

    assert!(task.script.is_some());
    let script = task.script.unwrap();
    assert_eq!(script.join("\n"), "".to_string());
    assert!(task.env.is_some());
    assert!(
        task.env
            .unwrap()
            .get("CARGO_MAKE_WORKSPACE_MAKEFILE")
            .is_some()
    );
}

#[test]
#[should_panic]
fn run_task_bad_script() {
    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
    };

    let mut task = Task::new();
    task.script = Some(vec!["exit 1".to_string()]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, &step);
}

#[test]
fn run_task_command() {
    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
    };

    let mut task = Task::new();
    task.command = Some("echo".to_string());
    task.args = Some(vec!["test".to_string()]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, &step);
}

#[test]
#[should_panic]
fn run_task_bad_command_valid_script() {
    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
    };

    let mut task = Task::new();
    task.command = Some("bad12345".to_string());
    task.script = Some(vec!["exit 0".to_string()]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, &step);
}

#[test]
fn run_task_no_command_valid_script() {
    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
    };

    let mut task = Task::new();
    task.script = Some(vec!["exit 0".to_string()]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, &step);
}

#[test]
#[should_panic]
fn run_task_bad_run_task_valid_command() {
    let mut sub_task = Task::new();
    sub_task.script = Some(vec!["exit 1".to_string()]);

    let mut tasks = IndexMap::new();
    tasks.insert("sub".to_string(), sub_task);

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
    };

    let mut task = Task::new();
    task.run_task = Some("sub".to_string());
    task.command = Some("echo".to_string());
    task.args = Some(vec!["test".to_string()]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, &step);
}

#[test]
fn run_task_valid_run_task() {
    let mut sub_task = Task::new();
    sub_task.script = Some(vec!["exit 0".to_string()]);

    let mut tasks = IndexMap::new();
    tasks.insert("sub".to_string(), sub_task);

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
    };

    let mut task = Task::new();
    task.run_task = Some("sub".to_string());
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, &step);
}

#[test]
#[should_panic]
fn run_task_invalid_task() {
    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
    };

    let mut task = Task::new();
    task.script = Some(vec!["exit 0".to_string()]);
    task.command = Some("echo".to_string());
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, &step);
}

#[test]
fn run_task_set_env() {
    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
    };

    let mut env = IndexMap::new();
    env.insert(
        "TEST_RUN_TASK_SET_ENV".to_string(),
        EnvValue::Value("VALID".to_string()),
    );

    let mut task = Task::new();
    task.script = Some(vec!["exit 0".to_string()]);
    task.env = Some(env);

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    env::set_var("TEST_RUN_TASK_SET_ENV", "EMPTY");

    run_task(&flow_info, &step);

    assert_eq!(env::var("TEST_RUN_TASK_SET_ENV").unwrap(), "VALID");
}

#[test]
#[should_panic]
fn run_task_cwd_no_such_dir() {
    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
    };

    let mut task = Task::new();
    task.script = Some(vec!["exit 0".to_string()]);
    task.cwd = Some("./bad/badagain".to_string());
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, &step);
}

#[test]
fn run_task_cwd_dir_exists() {
    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
    };

    let mut task = Task::new();
    task.script = Some(vec!["exit 0".to_string()]);
    task.cwd = Some("./src".to_string());
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, &step);
}

#[test]
fn get_skipped_workspace_members_not_defined_or_empty() {
    let members = get_skipped_workspace_members("".to_string());

    assert_eq!(members.len(), 0);
}

#[test]
fn get_skipped_workspace_members_single() {
    let members = get_skipped_workspace_members("test".to_string());

    assert_eq!(members.len(), 1);
    assert!(members.contains(&"test".to_string()));
}

#[test]
fn get_skipped_workspace_members_multiple() {
    let members = get_skipped_workspace_members("test1;test2;test3".to_string());

    assert_eq!(members.len(), 3);
    assert!(members.contains(&"test1".to_string()));
    assert!(members.contains(&"test2".to_string()));
    assert!(members.contains(&"test3".to_string()));
}
