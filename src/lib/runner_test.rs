use super::*;
use crate::types::{
    ConfigSection, CrateInfo, EnvInfo, EnvValue, FlowInfo, GitInfo, RunTaskInfo, Step, Task,
    TaskCondition,
};
use ci_info;
use indexmap::IndexMap;
use rust_info::types::RustInfo;
use std::env;

#[cfg(target_os = "linux")]
use crate::types::WatchOptions;

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
    task.watch = Some(TaskWatchOptions::Boolean(false));
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
fn should_watch_false_and_env_false() {
    env::set_var("CARGO_MAKE_DISABLE_WATCH", "FALSE");
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Boolean(false));
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
fn should_watch_false_and_env_true() {
    env::set_var("CARGO_MAKE_DISABLE_WATCH", "TRUE");
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Boolean(false));
    let watch = should_watch(&task);

    assert!(!watch);
}

#[test]
fn should_watch_true_and_env_not_set() {
    env::remove_var("CARGO_MAKE_DISABLE_WATCH");
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Boolean(true));
    let watch = should_watch(&task);

    assert!(watch);
}

#[test]
fn should_watch_true_and_env_false() {
    env::set_var("CARGO_MAKE_DISABLE_WATCH", "FALSE");
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Boolean(true));
    let watch = should_watch(&task);

    assert!(watch);
}

#[test]
fn should_watch_true_and_env_true() {
    env::set_var("CARGO_MAKE_DISABLE_WATCH", "TRUE");
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Boolean(true));
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
    let task = create_watch_task("some_task", None);

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
#[cfg(target_os = "linux")]
fn create_watch_task_with_makefile_and_bool_options() {
    let makefile = env::var("CARGO_MAKE_MAKEFILE_PATH").unwrap_or("EMPTY".to_string());
    env::set_var("CARGO_MAKE_MAKEFILE_PATH", &makefile);
    let task = create_watch_task("some_task", Some(TaskWatchOptions::Boolean(true)));

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
#[cfg(target_os = "linux")]
fn create_watch_task_with_makefile_and_empty_object_options() {
    let makefile = env::var("CARGO_MAKE_MAKEFILE_PATH").unwrap_or("EMPTY".to_string());
    env::set_var("CARGO_MAKE_MAKEFILE_PATH", &makefile);

    let watch_options = WatchOptions {
        version: None,
        postpone: None,
        ignore_pattern: None,
        no_git_ignore: None,
    };

    let task = create_watch_task("some_task", Some(TaskWatchOptions::Options(watch_options)));

    assert!(task.install_crate_args.is_some());
    assert_eq!(task.install_crate_args.unwrap().len(), 2);

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
#[cfg(target_os = "linux")]
fn create_watch_task_with_makefile_and_all_object_options() {
    let makefile = env::var("CARGO_MAKE_MAKEFILE_PATH").unwrap_or("EMPTY".to_string());
    env::set_var("CARGO_MAKE_MAKEFILE_PATH", &makefile);

    let watch_options = WatchOptions {
        version: Some("100.200.300.400".to_string()),
        postpone: Some(true),
        ignore_pattern: Some("tools/*".to_string()),
        no_git_ignore: Some(true),
    };

    let task = create_watch_task("some_task", Some(TaskWatchOptions::Options(watch_options)));

    assert!(task.install_crate_args.is_some());
    let install_crate_args = task.install_crate_args.unwrap();
    assert_eq!(install_crate_args[0], "--version");
    assert_eq!(install_crate_args[1], "100.200.300.400");

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
    assert_eq!(args.len(), 8);
    assert_eq!(args[0], "watch".to_string());
    assert_eq!(args[1], "-q".to_string());
    assert_eq!(args[2], "--postpone".to_string());
    assert_eq!(args[3], "-i".to_string());
    assert_eq!(args[4], "tools/*".to_string());
    assert_eq!(args[5], "--no-gitignore".to_string());
    assert_eq!(args[6], "-x".to_string());
    assert_eq!(args[7], make_command_line.to_string());
}

#[test]
#[cfg(target_os = "linux")]
fn create_watch_task_with_makefile_and_false_object_options() {
    let makefile = env::var("CARGO_MAKE_MAKEFILE_PATH").unwrap_or("EMPTY".to_string());
    env::set_var("CARGO_MAKE_MAKEFILE_PATH", &makefile);

    let watch_options = WatchOptions {
        version: None,
        postpone: Some(false),
        ignore_pattern: None,
        no_git_ignore: Some(false),
    };

    let task = create_watch_task("some_task", Some(TaskWatchOptions::Options(watch_options)));

    assert!(task.install_crate_args.is_some());
    assert_eq!(task.install_crate_args.unwrap().len(), 2);

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
    let step = create_watch_step("test_watch_step", None);
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
