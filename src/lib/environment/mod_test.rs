use super::*;

use crate::types::{ConfigSection, Task};
use indexmap::IndexMap;
use std::collections::HashMap;
use std::env;
use std::{thread, time};

#[test]
fn load_env_file_none() {
    let output = load_env_file(None);

    assert!(!output);
}

#[test]
#[should_panic]
fn load_env_file_no_exists() {
    load_env_file(Some("./bad.env".to_string()));
}

#[test]
fn load_env_file_exists() {
    envmnt::remove("ENV1_TEST");
    envmnt::remove("ENV2_TEST");
    envmnt::remove("ENV3_TEST");

    let output = load_env_file(Some("./examples/test.env".to_string()));

    assert!(output);

    assert!(envmnt::is_equal("ENV1_TEST", "TEST1"));
    assert!(envmnt::is_equal("ENV2_TEST", "TEST2"));
    assert!(envmnt::is_equal("ENV3_TEST", "VALUE OF ENV2 IS: TEST2"));
}

#[test]
fn evaluate_and_set_env_simple() {
    envmnt::remove("EVAL_SET_SIMPLE");
    evaluate_and_set_env("EVAL_SET_SIMPLE", "SIMPLE");
    assert_eq!(
        envmnt::get_or_panic("EVAL_SET_SIMPLE"),
        "SIMPLE".to_string()
    );
}

#[test]
fn evaluate_and_set_env_exists() {
    envmnt::set("eval_test1", "test");
    evaluate_and_set_env(
        "evaluate_and_set_env_exists",
        "testing: ${eval_test1} works",
    );
    assert_eq!(
        envmnt::get_or_panic("evaluate_and_set_env_exists"),
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
        envmnt::get_or_panic("evaluate_and_set_env_not_exists"),
        "testing: ${eval_test_bad} works".to_string()
    );
}

#[test]
fn evaluate_and_set_env_complex() {
    envmnt::set("eval_test10", "10");
    envmnt::set("eval_test20", "20");
    evaluate_and_set_env(
        "evaluate_and_set_env_complex",
        "checking 10 is ${eval_test10} empty is ${eval_test30} and 20 is ${eval_test20}",
    );
    assert_eq!(
        envmnt::get_or_panic("evaluate_and_set_env_complex"),
        "checking 10 is 10 empty is ${eval_test30} and 20 is 20".to_string()
    );
}

#[test]
fn set_env_for_bool_false() {
    envmnt::remove("BOOL_ENV_FALSE");
    set_env_for_bool("BOOL_ENV_FALSE", false);
    let output = envmnt::is_or("BOOL_ENV_FALSE", true);
    assert!(!output);
}

#[test]
fn set_env_for_bool_true() {
    envmnt::remove("BOOL_ENV_FALSE");
    set_env_for_bool("BOOL_ENV_FALSE", true);
    let output = envmnt::is_or("BOOL_ENV_FALSE", false);
    assert!(output);
}

#[test]
fn set_env_multi_types() {
    let current_profile_name = envmnt::get_or("CARGO_MAKE_PROFILE", "development");
    let mut profile_env = IndexMap::<String, EnvValue>::new();
    profile_env.insert(
        "profile_env".to_string(),
        EnvValue::Value("profile value".to_string()),
    );

    envmnt::remove("ENV_DECODE_EXPRESSIONS_MULTI_TYPE");
    envmnt::set("ENV_DECODE_EXPRESSIONS_MULTI_TYPE_VAR1", "ENV1");
    envmnt::set("ENV_DECODE_EXPRESSIONS_MULTI_TYPE_VAR2", "ENV2");
    envmnt::set("ENV_DECODE_EXPRESSIONS_MULTI_TYPE_VAR3", "ENV3");

    let mut mapping = HashMap::new();
    mapping.insert("key1".to_string(), "value1".to_string());
    mapping.insert("key2".to_string(), "value2".to_string());
    mapping.insert("key3".to_string(), "value3".to_string());
    mapping.insert(
        "ENV1".to_string(),
        "${ENV_DECODE_EXPRESSIONS_MULTI_TYPE_VAR2}-${ENV_DECODE_EXPRESSIONS_MULTI_TYPE_VAR3}"
            .to_string(),
    );

    let decode_info = EnvValueDecode {
        source: "${ENV_DECODE_EXPRESSIONS_MULTI_TYPE_VAR1}".to_string(),
        default_value: None,
        mapping,
    };

    let mut env = IndexMap::new();
    env.insert("value".to_string(), EnvValue::Value("test val".to_string()));
    env.insert("bool".to_string(), EnvValue::Boolean(false));
    env.insert(
        "script".to_string(),
        EnvValue::Script(EnvValueScript {
            script: vec!["echo script1".to_string()],
            multi_line: None,
        }),
    );
    env.insert(
        "ENV_DECODE_EXPRESSIONS_MULTI_TYPE".to_string(),
        EnvValue::Decode(decode_info),
    );
    env.insert(current_profile_name, EnvValue::Profile(profile_env));

    set_env(env);

    assert!(envmnt::is_equal("value", "test val"));
    assert!(!envmnt::is_or("bool", true));
    assert!(envmnt::is_equal("script", "script1"));
    assert!(envmnt::is_equal(
        "ENV_DECODE_EXPRESSIONS_MULTI_TYPE",
        "ENV2-ENV3"
    ));
    assert!(envmnt::is_equal("profile_env", "profile value"));
}

#[test]
fn set_env_for_decode_info_strings_found() {
    envmnt::remove("ENV_DECODE_STRING_FOUND");

    let mut mapping = HashMap::new();
    mapping.insert("key1".to_string(), "value1".to_string());
    mapping.insert("key2".to_string(), "value2".to_string());
    mapping.insert("key3".to_string(), "value3".to_string());

    let decode_info = EnvValueDecode {
        source: "key2".to_string(),
        default_value: None,
        mapping,
    };

    set_env_for_decode_info("ENV_DECODE_STRING_FOUND", &decode_info);

    assert!(envmnt::is_equal("ENV_DECODE_STRING_FOUND", "value2"));
}

#[test]
fn set_env_for_decode_info_strings_default() {
    envmnt::remove("ENV_DECODE_STRING_DEFAULT");

    let mut mapping = HashMap::new();
    mapping.insert("key1".to_string(), "value1".to_string());
    mapping.insert("key2".to_string(), "value2".to_string());
    mapping.insert("key3".to_string(), "value3".to_string());

    let decode_info = EnvValueDecode {
        source: "key0".to_string(),
        default_value: Some("default value".to_string()),
        mapping,
    };

    set_env_for_decode_info("ENV_DECODE_STRING_DEFAULT", &decode_info);

    assert!(envmnt::is_equal(
        "ENV_DECODE_STRING_DEFAULT",
        "default value"
    ));
}

#[test]
fn set_env_for_decode_info_strings_default_none() {
    envmnt::remove("ENV_DECODE_STRING_DEFAULT_NONE");

    let mut mapping = HashMap::new();
    mapping.insert("key1".to_string(), "value1".to_string());
    mapping.insert("key2".to_string(), "value2".to_string());
    mapping.insert("key3".to_string(), "value3".to_string());

    let decode_info = EnvValueDecode {
        source: "key0".to_string(),
        default_value: None,
        mapping,
    };

    set_env_for_decode_info("ENV_DECODE_STRING_DEFAULT_NONE", &decode_info);

    assert!(envmnt::is_equal("ENV_DECODE_STRING_DEFAULT_NONE", ""));
}

#[test]
fn set_env_for_decode_info_expressions() {
    envmnt::remove("ENV_DECODE_EXPRESSIONS");
    envmnt::set("ENV_DECODE_EXPRESSIONS_VAR1", "ENV1");
    envmnt::set("ENV_DECODE_EXPRESSIONS_VAR2", "ENV2");
    envmnt::set("ENV_DECODE_EXPRESSIONS_VAR3", "ENV3");

    let mut mapping = HashMap::new();
    mapping.insert("key1".to_string(), "value1".to_string());
    mapping.insert("key2".to_string(), "value2".to_string());
    mapping.insert("key3".to_string(), "value3".to_string());
    mapping.insert(
        "ENV1".to_string(),
        "${ENV_DECODE_EXPRESSIONS_VAR2}-${ENV_DECODE_EXPRESSIONS_VAR3}".to_string(),
    );

    let decode_info = EnvValueDecode {
        source: "${ENV_DECODE_EXPRESSIONS_VAR1}".to_string(),
        default_value: None,
        mapping,
    };

    set_env_for_decode_info("ENV_DECODE_EXPRESSIONS", &decode_info);

    assert!(envmnt::is_equal("ENV_DECODE_EXPRESSIONS", "ENV2-ENV3"));
}

#[test]
fn set_env_for_profile_none_not_found() {
    let mut env = IndexMap::new();
    env.insert(
        "TEST_PROFILE_NONE_NOT_FOUND".to_string(),
        EnvValue::Boolean(true),
    );

    set_env_for_profile("test_profile", &env, None);

    assert!(!envmnt::exists("TEST_PROFILE_NONE_NOT_FOUND"));
}

#[test]
fn set_env_for_profile_some_not_found() {
    let mut env = IndexMap::new();
    env.insert(
        "TEST_PROFILE_SOME_NOT_FOUND".to_string(),
        EnvValue::Boolean(true),
    );

    set_env_for_profile(
        "test_profile",
        &env,
        Some(&vec!["other_profile".to_string()]),
    );

    assert!(!envmnt::exists("TEST_PROFILE_SOME_NOT_FOUND"));
}

#[test]
fn set_env_for_profile_some_found() {
    let mut env = IndexMap::new();
    env.insert("TEST_PROFILE_FOUND".to_string(), EnvValue::Boolean(true));

    set_env_for_profile(
        "test_profile",
        &env,
        Some(&vec!["test_profile".to_string()]),
    );

    assert!(envmnt::exists("TEST_PROFILE_FOUND"));
    assert!(envmnt::is("TEST_PROFILE_FOUND"));
}

#[test]
fn setup_cwd_empty() {
    envmnt::set("CARGO_MAKE_WORKING_DIRECTORY", "EMPTY");

    setup_cwd(None);

    assert!(envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY") != "EMPTY");
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

    let mut value = envmnt::get_or_panic("CARGO_MAKE_TASK");
    assert_eq!(value, "setup_env_empty1");

    setup_env(&cli_args, &config, "setup_env_empty2");

    let delay = time::Duration::from_millis(10);
    thread::sleep(delay);

    value = envmnt::get_or_panic("CARGO_MAKE_TASK");
    assert_eq!(value, "setup_env_empty2");
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

    envmnt::set("CARGO_MAKE_TASK_ARGS", "EMPTY");

    setup_env(&cli_args, &config, "setup_env_empty1");

    let value = envmnt::get_or_panic("CARGO_MAKE_TASK_ARGS");
    assert_eq!(value, "arg1;arg2");
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

    assert_eq!(envmnt::get_or("MY_ENV_KEY", "NONE"), "NONE".to_string());
    assert_eq!(envmnt::get_or("MY_ENV_KEY2", "NONE"), "NONE".to_string());

    setup_env(&cli_args, &config, "set_env_values");

    assert_eq!(envmnt::get_or_panic("MY_ENV_KEY"), "MY_ENV_VALUE");
    assert_eq!(envmnt::get_or_panic("MY_ENV_KEY2"), "MY_ENV_VALUE2");
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
        EnvValue::Script(EnvValueScript {
            script: vec!["echo script1".to_string()],
            multi_line: None,
        }),
    );

    assert_eq!(
        envmnt::get_or("MY_ENV_SCRIPT_KEY", "NONE"),
        "NONE".to_string()
    );
    assert_eq!(
        envmnt::get_or("MY_ENV_SCRIPT_KEY2", "NONE"),
        "NONE".to_string()
    );

    setup_env(&cli_args, &config, "set_env_values");

    assert_eq!(envmnt::get_or_panic("MY_ENV_SCRIPT_KEY"), "MY_ENV_VALUE");
    assert_eq!(envmnt::get_or_panic("MY_ENV_SCRIPT_KEY2"), "script1");
}

#[test]
fn evaluate_env_value_valid() {
    let output = evaluate_env_value(&EnvValueScript {
        script: vec!["echo script1".to_string()],
        multi_line: None,
    });

    assert_eq!(output, "script1".to_string());
}

#[test]
#[cfg(target_os = "linux")]
fn evaluate_env_value_empty() {
    let output = evaluate_env_value(&EnvValueScript {
        script: vec!["".to_string()],
        multi_line: None,
    });

    assert_eq!(output, "".to_string());
}

#[test]
#[should_panic]
fn evaluate_env_error() {
    evaluate_env_value(&EnvValueScript {
        script: vec!["exit 1".to_string()],
        multi_line: None,
    });
}

#[test]
fn evaluate_env_value_single_line() {
    let output = evaluate_env_value(&EnvValueScript {
        script: vec!["echo test".to_string()],
        multi_line: Some(false),
    });

    assert!(output.contains("test"));
}

#[test]
fn evaluate_env_value_multi_line() {
    let output = evaluate_env_value(&EnvValueScript {
        script: vec!["echo 1\necho 2".to_string()],
        multi_line: Some(true),
    });

    assert!(output.contains("1"));
    assert!(output.contains("2"));
}

#[test]
fn setup_env_for_crate_load_toml_found() {
    envmnt::set("CARGO_MAKE_CRATE_NAME", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_FS_NAME", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_VERSION", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_DESCRIPTION", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_LICENSE", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_DOCUMENTATION", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_HOMEPAGE", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_REPOSITORY", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_IS_WORKSPACE", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_HAS_DEPENDENCIES", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS", "EMPTY");

    setup_env_for_crate();

    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_NAME"), "cargo-make");
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_FS_NAME"),
        "cargo_make"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_VERSION"),
        env!("CARGO_PKG_VERSION")
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_DESCRIPTION"),
        env!("CARGO_PKG_DESCRIPTION")
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_LICENSE"),
        "Apache-2.0"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_DOCUMENTATION"),
        "https://sagiegurari.github.io/cargo-make"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_HOMEPAGE"),
        "https://sagiegurari.github.io/cargo-make"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_REPOSITORY"),
        "https://github.com/sagiegurari/cargo-make.git"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_HAS_DEPENDENCIES"),
        "true"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_IS_WORKSPACE"),
        "false"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS"),
        ""
    );
}

#[test]
fn setup_env_for_crate_load_toml_not_found_and_cwd() {
    envmnt::set("CARGO_MAKE_CRATE_NAME", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_FS_NAME", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_VERSION", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_DESCRIPTION", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_LICENSE", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_DOCUMENTATION", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_HOMEPAGE", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_REPOSITORY", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_IS_WORKSPACE", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_HAS_DEPENDENCIES", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS", "EMPTY");

    envmnt::set("CARGO_MAKE_WORKING_DIRECTORY", "EMPTY");
    assert!(envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY") == "EMPTY");

    setup_cwd(Some("examples"));
    setup_env_for_crate();
    setup_cwd(Some(".."));

    assert!(envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY") != "EMPTY");

    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_NAME"), "EMPTY");
    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_FS_NAME"), "EMPTY");
    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_VERSION"), "EMPTY");
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_DESCRIPTION"),
        "EMPTY"
    );
    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_LICENSE"), "EMPTY");
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_DOCUMENTATION"),
        "EMPTY"
    );
    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_HOMEPAGE"), "EMPTY");
    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_REPOSITORY"), "EMPTY");
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_HAS_DEPENDENCIES"),
        "false"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_IS_WORKSPACE"),
        "false"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS"),
        ""
    );

    setup_env_for_crate();

    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_NAME"), "cargo-make");
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_FS_NAME"),
        "cargo_make"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_VERSION"),
        env!("CARGO_PKG_VERSION")
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_DESCRIPTION"),
        env!("CARGO_PKG_DESCRIPTION")
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_LICENSE"),
        "Apache-2.0"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_DOCUMENTATION"),
        "https://sagiegurari.github.io/cargo-make"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_HOMEPAGE"),
        "https://sagiegurari.github.io/cargo-make"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_REPOSITORY"),
        "https://github.com/sagiegurari/cargo-make.git"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_HAS_DEPENDENCIES"),
        "true"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_IS_WORKSPACE"),
        "false"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS"),
        ""
    );
}

#[test]
fn setup_env_for_crate_workspace() {
    envmnt::set("CARGO_MAKE_CRATE_NAME", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_FS_NAME", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_VERSION", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_DESCRIPTION", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_LICENSE", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_DOCUMENTATION", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_HOMEPAGE", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_REPOSITORY", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_HAS_DEPENDENCIES", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_IS_WORKSPACE", "EMPTY");
    envmnt::set("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS", "EMPTY");

    setup_cwd(Some("examples/workspace"));
    setup_env_for_crate();
    setup_cwd(Some("../.."));

    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_NAME"), "EMPTY");
    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_FS_NAME"), "EMPTY");
    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_VERSION"), "EMPTY");
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_DESCRIPTION"),
        "EMPTY"
    );
    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_LICENSE"), "EMPTY");
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_DOCUMENTATION"),
        "EMPTY"
    );
    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_HOMEPAGE"), "EMPTY");
    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CRATE_REPOSITORY"), "EMPTY");
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_HAS_DEPENDENCIES"),
        "true"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_IS_WORKSPACE"),
        "true"
    );
    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_CRATE_WORKSPACE_MEMBERS"),
        "member1,member2"
    );
}

#[test]
fn setup_env_for_git_repo_with_values() {
    envmnt::set("CARGO_MAKE_GIT_BRANCH", "EMPTY");
    envmnt::set("CARGO_MAKE_GIT_USER_NAME", "EMPTY");
    envmnt::set("CARGO_MAKE_GIT_USER_EMAIL", "EMPTY");

    let git_info = setup_env_for_git_repo();

    if git_info.branch.is_some() {
        assert_eq!(
            envmnt::get_or_panic("CARGO_MAKE_GIT_BRANCH"),
            git_info.branch.unwrap()
        );
    }
    if git_info.user_name.is_some() {
        assert_eq!(
            envmnt::get_or_panic("CARGO_MAKE_GIT_USER_NAME"),
            git_info.user_name.unwrap()
        );
    }
    if git_info.user_email.is_some() {
        assert_eq!(
            envmnt::get_or_panic("CARGO_MAKE_GIT_USER_EMAIL"),
            git_info.user_email.unwrap()
        );
    }
}

#[test]
fn setup_env_for_rust_simple_check() {
    envmnt::set("CARGO_MAKE_RUST_VERSION", "EMPTY");
    envmnt::set("CARGO_MAKE_RUST_CHANNEL", "EMPTY");
    envmnt::set("CARGO_MAKE_RUST_TARGET_ARCH", "EMPTY");
    envmnt::set("CARGO_MAKE_RUST_TARGET_ENV", "EMPTY");
    envmnt::set("CARGO_MAKE_RUST_TARGET_OS", "EMPTY");
    envmnt::set("CARGO_MAKE_RUST_TARGET_POINTER_WIDTH", "EMPTY");
    envmnt::set("CARGO_MAKE_RUST_TARGET_VENDOR", "EMPTY");

    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_VERSION") == "EMPTY");
    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_CHANNEL") == "EMPTY");
    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_TARGET_ARCH") == "EMPTY");
    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_TARGET_ENV") == "EMPTY");
    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_TARGET_OS") == "EMPTY");
    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_TARGET_POINTER_WIDTH") == "EMPTY");
    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_TARGET_VENDOR") == "EMPTY");

    setup_env_for_rust();

    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_VERSION") != "EMPTY");
    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_CHANNEL") != "EMPTY");
    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_TARGET_ARCH") != "EMPTY");
    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_TARGET_ENV") != "EMPTY");
    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_TARGET_OS") != "EMPTY");
    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_TARGET_POINTER_WIDTH") != "EMPTY");
    assert!(envmnt::get_or_panic("CARGO_MAKE_RUST_TARGET_VENDOR") != "EMPTY");
}

#[test]
fn setup_env_for_ci_simple_check() {
    envmnt::set("CARGO_MAKE_CI", "EMPTY");

    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CI"), "EMPTY");

    let ci = ci_info::is_ci();
    let env_value = if ci { "true" } else { "false" };

    setup_env_for_ci();

    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_CI"), env_value);
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
    envmnt::set("TEST_ENV_EXPAND1", "ENV1");
    envmnt::set("TEST_ENV_EXPAND2", "ENV2");

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
    envmnt::set("TEST_ENV_EXPAND1", "ENV1");
    envmnt::set("TEST_ENV_EXPAND2", "ENV2");
    envmnt::set("CARGO_MAKE_TASK_ARGS", "targ1;targ2;targ3;targ4");

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
    envmnt::set("TEST_ENV_EXPAND1", "ENV1");
    envmnt::set("TEST_ENV_EXPAND2", "ENV2");
    envmnt::set("CARGO_MAKE_TASK_ARGS", "");

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
