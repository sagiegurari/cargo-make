use super::*;

use crate::types::{ConfigSection, Task};
use indexmap::IndexMap;
use std::env;
use std::{thread, time};

#[test]
fn get_env_exists() {
    env::set_var("TEST_GET_ENV_EXISTS", "EXISTS");
    let output = get_env("TEST_GET_ENV_EXISTS", "bad");
    assert_eq!(output, "EXISTS".to_string());
}

#[test]
fn get_env_not_exists() {
    let output = get_env("TEST_GET_ENV_NOT_EXISTS", "good");
    assert_eq!(output, "good".to_string());
}

#[test]
fn get_env_as_bool_set_true() {
    env::set_var("TEST_BOOL_TRUE", "true");
    let output = get_env_as_bool("TEST_BOOL_TRUE", false);
    assert!(output);
}

#[test]
fn get_env_as_bool_set_true_uppercase() {
    env::set_var("TEST_BOOL_TRUE_UPPER", "TRUE");
    let output = get_env_as_bool("TEST_BOOL_TRUE_UPPER", false);
    assert!(output);
}

#[test]
fn get_env_as_bool_set_yes() {
    env::set_var("TEST_BOOL_YES", "yes");
    let output = get_env_as_bool("TEST_BOOL_YES", false);
    assert!(output);
}

#[test]
fn get_env_as_bool_set_yes_uppercase() {
    env::set_var("TEST_BOOL_YES_UPPER", "YES");
    let output = get_env_as_bool("TEST_BOOL_YES_UPPER", false);
    assert!(output);
}

#[test]
fn get_env_as_bool_set_1() {
    env::set_var("TEST_BOOL_1", "1");
    let output = get_env_as_bool("TEST_BOOL_1", false);
    assert!(output);
}

#[test]
fn get_env_as_bool_set_false() {
    env::set_var("TEST_BOOL_FALSE", "false");
    let output = get_env_as_bool("TEST_BOOL_FALSE", true);
    assert!(!output);
}

#[test]
fn get_env_as_bool_default_true() {
    let output = get_env_as_bool("TEST_BOOL_NO_EXISTS_TRUE", true);
    assert!(output);
}

#[test]
fn get_env_as_bool_default_false() {
    let output = get_env_as_bool("TEST_BOOL_NO_EXISTS_FALSE", false);
    assert!(!output);
}

#[test]
fn parse_env_file_none() {
    let output = parse_env_file(None);

    assert!(output.is_none());
}

#[test]
fn parse_env_file_no_exists() {
    let output = parse_env_file(Some("./bad.env".to_string()));

    assert!(output.is_none());
}

#[test]
fn parse_env_file_exists() {
    let output = parse_env_file(Some("./examples/test.env".to_string()));

    assert!(output.is_some());

    let env = output.unwrap();
    assert_eq!(env.len(), 3);
    assert_eq!(env[0], "ENV1_TEST=TEST1");
    assert_eq!(
        env[env.len() - 1],
        "ENV3_TEST=VALUE OF ENV2 IS: ${ENV2_TEST}"
    );
}

#[test]
fn evaluate_and_set_env_simple() {
    env::remove_var("EVAL_SET_SIMPLE");
    evaluate_and_set_env("EVAL_SET_SIMPLE", "SIMPLE");
    assert_eq!(env::var("EVAL_SET_SIMPLE").unwrap(), "SIMPLE".to_string());
}

#[test]
fn evaluate_and_set_env_exists() {
    env::set_var("eval_test1", "test");
    evaluate_and_set_env(
        "evaluate_and_set_env_exists",
        "testing: ${eval_test1} works",
    );
    assert_eq!(
        env::var("evaluate_and_set_env_exists").unwrap(),
        "testing: test works".to_string()
    );
}

#[test]
fn evaluate_and_set_env_not_exists() {
    evaluate_and_set_env(
        "evaluate_and_set_env_not_exists",
        "testing: ${eval_test_bad} works",
    );
    assert_eq!(
        env::var("evaluate_and_set_env_not_exists").unwrap(),
        "testing: ${eval_test_bad} works".to_string()
    );
}

#[test]
fn evaluate_and_set_env_complex() {
    env::set_var("eval_test10", "10");
    env::set_var("eval_test20", "20");
    evaluate_and_set_env(
        "evaluate_and_set_env_complex",
        "checking 10 is ${eval_test10} empty is ${eval_test30} and 20 is ${eval_test20}",
    );
    assert_eq!(
        env::var("evaluate_and_set_env_complex").unwrap(),
        "checking 10 is 10 empty is ${eval_test30} and 20 is 20".to_string()
    );
}

#[test]
fn setup_cwd_empty() {
    env::set_var("CARGO_MAKE_WORKING_DIRECTORY", "EMPTY");

    setup_cwd(None);

    assert!(env::var("CARGO_MAKE_WORKING_DIRECTORY").unwrap() != "EMPTY");
}

#[test]
fn setup_env_empty() {
    let cli_args = CliArgs::new();

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    setup_env(&cli_args, &config, "setup_env_empty1");

    let mut value = env::var("CARGO_MAKE_TASK");
    assert_eq!(value.unwrap(), "setup_env_empty1");

    setup_env(&cli_args, &config, "setup_env_empty2");

    let delay = time::Duration::from_millis(10);
    thread::sleep(delay);

    value = env::var("CARGO_MAKE_TASK");
    assert_eq!(value.unwrap(), "setup_env_empty2");
}

#[test]
fn setup_env_cli_arguments() {
    let mut cli_args = CliArgs::new();
    cli_args.arguments = Some(vec!["arg1".to_string(), "arg2".to_string()]);

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };

    env::set_var("CARGO_MAKE_TASK_ARGS", "EMPTY");

    setup_env(&cli_args, &config, "setup_env_empty1");

    let value = env::var("CARGO_MAKE_TASK_ARGS");
    assert_eq!(value.unwrap(), "arg1;arg2");
}

#[test]
fn setup_env_values() {
    let cli_args = CliArgs::new();

    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    config.env.insert(
        "MY_ENV_KEY".to_string(),
        EnvValue::Value("MY_ENV_VALUE".to_string()),
    );
    config.env.insert(
        "MY_ENV_KEY2".to_string(),
        EnvValue::Value("MY_ENV_VALUE2".to_string()),
    );

    assert_eq!(
        env::var("MY_ENV_KEY").unwrap_or("NONE".to_string()),
        "NONE".to_string()
    );
    assert_eq!(
        env::var("MY_ENV_KEY2").unwrap_or("NONE".to_string()),
        "NONE".to_string()
    );

    setup_env(&cli_args, &config, "set_env_values");

    assert_eq!(env::var("MY_ENV_KEY").unwrap(), "MY_ENV_VALUE");
    assert_eq!(env::var("MY_ENV_KEY2").unwrap(), "MY_ENV_VALUE2");
}

#[test]
fn setup_env_script() {
    let cli_args = CliArgs::new();

    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    config.env.insert(
        "MY_ENV_SCRIPT_KEY".to_string(),
        EnvValue::Value("MY_ENV_VALUE".to_string()),
    );
    config.env.insert(
        "MY_ENV_SCRIPT_KEY2".to_string(),
        EnvValue::Info(EnvValueInfo {
            script: vec!["echo script1".to_string()],
        }),
    );

    assert_eq!(
        env::var("MY_ENV_SCRIPT_KEY").unwrap_or("NONE".to_string()),
        "NONE".to_string()
    );
    assert_eq!(
        env::var("MY_ENV_SCRIPT_KEY2").unwrap_or("NONE".to_string()),
        "NONE".to_string()
    );

    setup_env(&cli_args, &config, "set_env_values");

    assert_eq!(env::var("MY_ENV_SCRIPT_KEY").unwrap(), "MY_ENV_VALUE");
    assert_eq!(env::var("MY_ENV_SCRIPT_KEY2").unwrap(), "script1");
}

#[test]
fn evaluate_env_value_valid() {
    let output = evaluate_env_value(&EnvValueInfo {
        script: vec!["echo script1".to_string()],
    });

    assert_eq!(output, "script1".to_string());
}

#[test]
#[cfg(target_os = "linux")]
fn evaluate_env_value_empty() {
    let output = evaluate_env_value(&EnvValueInfo {
        script: vec!["".to_string()],
    });

    assert_eq!(output, "".to_string());
}

#[test]
#[should_panic]
fn evaluate_env_error() {
    evaluate_env_value(&EnvValueInfo {
        script: vec!["exit 1".to_string()],
    });
}

#[test]
fn setup_env_for_crate_load_toml_found() {
    env::set_var("CARGO_MAKE_CRATE_NAME", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_FS_NAME", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_VERSION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_DESCRIPTION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_LICENSE", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_DOCUMENTATION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_HOMEPAGE", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_REPOSITORY", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_IS_WORKSPACE", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_HAS_DEPENDENCIES", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS", "EMPTY");

    setup_env_for_crate();

    assert_eq!(env::var("CARGO_MAKE_CRATE_NAME").unwrap(), "cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_FS_NAME").unwrap(), "cargo_make");
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_VERSION").unwrap(),
        env!("CARGO_PKG_VERSION")
    );
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_DESCRIPTION").unwrap(),
        env!("CARGO_PKG_DESCRIPTION")
    );
    assert_eq!(env::var("CARGO_MAKE_CRATE_LICENSE").unwrap(), "Apache-2.0");
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_DOCUMENTATION").unwrap(),
        "https://sagiegurari.github.io/cargo-make"
    );
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_HOMEPAGE").unwrap(),
        "https://sagiegurari.github.io/cargo-make"
    );
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_REPOSITORY").unwrap(),
        "https://github.com/sagiegurari/cargo-make.git"
    );
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_HAS_DEPENDENCIES").unwrap(),
        "TRUE"
    );
    assert_eq!(env::var("CARGO_MAKE_CRATE_IS_WORKSPACE").unwrap(), "FALSE");
    assert_eq!(env::var("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS").unwrap(), "");
}

#[test]
fn setup_env_for_crate_load_toml_not_found_and_cwd() {
    env::set_var("CARGO_MAKE_CRATE_NAME", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_FS_NAME", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_VERSION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_DESCRIPTION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_LICENSE", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_DOCUMENTATION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_HOMEPAGE", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_REPOSITORY", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_IS_WORKSPACE", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_HAS_DEPENDENCIES", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS", "EMPTY");

    env::set_var("CARGO_MAKE_WORKING_DIRECTORY", "EMPTY");
    assert!(env::var("CARGO_MAKE_WORKING_DIRECTORY").unwrap() == "EMPTY");

    setup_cwd(Some("examples"));
    setup_env_for_crate();
    setup_cwd(Some(".."));

    assert!(env::var("CARGO_MAKE_WORKING_DIRECTORY").unwrap() != "EMPTY");

    assert_eq!(env::var("CARGO_MAKE_CRATE_NAME").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_FS_NAME").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_VERSION").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_DESCRIPTION").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_LICENSE").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_DOCUMENTATION").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_HOMEPAGE").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_REPOSITORY").unwrap(), "EMPTY");
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_HAS_DEPENDENCIES").unwrap(),
        "FALSE"
    );
    assert_eq!(env::var("CARGO_MAKE_CRATE_IS_WORKSPACE").unwrap(), "FALSE");
    assert_eq!(env::var("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS").unwrap(), "");

    setup_env_for_crate();

    assert_eq!(env::var("CARGO_MAKE_CRATE_NAME").unwrap(), "cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_FS_NAME").unwrap(), "cargo_make");
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_VERSION").unwrap(),
        env!("CARGO_PKG_VERSION")
    );
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_DESCRIPTION").unwrap(),
        env!("CARGO_PKG_DESCRIPTION")
    );
    assert_eq!(env::var("CARGO_MAKE_CRATE_LICENSE").unwrap(), "Apache-2.0");
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_DOCUMENTATION").unwrap(),
        "https://sagiegurari.github.io/cargo-make"
    );
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_HOMEPAGE").unwrap(),
        "https://sagiegurari.github.io/cargo-make"
    );
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_REPOSITORY").unwrap(),
        "https://github.com/sagiegurari/cargo-make.git"
    );
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_HAS_DEPENDENCIES").unwrap(),
        "TRUE"
    );
    assert_eq!(env::var("CARGO_MAKE_CRATE_IS_WORKSPACE").unwrap(), "FALSE");
    assert_eq!(env::var("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS").unwrap(), "");
}

#[test]
fn setup_env_for_crate_workspace() {
    env::set_var("CARGO_MAKE_CRATE_NAME", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_FS_NAME", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_VERSION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_DESCRIPTION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_LICENSE", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_DOCUMENTATION", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_HOMEPAGE", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_REPOSITORY", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_HAS_DEPENDENCIES", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_IS_WORKSPACE", "EMPTY");
    env::set_var("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS", "EMPTY");

    setup_cwd(Some("examples/workspace"));
    setup_env_for_crate();
    setup_cwd(Some("../.."));

    assert_eq!(env::var("CARGO_MAKE_CRATE_NAME").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_FS_NAME").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_VERSION").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_DESCRIPTION").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_LICENSE").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_DOCUMENTATION").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_HOMEPAGE").unwrap(), "EMPTY");
    assert_eq!(env::var("CARGO_MAKE_CRATE_REPOSITORY").unwrap(), "EMPTY");
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_HAS_DEPENDENCIES").unwrap(),
        "TRUE"
    );
    assert_eq!(env::var("CARGO_MAKE_CRATE_IS_WORKSPACE").unwrap(), "TRUE");
    assert_eq!(
        env::var("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS").unwrap(),
        "member1,member2"
    );
}

#[test]
fn setup_env_for_git_repo_with_values() {
    env::set_var("CARGO_MAKE_GIT_BRANCH", "EMPTY");
    env::set_var("CARGO_MAKE_GIT_USER_NAME", "EMPTY");
    env::set_var("CARGO_MAKE_GIT_USER_EMAIL", "EMPTY");

    let git_info = setup_env_for_git_repo();

    if git_info.branch.is_some() {
        assert_eq!(
            env::var("CARGO_MAKE_GIT_BRANCH").unwrap(),
            git_info.branch.unwrap()
        );
    }
    if git_info.user_name.is_some() {
        assert_eq!(
            env::var("CARGO_MAKE_GIT_USER_NAME").unwrap(),
            git_info.user_name.unwrap()
        );
    }
    if git_info.user_email.is_some() {
        assert_eq!(
            env::var("CARGO_MAKE_GIT_USER_EMAIL").unwrap(),
            git_info.user_email.unwrap()
        );
    }
}

#[test]
fn setup_env_for_rust_simple_check() {
    env::set_var("CARGO_MAKE_RUST_VERSION", "EMPTY");
    env::set_var("CARGO_MAKE_RUST_CHANNEL", "EMPTY");
    env::set_var("CARGO_MAKE_RUST_TARGET_ARCH", "EMPTY");
    env::set_var("CARGO_MAKE_RUST_TARGET_ENV", "EMPTY");
    env::set_var("CARGO_MAKE_RUST_TARGET_OS", "EMPTY");
    env::set_var("CARGO_MAKE_RUST_TARGET_POINTER_WIDTH", "EMPTY");
    env::set_var("CARGO_MAKE_RUST_TARGET_VENDOR", "EMPTY");

    assert!(env::var("CARGO_MAKE_RUST_VERSION").unwrap() == "EMPTY");
    assert!(env::var("CARGO_MAKE_RUST_CHANNEL").unwrap() == "EMPTY");
    assert!(env::var("CARGO_MAKE_RUST_TARGET_ARCH").unwrap() == "EMPTY");
    assert!(env::var("CARGO_MAKE_RUST_TARGET_ENV").unwrap() == "EMPTY");
    assert!(env::var("CARGO_MAKE_RUST_TARGET_OS").unwrap() == "EMPTY");
    assert!(env::var("CARGO_MAKE_RUST_TARGET_POINTER_WIDTH").unwrap() == "EMPTY");
    assert!(env::var("CARGO_MAKE_RUST_TARGET_VENDOR").unwrap() == "EMPTY");

    setup_env_for_rust();

    assert!(env::var("CARGO_MAKE_RUST_VERSION").unwrap() != "EMPTY");
    assert!(env::var("CARGO_MAKE_RUST_CHANNEL").unwrap() != "EMPTY");
    assert!(env::var("CARGO_MAKE_RUST_TARGET_ARCH").unwrap() != "EMPTY");
    assert!(env::var("CARGO_MAKE_RUST_TARGET_ENV").unwrap() != "EMPTY");
    assert!(env::var("CARGO_MAKE_RUST_TARGET_OS").unwrap() != "EMPTY");
    assert!(env::var("CARGO_MAKE_RUST_TARGET_POINTER_WIDTH").unwrap() != "EMPTY");
    assert!(env::var("CARGO_MAKE_RUST_TARGET_VENDOR").unwrap() != "EMPTY");
}

#[test]
fn get_project_root_test() {
    let directory = env::current_dir().unwrap().to_str().unwrap().to_string();
    let project_root = get_project_root().unwrap();

    assert_eq!(directory, project_root);
}

#[test]
fn get_project_root_for_path_cwd() {
    let path = env::current_dir().unwrap();
    let directory = path.to_str().unwrap().to_string();
    let project_root = get_project_root_for_path(&path).unwrap();

    assert_eq!(directory, project_root);
}

#[test]
fn get_project_root_for_path_sub_path() {
    let path = env::current_dir().unwrap();
    let directory = path.to_str().unwrap().to_string();
    let search_path = path.join("examples/files");
    let project_root = get_project_root_for_path(&search_path).unwrap();

    assert_eq!(directory, project_root);
}

#[test]
fn get_project_root_for_path_parent_path() {
    let path = env::current_dir().unwrap();
    let search_path = path.parent().unwrap().to_path_buf();
    let project_root = get_project_root_for_path(&search_path);

    assert!(project_root.is_none());
}

#[test]
fn expand_env_empty() {
    let step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };
    let updated_step = expand_env(&step);

    assert_eq!(updated_step.name, "test".to_string());
    assert!(updated_step.config.command.is_none());
    assert!(updated_step.config.args.is_none());
}

#[test]
fn expand_env_no_env_vars() {
    let mut task = Task::new();
    task.command = Some("command".to_string());
    task.args = Some(vec![
        "arg0".to_string(),
        "arg1".to_string(),
        "arg2".to_string(),
        "arg3".to_string(),
        "arg4".to_string(),
    ]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };
    let updated_step = expand_env(&step);

    assert_eq!(updated_step.name, "test".to_string());
    assert_eq!(updated_step.config.command.unwrap(), "command".to_string());
    let args = updated_step.config.args.unwrap();
    assert_eq!(args.len(), 5);
    assert_eq!(args[3], "arg3".to_string());
}

#[test]
fn expand_env_with_env_vars() {
    env::set_var("TEST_ENV_EXPAND1", "ENV1");
    env::set_var("TEST_ENV_EXPAND2", "ENV2");

    let mut task = Task::new();
    task.command = Some("command-${TEST_ENV_EXPAND1}-${TEST_ENV_EXPAND2}".to_string());
    task.args = Some(vec![
        "arg0".to_string(),
        "arg1".to_string(),
        "arg2".to_string(),
        "arg3-${TEST_ENV_EXPAND1}-${TEST_ENV_EXPAND2}".to_string(),
        "arg4".to_string(),
    ]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };
    let updated_step = expand_env(&step);

    assert_eq!(updated_step.name, "test".to_string());
    assert_eq!(
        updated_step.config.command.unwrap(),
        "command-ENV1-ENV2".to_string()
    );
    let args = updated_step.config.args.unwrap();
    assert_eq!(args.len(), 5);
    assert_eq!(args[3], "arg3-ENV1-ENV2".to_string());
}

#[test]
fn expand_env_with_env_vars_and_task_args() {
    env::set_var("TEST_ENV_EXPAND1", "ENV1");
    env::set_var("TEST_ENV_EXPAND2", "ENV2");
    env::set_var("CARGO_MAKE_TASK_ARGS", "targ1;targ2;targ3;targ4");

    let mut task = Task::new();
    task.command = Some("command-${TEST_ENV_EXPAND1}-${TEST_ENV_EXPAND2}".to_string());
    task.args = Some(vec![
        "arg0".to_string(),
        "${@}".to_string(),
        "-o=${@}".to_string(),
        "arg1".to_string(),
        "arg2".to_string(),
        "arg3-${TEST_ENV_EXPAND1}-${TEST_ENV_EXPAND2}".to_string(),
        "arg4".to_string(),
    ]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };
    let updated_step = expand_env(&step);

    assert_eq!(updated_step.name, "test".to_string());
    assert_eq!(
        updated_step.config.command.unwrap(),
        "command-ENV1-ENV2".to_string()
    );
    let args = updated_step.config.args.unwrap();
    assert_eq!(args.len(), 13);
    assert_eq!(args[11], "arg3-ENV1-ENV2".to_string());
    assert_eq!(args[1], "targ1".to_string());
    assert_eq!(args[2], "targ2".to_string());
    assert_eq!(args[3], "targ3".to_string());
    assert_eq!(args[4], "targ4".to_string());
    assert_eq!(args[5], "-o=targ1".to_string());
    assert_eq!(args[6], "-o=targ2".to_string());
    assert_eq!(args[7], "-o=targ3".to_string());
    assert_eq!(args[8], "-o=targ4".to_string());
}

#[test]
fn expand_env_with_env_vars_and_empty_task_args() {
    env::set_var("TEST_ENV_EXPAND1", "ENV1");
    env::set_var("TEST_ENV_EXPAND2", "ENV2");
    env::set_var("CARGO_MAKE_TASK_ARGS", "");

    let mut task = Task::new();
    task.command = Some("command-${TEST_ENV_EXPAND1}-${TEST_ENV_EXPAND2}".to_string());
    task.args = Some(vec![
        "arg0".to_string(),
        "${@}".to_string(),
        "-o=${@}".to_string(),
        "arg1".to_string(),
        "arg2".to_string(),
        "arg3-${TEST_ENV_EXPAND1}-${TEST_ENV_EXPAND2}".to_string(),
        "arg4".to_string(),
    ]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };
    let updated_step = expand_env(&step);

    assert_eq!(updated_step.name, "test".to_string());
    assert_eq!(
        updated_step.config.command.unwrap(),
        "command-ENV1-ENV2".to_string()
    );
    let args = updated_step.config.args.unwrap();
    assert_eq!(args.len(), 5);
    assert_eq!(args[3], "arg3-ENV1-ENV2".to_string());
}

#[test]
fn remove_unc_prefix_found() {
    let output = remove_unc_prefix(&PathBuf::from(r"\\?\C:\test"));

    assert_eq!(output, PathBuf::from(r"C:\test"));
}

#[test]
fn remove_unc_prefix_not_found() {
    let output = remove_unc_prefix(&PathBuf::from(r"C:\test"));

    assert_eq!(output, PathBuf::from(r"C:\test"));
}
