use super::*;
use crate::types::{
    ConfigSection, CrateInfo, EnvInfo, EnvValue, FlowInfo, GitInfo, PlatformOverrideTask,
    RunTaskInfo, Step, Task, TaskCondition,
};
use ci_info;
use indexmap::IndexMap;
use rust_info::types::RustInfo;
use std::env;

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

    let execution_plan = create_execution_plan(&config, "test", false, true);
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

    let execution_plan = create_execution_plan(&config, "test", false, true);
    assert_eq!(execution_plan.steps.len(), 2);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "end");
}

#[test]
#[should_panic]
fn create_execution_plan_single_private() {
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
    task.private = Some(true);

    config.tasks.insert("test-private".to_string(), task);

    create_execution_plan(&config, "test-private", false, false);
}

#[test]
fn create_execution_plan_single_allow_private() {
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
    task.private = Some(true);

    config.tasks.insert("test-private".to_string(), task);

    let execution_plan = create_execution_plan(&config, "test-private", false, true);
    assert_eq!(execution_plan.steps.len(), 3);
    assert_eq!(execution_plan.steps[0].name, "init");
    assert_eq!(execution_plan.steps[1].name, "test-private");
    assert_eq!(execution_plan.steps[2].name, "end");
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

    let execution_plan = create_execution_plan(&config, "test", false, true);
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

    let execution_plan = create_execution_plan(&config, "test", false, true);
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

    let execution_plan = create_execution_plan(&config, "test", false, true);
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
        private: Some(false),
        watch: Some(false),
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
        script_extension: None,
        run_task: None,
        dependencies: None,
        toolchain: None,
    });
    task.windows = Some(PlatformOverrideTask {
        clear: Some(true),
        disabled: Some(true),
        private: Some(false),
        watch: Some(false),
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
        script_extension: None,
        run_task: None,
        dependencies: None,
        toolchain: None,
    });
    task.mac = Some(PlatformOverrideTask {
        clear: Some(true),
        disabled: Some(true),
        private: Some(false),
        watch: Some(false),
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
        script_extension: None,
        run_task: None,
        dependencies: None,
        toolchain: None,
    });

    config.tasks.insert("test".to_string(), task);

    let execution_plan = create_execution_plan(&config, "test", false, true);
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
    let execution_plan = create_execution_plan(&config, "test", false, true);
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
    let execution_plan = create_execution_plan(&config, "test", true, true);
    env::set_current_dir("../../").unwrap();
    assert_eq!(execution_plan.steps.len(), 1);
    assert_eq!(execution_plan.steps[0].name, "test");
}

#[test]
#[cfg(target_os = "linux")]
fn create_proxy_task_no_makefile() {
    let makefile = env::var("CARGO_MAKE_MAKEFILE_PATH").unwrap_or("EMPTY".to_string());
    env::remove_var("CARGO_MAKE_MAKEFILE_PATH");
    let task = create_proxy_task("some_task");
    env::set_var("CARGO_MAKE_MAKEFILE_PATH", &makefile);

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut log_level_arg = "--loglevel=".to_string();
    log_level_arg.push_str(&log_level);

    let mut profile_arg = "--profile=\"".to_string();
    profile_arg.push_str(&profile::get());
    profile_arg.push_str("\"");

    let args = task.args.unwrap();
    assert_eq!(args.len(), 6);
    assert_eq!(args[0], "make".to_string());
    assert_eq!(args[1], "--disable-check-for-updates".to_string());
    assert_eq!(args[2], "--no-on-error".to_string());
    assert_eq!(args[3], log_level_arg.to_string());
    assert_eq!(args[4], profile_arg.to_string());
    assert_eq!(args[5], "some_task".to_string());
}

#[test]
#[cfg(target_os = "linux")]
fn create_proxy_task_with_makefile() {
    let makefile = env::var("CARGO_MAKE_MAKEFILE_PATH").unwrap_or("EMPTY".to_string());
    env::set_var("CARGO_MAKE_MAKEFILE_PATH", &makefile);
    let task = create_proxy_task("some_task");

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut log_level_arg = "--loglevel=".to_string();
    log_level_arg.push_str(&log_level);

    let mut profile_arg = "--profile=\"".to_string();
    profile_arg.push_str(&profile::get());
    profile_arg.push_str("\"");

    let mut makefile_arg = "--makefile=".to_string();
    makefile_arg.push_str(&makefile.clone());

    let args = task.args.unwrap();
    assert_eq!(args.len(), 7);
    assert_eq!(args[0], "make".to_string());
    assert_eq!(args[1], "--disable-check-for-updates".to_string());
    assert_eq!(args[2], "--no-on-error".to_string());
    assert_eq!(args[3], log_level_arg.to_string());
    assert_eq!(args[4], profile_arg.to_string());
    assert_eq!(args[5], makefile_arg.to_string());
    assert_eq!(args[6], "some_task".to_string());
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
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
#[should_panic]
#[cfg(target_os = "linux")]
fn run_task_script_with_args_error() {
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: Some(vec!["1".to_string()]),
    };

    let mut task = Task::new();
    task.script = Some(vec!["exit $1".to_string()]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    run_task(&flow_info, &step);
}

#[test]
#[cfg(target_os = "linux")]
fn run_task_script_with_args_valid() {
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: Some(vec!["0".to_string()]),
    };

    let mut task = Task::new();
    task.script = Some(vec!["exit $1".to_string()]);
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let mut task = Task::new();
    task.run_task = Some(RunTaskInfo::Name("sub".to_string()));
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
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
fn should_watch_none_and_env_not_set() {
    env::remove_var("CARGO_MAKE_DISABLE_WATCH");
    let task = Task::new();
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
fn should_watch_none_and_env_false() {
    env::set_var("CARGO_MAKE_DISABLE_WATCH", "FALSE");
    let task = Task::new();
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
fn should_watch_none_and_env_true() {
    env::set_var("CARGO_MAKE_DISABLE_WATCH", "TRUE");
    let task = Task::new();
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
fn should_watch_false_and_env_not_set() {
    env::remove_var("CARGO_MAKE_DISABLE_WATCH");
    let mut task = Task::new();
    task.watch = Some(false);
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
fn should_watch_false_and_env_false() {
    env::set_var("CARGO_MAKE_DISABLE_WATCH", "FALSE");
    let mut task = Task::new();
    task.watch = Some(false);
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
fn should_watch_false_and_env_true() {
    env::set_var("CARGO_MAKE_DISABLE_WATCH", "TRUE");
    let mut task = Task::new();
    task.watch = Some(false);
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
fn should_watch_true_and_env_not_set() {
    env::remove_var("CARGO_MAKE_DISABLE_WATCH");
    let mut task = Task::new();
    task.watch = Some(true);
    let watch = should_watch(&task);

    assert!(watch);
}

#[test]
fn should_watch_true_and_env_false() {
    env::set_var("CARGO_MAKE_DISABLE_WATCH", "FALSE");
    let mut task = Task::new();
    task.watch = Some(true);
    let watch = should_watch(&task);

    assert!(watch);
}

#[test]
fn should_watch_true_and_env_true() {
    env::set_var("CARGO_MAKE_DISABLE_WATCH", "TRUE");
    let mut task = Task::new();
    task.watch = Some(true);
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
fn create_watch_task_name_valid() {
    let output = create_watch_task_name("test_task");

    assert_eq!(output, "test_task-watch");
}

#[test]
#[cfg(target_os = "linux")]
fn create_watch_task_with_makefile() {
    let makefile = env::var("CARGO_MAKE_MAKEFILE_PATH").unwrap_or("EMPTY".to_string());
    env::set_var("CARGO_MAKE_MAKEFILE_PATH", &makefile);
    let task = create_watch_task("some_task");

    match task.env.unwrap().get("CARGO_MAKE_DISABLE_WATCH").unwrap() {
        EnvValue::Value(value) => assert_eq!(value, "TRUE"),
        _ => panic!("CARGO_MAKE_DISABLE_WATCH not defined."),
    };

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut make_command_line =
        "make --disable-check-for-updates --no-on-error --loglevel=".to_string();
    make_command_line.push_str(&log_level);
    make_command_line.push_str(" --profile=\"");
    make_command_line.push_str(&profile::get());
    make_command_line.push_str("\" --makefile=");
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
    let step = create_watch_step("test_watch_step");
    let task = step.config;

    assert_eq!(&step.name, "test_watch_step-watch");

    match task.env.unwrap().get("CARGO_MAKE_DISABLE_WATCH").unwrap() {
        EnvValue::Value(value) => assert_eq!(value, "TRUE"),
        _ => panic!("CARGO_MAKE_DISABLE_WATCH not defined."),
    };

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let args = task.args.unwrap();
    assert_eq!(args.len(), 4);
}

#[test]
fn run_sub_task_and_report_for_name() {
    let mut task = Task::new();
    task.script = Some(vec!["echo test".to_string()]);

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Name("test".to_string());

    let output = run_sub_task_and_report(&flow_info, &sub_task);

    assert!(output);
}

#[test]
#[should_panic]
fn run_sub_task_and_report_for_name_not_found() {
    let mut task = Task::new();
    task.script = Some(vec!["echo test".to_string()]);

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Name("test2".to_string());

    run_sub_task_and_report(&flow_info, &sub_task);
}

#[test]
fn run_sub_task_and_report_routing_empty() {
    let mut task = Task::new();
    task.script = Some(vec!["echo test".to_string()]);

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Routing(vec![]);

    let output = run_sub_task_and_report(&flow_info, &sub_task);

    assert!(!output);
}

#[test]
fn run_sub_task_and_report_routing_no_condition() {
    let mut task = Task::new();
    task.script = Some(vec!["echo test".to_string()]);

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Routing(vec![RunTaskRoutingInfo {
        name: "test".to_string(),
        condition: None,
        condition_script: None,
    }]);

    let output = run_sub_task_and_report(&flow_info, &sub_task);

    assert!(output);
}

#[test]
fn run_sub_task_and_report_routing_condition_not_met() {
    let mut task = Task::new();
    task.script = Some(vec!["echo test".to_string()]);

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Routing(vec![RunTaskRoutingInfo {
        name: "test".to_string(),
        condition: Some(TaskCondition {
            profiles: None,
            platforms: None,
            channels: Some(vec!["bad1".to_string(), "bad2".to_string()]),
            env_set: None,
            env_not_set: None,
            env: None,
            rust_version: None,
        }),
        condition_script: None,
    }]);

    let output = run_sub_task_and_report(&flow_info, &sub_task);

    assert!(!output);
}

#[test]
#[should_panic]
fn run_sub_task_and_report_routing_not_found() {
    let mut task = Task::new();
    task.script = Some(vec!["echo test".to_string()]);

    let mut tasks = IndexMap::new();
    tasks.insert("test".to_string(), task);

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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let sub_task = RunTaskInfo::Routing(vec![RunTaskRoutingInfo {
        name: "test2".to_string(),
        condition: None,
        condition_script: None,
    }]);

    run_sub_task_and_report(&flow_info, &sub_task);
}

#[test]
fn get_sub_task_name_for_routing_info_empty() {
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let output = get_sub_task_name_for_routing_info(&flow_info, &vec![]);

    assert!(output.is_none());
}

#[test]
fn get_sub_task_name_for_routing_info_condition_not_met() {
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let output = get_sub_task_name_for_routing_info(
        &flow_info,
        &vec![RunTaskRoutingInfo {
            name: "test".to_string(),
            condition: Some(TaskCondition {
                profiles: None,
                platforms: None,
                channels: Some(vec!["bad1".to_string(), "bad2".to_string()]),
                env_set: None,
                env_not_set: None,
                env: None,
                rust_version: None,
            }),
            condition_script: None,
        }],
    );

    assert!(output.is_none());
}

#[test]
fn get_sub_task_name_for_routing_info_condition_found() {
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let output = get_sub_task_name_for_routing_info(
        &flow_info,
        &vec![RunTaskRoutingInfo {
            name: "test".to_string(),
            condition: Some(TaskCondition {
                profiles: None,
                platforms: None,
                channels: None,
                env_set: Some(vec!["CARGO_MAKE".to_string()]),
                env_not_set: None,
                env: None,
                rust_version: None,
            }),
            condition_script: None,
        }],
    );

    assert_eq!(output.unwrap(), "test");
}

#[test]
fn get_sub_task_name_for_routing_info_script_not_met() {
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let output = get_sub_task_name_for_routing_info(
        &flow_info,
        &vec![RunTaskRoutingInfo {
            name: "test".to_string(),
            condition: None,
            condition_script: Some(vec!["exit 1".to_string()]),
        }],
    );

    assert!(output.is_none());
}

#[test]
fn get_sub_task_name_for_routing_info_script_found() {
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let output = get_sub_task_name_for_routing_info(
        &flow_info,
        &vec![RunTaskRoutingInfo {
            name: "test".to_string(),
            condition: None,
            condition_script: Some(vec!["exit 0".to_string()]),
        }],
    );

    assert_eq!(output.unwrap(), "test");
}

#[test]
fn get_sub_task_name_for_routing_info_multiple_found() {
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let output = get_sub_task_name_for_routing_info(
        &flow_info,
        &vec![
            RunTaskRoutingInfo {
                name: "test1".to_string(),
                condition: Some(TaskCondition {
                    profiles: None,
                    platforms: None,
                    channels: None,
                    env_set: Some(vec!["CARGO_MAKE".to_string()]),
                    env_not_set: None,
                    env: None,
                    rust_version: None,
                }),
                condition_script: None,
            },
            RunTaskRoutingInfo {
                name: "test2".to_string(),
                condition: None,
                condition_script: Some(vec!["exit 0".to_string()]),
            },
        ],
    );

    assert_eq!(output.unwrap(), "test1");
}

#[test]
fn get_sub_task_name_for_routing_info_default() {
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let output = get_sub_task_name_for_routing_info(
        &flow_info,
        &vec![
            RunTaskRoutingInfo {
                name: "test1".to_string(),
                condition: Some(TaskCondition {
                    profiles: None,
                    platforms: None,
                    channels: None,
                    env_set: None,
                    env_not_set: Some(vec!["CARGO_MAKE".to_string()]),
                    env: None,
                    rust_version: None,
                }),
                condition_script: None,
            },
            RunTaskRoutingInfo {
                name: "test2".to_string(),
                condition: None,
                condition_script: Some(vec!["exit 1".to_string()]),
            },
            RunTaskRoutingInfo {
                name: "default".to_string(),
                condition: None,
                condition_script: None,
            },
        ],
    );

    assert_eq!(output.unwrap(), "default");
}

#[test]
fn get_sub_task_name_for_routing_info_multiple() {
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
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    };

    let output = get_sub_task_name_for_routing_info(
        &flow_info,
        &vec![
            RunTaskRoutingInfo {
                name: "test1".to_string(),
                condition: Some(TaskCondition {
                    profiles: None,
                    platforms: None,
                    channels: None,
                    env_set: None,
                    env_not_set: Some(vec!["CARGO_MAKE".to_string()]),
                    env: None,
                    rust_version: None,
                }),
                condition_script: None,
            },
            RunTaskRoutingInfo {
                name: "test2".to_string(),
                condition: None,
                condition_script: Some(vec!["exit 1".to_string()]),
            },
            RunTaskRoutingInfo {
                name: "test3".to_string(),
                condition: None,
                condition_script: Some(vec!["exit 0".to_string()]),
            },
            RunTaskRoutingInfo {
                name: "default".to_string(),
                condition: None,
                condition_script: None,
            },
        ],
    );

    assert_eq!(output.unwrap(), "test3");
}
