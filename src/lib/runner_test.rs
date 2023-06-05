use super::*;
use crate::test;
use crate::types::{
    ConfigSection, CrateInfo, DeprecationInfo, EnvFile, EnvInfo, EnvValue, FlowInfo,
    RunTaskDetails, RunTaskInfo, ScriptValue, Step, Task, TaskCondition,
};
use ci_info;
use git_info::types::GitInfo;
use indexmap::IndexMap;
use rust_info::types::RustInfo;

#[cfg(target_os = "linux")]
use crate::types::WatchOptions;

#[test]
#[ignore]
#[should_panic]
fn run_flow_private() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    task.private = Some(true);
    config.tasks.insert("test".to_string(), task);

    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    run_flow(&flow_info, Rc::new(RefCell::new(FlowState::new())), false);
}

#[test]
#[ignore]
fn run_flow_private_skipped() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    task.private = Some(true);
    config.tasks.insert("test".to_string(), task);

    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: Some(Regex::new("test").unwrap()),
        cli_arguments: None,
    };

    run_flow(&flow_info, Rc::new(RefCell::new(FlowState::new())), false);
}

#[test]
#[ignore]
fn run_flow_private_sub_task() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    task.private = Some(true);
    config.tasks.insert("test".to_string(), task);

    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    run_flow(&flow_info, Rc::new(RefCell::new(FlowState::new())), true);
}

#[test]
#[ignore]
fn run_flow_allow_private() {
    let mut config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    task.private = Some(true);
    config.tasks.insert("test".to_string(), task);

    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: true,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    run_flow(&flow_info, Rc::new(RefCell::new(FlowState::new())), false);
}

#[test]
#[ignore]
#[should_panic]
fn run_task_bad_script() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit 1".to_string()]));
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
#[should_panic]
#[cfg(target_os = "linux")]
fn run_task_script_with_args_error() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: Some(vec!["1".to_string()]),
    };

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit $1".to_string()]));
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn run_task_script_with_args_valid() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: Some(vec!["0".to_string()]),
    };

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit $1".to_string()]));
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
fn run_task_command() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut task = Task::new();
    task.command = Some("echo".to_string());
    task.args = Some(vec!["test".to_string()]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
#[should_panic]
fn run_task_bad_command_valid_script() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut task = Task::new();
    task.command = Some("bad12345".to_string());
    task.script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
fn run_task_no_command_valid_script() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
#[should_panic]
fn run_task_bad_run_task_valid_command() {
    let mut sub_task = Task::new();
    sub_task.script = Some(ScriptValue::Text(vec!["exit 1".to_string()]));

    let mut tasks = IndexMap::new();
    tasks.insert("sub".to_string(), sub_task);

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut task = Task::new();
    task.run_task = Some(RunTaskInfo::Name("sub".to_string()));
    task.command = Some("echo".to_string());
    task.args = Some(vec!["test".to_string()]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
fn run_task_valid_run_task() {
    let mut sub_task = Task::new();
    sub_task.script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));

    let mut tasks = IndexMap::new();
    tasks.insert("sub".to_string(), sub_task);

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut task = Task::new();
    task.run_task = Some(RunTaskInfo::Name("sub".to_string()));
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
#[should_panic]
fn run_task_invalid_task() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    task.command = Some("echo".to_string());
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
fn run_task_set_env_file() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let env_data = envmnt::parse_file("./src/lib/test/test_files/env.env").unwrap();
    for (key, _) in env_data.clone().iter() {
        envmnt::remove(&key);
    }

    assert!(!envmnt::exists("CARGO_MAKE_ENV_FILE_TEST1"));

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    task.env_files = Some(vec![EnvFile::Path(
        "./src/lib/test/test_files/env.env".to_string(),
    )]);

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);

    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_ENV_FILE_TEST1"), "1");

    for (key, _) in env_data.iter() {
        envmnt::remove(&key);
    }
}

#[test]
#[ignore]
fn run_task_set_env() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut env = IndexMap::new();
    env.insert(
        "TEST_RUN_TASK_SET_ENV".to_string(),
        EnvValue::Value("VALID".to_string()),
    );

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    task.env = Some(env);

    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    envmnt::set("TEST_RUN_TASK_SET_ENV", "EMPTY");

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);

    assert_eq!(envmnt::get_or_panic("TEST_RUN_TASK_SET_ENV"), "VALID");
}

#[test]
#[ignore]
#[should_panic]
fn run_task_cwd_no_such_dir() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    task.cwd = Some("./bad/badagain".to_string());
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
fn run_task_cwd_dir_exists() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    task.cwd = Some("./src".to_string());
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
fn run_task_cwd_env_expansion() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    task.cwd = Some("${CARGO_MAKE_WORKING_DIRECTORY}/src".to_string());
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
fn run_task_deprecated_message() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut task = Task::new();
    task.command = Some("echo".to_string());
    task.args = Some(vec!["test".to_string()]);
    task.deprecated = Some(DeprecationInfo::Message("test message".to_string()));
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
fn run_task_deprecated_flag() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let mut task = Task::new();
    task.command = Some("echo".to_string());
    task.args = Some(vec!["test".to_string()]);
    task.deprecated = Some(DeprecationInfo::Boolean(true));
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}

#[test]
#[ignore]
fn should_watch_none_and_env_not_set() {
    envmnt::remove("CARGO_MAKE_DISABLE_WATCH");
    let task = Task::new();
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
#[ignore]
fn should_watch_none_and_env_false() {
    envmnt::set_bool("CARGO_MAKE_DISABLE_WATCH", false);
    let task = Task::new();
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
#[ignore]
fn should_watch_none_and_env_true() {
    envmnt::set_bool("CARGO_MAKE_DISABLE_WATCH", true);
    let task = Task::new();
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
#[ignore]
fn should_watch_false_and_env_not_set() {
    envmnt::remove("CARGO_MAKE_DISABLE_WATCH");
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Boolean(false));
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
#[ignore]
fn should_watch_false_and_env_false() {
    envmnt::set_bool("CARGO_MAKE_DISABLE_WATCH", false);
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Boolean(false));
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
#[ignore]
fn should_watch_false_and_env_true() {
    envmnt::set_bool("CARGO_MAKE_DISABLE_WATCH", true);
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Boolean(false));
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
#[ignore]
fn should_watch_true_and_env_not_set() {
    envmnt::remove("CARGO_MAKE_DISABLE_WATCH");
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Boolean(true));
    let watch = should_watch(&task);

    assert!(watch);
}

#[test]
#[ignore]
fn should_watch_true_and_env_false() {
    envmnt::set_bool("CARGO_MAKE_DISABLE_WATCH", false);
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Boolean(true));
    let watch = should_watch(&task);

    assert!(watch);
}

#[test]
#[ignore]
fn should_watch_true_and_env_true() {
    envmnt::set_bool("CARGO_MAKE_DISABLE_WATCH", true);
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Boolean(true));
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
#[ignore]
fn create_watch_task_name_valid() {
    let output = create_watch_task_name("test_task");

    assert_eq!(output, "test_task-watch");
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn create_watch_task_with_makefile() {
    let makefile = envmnt::get_or("CARGO_MAKE_MAKEFILE_PATH", "EMPTY");
    envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &makefile);

    let flow_info = test::create_empty_flow_info();

    let task = create_watch_task("some_task", None, &flow_info);

    match task.env.unwrap().get("CARGO_MAKE_DISABLE_WATCH").unwrap() {
        EnvValue::Value(value) => assert_eq!(value, "true"),
        _ => panic!("CARGO_MAKE_DISABLE_WATCH not defined."),
    };

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut make_command_line =
        "make --disable-check-for-updates --no-on-error --loglevel=".to_string();
    make_command_line.push_str(&log_level);
    make_command_line.push_str(" --profile=");
    make_command_line.push_str(&profile::get());
    make_command_line.push_str(" --allow-private --skip-init-end-tasks --makefile ");
    make_command_line.push_str(&makefile.clone());
    make_command_line.push_str(" some_task");

    let args = task.args.unwrap();
    assert_eq!(args.len(), 4);
    assert_eq!(args[0], "watch".to_string());
    assert_eq!(args[1], "-q".to_string());
    assert_eq!(args[2], "-x".to_string());
    assert_eq!(args[3], make_command_line.to_string());
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn create_watch_task_with_makefile_with_spaces_in_path() {
    let makefile = envmnt::get_or("CARGO_MAKE_MAKEFILE_PATH", "EMPTY");
    let test_makefile = "/path with spaces/mymakefile.toml";
    envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &test_makefile);

    let flow_info = test::create_empty_flow_info();

    let task = create_watch_task("some_task", None, &flow_info);
    envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &makefile);

    match task.env.unwrap().get("CARGO_MAKE_DISABLE_WATCH").unwrap() {
        EnvValue::Value(value) => assert_eq!(value, "true"),
        _ => panic!("CARGO_MAKE_DISABLE_WATCH not defined."),
    };

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut make_command_line =
        "make --disable-check-for-updates --no-on-error --loglevel=".to_string();
    make_command_line.push_str(&log_level);
    make_command_line.push_str(" --profile=");
    make_command_line.push_str(&profile::get());
    make_command_line.push_str(
        " --allow-private --skip-init-end-tasks --makefile \"/path with spaces/mymakefile.toml\" some_task"
    );

    let args = task.args.unwrap();
    assert_eq!(args.len(), 4);
    assert_eq!(args[0], "watch".to_string());
    assert_eq!(args[1], "-q".to_string());
    assert_eq!(args[2], "-x".to_string());
    assert_eq!(args[3], make_command_line.to_string());
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn create_watch_task_with_makefile_and_bool_options() {
    let makefile = envmnt::get_or("CARGO_MAKE_MAKEFILE_PATH", "EMPTY");
    envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &makefile);

    let flow_info = test::create_empty_flow_info();

    let task = create_watch_task(
        "some_task",
        Some(TaskWatchOptions::Boolean(true)),
        &flow_info,
    );

    match task.env.unwrap().get("CARGO_MAKE_DISABLE_WATCH").unwrap() {
        EnvValue::Value(value) => assert_eq!(value, "true"),
        _ => panic!("CARGO_MAKE_DISABLE_WATCH not defined."),
    };

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut make_command_line =
        "make --disable-check-for-updates --no-on-error --loglevel=".to_string();
    make_command_line.push_str(&log_level);
    make_command_line.push_str(" --profile=");
    make_command_line.push_str(&profile::get());
    make_command_line.push_str(" --allow-private --skip-init-end-tasks --makefile ");
    make_command_line.push_str(&makefile.clone());
    make_command_line.push_str(" some_task");

    let args = task.args.unwrap();
    assert_eq!(args.len(), 4);
    assert_eq!(args[0], "watch".to_string());
    assert_eq!(args[1], "-q".to_string());
    assert_eq!(args[2], "-x".to_string());
    assert_eq!(args[3], make_command_line.to_string());
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn create_watch_task_with_makefile_and_empty_object_options() {
    let makefile = envmnt::get_or("CARGO_MAKE_MAKEFILE_PATH", "EMPTY");
    envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &makefile);

    let watch_options = WatchOptions {
        version: None,
        postpone: None,
        ignore_pattern: None,
        no_git_ignore: None,
        why: None,
        watch: None,
    };

    let flow_info = test::create_empty_flow_info();

    let task = create_watch_task(
        "some_task",
        Some(TaskWatchOptions::Options(watch_options)),
        &flow_info,
    );

    assert!(task.install_crate_args.is_some());
    assert_eq!(task.install_crate_args.unwrap().len(), 2);

    match task.env.unwrap().get("CARGO_MAKE_DISABLE_WATCH").unwrap() {
        EnvValue::Value(value) => assert_eq!(value, "true"),
        _ => panic!("CARGO_MAKE_DISABLE_WATCH not defined."),
    };

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut make_command_line =
        "make --disable-check-for-updates --no-on-error --loglevel=".to_string();
    make_command_line.push_str(&log_level);
    make_command_line.push_str(" --profile=");
    make_command_line.push_str(&profile::get());
    make_command_line.push_str(" --allow-private --skip-init-end-tasks --makefile ");
    make_command_line.push_str(&makefile.clone());
    make_command_line.push_str(" some_task");

    let args = task.args.unwrap();
    assert_eq!(args.len(), 4);
    assert_eq!(args[0], "watch".to_string());
    assert_eq!(args[1], "-q".to_string());
    assert_eq!(args[2], "-x".to_string());
    assert_eq!(args[3], make_command_line.to_string());
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn create_watch_task_with_makefile_and_all_object_options_and_cli_args() {
    let makefile = envmnt::get_or("CARGO_MAKE_MAKEFILE_PATH", "EMPTY");
    envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &makefile);

    let watch_options = WatchOptions {
        version: Some("100.200.300.400".to_string()),
        postpone: Some(true),
        ignore_pattern: Some("tools/*".to_string()),
        no_git_ignore: Some(true),
        why: Some(true),
        watch: Some(vec!["dir1".to_string(), "dir2".to_string()]),
    };

    let mut flow_info = test::create_empty_flow_info();
    flow_info.cli_arguments = Some(vec!["1".to_string(), "2".to_string(), "3 4".to_string()]);

    let task = create_watch_task(
        "some_task",
        Some(TaskWatchOptions::Options(watch_options)),
        &flow_info,
    );

    assert!(task.install_crate_args.is_some());
    let install_crate_args = task.install_crate_args.unwrap();
    assert_eq!(install_crate_args[0], "--version");
    assert_eq!(install_crate_args[1], "100.200.300.400");

    match task.env.unwrap().get("CARGO_MAKE_DISABLE_WATCH").unwrap() {
        EnvValue::Value(value) => assert_eq!(value, "true"),
        _ => panic!("CARGO_MAKE_DISABLE_WATCH not defined."),
    };

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut make_command_line =
        "make --disable-check-for-updates --no-on-error --loglevel=".to_string();
    make_command_line.push_str(&log_level);
    make_command_line.push_str(" --profile=");
    make_command_line.push_str(&profile::get());
    make_command_line.push_str(" --allow-private --skip-init-end-tasks --makefile ");
    make_command_line.push_str(&makefile.clone());
    make_command_line.push_str(" some_task 1 2 \"3 4\"");

    let args = task.args.unwrap();
    assert_eq!(args.len(), 12);
    assert_eq!(args[0], "watch".to_string());
    assert_eq!(args[1], "--why".to_string());
    assert_eq!(args[2], "--postpone".to_string());
    assert_eq!(args[3], "-i".to_string());
    assert_eq!(args[4], "tools/*".to_string());
    assert_eq!(args[5], "--no-gitignore".to_string());
    assert_eq!(args[6], "-w".to_string());
    assert_eq!(args[7], "dir1".to_string());
    assert_eq!(args[8], "-w".to_string());
    assert_eq!(args[9], "dir2".to_string());
    assert_eq!(args[10], "-x".to_string());
    assert_eq!(args[11], make_command_line.to_string());
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn create_watch_task_with_makefile_and_all_object_options() {
    let makefile = envmnt::get_or("CARGO_MAKE_MAKEFILE_PATH", "EMPTY");
    envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &makefile);

    let watch_options = WatchOptions {
        version: Some("100.200.300.400".to_string()),
        postpone: Some(true),
        ignore_pattern: Some("tools/*".to_string()),
        no_git_ignore: Some(true),
        why: Some(true),
        watch: Some(vec!["dir1".to_string(), "dir2".to_string()]),
    };

    let flow_info = test::create_empty_flow_info();

    let task = create_watch_task(
        "some_task",
        Some(TaskWatchOptions::Options(watch_options)),
        &flow_info,
    );

    assert!(task.install_crate_args.is_some());
    let install_crate_args = task.install_crate_args.unwrap();
    assert_eq!(install_crate_args[0], "--version");
    assert_eq!(install_crate_args[1], "100.200.300.400");

    match task.env.unwrap().get("CARGO_MAKE_DISABLE_WATCH").unwrap() {
        EnvValue::Value(value) => assert_eq!(value, "true"),
        _ => panic!("CARGO_MAKE_DISABLE_WATCH not defined."),
    };

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut make_command_line =
        "make --disable-check-for-updates --no-on-error --loglevel=".to_string();
    make_command_line.push_str(&log_level);
    make_command_line.push_str(" --profile=");
    make_command_line.push_str(&profile::get());
    make_command_line.push_str(" --allow-private --skip-init-end-tasks --makefile ");
    make_command_line.push_str(&makefile.clone());
    make_command_line.push_str(" some_task");

    let args = task.args.unwrap();
    assert_eq!(args.len(), 12);
    assert_eq!(args[0], "watch".to_string());
    assert_eq!(args[1], "--why".to_string());
    assert_eq!(args[2], "--postpone".to_string());
    assert_eq!(args[3], "-i".to_string());
    assert_eq!(args[4], "tools/*".to_string());
    assert_eq!(args[5], "--no-gitignore".to_string());
    assert_eq!(args[6], "-w".to_string());
    assert_eq!(args[7], "dir1".to_string());
    assert_eq!(args[8], "-w".to_string());
    assert_eq!(args[9], "dir2".to_string());
    assert_eq!(args[10], "-x".to_string());
    assert_eq!(args[11], make_command_line.to_string());
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn create_watch_task_with_makefile_and_false_object_options() {
    let makefile = envmnt::get_or("CARGO_MAKE_MAKEFILE_PATH", "EMPTY");
    envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &makefile);

    let watch_options = WatchOptions {
        version: None,
        postpone: Some(false),
        ignore_pattern: None,
        no_git_ignore: Some(false),
        why: Some(false),
        watch: None,
    };

    let flow_info = test::create_empty_flow_info();

    let task = create_watch_task(
        "some_task",
        Some(TaskWatchOptions::Options(watch_options)),
        &flow_info,
    );

    assert!(task.install_crate_args.is_some());
    assert_eq!(task.install_crate_args.unwrap().len(), 2);

    match task.env.unwrap().get("CARGO_MAKE_DISABLE_WATCH").unwrap() {
        EnvValue::Value(value) => assert_eq!(value, "true"),
        _ => panic!("CARGO_MAKE_DISABLE_WATCH not defined."),
    };

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut make_command_line =
        "make --disable-check-for-updates --no-on-error --loglevel=".to_string();
    make_command_line.push_str(&log_level);
    make_command_line.push_str(" --profile=");
    make_command_line.push_str(&profile::get());
    make_command_line.push_str(" --allow-private --skip-init-end-tasks --makefile ");
    make_command_line.push_str(&makefile.clone());
    make_command_line.push_str(" some_task");

    let args = task.args.unwrap();
    assert_eq!(args.len(), 4);
    assert_eq!(args[0], "watch".to_string());
    assert_eq!(args[1], "-q".to_string());
    assert_eq!(args[2], "-x".to_string());
    assert_eq!(args[3], make_command_line.to_string());
}

#[test]
fn create_watch_step_valid() {
    let flow_info = test::create_empty_flow_info();

    let step = create_watch_step("test_watch_step", None, &flow_info);
    let task = step.config;

    assert_eq!(&step.name, "test_watch_step-watch");

    match task.env.unwrap().get("CARGO_MAKE_DISABLE_WATCH").unwrap() {
        EnvValue::Value(value) => assert_eq!(value, "true"),
        _ => panic!("CARGO_MAKE_DISABLE_WATCH not defined."),
    };

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let args = task.args.unwrap();
    assert_eq!(args.len(), 4);
}

#[test]
#[ignore]
fn run_sub_task_and_report_for_name() {
    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Name("test".to_string());

    let output = run_sub_task_and_report(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &sub_task,
    );

    assert!(output);
}

#[test]
#[ignore]
#[should_panic]
fn run_sub_task_and_report_for_name_not_found() {
    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Name("test2".to_string());

    run_sub_task_and_report(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &sub_task,
    );
}

#[test]
#[ignore]
fn run_sub_task_and_report_for_details_single() {
    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Details(RunTaskDetails {
        name: RunTaskName::Single("test".to_string()),
        fork: Some(false),
        parallel: None,
        cleanup_task: None,
    });

    let output = run_sub_task_and_report(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &sub_task,
    );

    assert!(output);
}

#[test]
#[ignore]
fn run_sub_task_and_report_for_details_multiple() {
    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let mut tasks = IndexMap::new();
    tasks.insert("test1".to_string(), task.clone());
    tasks.insert("test2".to_string(), task.clone());

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Details(RunTaskDetails {
        name: RunTaskName::Multiple(vec!["test1".to_string(), "test2".to_string()]),
        fork: Some(false),
        parallel: None,
        cleanup_task: None,
    });

    let output = run_sub_task_and_report(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &sub_task,
    );

    assert!(output);
}

#[test]
#[ignore]
fn run_sub_task_and_report_routing_empty() {
    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Routing(vec![]);

    let output = run_sub_task_and_report(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &sub_task,
    );

    assert!(!output);
}

#[test]
#[ignore]
fn run_sub_task_and_report_routing_no_condition() {
    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Routing(vec![RunTaskRoutingInfo {
        name: RunTaskName::Single("test".to_string()),
        fork: None,
        parallel: None,
        cleanup_task: None,
        condition: None,
        condition_script: None,
    }]);

    let output = run_sub_task_and_report(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &sub_task,
    );

    assert!(output);
}

#[test]
#[ignore]
fn run_sub_task_and_report_routing_condition_not_met() {
    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Routing(vec![RunTaskRoutingInfo {
        name: RunTaskName::Single("test".to_string()),
        fork: None,
        parallel: None,
        cleanup_task: None,
        condition: Some(TaskCondition {
            fail_message: None,
            profiles: None,
            platforms: None,
            channels: Some(vec!["bad1".to_string(), "bad2".to_string()]),
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
            files_modified: None,
        }),
        condition_script: None,
    }]);

    let output = run_sub_task_and_report(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &sub_task,
    );

    assert!(!output);
}

#[test]
#[ignore]
#[should_panic]
fn run_sub_task_and_report_routing_not_found() {
    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Routing(vec![RunTaskRoutingInfo {
        name: RunTaskName::Single("test2".to_string()),
        fork: None,
        parallel: None,
        cleanup_task: None,
        condition: None,
        condition_script: None,
    }]);

    run_sub_task_and_report(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &sub_task,
    );
}

#[test]
#[ignore]
#[should_panic]
fn run_sub_task_and_report_with_cleanup_task_but_no_fork() {
    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["echo test".to_string()]));

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Details(RunTaskDetails {
        name: RunTaskName::Single("test".to_string()),
        fork: Some(false),
        parallel: None,
        cleanup_task: Some("test".to_string()),
    });

    run_sub_task_and_report(
        &flow_info,
        Rc::new(RefCell::new(FlowState::new())),
        &sub_task,
    );
}

#[test]
fn get_sub_task_info_for_routing_info_empty() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let (task_name, fork, parallel, cleanup_task) =
        get_sub_task_info_for_routing_info(&flow_info, &vec![]);

    assert!(task_name.is_none());
    assert!(!fork);
    assert!(!parallel);
    assert!(cleanup_task.is_none());
}

#[test]
fn get_sub_task_info_for_routing_info_condition_not_met() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let (task_name, fork, parallel, cleanup_task) = get_sub_task_info_for_routing_info(
        &flow_info,
        &vec![RunTaskRoutingInfo {
            name: RunTaskName::Single("test".to_string()),
            fork: None,
            parallel: None,
            cleanup_task: None,
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: None,
                platforms: None,
                channels: Some(vec!["bad1".to_string(), "bad2".to_string()]),
                env_set: None,
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
                files_modified: None,
            }),
            condition_script: None,
        }],
    );

    assert!(task_name.is_none());
    assert!(!fork);
    assert!(!parallel);
    assert!(cleanup_task.is_none());
}

#[test]
fn get_sub_task_info_for_routing_info_condition_found() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let (task_name, fork, parallel, cleanup_task) = get_sub_task_info_for_routing_info(
        &flow_info,
        &vec![RunTaskRoutingInfo {
            name: RunTaskName::Single("test".to_string()),
            fork: None,
            parallel: None,
            cleanup_task: None,
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: None,
                platforms: None,
                channels: None,
                env_set: Some(vec!["CARGO_MAKE".to_string()]),
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
                files_modified: None,
            }),
            condition_script: None,
        }],
    );

    assert_eq!(task_name.unwrap(), vec!["test"]);
    assert!(!fork);
    assert!(!parallel);
    assert!(cleanup_task.is_none());
}

#[test]
fn get_sub_task_info_for_routing_info_condition_found_multiple_tasks() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let (task_name, fork, parallel, cleanup_task) = get_sub_task_info_for_routing_info(
        &flow_info,
        &vec![RunTaskRoutingInfo {
            name: RunTaskName::Multiple(vec!["test1".to_string(), "test2".to_string()]),
            fork: None,
            parallel: None,
            cleanup_task: None,
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: None,
                platforms: None,
                channels: None,
                env_set: Some(vec!["CARGO_MAKE".to_string()]),
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
                files_modified: None,
            }),
            condition_script: None,
        }],
    );

    assert_eq!(task_name.unwrap(), vec!["test1", "test2"]);
    assert!(!fork);
    assert!(!parallel);
    assert!(cleanup_task.is_none());
}

#[test]
fn get_sub_task_info_for_routing_info_script_not_met() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let (task_name, fork, parallel, cleanup_task) = get_sub_task_info_for_routing_info(
        &flow_info,
        &vec![RunTaskRoutingInfo {
            name: RunTaskName::Single("test".to_string()),
            fork: None,
            parallel: None,
            cleanup_task: None,
            condition: None,
            condition_script: Some(vec!["exit 1".to_string()]),
        }],
    );

    assert!(task_name.is_none());
    assert!(!fork);
    assert!(!parallel);
    assert!(cleanup_task.is_none());
}

#[test]
fn get_sub_task_info_for_routing_info_script_found() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let (task_name, fork, parallel, cleanup_task) = get_sub_task_info_for_routing_info(
        &flow_info,
        &vec![RunTaskRoutingInfo {
            name: RunTaskName::Single("test".to_string()),
            fork: None,
            parallel: None,
            cleanup_task: None,
            condition: None,
            condition_script: Some(vec!["exit 0".to_string()]),
        }],
    );

    assert_eq!(task_name.unwrap(), vec!["test"]);
    assert!(!fork);
    assert!(!parallel);
    assert!(cleanup_task.is_none());
}

#[test]
fn get_sub_task_info_for_routing_info_multiple_found() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let (task_name, fork, parallel, cleanup_task) = get_sub_task_info_for_routing_info(
        &flow_info,
        &vec![
            RunTaskRoutingInfo {
                name: RunTaskName::Single("test1".to_string()),
                fork: None,
                parallel: None,
                cleanup_task: None,
                condition: Some(TaskCondition {
                    fail_message: None,
                    profiles: None,
                    platforms: None,
                    channels: None,
                    env_set: Some(vec!["CARGO_MAKE".to_string()]),
                    env_not_set: None,
                    env_true: None,
                    env_false: None,
                    env: None,
                    env_contains: None,
                    rust_version: None,
                    files_exist: None,
                    files_not_exist: None,
                    files_modified: None,
                }),
                condition_script: None,
            },
            RunTaskRoutingInfo {
                name: RunTaskName::Single("test2".to_string()),
                fork: None,
                parallel: None,
                cleanup_task: None,
                condition: None,
                condition_script: Some(vec!["exit 0".to_string()]),
            },
        ],
    );

    assert_eq!(task_name.unwrap(), vec!["test1"]);
    assert!(!fork);
    assert!(!parallel);
    assert!(cleanup_task.is_none());
}

#[test]
fn get_sub_task_info_for_routing_info_default() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let (task_name, fork, parallel, cleanup_task) = get_sub_task_info_for_routing_info(
        &flow_info,
        &vec![
            RunTaskRoutingInfo {
                name: RunTaskName::Single("test1".to_string()),
                fork: None,
                parallel: None,
                cleanup_task: None,
                condition: Some(TaskCondition {
                    fail_message: None,
                    profiles: None,
                    platforms: None,
                    channels: None,
                    env_set: None,
                    env_not_set: Some(vec!["CARGO_MAKE".to_string()]),
                    env_true: None,
                    env_false: None,
                    env: None,
                    env_contains: None,
                    rust_version: None,
                    files_exist: None,
                    files_not_exist: None,
                    files_modified: None,
                }),
                condition_script: None,
            },
            RunTaskRoutingInfo {
                name: RunTaskName::Single("test2".to_string()),
                fork: None,
                parallel: None,
                cleanup_task: None,
                condition: None,
                condition_script: Some(vec!["exit 1".to_string()]),
            },
            RunTaskRoutingInfo {
                name: RunTaskName::Single("default".to_string()),
                fork: None,
                parallel: None,
                cleanup_task: None,
                condition: None,
                condition_script: None,
            },
        ],
    );

    assert_eq!(task_name.unwrap(), vec!["default"]);
    assert!(!fork);
    assert!(!parallel);
    assert!(cleanup_task.is_none());
}

#[test]
fn get_sub_task_info_for_routing_info_multiple() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let (task_name, fork, parallel, cleanup_task) = get_sub_task_info_for_routing_info(
        &flow_info,
        &vec![
            RunTaskRoutingInfo {
                name: RunTaskName::Single("test1".to_string()),
                fork: None,
                parallel: None,
                cleanup_task: None,
                condition: Some(TaskCondition {
                    fail_message: None,
                    profiles: None,
                    platforms: None,
                    channels: None,
                    env_set: None,
                    env_not_set: Some(vec!["CARGO_MAKE".to_string()]),
                    env_true: None,
                    env_false: None,
                    env: None,
                    env_contains: None,
                    rust_version: None,
                    files_exist: None,
                    files_not_exist: None,
                    files_modified: None,
                }),
                condition_script: None,
            },
            RunTaskRoutingInfo {
                name: RunTaskName::Single("test2".to_string()),
                fork: None,
                parallel: None,
                cleanup_task: None,
                condition: None,
                condition_script: Some(vec!["exit 1".to_string()]),
            },
            RunTaskRoutingInfo {
                name: RunTaskName::Single("test3".to_string()),
                fork: None,
                parallel: None,
                cleanup_task: None,
                condition: None,
                condition_script: Some(vec!["exit 0".to_string()]),
            },
            RunTaskRoutingInfo {
                name: RunTaskName::Single("default".to_string()),
                fork: None,
                parallel: None,
                cleanup_task: None,
                condition: None,
                condition_script: None,
            },
        ],
    );

    assert_eq!(task_name.unwrap(), vec!["test3"]);
    assert!(!fork);
    assert!(!parallel);
    assert!(cleanup_task.is_none());
}

#[test]
fn get_sub_task_info_for_routing_info_fork_false() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let (task_name, fork, parallel, cleanup_task) = get_sub_task_info_for_routing_info(
        &flow_info,
        &vec![RunTaskRoutingInfo {
            name: RunTaskName::Single("test".to_string()),
            fork: Some(false),
            parallel: None,
            cleanup_task: None,
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: None,
                platforms: None,
                channels: None,
                env_set: Some(vec!["CARGO_MAKE".to_string()]),
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
                files_modified: None,
            }),
            condition_script: None,
        }],
    );

    assert_eq!(task_name.unwrap(), vec!["test"]);
    assert!(!fork);
    assert!(!parallel);
    assert!(cleanup_task.is_none());
}

#[test]
fn get_sub_task_info_for_routing_info_fork_true() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let (task_name, fork, parallel, cleanup_task) = get_sub_task_info_for_routing_info(
        &flow_info,
        &vec![RunTaskRoutingInfo {
            name: RunTaskName::Single("test".to_string()),
            fork: Some(true),
            parallel: None,
            cleanup_task: None,
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: None,
                platforms: None,
                channels: None,
                env_set: Some(vec!["CARGO_MAKE".to_string()]),
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
                files_modified: None,
            }),
            condition_script: None,
        }],
    );

    assert_eq!(task_name.unwrap(), vec!["test"]);
    assert!(fork);
    assert!(!parallel);
    assert!(cleanup_task.is_none());
}

#[test]
fn create_fork_step_valid() {
    let config = Config {
        config: ConfigSection::new(),
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks: IndexMap::new(),
        plugins: None,
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    let step = create_fork_step(&flow_info);
    let task = step.config;

    assert_eq!(step.name, "cargo_make_run_fork");
    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut log_level_arg = "--loglevel=".to_string();
    log_level_arg.push_str(&log_level);

    let mut profile_arg = "--profile=".to_string();
    profile_arg.push_str(&profile::get());

    let makefile = envmnt::get_or("CARGO_MAKE_MAKEFILE_PATH", "EMPTY");

    let args = task.args.unwrap();
    assert_eq!(args.len(), 10);
    assert_eq!(args[0], "make".to_string());
    assert_eq!(args[1], "--disable-check-for-updates".to_string());
    assert_eq!(args[2], "--no-on-error".to_string());
    assert_eq!(args[3], log_level_arg.to_string());
    assert_eq!(args[4], profile_arg.to_string());
    assert_eq!(args[5], "--allow-private".to_string());
    assert_eq!(args[6], "--skip-init-end-tasks".to_string());
    assert_eq!(args[7], "--makefile".to_string());
    assert_eq!(args[8], makefile);
    assert_eq!(args[9], "test".to_string());
}
