use super::*;

use std::{thread, time};
use std::collections::HashMap;
use std::env;
use types::ConfigSection;

#[test]
fn setup_cwd_empty() {
    env::set_var("CARGO_MAKE_WORKING_DIRECTORY", "EMPTY");

    setup_cwd(None);

    assert!(env::var("CARGO_MAKE_WORKING_DIRECTORY").unwrap() != "EMPTY");
}

#[test]
fn setup_env_empty() {
    let config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };

    setup_env(&config, "setup_env_empty1");

    let mut value = env::var("CARGO_MAKE_TASK");
    assert_eq!(value.unwrap(), "setup_env_empty1");

    setup_env(&config, "setup_env_empty2");

    let delay = time::Duration::from_millis(10);
    thread::sleep(delay);

    value = env::var("CARGO_MAKE_TASK");
    assert_eq!(value.unwrap(), "setup_env_empty2");
}

#[test]
fn setup_env_values() {
    let mut config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };
    config.env.insert("MY_ENV_KEY".to_string(), EnvValue::Value("MY_ENV_VALUE".to_string()));
    config.env.insert("MY_ENV_KEY2".to_string(), EnvValue::Value("MY_ENV_VALUE2".to_string()));

    assert_eq!(env::var("MY_ENV_KEY").unwrap_or("NONE".to_string()), "NONE".to_string());
    assert_eq!(env::var("MY_ENV_KEY2").unwrap_or("NONE".to_string()), "NONE".to_string());

    setup_env(&config, "set_env_values");

    assert_eq!(env::var("MY_ENV_KEY").unwrap(), "MY_ENV_VALUE");
    assert_eq!(env::var("MY_ENV_KEY2").unwrap(), "MY_ENV_VALUE2");
}

#[test]
fn setup_env_script() {
    let mut config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };
    config.env.insert("MY_ENV_SCRIPT_KEY".to_string(), EnvValue::Value("MY_ENV_VALUE".to_string()));
    config.env.insert(
        "MY_ENV_SCRIPT_KEY2".to_string(),
        EnvValue::Info(EnvValueInfo { script: vec!["echo script1".to_string()] })
    );

    assert_eq!(env::var("MY_ENV_SCRIPT_KEY").unwrap_or("NONE".to_string()), "NONE".to_string());
    assert_eq!(env::var("MY_ENV_SCRIPT_KEY2").unwrap_or("NONE".to_string()), "NONE".to_string());

    setup_env(&config, "set_env_values");

    assert_eq!(env::var("MY_ENV_SCRIPT_KEY").unwrap(), "MY_ENV_VALUE");
    assert_eq!(env::var("MY_ENV_SCRIPT_KEY2").unwrap(), "script1");
}

#[test]
fn evaluate_env_value_valid() {
    let output = evaluate_env_value(&EnvValueInfo { script: vec!["echo script1".to_string()] });

    assert_eq!(output, "script1".to_string());
}

#[test]
fn evaluate_env_value_empty() {
    let output = evaluate_env_value(&EnvValueInfo { script: vec!["".to_string()] });

    assert_eq!(output, "".to_string());
}

#[test]
#[should_panic]
fn evaluate_env_error() {
    evaluate_env_value(&EnvValueInfo { script: vec!["exit 1".to_string()] });
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
    env::set_var("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS", "EMPTY");

    setup_env_for_crate();

    assert_eq!(env::var("CARGO_MAKE_CRATE_NAME").unwrap(), "cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_FS_NAME").unwrap(), "cargo_make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_VERSION").unwrap(), env!("CARGO_PKG_VERSION"));
    assert_eq!(env::var("CARGO_MAKE_CRATE_DESCRIPTION").unwrap(), env!("CARGO_PKG_DESCRIPTION"));
    assert_eq!(env::var("CARGO_MAKE_CRATE_LICENSE").unwrap(), "Apache-2.0");
    assert_eq!(env::var("CARGO_MAKE_CRATE_DOCUMENTATION").unwrap(), "https://sagiegurari.github.io/cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_HOMEPAGE").unwrap(), "https://sagiegurari.github.io/cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_REPOSITORY").unwrap(), "https://github.com/sagiegurari/cargo-make.git");
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
    assert_eq!(env::var("CARGO_MAKE_CRATE_IS_WORKSPACE").unwrap(), "FALSE");
    assert_eq!(env::var("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS").unwrap(), "");

    setup_env_for_crate();

    assert_eq!(env::var("CARGO_MAKE_CRATE_NAME").unwrap(), "cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_FS_NAME").unwrap(), "cargo_make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_VERSION").unwrap(), env!("CARGO_PKG_VERSION"));
    assert_eq!(env::var("CARGO_MAKE_CRATE_DESCRIPTION").unwrap(), env!("CARGO_PKG_DESCRIPTION"));
    assert_eq!(env::var("CARGO_MAKE_CRATE_LICENSE").unwrap(), "Apache-2.0");
    assert_eq!(env::var("CARGO_MAKE_CRATE_DOCUMENTATION").unwrap(), "https://sagiegurari.github.io/cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_HOMEPAGE").unwrap(), "https://sagiegurari.github.io/cargo-make");
    assert_eq!(env::var("CARGO_MAKE_CRATE_REPOSITORY").unwrap(), "https://github.com/sagiegurari/cargo-make.git");
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
    assert_eq!(env::var("CARGO_MAKE_CRATE_IS_WORKSPACE").unwrap(), "TRUE");
    assert_eq!(env::var("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS").unwrap(), "member1,member2");
}

#[test]
fn setup_env_for_git_repo_with_values() {
    env::set_var("CARGO_MAKE_GIT_BRANCH", "EMPTY");
    env::set_var("CARGO_MAKE_GIT_USER_NAME", "EMPTY");
    env::set_var("CARGO_MAKE_GIT_USER_EMAIL", "EMPTY");

    let git_info = setup_env_for_git_repo();

    if git_info.branch.is_some() {
        assert_eq!(env::var("CARGO_MAKE_GIT_BRANCH").unwrap(), git_info.branch.unwrap());
    }
    if git_info.user_name.is_some() {
        assert_eq!(env::var("CARGO_MAKE_GIT_USER_NAME").unwrap(), git_info.user_name.unwrap());
    }
    if git_info.user_email.is_some() {
        assert_eq!(env::var("CARGO_MAKE_GIT_USER_EMAIL").unwrap(), git_info.user_email.unwrap());
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
fn is_env_defined_found() {
    env::set_var("ENV_VAR_FOUND_VALUE", "EMPTY");

    let found = is_env_defined("ENV_VAR_FOUND_VALUE");

    assert!(found);
}

#[test]
fn is_env_defined_empty() {
    env::set_var("ENV_VAR_FOUND_EMPTY", "");

    let found = is_env_defined("ENV_VAR_FOUND_EMPTY");

    assert!(found);
}

#[test]
fn is_env_defined_not_found() {
    let found = is_env_defined("ENV_VAR_NOT_FOUND");

    assert!(!found);
}
