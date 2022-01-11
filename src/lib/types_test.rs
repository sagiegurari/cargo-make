use super::*;

fn get_script_as_vec(script: Option<ScriptValue>) -> Vec<String> {
    match script.unwrap() {
        ScriptValue::Text(value) => value.clone(),
        _ => panic!("Invalid script value type."),
    }
}

fn get_script_vec(task: &Task) -> Vec<String> {
    get_script_as_vec(task.script.clone())
}

#[test]
fn extend_script_value_both_none() {
    let output = extend_script_value(None, None);

    assert!(output.is_none());
}

#[test]
fn extend_script_value_current_text_new_none() {
    let output = extend_script_value(Some(ScriptValue::SingleLine("test".to_string())), None);

    match output.unwrap() {
        ScriptValue::SingleLine(value) => assert_eq!(value, "test"),
        _ => panic!("invalid type"),
    };
}

#[test]
fn extend_script_value_current_none_new_text() {
    let output = extend_script_value(None, Some(ScriptValue::SingleLine("test".to_string())));

    match output.unwrap() {
        ScriptValue::SingleLine(value) => assert_eq!(value, "test"),
        _ => panic!("invalid type"),
    };
}

#[test]
fn extend_script_value_current_text_new_text() {
    let output = extend_script_value(
        Some(ScriptValue::SingleLine("current".to_string())),
        Some(ScriptValue::SingleLine("new".to_string())),
    );

    match output.unwrap() {
        ScriptValue::SingleLine(value) => assert_eq!(value, "new"),
        _ => panic!("invalid type"),
    };
}

#[test]
fn extend_script_value_current_new_different_types() {
    let output = extend_script_value(
        Some(ScriptValue::Text(vec!["current".to_string()])),
        Some(ScriptValue::SingleLine("new".to_string())),
    );

    match output.unwrap() {
        ScriptValue::SingleLine(value) => assert_eq!(value, "new"),
        _ => panic!("invalid type"),
    };
}

#[test]
fn extend_script_value_new_all_content_sections() {
    let output = extend_script_value(
        Some(ScriptValue::Sections(ScriptSections {
            pre: Some("current_pre".to_string()),
            main: Some("current_main".to_string()),
            post: Some("current_post".to_string()),
        })),
        Some(ScriptValue::Sections(ScriptSections {
            pre: Some("new_pre".to_string()),
            main: Some("new_main".to_string()),
            post: Some("new_post".to_string()),
        })),
    );

    match output.unwrap() {
        ScriptValue::Sections(sections) => {
            assert_eq!(sections.pre.unwrap(), "new_pre");
            assert_eq!(sections.main.unwrap(), "new_main");
            assert_eq!(sections.post.unwrap(), "new_post");
        }
        _ => panic!("invalid type"),
    };
}

#[test]
fn extend_script_value_new_only_pre_content_section() {
    let output = extend_script_value(
        Some(ScriptValue::Sections(ScriptSections {
            pre: Some("current_pre".to_string()),
            main: Some("current_main".to_string()),
            post: Some("current_post".to_string()),
        })),
        Some(ScriptValue::Sections(ScriptSections {
            pre: Some("new_pre".to_string()),
            main: None,
            post: None,
        })),
    );

    match output.unwrap() {
        ScriptValue::Sections(sections) => {
            assert_eq!(sections.pre.unwrap(), "new_pre");
            assert_eq!(sections.main.unwrap(), "current_main");
            assert_eq!(sections.post.unwrap(), "current_post");
        }
        _ => panic!("invalid type"),
    };
}

#[test]
fn extend_script_value_new_only_post_content_section() {
    let output = extend_script_value(
        Some(ScriptValue::Sections(ScriptSections {
            pre: Some("current_pre".to_string()),
            main: Some("current_main".to_string()),
            post: Some("current_post".to_string()),
        })),
        Some(ScriptValue::Sections(ScriptSections {
            pre: None,
            main: None,
            post: Some("new_post".to_string()),
        })),
    );

    match output.unwrap() {
        ScriptValue::Sections(sections) => {
            assert_eq!(sections.pre.unwrap(), "current_pre");
            assert_eq!(sections.main.unwrap(), "current_main");
            assert_eq!(sections.post.unwrap(), "new_post");
        }
        _ => panic!("invalid type"),
    };
}

#[test]
fn extend_script_value_new_only_main_content_section() {
    let output = extend_script_value(
        Some(ScriptValue::Sections(ScriptSections {
            pre: Some("current_pre".to_string()),
            main: Some("current_main".to_string()),
            post: Some("current_post".to_string()),
        })),
        Some(ScriptValue::Sections(ScriptSections {
            pre: None,
            main: Some("new_main".to_string()),
            post: None,
        })),
    );

    match output.unwrap() {
        ScriptValue::Sections(sections) => {
            assert_eq!(sections.pre.unwrap(), "current_pre");
            assert_eq!(sections.main.unwrap(), "new_main");
            assert_eq!(sections.post.unwrap(), "current_post");
        }
        _ => panic!("invalid type"),
    };
}

#[test]
fn cli_args_new() {
    let cli_args = CliArgs::new();

    assert_eq!(cli_args.command, "");
    assert!(cli_args.build_file.is_none());
    assert_eq!(cli_args.task, "default");
    assert!(cli_args.profile.is_none());
    assert_eq!(cli_args.log_level, "info");
    assert!(!cli_args.disable_color);
    assert!(cli_args.cwd.is_none());
    assert!(cli_args.env.is_none());
    assert!(cli_args.env_file.is_none());
    assert!(!cli_args.disable_workspace);
    assert!(!cli_args.disable_on_error);
    assert!(!cli_args.allow_private);
    assert!(!cli_args.skip_init_end_tasks);
    assert!(cli_args.skip_tasks_pattern.is_none());
    assert!(!cli_args.disable_check_for_updates);
    assert!(!cli_args.print_only);
    assert!(!cli_args.list_all_steps);
    assert!(!cli_args.diff_execution_plan);
    assert!(!cli_args.experimental);
    assert!(cli_args.arguments.is_none());
    assert_eq!(cli_args.output_format, "default");
    assert!(cli_args.output_file.is_none());
    assert!(!cli_args.print_time_summary);
}

#[test]
fn flow_state_new() {
    let flow_state = FlowState::new();

    assert!(flow_state.time_summary.is_empty());
    assert!(flow_state.forced_plugin.is_none());
}

#[test]
fn global_config_new() {
    let global_config = GlobalConfig::new();

    assert!(global_config.file_name.is_none());
    assert!(global_config.log_level.is_none());
    assert!(global_config.default_task_name.is_none());
    assert!(global_config.update_check_minimum_interval.is_none());
    assert!(!global_config.search_project_root.unwrap());
    assert!(global_config.disable_color.is_none());
}

#[test]
fn cache_new() {
    let cache = Cache::new();

    assert!(cache.file_name.is_none());
    assert!(cache.last_update_check.is_none());
}

#[test]
fn install_cargo_plugin_info_eq_same_all() {
    let first = InstallCargoPluginInfo {
        crate_name: Some("test".to_string()),
        min_version: Some("1.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCargoPluginInfo {
        crate_name: Some("test".to_string()),
        min_version: Some("1.0.0".to_string()),
        install_command: Some("install".to_string()),
    };

    assert_eq!(first, second);
}

#[test]
fn install_cargo_plugin_info_eq_same_no_crate_name() {
    let first = InstallCargoPluginInfo {
        crate_name: None,
        min_version: Some("1.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCargoPluginInfo {
        crate_name: None,
        min_version: Some("1.0.0".to_string()),
        install_command: Some("install".to_string()),
    };

    assert_eq!(first, second);
}

#[test]
fn install_cargo_plugin_info_eq_same_no_install_command() {
    let first = InstallCargoPluginInfo {
        crate_name: Some("test".to_string()),
        min_version: Some("1.0.0".to_string()),
        install_command: None,
    };
    let second = InstallCargoPluginInfo {
        crate_name: Some("test".to_string()),
        min_version: Some("1.0.0".to_string()),
        install_command: None,
    };

    assert_eq!(first, second);
}

#[test]
fn install_cargo_plugin_info_eq_different_crate_name_type() {
    let first = InstallCargoPluginInfo {
        crate_name: Some("test".to_string()),
        min_version: Some("1.0.0".to_string()),
        install_command: None,
    };
    let second = InstallCargoPluginInfo {
        crate_name: None,
        min_version: Some("1.0.0".to_string()),
        install_command: None,
    };

    assert!(first != second);
}

#[test]
fn install_cargo_plugin_info_eq_different_crate_name_value() {
    let first = InstallCargoPluginInfo {
        crate_name: Some("test1".to_string()),
        min_version: Some("1.0.0".to_string()),
        install_command: None,
    };
    let second = InstallCargoPluginInfo {
        crate_name: Some("test2".to_string()),
        min_version: Some("1.0.0".to_string()),
        install_command: None,
    };

    assert!(first != second);
}

#[test]
fn install_cargo_plugin_info_eq_different_install_command_type() {
    let first = InstallCargoPluginInfo {
        crate_name: None,
        min_version: Some("1.0.0".to_string()),
        install_command: Some("test".to_string()),
    };
    let second = InstallCargoPluginInfo {
        crate_name: None,
        min_version: Some("1.0.0".to_string()),
        install_command: None,
    };

    assert!(first != second);
}

#[test]
fn install_cargo_plugin_info_eq_different_install_command_value() {
    let first = InstallCargoPluginInfo {
        crate_name: None,
        min_version: Some("1.0.0".to_string()),
        install_command: Some("test1".to_string()),
    };
    let second = InstallCargoPluginInfo {
        crate_name: None,
        min_version: Some("1.0.0".to_string()),
        install_command: Some("test2".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_cargo_plugin_info_eq_different_min_version_value() {
    let first = InstallCargoPluginInfo {
        crate_name: None,
        min_version: Some("1.0.0".to_string()),
        install_command: None,
    };
    let second = InstallCargoPluginInfo {
        crate_name: None,
        min_version: Some("2.0.0".to_string()),
        install_command: None,
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_same_all() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("component".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("component".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };

    assert_eq!(first, second);
}

#[test]
fn install_crate_info_eq_same_no_component() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: None,
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: None,
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };

    assert_eq!(first, second);
}

#[test]
fn install_crate_info_eq_same_no_min_version() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("component".to_string()),
        min_version: None,
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("component".to_string()),
        min_version: None,
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };

    assert_eq!(first, second);
}

#[test]
fn install_crate_info_eq_same_no_version() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("component".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: None,
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("component".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: None,
        install_command: Some("install".to_string()),
    };

    assert_eq!(first, second);
}

#[test]
fn install_crate_info_eq_same_no_install_command() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("component".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: None,
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("component".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: None,
    };

    assert_eq!(first, second);
}

#[test]
fn install_crate_info_eq_different_crate_name() {
    let first = InstallCrateInfo {
        crate_name: "test1".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: None,
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test2".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: None,
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_binary() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin1".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: None,
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin2".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: None,
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_test_arg() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help1".to_string()],
        },
        rustup_component_name: None,
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help2".to_string()],
        },
        rustup_component_name: None,
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_component_type() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: None,
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_component_value() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value1".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value2".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_min_version_type() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: None,
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_min_version_value() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: Some("2.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_version_type() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: None,
        install_command: Some("install".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_version_value() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("1.0.0".to_string()),
        install_command: Some("install".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_install_command_type() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: None,
    };

    assert!(first != second);
}

#[test]
fn install_crate_info_eq_different_install_command_value() {
    let first = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install1".to_string()),
    };
    let second = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install2".to_string()),
    };

    assert!(first != second);
}

#[test]
fn install_rustup_component_info_eq_same_all() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };

    assert_eq!(first, second);
}

#[test]
fn install_rustup_component_info_eq_same_no_binary() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: None,
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: None,
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };

    assert_eq!(first, second);
}

#[test]
fn install_rustup_component_info_eq_same_no_test_arg() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: None,
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: None,
    };

    assert_eq!(first, second);
}

#[test]
fn install_rustup_component_info_eq_different_component() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component1".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component2".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };

    assert!(first != second);
}

#[test]
fn install_rustup_component_info_eq_different_binary() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin1".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin2".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };

    assert!(first != second);
}

#[test]
fn install_rustup_component_info_eq_different_binary_type() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: None,
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };

    assert!(first != second);
}

#[test]
fn install_rustup_component_info_eq_different_test_arg() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--hel1p".to_string()],
        }),
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help2".to_string()],
        }),
    };

    assert!(first != second);
}

#[test]
fn install_rustup_component_info_eq_different_test_arg_type() {
    let first = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: Some("bin".to_string()),
        test_arg: None,
    };
    let second = InstallRustupComponentInfo {
        rustup_component_name: "component".to_string(),
        binary: None,
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };

    assert!(first != second);
}

#[test]
fn install_crate_eq_same_enabled_value() {
    let first = InstallCrate::Enabled(true);
    let second = InstallCrate::Enabled(true);

    assert_eq!(first, second);
}

#[test]
fn install_crate_eq_same_disabled_value() {
    let first = InstallCrate::Enabled(false);
    let second = InstallCrate::Enabled(false);

    assert_eq!(first, second);
}

#[test]
fn install_crate_eq_same_value() {
    let first = InstallCrate::Value("crate".to_string());
    let second = InstallCrate::Value("crate".to_string());

    assert_eq!(first, second);
}

#[test]
fn install_crate_eq_same_cargo_plugin_info() {
    let info = InstallCargoPluginInfo {
        crate_name: Some("test".to_string()),
        min_version: Some("1.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let first = InstallCrate::CargoPluginInfo(info.clone());
    let second = InstallCrate::CargoPluginInfo(info.clone());

    assert_eq!(first, second);
}

#[test]
fn install_crate_eq_same_crate_info() {
    let info = InstallCrateInfo {
        crate_name: "test".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: Some("1.0.0".to_string()),
        version: Some("2.0.0".to_string()),
        install_command: Some("install".to_string()),
    };
    let first = InstallCrate::CrateInfo(info.clone());
    let second = InstallCrate::CrateInfo(info.clone());

    assert_eq!(first, second);
}

#[test]
fn install_crate_eq_same_rustup_component_info() {
    let info = InstallRustupComponentInfo {
        rustup_component_name: "value".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    };
    let first = InstallCrate::RustupComponentInfo(info.clone());
    let second = InstallCrate::RustupComponentInfo(info.clone());

    assert_eq!(first, second);
}

#[test]
fn install_crate_eq_different_enabled_value() {
    let first = InstallCrate::Enabled(true);
    let second = InstallCrate::Enabled(false);

    assert!(first != second);
}

#[test]
fn install_crate_eq_different_value() {
    let first = InstallCrate::Value("crate1".to_string());
    let second = InstallCrate::Value("crate2".to_string());

    assert!(first != second);
}

#[test]
fn install_crate_eq_different_cargo_plugin_info() {
    let first = InstallCrate::CargoPluginInfo(InstallCargoPluginInfo {
        crate_name: Some("test1".to_string()),
        min_version: Some("1.0.0".to_string()),
        install_command: Some("install".to_string()),
    });
    let second = InstallCrate::CargoPluginInfo(InstallCargoPluginInfo {
        crate_name: Some("test2".to_string()),
        min_version: Some("1.0.0".to_string()),
        install_command: Some("install".to_string()),
    });

    assert!(first != second);
}

#[test]
fn install_crate_eq_different_crate_info() {
    let first = InstallCrate::CrateInfo(InstallCrateInfo {
        crate_name: "test1".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: None,
        version: None,
        install_command: Some("install".to_string()),
    });
    let second = InstallCrate::CrateInfo(InstallCrateInfo {
        crate_name: "test2".to_string(),
        binary: "bin".to_string(),
        test_arg: TestArg {
            inner: vec!["--help".to_string()],
        },
        rustup_component_name: Some("value".to_string()),
        min_version: None,
        version: None,
        install_command: Some("install".to_string()),
    });

    assert!(first != second);
}

#[test]
fn install_crate_eq_different_rustup_component_info() {
    let first = InstallCrate::RustupComponentInfo(InstallRustupComponentInfo {
        rustup_component_name: "component1".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    });
    let second = InstallCrate::RustupComponentInfo(InstallRustupComponentInfo {
        rustup_component_name: "component2".to_string(),
        binary: Some("bin".to_string()),
        test_arg: Some(TestArg {
            inner: vec!["--help".to_string()],
        }),
    });

    assert!(first != second);
}

#[test]
fn install_crate_info_deserialize_string_test_arg() {
    let install_crate: InstallCrate = toml::from_str(
        r#"
        crate_name = "mkisofs-rs"
        binary = "mkisofs-rs"
        test_arg = "--help"
        "#,
    )
    .unwrap();
    if let InstallCrate::CrateInfo(info) = install_crate {
        assert_eq!(*info.test_arg, &["--help"]);
    } else {
        panic!("invalid install crate type");
    }
}

#[test]
fn install_crate_info_deserialize_array_test_arg() {
    let install_crate: InstallCrate = toml::from_str(
        r#"
        crate_name = "mkisofs-rs"
        binary = "mkisofs-rs"
        test_arg = ["--help", "--test"]
        "#,
    )
    .unwrap();
    if let InstallCrate::CrateInfo(info) = install_crate {
        assert_eq!(*info.test_arg, &["--help", "--test"]);
    } else {
        panic!("invalid install crate type");
    }
}

#[test]
fn install_crate_info_deserialize_install_command() {
    let install_crate: InstallCrate = toml::from_str(
        r#"
        crate_name = "mkisofs-rs"
        binary = "mkisofs-rs"
        test_arg = "--help"
        install_command = "test_install"
        "#,
    )
    .unwrap();
    if let InstallCrate::CrateInfo(info) = install_crate {
        assert_eq!(*info.test_arg, &["--help"]);
        assert_eq!(info.install_command.unwrap(), "test_install");
    } else {
        panic!("invalid install crate type");
    }
}

#[test]
fn install_crate_info_deserialize_only_install_command() {
    let toml_str = r#"
        install_command = "test_install"
        "#;
    let install_crate: InstallCrate = toml::from_str(&toml_str).unwrap();
    if let InstallCrate::CargoPluginInfo(info) = install_crate {
        assert_eq!(info.install_command.unwrap(), "test_install");
    } else {
        panic!("invalid install crate type");
    }
}

#[test]
fn install_crate_info_deserialize_crate_and_install_command() {
    let toml_str = r#"
        crate_name = "some_crate"
        install_command = "test_install"
        "#;
    let install_crate: InstallCrate = toml::from_str(&toml_str).unwrap();
    if let InstallCrate::CargoPluginInfo(info) = install_crate {
        assert_eq!(info.crate_name.unwrap(), "some_crate");
        assert_eq!(info.install_command.unwrap(), "test_install");
    } else {
        panic!("invalid install crate type");
    }
}

#[test]
#[should_panic]
fn install_crate_info_deserialize_missing_test_arg() {
    let _info: InstallCrateInfo = toml::from_str(
        r#"
        crate_name = "mkisofs-rs"
        binary = "mkisofs-rs
        "#,
    )
    .unwrap();
}

#[test]
fn install_rustup_component_info_deserialize_string_test_arg() {
    let info: InstallRustupComponentInfo = toml::from_str(
        r#"
        rustup_component_name = "clippy-preview"
        binary = "cargo-clippy"
        test_arg = "--help"
        "#,
    )
    .unwrap();
    assert_eq!(*info.test_arg.unwrap(), &["--help"]);
}

#[test]
fn install_rustup_component_info_deserialize_array_test_arg() {
    let info: InstallRustupComponentInfo = toml::from_str(
        r#"
        rustup_component_name = "clippy-preview"
        binary = "cargo"
        test_arg = ["clippy", "--help"]
        "#,
    )
    .unwrap();
    assert_eq!(*info.test_arg.unwrap(), &["clippy", "--help"]);
}

#[test]
fn install_rustup_component_info_deserialize_missing_test_arg() {
    let info: InstallRustupComponentInfo = toml::from_str(
        r#"
        rustup_component_name = "clippy-preview"
        binary = "cargo"
        "#,
    )
    .unwrap();

    assert_eq!(info.test_arg, None);
}

#[test]
fn env_value_deserialize_string() {
    let config: ExternalConfig = toml::from_str(
        r#"
        [env]
        key = "value"
        "#,
    )
    .unwrap();
    let env = config.env.unwrap();

    for (_, info) in &env {
        match info {
            EnvValue::Value(value) => assert_eq!(value, "value"),
            _ => panic!("invalid env value type"),
        };
    }
}

#[test]
fn env_value_deserialize_bool_true() {
    let config: ExternalConfig = toml::from_str(
        r#"
        [env]
        key = true
        "#,
    )
    .unwrap();
    let env = config.env.unwrap();

    for (_, info) in &env {
        match info {
            EnvValue::Boolean(value) => assert!(value),
            _ => panic!("invalid env value type"),
        };
    }
}

#[test]
fn env_value_deserialize_bool_false() {
    let config: ExternalConfig = toml::from_str(
        r#"
        [env]
        key = false
        "#,
    )
    .unwrap();
    let env = config.env.unwrap();

    for (_, info) in &env {
        match info {
            EnvValue::Boolean(value) => assert!(!value),
            _ => panic!("invalid env value type"),
        };
    }
}

#[test]
fn env_value_deserialize_list_empty() {
    let config: ExternalConfig = toml::from_str(
        r#"
        [env]
        key = []
        "#,
    )
    .unwrap();
    let env = config.env.unwrap();

    for (_, info) in &env {
        match info {
            EnvValue::List(value) => assert!(value.is_empty()),
            _ => panic!("invalid env value type"),
        };
    }
}

#[test]
fn env_value_deserialize_list_with_values() {
    let config: ExternalConfig = toml::from_str(
        r#"
        [env]
        key = ["1", "2", "3"]
        "#,
    )
    .unwrap();
    let env = config.env.unwrap();

    for (_, info) in &env {
        match info {
            EnvValue::List(value) => assert_eq!(value, &vec!["1", "2", "3"]),
            _ => panic!("invalid env value type"),
        };
    }
}

#[test]
fn env_value_deserialize_script() {
    let config: ExternalConfig = toml::from_str(
        r#"
        [env]
        key = { script = ["echo test"] }
        "#,
    )
    .unwrap();
    let env = config.env.unwrap();

    for (_, info) in &env {
        match info {
            EnvValue::Script(value) => assert_eq!(value.script[0], "echo test"),
            _ => panic!("invalid env value type"),
        };
    }
}

#[test]
fn env_value_deserialize_decode() {
    let config: ExternalConfig = toml::from_str(
        r#"
        [env]
        key = { source = "source value", default_value = "default value", mapping = { "key1" = "value1", "key2" = "value2" } }
        "#,
    )
    .unwrap();
    let env = config.env.unwrap();

    for (_, info) in &env {
        match info {
            EnvValue::Decode(value) => {
                assert_eq!(value.source, "source value");
                assert_eq!(value.default_value, Some("default value".to_string()));
                assert_eq!(value.mapping.len(), 2);

                ()
            }
            _ => panic!("invalid env value type"),
        };
    }
}

#[test]
fn env_value_deserialize_conditional_env_value_no_condition() {
    let config: ExternalConfig = toml::from_str(
        r#"
        [env]
        key = { value = "value" }
        "#,
    )
    .unwrap();
    let env = config.env.unwrap();

    for (_, info) in &env {
        match info {
            EnvValue::Conditional(value) => {
                assert_eq!(value.value, "value");
                assert!(value.condition.is_none());

                ()
            }
            _ => panic!("invalid env value type"),
        };
    }
}

#[test]
fn env_value_deserialize_conditional_env_value_with_condition() {
    let config: ExternalConfig = toml::from_str(
        r#"
        [env]
        key = { value = "value", condition = {} }
        "#,
    )
    .unwrap();
    let env = config.env.unwrap();

    for (_, info) in &env {
        match info {
            EnvValue::Conditional(value) => {
                assert_eq!(value.value, "value");
                assert!(value.condition.is_some());

                ()
            }
            _ => panic!("invalid env value type"),
        };
    }
}

#[test]
fn env_value_deserialize_profile() {
    let config: ExternalConfig = toml::from_str(
        r#"
        [env.my-profile]
        bool_key = true
        string_key = "value"
        "#,
    )
    .unwrap();
    let env = config.env.unwrap();

    for (_, info) in &env {
        match info {
            EnvValue::Profile(value) => {
                assert_eq!(value.len(), 2);

                ()
            }
            _ => panic!("invalid env value type"),
        };
    }
}

#[test]
fn env_value_deserialize_unset() {
    let config: ExternalConfig = toml::from_str(
        r#"
        [env]
        key = { unset = true }
        "#,
    )
    .unwrap();
    let env = config.env.unwrap();

    for (_, info) in &env {
        match info {
            EnvValue::Unset(value) => {
                assert!(value.unset);

                ()
            }
            _ => panic!("invalid env value type"),
        };
    }
}

#[test]
fn toolchain_specifier_deserialize_string() {
    #[derive(Deserialize)]
    struct Value {
        toolchain: ToolchainSpecifier,
    }

    let v: Value = toml::from_str(
        r#"
        toolchain = "stable"
        "#,
    )
    .unwrap();
    assert_eq!(
        v.toolchain,
        ToolchainSpecifier::Simple("stable".to_string())
    );
}

#[test]
fn toolchain_specifier_deserialize_min_version() {
    #[derive(Deserialize)]
    struct Value {
        toolchain: ToolchainSpecifier,
    }

    let v: Value = toml::from_str(
        r#"
        toolchain = { channel = "beta", min_version = "1.56" }
        "#,
    )
    .unwrap();
    assert_eq!(
        v.toolchain,
        ToolchainSpecifier::Bounded(ToolchainBoundedSpecifier {
            channel: "beta".to_string(),
            min_version: "1.56".to_string(),
        })
    );
}

#[test]
fn task_new() {
    let task = Task::new();

    assert!(task.clear.is_none());
    assert!(task.install_crate.is_none());
    assert!(task.install_crate_args.is_none());
    assert!(task.command.is_none());
    assert!(task.disabled.is_none());
    assert!(task.private.is_none());
    assert!(task.deprecated.is_none());
    assert!(task.extend.is_none());
    assert!(task.watch.is_none());
    assert!(task.condition.is_none());
    assert!(task.condition_script.is_none());
    assert!(task.description.is_none());
    assert!(task.category.is_none());
    assert!(task.workspace.is_none());
    assert!(task.plugin.is_none());
    assert!(task.ignore_errors.is_none());
    assert!(task.force.is_none());
    assert!(task.env_files.is_none());
    assert!(task.env.is_none());
    assert!(task.cwd.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.install_script.is_none());
    assert!(task.args.is_none());
    assert!(task.script.is_none());
    assert!(task.script_runner.is_none());
    assert!(task.script_runner_args.is_none());
    assert!(task.script_extension.is_none());
    assert!(task.run_task.is_none());
    assert!(task.dependencies.is_none());
    assert!(task.toolchain.is_none());
    assert!(task.linux.is_none());
    assert!(task.windows.is_none());
    assert!(task.mac.is_none());
}

#[test]
fn external_config_new() {
    let config = ExternalConfig::new();

    assert!(config.extend.is_none());
    assert!(config.config.is_none());
    assert!(config.env_files.is_none());
    assert!(config.env.is_none());
    assert!(config.env_scripts.is_none());
    assert!(config.tasks.is_none());
    assert!(config.plugins.is_none());
}

#[test]
fn task_should_ignore_errors_none() {
    let task = Task::new();
    assert!(!task.should_ignore_errors());
}

#[test]
fn task_should_ignore_errors_false() {
    let mut task = Task::new();
    task.ignore_errors = Some(false);
    assert!(!task.should_ignore_errors());
}

#[test]
fn task_should_ignore_errors_true() {
    let mut task = Task::new();
    task.ignore_errors = Some(true);
    assert!(task.should_ignore_errors());
}

#[test]
fn task_should_ignore_errors_force_false() {
    let mut task = Task::new();
    task.force = Some(false);
    assert!(!task.should_ignore_errors());
}

#[test]
fn task_should_ignore_errors_force_true() {
    let mut task = Task::new();
    task.force = Some(true);
    assert!(task.should_ignore_errors());
}

#[test]
fn task_should_ignore_errors_false_force_true() {
    let mut task = Task::new();
    task.ignore_errors = Some(false);
    task.force = Some(true);
    assert!(!task.should_ignore_errors());
}

#[test]
fn task_extend_both_have_misc_data() {
    let mut base = Task::new();
    base.install_crate = Some(InstallCrate::Value("my crate1".to_string()));
    base.command = Some("test1".to_string());
    base.disabled = Some(false);
    base.private = Some(false);
    base.deprecated = Some(DeprecationInfo::Message("base".to_string()));
    base.watch = Some(TaskWatchOptions::Boolean(false));
    base.script = Some(ScriptValue::Text(vec!["1".to_string(), "2".to_string()]));

    let extended = Task {
        clear: Some(false),
        install_crate: Some(InstallCrate::Value("my crate2".to_string())),
        command: None,
        description: None,
        category: None,
        workspace: None,
        plugin: None,
        disabled: Some(true),
        private: Some(true),
        deprecated: Some(DeprecationInfo::Message("extended".to_string())),
        extend: None,
        watch: Some(TaskWatchOptions::Boolean(true)),
        condition: None,
        condition_script: None,
        ignore_errors: Some(true),
        force: Some(true),
        env_files: Some(vec![]),
        env: Some(IndexMap::new()),
        cwd: None,
        alias: Some("alias2".to_string()),
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        install_crate_args: None,
        install_script: None,
        args: None,
        script: None,
        script_runner: None,
        script_runner_args: None,
        script_extension: None,
        run_task: None,
        dependencies: None,
        toolchain: None,
        linux: None,
        windows: None,
        mac: None,
    };

    base.extend(&extended);

    assert!(!base.clear.unwrap());
    assert!(base.install_crate.is_some());
    assert!(base.command.is_some());
    assert!(base.description.is_none());
    assert!(base.category.is_none());
    assert!(base.workspace.is_none());
    assert!(base.plugin.is_none());
    assert!(base.disabled.is_some());
    assert!(base.private.is_some());
    assert!(base.deprecated.is_some());
    assert!(base.extend.is_none());
    assert!(base.watch.is_some());
    assert!(base.condition.is_none());
    assert!(base.condition_script.is_none());
    assert!(base.ignore_errors.is_some());
    assert!(base.force.is_some());
    assert!(base.env_files.is_some());
    assert!(base.env.is_some());
    assert!(base.cwd.is_none());
    assert!(base.alias.is_some());
    assert!(base.linux_alias.is_none());
    assert!(base.windows_alias.is_none());
    assert!(base.mac_alias.is_none());
    assert!(base.install_crate_args.is_none());
    assert!(base.install_script.is_none());
    assert!(base.script_runner.is_none());
    assert!(base.script_runner_args.is_none());
    assert!(base.script_extension.is_none());
    assert!(base.run_task.is_none());
    assert!(base.args.is_none());
    assert!(base.script.is_some());
    assert!(base.dependencies.is_none());
    assert!(base.toolchain.is_none());
    assert!(base.linux.is_none());
    assert!(base.windows.is_none());
    assert!(base.mac.is_none());

    assert_eq!(get_script_vec(&base).len(), 2);
    assert_eq!(
        base.install_crate.unwrap(),
        InstallCrate::Value("my crate2".to_string())
    );
    assert_eq!(base.command.unwrap(), "test1");
    assert!(base.disabled.unwrap());
    assert!(base.private.unwrap());
    assert_eq!(
        base.deprecated.unwrap(),
        DeprecationInfo::Message("extended".to_string())
    );
    assert_eq!(base.watch.unwrap(), TaskWatchOptions::Boolean(true));
    assert!(base.ignore_errors.unwrap());
    assert!(base.force.unwrap());
    assert_eq!(base.env_files.unwrap().len(), 0);
    assert_eq!(base.env.unwrap().len(), 0);
    assert_eq!(base.alias.unwrap(), "alias2");
}

#[test]
fn task_extend_extended_have_all_fields() {
    let mut base = Task {
        clear: Some(true),
        install_crate: Some(InstallCrate::Value("my crate1".to_string())),
        command: Some("test1".to_string()),
        description: None,
        category: None,
        workspace: None,
        plugin: None,
        disabled: Some(false),
        private: Some(true),
        deprecated: Some(DeprecationInfo::Boolean(true)),
        extend: Some("base".to_string()),
        watch: Some(TaskWatchOptions::Boolean(true)),
        condition: None,
        condition_script: None,
        ignore_errors: Some(true),
        force: Some(true),
        env_files: Some(vec![]),
        env: Some(IndexMap::new()),
        cwd: None,
        alias: None,
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        install_crate_args: None,
        install_script: None,
        args: None,
        script: Some(ScriptValue::Text(vec!["1".to_string(), "2".to_string()])),
        script_runner: Some("sh1".to_string()),
        script_runner_args: None,
        script_extension: Some("ext1".to_string()),
        run_task: Some(RunTaskInfo::Name("task1".to_string())),
        dependencies: None,
        toolchain: None,
        linux: None,
        windows: None,
        mac: None,
    };

    let mut env = IndexMap::new();
    env.insert("test".to_string(), EnvValue::Value("value".to_string()));
    let extended = Task {
        clear: Some(false),
        install_crate: Some(InstallCrate::Value("my crate2".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("test2".to_string()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(true),
        plugin: Some("plugin".to_string()),
        disabled: Some(true),
        private: Some(false),
        deprecated: Some(DeprecationInfo::Boolean(false)),
        extend: Some("extended".to_string()),
        watch: Some(TaskWatchOptions::Boolean(false)),
        condition: Some(TaskCondition {
            fail_message: None,
            profiles: Some(vec!["development".to_string()]),
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        ignore_errors: Some(false),
        force: Some(false),
        env_files: Some(vec![EnvFile::Path("extended".to_string())]),
        env: Some(env.clone()),
        cwd: Some("cwd".to_string()),
        alias: Some("alias2".to_string()),
        linux_alias: Some("linux".to_string()),
        windows_alias: Some("windows".to_string()),
        mac_alias: Some("mac".to_string()),
        install_script: Some(ScriptValue::Text(vec!["i1".to_string(), "i2".to_string()])),
        args: Some(vec!["a1".to_string(), "a2".to_string()]),
        script: Some(ScriptValue::Text(vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
        ])),
        script_runner: Some("sh2".to_string()),
        script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
        script_extension: Some("ext2".to_string()),
        run_task: Some(RunTaskInfo::Name("task2".to_string())),
        dependencies: Some(vec!["A".into()]),
        toolchain: Some("toolchain".into()),
        linux: Some(PlatformOverrideTask {
            clear: Some(true),
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            deprecated: Some(DeprecationInfo::Boolean(false)),
            extend: Some("extended".to_string()),
            plugin: Some("plugin".to_string()),
            watch: Some(TaskWatchOptions::Boolean(false)),
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: Some(vec!["development".to_string()]),
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            ignore_errors: Some(true),
            force: Some(true),
            env_files: Some(vec![EnvFile::Path("extended".to_string())]),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(ScriptValue::Text(vec!["i1".to_string(), "i2".to_string()])),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(ScriptValue::Text(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ])),
            script_runner: Some("sh3".to_string()),
            script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
            script_extension: Some("ext3".to_string()),
            run_task: Some(RunTaskInfo::Name("task3".to_string())),
            dependencies: Some(vec!["A".into()]),
            toolchain: Some("toolchain".into()),
        }),
        windows: Some(PlatformOverrideTask {
            clear: Some(false),
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            deprecated: Some(DeprecationInfo::Boolean(false)),
            extend: Some("extended".to_string()),
            plugin: Some("plugin".to_string()),
            watch: Some(TaskWatchOptions::Boolean(false)),
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: Some(vec!["development".to_string()]),
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            ignore_errors: Some(true),
            force: Some(true),
            env_files: Some(vec![EnvFile::Path("extended".to_string())]),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(ScriptValue::Text(vec!["i1".to_string(), "i2".to_string()])),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(ScriptValue::Text(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ])),
            script_runner: Some("sh3".to_string()),
            script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
            script_extension: Some("ext3".to_string()),
            run_task: Some(RunTaskInfo::Name("task3".to_string())),
            dependencies: Some(vec!["A".into()]),
            toolchain: Some("toolchain".into()),
        }),
        mac: Some(PlatformOverrideTask {
            clear: None,
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            deprecated: Some(DeprecationInfo::Boolean(false)),
            extend: Some("extended".to_string()),
            plugin: Some("plugin".to_string()),
            watch: Some(TaskWatchOptions::Boolean(false)),
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: Some(vec!["development".to_string()]),
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            ignore_errors: Some(true),
            force: Some(true),
            env_files: Some(vec![EnvFile::Path("extended".to_string())]),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(ScriptValue::Text(vec!["i1".to_string(), "i2".to_string()])),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(ScriptValue::Text(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ])),
            script_runner: Some("sh3".to_string()),
            script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
            script_extension: Some("ext3".to_string()),
            run_task: Some(RunTaskInfo::Name("task3".to_string())),
            dependencies: Some(vec!["A".into()]),
            toolchain: Some("toolchain".into()),
        }),
    };

    base.extend(&extended);

    assert!(!base.clear.unwrap());
    assert!(base.install_crate.is_some());
    assert!(base.install_crate_args.is_some());
    assert!(base.command.is_some());
    assert!(base.description.is_some());
    assert!(base.category.is_some());
    assert!(base.workspace.is_some());
    assert!(base.plugin.is_some());
    assert!(base.disabled.is_some());
    assert!(base.private.is_some());
    assert!(base.deprecated.is_some());
    assert!(base.extend.is_some());
    assert!(base.watch.is_some());
    assert!(base.condition.is_some());
    assert!(base.condition_script.is_some());
    assert!(base.ignore_errors.is_some());
    assert!(base.force.is_some());
    assert!(base.env_files.is_some());
    assert!(base.env.is_some());
    assert!(base.cwd.is_some());
    assert!(base.alias.is_some());
    assert!(base.linux_alias.is_some());
    assert!(base.windows_alias.is_some());
    assert!(base.mac_alias.is_some());
    assert!(base.install_script.is_some());
    assert!(base.args.is_some());
    assert!(base.script.is_some());
    assert!(base.script_runner.is_some());
    assert!(base.script_runner_args.is_some());
    assert!(base.script_extension.is_some());
    assert!(base.run_task.is_some());
    assert!(base.dependencies.is_some());
    assert!(base.toolchain.is_some());
    assert!(base.linux.is_some());
    assert!(base.windows.is_some());
    assert!(base.mac.is_some());

    assert_eq!(get_script_vec(&base).len(), 3);
    assert_eq!(
        base.install_crate.unwrap(),
        InstallCrate::Value("my crate2".to_string())
    );
    assert_eq!(base.install_crate_args.unwrap().len(), 2);
    assert_eq!(base.command.unwrap(), "test2");
    assert_eq!(base.description.unwrap(), "description");
    assert_eq!(base.category.unwrap(), "category");
    assert!(base.workspace.unwrap());
    assert_eq!(base.plugin.unwrap(), "plugin");
    assert!(base.disabled.unwrap());
    assert!(!base.private.unwrap());
    assert_eq!(base.deprecated.unwrap(), DeprecationInfo::Boolean(false));
    assert_eq!(base.extend.unwrap(), "extended");
    assert_eq!(base.watch.unwrap(), TaskWatchOptions::Boolean(false));
    assert_eq!(base.condition_script.unwrap().len(), 1);
    assert!(!base.ignore_errors.unwrap());
    assert!(!base.force.unwrap());
    assert_eq!(base.env_files.unwrap().len(), 1);
    assert_eq!(base.env.unwrap().len(), 1);
    assert_eq!(base.cwd.unwrap(), "cwd".to_string());
    assert_eq!(base.alias.unwrap(), "alias2");
    assert_eq!(base.linux_alias.unwrap(), "linux");
    assert_eq!(base.windows_alias.unwrap(), "windows");
    assert_eq!(base.mac_alias.unwrap(), "mac");
    assert_eq!(get_script_as_vec(base.install_script).len(), 2);
    assert_eq!(base.args.unwrap().len(), 2);
    assert_eq!(base.script_runner.unwrap(), "sh2");
    assert_eq!(base.script_runner_args.unwrap().len(), 2);
    assert_eq!(base.script_extension.unwrap(), "ext2");
    let run_task_name = match base.run_task.unwrap() {
        RunTaskInfo::Name(name) => name,
        _ => panic!("Invalid run task value."),
    };
    assert_eq!(run_task_name, "task2".to_string());
    assert_eq!(base.dependencies.unwrap().len(), 1);
    assert_eq!(base.toolchain.unwrap(), "toolchain".into());
    assert!(base.linux.unwrap().clear.unwrap());
    assert!(!base.windows.unwrap().clear.unwrap());
    assert!(base.mac.unwrap().clear.is_none());

    let condition = base.condition.unwrap();
    assert_eq!(condition.platforms.unwrap().len(), 2);
    assert_eq!(condition.channels.unwrap().len(), 2);
}

#[test]
fn task_extend_clear_with_no_data() {
    let env = IndexMap::new();
    let mut base = Task {
        clear: Some(false),
        install_crate: Some(InstallCrate::Value("my crate2".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("test2".to_string()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(false),
        plugin: Some("base".to_string()),
        disabled: Some(true),
        private: Some(false),
        deprecated: Some(DeprecationInfo::Boolean(true)),
        extend: Some("base".to_string()),
        watch: Some(TaskWatchOptions::Boolean(false)),
        condition: Some(TaskCondition {
            fail_message: None,
            profiles: Some(vec!["development".to_string()]),
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        ignore_errors: Some(false),
        force: Some(false),
        env_files: Some(vec![]),
        env: Some(env.clone()),
        cwd: Some("cwd".to_string()),
        alias: Some("alias2".to_string()),
        linux_alias: Some("linux".to_string()),
        windows_alias: Some("windows".to_string()),
        mac_alias: Some("mac".to_string()),
        install_script: Some(ScriptValue::Text(vec!["i1".to_string(), "i2".to_string()])),
        args: Some(vec!["a1".to_string(), "a2".to_string()]),
        script: Some(ScriptValue::Text(vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
        ])),
        script_runner: Some("sh2".to_string()),
        script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
        script_extension: Some("ext2".to_string()),
        run_task: Some(RunTaskInfo::Name("task2".to_string())),
        dependencies: Some(vec!["A".into()]),
        toolchain: Some("toolchain".into()),
        linux: Some(PlatformOverrideTask {
            clear: Some(true),
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            deprecated: Some(DeprecationInfo::Boolean(true)),
            extend: Some("base".to_string()),
            plugin: Some("base".to_string()),
            watch: Some(TaskWatchOptions::Boolean(false)),
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: Some(vec!["development".to_string()]),
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            ignore_errors: Some(true),
            force: Some(true),
            env_files: Some(vec![]),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(ScriptValue::Text(vec!["i1".to_string(), "i2".to_string()])),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(ScriptValue::Text(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ])),
            script_runner: Some("sh3".to_string()),
            script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
            script_extension: Some("ext3".to_string()),
            run_task: Some(RunTaskInfo::Name("task3".to_string())),
            dependencies: Some(vec!["A".into()]),
            toolchain: Some("toolchain".into()),
        }),
        windows: Some(PlatformOverrideTask {
            clear: Some(false),
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            deprecated: Some(DeprecationInfo::Boolean(true)),
            extend: Some("base".to_string()),
            plugin: Some("base".to_string()),
            watch: Some(TaskWatchOptions::Boolean(false)),
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: Some(vec!["development".to_string()]),
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            ignore_errors: Some(true),
            force: Some(true),
            env_files: Some(vec![]),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(ScriptValue::Text(vec!["i1".to_string(), "i2".to_string()])),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(ScriptValue::Text(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ])),
            script_runner: Some("sh3".to_string()),
            script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
            script_extension: Some("ext3".to_string()),
            run_task: Some(RunTaskInfo::Name("task3".to_string())),
            dependencies: Some(vec!["A".into()]),
            toolchain: Some("toolchain".into()),
        }),
        mac: Some(PlatformOverrideTask {
            clear: None,
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            deprecated: Some(DeprecationInfo::Boolean(true)),
            extend: Some("base".to_string()),
            plugin: Some("base".to_string()),
            watch: Some(TaskWatchOptions::Boolean(false)),
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: Some(vec!["development".to_string()]),
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            ignore_errors: Some(true),
            force: Some(true),
            env_files: Some(vec![]),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(ScriptValue::Text(vec!["i1".to_string(), "i2".to_string()])),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(ScriptValue::Text(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ])),
            script_runner: Some("sh3".to_string()),
            script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
            script_extension: Some("ext3".to_string()),
            run_task: Some(RunTaskInfo::Name("task3".to_string())),
            dependencies: Some(vec!["A".into()]),
            toolchain: Some("toolchain".into()),
        }),
    };

    let mut extended = Task::new();
    extended.clear = Some(true);

    base.extend(&extended);

    assert!(base.clear.unwrap());
    assert!(base.install_crate.is_none());
    assert!(base.command.is_none());
    assert!(base.description.is_none());
    assert!(base.category.is_none());
    assert!(base.workspace.is_none());
    assert!(base.plugin.is_none());
    assert!(base.disabled.is_none());
    assert!(base.private.is_none());
    assert!(base.deprecated.is_none());
    assert!(base.extend.is_none());
    assert!(base.watch.is_none());
    assert!(base.condition.is_none());
    assert!(base.condition_script.is_none());
    assert!(base.ignore_errors.is_none());
    assert!(base.force.is_none());
    assert!(base.env_files.is_none());
    assert!(base.env.is_none());
    assert!(base.cwd.is_none());
    assert!(base.alias.is_none());
    assert!(base.linux_alias.is_none());
    assert!(base.windows_alias.is_none());
    assert!(base.mac_alias.is_none());
    assert!(base.install_crate_args.is_none());
    assert!(base.install_script.is_none());
    assert!(base.script_runner.is_none());
    assert!(base.script_runner_args.is_none());
    assert!(base.script_extension.is_none());
    assert!(base.run_task.is_none());
    assert!(base.args.is_none());
    assert!(base.script.is_none());
    assert!(base.dependencies.is_none());
    assert!(base.toolchain.is_none());
    assert!(base.linux.is_none());
    assert!(base.windows.is_none());
    assert!(base.mac.is_none());
}

#[test]
fn task_extend_clear_with_all_data() {
    let mut base = Task::new();

    let env = IndexMap::new();
    let extended = Task {
        clear: Some(true),
        install_crate: Some(InstallCrate::Value("my crate2".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("test2".to_string()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(true),
        plugin: Some("plugin".to_string()),
        disabled: Some(true),
        private: Some(false),
        deprecated: Some(DeprecationInfo::Boolean(true)),
        extend: Some("base".to_string()),
        watch: Some(TaskWatchOptions::Boolean(false)),
        condition: Some(TaskCondition {
            fail_message: None,
            profiles: Some(vec!["development".to_string()]),
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        ignore_errors: Some(false),
        force: Some(false),
        env_files: Some(vec![]),
        env: Some(env.clone()),
        cwd: Some("cwd".to_string()),
        alias: Some("alias2".to_string()),
        linux_alias: Some("linux".to_string()),
        windows_alias: Some("windows".to_string()),
        mac_alias: Some("mac".to_string()),
        install_script: Some(ScriptValue::Text(vec!["i1".to_string(), "i2".to_string()])),
        args: Some(vec!["a1".to_string(), "a2".to_string()]),
        script: Some(ScriptValue::Text(vec![
            "1".to_string(),
            "2".to_string(),
            "3".to_string(),
        ])),
        script_runner: Some("sh2".to_string()),
        script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
        script_extension: Some("ext2".to_string()),
        run_task: Some(RunTaskInfo::Name("task2".to_string())),
        dependencies: Some(vec!["A".into()]),
        toolchain: Some("toolchain".into()),
        linux: Some(PlatformOverrideTask {
            clear: Some(true),
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            deprecated: Some(DeprecationInfo::Boolean(true)),
            extend: Some("base".to_string()),
            plugin: Some("plugin".to_string()),
            watch: Some(TaskWatchOptions::Boolean(false)),
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: Some(vec!["development".to_string()]),
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            ignore_errors: Some(true),
            force: Some(true),
            env_files: Some(vec![]),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(ScriptValue::Text(vec!["i1".to_string(), "i2".to_string()])),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(ScriptValue::Text(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ])),
            script_runner: Some("sh3".to_string()),
            script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
            script_extension: Some("ext3".to_string()),
            run_task: Some(RunTaskInfo::Name("task3".to_string())),
            dependencies: Some(vec!["A".into()]),
            toolchain: Some("toolchain".into()),
        }),
        windows: Some(PlatformOverrideTask {
            clear: Some(false),
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            deprecated: Some(DeprecationInfo::Boolean(true)),
            extend: Some("base".to_string()),
            plugin: Some("plugin".to_string()),
            watch: Some(TaskWatchOptions::Boolean(false)),
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: Some(vec!["development".to_string()]),
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            ignore_errors: Some(true),
            force: Some(true),
            env_files: Some(vec![]),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(ScriptValue::Text(vec!["i1".to_string(), "i2".to_string()])),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(ScriptValue::Text(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ])),
            script_runner: Some("sh3".to_string()),
            script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
            script_extension: Some("ext3".to_string()),
            run_task: Some(RunTaskInfo::Name("task3".to_string())),
            dependencies: Some(vec!["A".into()]),
            toolchain: Some("toolchain".into()),
        }),
        mac: Some(PlatformOverrideTask {
            clear: None,
            install_crate: Some(InstallCrate::Value("my crate2".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
            command: Some("test2".to_string()),
            disabled: Some(true),
            private: Some(false),
            deprecated: Some(DeprecationInfo::Boolean(true)),
            extend: Some("base".to_string()),
            plugin: Some("plugin".to_string()),
            watch: Some(TaskWatchOptions::Boolean(false)),
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: Some(vec!["development".to_string()]),
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            ignore_errors: Some(true),
            force: Some(true),
            env_files: Some(vec![]),
            env: Some(env.clone()),
            cwd: Some("cwd".to_string()),
            install_script: Some(ScriptValue::Text(vec!["i1".to_string(), "i2".to_string()])),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(ScriptValue::Text(vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
            ])),
            script_runner: Some("sh3".to_string()),
            script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
            script_extension: Some("ext3".to_string()),
            run_task: Some(RunTaskInfo::Name("task3".to_string())),
            dependencies: Some(vec!["A".into()]),
            toolchain: Some("toolchain".into()),
        }),
    };

    base.extend(&extended);

    assert!(base.clear.unwrap());
    assert!(base.install_crate.is_some());
    assert!(base.command.is_some());
    assert!(base.description.is_some());
    assert!(base.category.is_some());
    assert!(base.workspace.is_some());
    assert!(base.plugin.is_some());
    assert!(base.disabled.is_some());
    assert!(base.private.is_some());
    assert!(base.deprecated.is_some());
    assert!(base.extend.is_some());
    assert!(base.watch.is_some());
    assert!(base.condition.is_some());
    assert!(base.condition_script.is_some());
    assert!(base.ignore_errors.is_some());
    assert!(base.force.is_some());
    assert!(base.env_files.is_some());
    assert!(base.env.is_some());
    assert!(base.cwd.is_some());
    assert!(base.alias.is_some());
    assert!(base.linux_alias.is_some());
    assert!(base.windows_alias.is_some());
    assert!(base.mac_alias.is_some());
    assert!(base.install_crate_args.is_some());
    assert!(base.install_script.is_some());
    assert!(base.script_runner.is_some());
    assert!(base.script_runner_args.is_some());
    assert!(base.script_extension.is_some());
    assert!(base.run_task.is_some());
    assert!(base.args.is_some());
    assert!(base.script.is_some());
    assert!(base.dependencies.is_some());
    assert!(base.toolchain.is_some());
    assert!(base.linux.is_some());
    assert!(base.windows.is_some());
    assert!(base.mac.is_some());
}

#[test]
fn task_get_alias_all_none() {
    let task = Task::new();

    let alias = task.get_alias();
    assert!(alias.is_none());
}

#[test]
fn task_get_alias_common_defined() {
    let mut task = Task::new();
    task.alias = Some("other".to_string());

    let alias = task.get_alias();
    assert_eq!(alias.unwrap(), "other");
}

#[test]
fn task_get_alias_platform_defined() {
    let mut task = Task::new();
    task.alias = Some("other".to_string());
    task.linux_alias = Some("linux".to_string());
    task.windows_alias = Some("windows".to_string());
    task.mac_alias = Some("mac".to_string());

    let alias = task.get_alias();
    if cfg!(windows) {
        assert_eq!(alias.unwrap(), "windows");
    } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        assert_eq!(alias.unwrap(), "mac");
    } else {
        assert_eq!(alias.unwrap(), "linux");
    };
}

#[test]
fn task_get_normalized_task_undefined() {
    let mut task = Task {
        clear: Some(false),
        alias: Some("alias".to_string()),
        linux_alias: Some("linux".to_string()),
        windows_alias: Some("windows".to_string()),
        mac_alias: Some("mac".to_string()),
        install_crate: Some(InstallCrate::Value("install_crate".to_string())),
        install_crate_args: None,
        command: Some("command".to_string()),
        disabled: Some(false),
        private: Some(true),
        deprecated: None,
        extend: Some("base".to_string()),
        watch: Some(TaskWatchOptions::Boolean(true)),
        condition: None,
        condition_script: None,
        ignore_errors: None,
        force: None,
        env_files: None,
        env: None,
        cwd: None,
        install_script: Some(ScriptValue::Text(vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
        ])),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(ScriptValue::Text(vec!["a".to_string(), "b".to_string()])),
        script_runner: Some("sh1".to_string()),
        script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
        script_extension: Some("ext1".to_string()),
        run_task: Some(RunTaskInfo::Name("task1".to_string())),
        dependencies: Some(vec!["1".into()]),
        toolchain: Some("toolchain2".into()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(false),
        plugin: Some("bplugin".to_string()),
        linux: None,
        windows: None,
        mac: None,
    };

    let normalized_task = task.get_normalized_task();

    assert!(!normalized_task.clear.unwrap());
    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.install_crate_args.is_none());
    assert!(normalized_task.command.is_some());
    assert!(normalized_task.disabled.is_some());
    assert!(normalized_task.private.is_some());
    assert!(normalized_task.deprecated.is_none());
    assert!(normalized_task.extend.is_some());
    assert!(normalized_task.watch.is_some());
    assert!(normalized_task.condition.is_none());
    assert!(normalized_task.condition_script.is_none());
    assert!(normalized_task.ignore_errors.is_none());
    assert!(normalized_task.force.is_none());
    assert!(normalized_task.env_files.is_none());
    assert!(normalized_task.env.is_none());
    assert!(normalized_task.cwd.is_none());
    assert!(normalized_task.alias.is_some());
    assert!(normalized_task.linux_alias.is_some());
    assert!(normalized_task.windows_alias.is_some());
    assert!(normalized_task.mac_alias.is_some());
    assert!(normalized_task.install_script.is_some());
    assert!(normalized_task.args.is_some());
    assert!(normalized_task.script.is_some());
    assert!(normalized_task.script_runner.is_some());
    assert!(normalized_task.script_runner_args.is_some());
    assert!(normalized_task.script_extension.is_some());
    assert!(normalized_task.run_task.is_some());
    assert!(normalized_task.dependencies.is_some());
    assert!(normalized_task.toolchain.is_some());
    assert!(normalized_task.description.is_some());
    assert!(normalized_task.category.is_some());
    assert!(normalized_task.workspace.is_some());
    assert!(normalized_task.plugin.is_some());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(get_script_vec(&normalized_task).len(), 2);
    assert_eq!(
        normalized_task.install_crate.unwrap(),
        InstallCrate::Value("install_crate".to_string())
    );
    assert_eq!(normalized_task.command.unwrap(), "command");
    assert_eq!(normalized_task.description.unwrap(), "description");
    assert_eq!(normalized_task.category.unwrap(), "category");
    assert!(!normalized_task.workspace.unwrap());
    assert_eq!(normalized_task.plugin.unwrap(), "bplugin");
    assert!(!normalized_task.disabled.unwrap());
    assert!(normalized_task.private.unwrap());
    assert_eq!(normalized_task.extend.unwrap(), "base");
    assert_eq!(
        normalized_task.watch.unwrap(),
        TaskWatchOptions::Boolean(true)
    );
    assert!(!normalized_task.ignore_errors.unwrap_or(false));
    assert!(!normalized_task.force.unwrap_or(false));
    assert_eq!(normalized_task.alias.unwrap(), "alias");
    assert_eq!(normalized_task.linux_alias.unwrap(), "linux");
    assert_eq!(normalized_task.windows_alias.unwrap(), "windows");
    assert_eq!(normalized_task.mac_alias.unwrap(), "mac");
    assert_eq!(get_script_as_vec(normalized_task.install_script).len(), 3);
    assert_eq!(normalized_task.args.unwrap().len(), 2);
    assert_eq!(normalized_task.script_runner.unwrap(), "sh1");
    assert_eq!(normalized_task.script_runner_args.unwrap().len(), 2);
    assert_eq!(normalized_task.script_extension.unwrap(), "ext1");
    let run_task_name = match normalized_task.run_task.unwrap() {
        RunTaskInfo::Name(name) => name,
        _ => panic!("Invalid run task value."),
    };
    assert_eq!(run_task_name, "task1".to_string());
    assert_eq!(normalized_task.dependencies.unwrap().len(), 1);
    assert_eq!(normalized_task.toolchain.unwrap(), "toolchain2".into());
}

#[test]
#[cfg(target_os = "linux")]
fn task_get_normalized_task_with_override_no_clear() {
    let mut env = IndexMap::new();
    env.insert("test".to_string(), EnvValue::Value("value".to_string()));

    let mut task = Task {
        clear: Some(true),
        alias: Some("bad".to_string()),
        linux_alias: Some("bad".to_string()),
        windows_alias: Some("bad".to_string()),
        mac_alias: Some("bad".to_string()),
        install_crate: Some(InstallCrate::Value("install_crate".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("command".to_string()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(true),
        plugin: Some("plugin".to_string()),
        disabled: Some(false),
        private: Some(true),
        deprecated: None,
        extend: Some("base".to_string()),
        watch: Some(TaskWatchOptions::Boolean(true)),
        condition: Some(TaskCondition {
            fail_message: None,
            profiles: Some(vec!["development".to_string()]),
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        ignore_errors: Some(false),
        force: Some(false),
        env_files: Some(vec![]),
        env: Some(IndexMap::new()),
        cwd: Some("cwd".to_string()),
        install_script: Some(ScriptValue::Text(vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
        ])),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(ScriptValue::Text(vec!["a".to_string(), "b".to_string()])),
        script_runner: Some("sh1".to_string()),
        script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
        script_extension: Some("ext1".to_string()),
        run_task: Some(RunTaskInfo::Name("task1".to_string())),
        dependencies: Some(vec!["1".into()]),
        toolchain: Some("toolchain1".into()),
        linux: Some(PlatformOverrideTask {
            clear: None,
            install_crate: Some(InstallCrate::Value("linux_crate".to_string())),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string(), "c3".to_string()]),
            command: Some("linux_command".to_string()),
            disabled: Some(true),
            private: Some(false),
            deprecated: Some(DeprecationInfo::Boolean(true)),
            extend: Some("linux".to_string()),
            plugin: Some("linux_plugin".to_string()),
            watch: Some(TaskWatchOptions::Boolean(false)),
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: Some(vec!["development".to_string()]),
                platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
                channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
                env_set: None,
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
            }),
            condition_script: Some(vec!["exit 0".to_string()]),
            ignore_errors: Some(true),
            force: Some(true),
            env_files: Some(vec![EnvFile::Path("extended".to_string())]),
            env: Some(env),
            cwd: Some("cwd2".to_string()),
            install_script: Some(ScriptValue::Text(vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
            ])),
            args: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script: Some(ScriptValue::Text(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ])),
            script_runner: Some("sh2".to_string()),
            script_runner_args: Some(vec![
                "sr_a1".to_string(),
                "sr_a2".to_string(),
                "sr_a3".to_string(),
            ]),
            script_extension: Some("ext2".to_string()),
            run_task: Some(RunTaskInfo::Name("task2".to_string())),
            dependencies: Some(vec!["1".into(), "2".into()]),
            toolchain: Some("toolchain2".into()),
        }),
        windows: None,
        mac: None,
    };

    let normalized_task = task.get_normalized_task();

    assert!(normalized_task.clear.unwrap());
    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.install_crate_args.is_some());
    assert!(normalized_task.command.is_some());
    assert!(normalized_task.description.is_some());
    assert!(normalized_task.category.is_some());
    assert!(normalized_task.workspace.is_some());
    assert!(normalized_task.plugin.is_some());
    assert!(normalized_task.disabled.is_some());
    assert!(normalized_task.private.is_some());
    assert!(normalized_task.deprecated.is_some());
    assert!(normalized_task.extend.is_some());
    assert!(normalized_task.watch.is_some());
    assert!(normalized_task.condition.is_some());
    assert!(normalized_task.condition_script.is_some());
    assert!(normalized_task.ignore_errors.is_some());
    assert!(normalized_task.force.is_some());
    assert!(normalized_task.env_files.is_some());
    assert!(normalized_task.env.is_some());
    assert!(normalized_task.cwd.is_some());
    assert!(normalized_task.alias.is_none());
    assert!(normalized_task.linux_alias.is_none());
    assert!(normalized_task.windows_alias.is_none());
    assert!(normalized_task.mac_alias.is_none());
    assert!(normalized_task.install_script.is_some());
    assert!(normalized_task.args.is_some());
    assert!(normalized_task.script.is_some());
    assert!(normalized_task.script_runner.is_some());
    assert!(normalized_task.script_runner_args.is_some());
    assert!(normalized_task.script_extension.is_some());
    assert!(normalized_task.run_task.is_some());
    assert!(normalized_task.dependencies.is_some());
    assert!(normalized_task.toolchain.is_some());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(get_script_vec(&normalized_task).len(), 3);
    assert_eq!(
        normalized_task.install_crate.unwrap(),
        InstallCrate::Value("linux_crate".to_string())
    );
    assert_eq!(normalized_task.install_crate_args.unwrap().len(), 3);
    assert_eq!(normalized_task.command.unwrap(), "linux_command");
    assert_eq!(normalized_task.description.unwrap(), "description");
    assert_eq!(normalized_task.category.unwrap(), "category");
    assert!(normalized_task.workspace.unwrap());
    assert_eq!(normalized_task.plugin.unwrap(), "linux_plugin");
    assert!(normalized_task.disabled.unwrap());
    assert!(!normalized_task.private.unwrap());
    assert_eq!(
        normalized_task.deprecated.unwrap(),
        DeprecationInfo::Boolean(true)
    );
    assert_eq!(normalized_task.extend.unwrap(), "linux");
    assert_eq!(
        normalized_task.watch.unwrap(),
        TaskWatchOptions::Boolean(false)
    );
    assert_eq!(normalized_task.condition_script.unwrap().len(), 1);
    assert!(normalized_task.ignore_errors.unwrap());
    assert!(normalized_task.force.unwrap());
    assert_eq!(normalized_task.env_files.unwrap().len(), 1);
    assert_eq!(normalized_task.env.unwrap().len(), 1);
    assert_eq!(normalized_task.cwd.unwrap(), "cwd2".to_string());
    assert_eq!(get_script_as_vec(normalized_task.install_script).len(), 4);
    assert_eq!(normalized_task.args.unwrap().len(), 3);
    assert_eq!(normalized_task.script_runner.unwrap(), "sh2");
    assert_eq!(normalized_task.script_runner_args.unwrap().len(), 3);
    assert_eq!(normalized_task.script_extension.unwrap(), "ext2");
    let run_task_name = match normalized_task.run_task.unwrap() {
        RunTaskInfo::Name(name) => name,
        _ => panic!("Invalid run task value."),
    };
    assert_eq!(run_task_name, "task2".to_string());
    assert_eq!(normalized_task.dependencies.unwrap().len(), 2);
    assert_eq!(normalized_task.toolchain.unwrap(), "toolchain2".into());

    let condition = normalized_task.condition.unwrap();
    assert_eq!(condition.platforms.unwrap().len(), 2);
    assert_eq!(condition.channels.unwrap().len(), 2);
}

#[test]
#[cfg(target_os = "linux")]
fn task_get_normalized_task_with_override_clear_false() {
    let mut env = IndexMap::new();
    env.insert("test".to_string(), EnvValue::Value("value".to_string()));

    let mut task = Task {
        clear: Some(true),
        alias: Some("bad".to_string()),
        linux_alias: Some("bad".to_string()),
        windows_alias: Some("bad".to_string()),
        mac_alias: Some("bad".to_string()),
        install_crate: Some(InstallCrate::Value("install_crate".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("command".to_string()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(true),
        plugin: Some("plugin".to_string()),
        disabled: Some(false),
        private: Some(true),
        deprecated: Some(DeprecationInfo::Boolean(false)),
        extend: Some("base".to_string()),
        watch: Some(TaskWatchOptions::Boolean(true)),
        condition: Some(TaskCondition {
            fail_message: None,
            profiles: Some(vec!["development".to_string()]),
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        ignore_errors: Some(false),
        force: Some(false),
        env_files: Some(vec![]),
        env: Some(IndexMap::new()),
        cwd: Some("cwd".to_string()),
        install_script: Some(ScriptValue::Text(vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
        ])),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(ScriptValue::Text(vec!["a".to_string(), "b".to_string()])),
        script_runner: Some("sh1".to_string()),
        script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
        script_extension: Some("ext1".to_string()),
        run_task: Some(RunTaskInfo::Name("task1".to_string())),
        dependencies: Some(vec!["1".into()]),
        toolchain: Some("toolchain1".into()),
        linux: Some(PlatformOverrideTask {
            clear: Some(false),
            install_crate: Some(InstallCrate::Value("linux_crate".to_string())),
            command: Some("linux_command".to_string()),
            disabled: Some(true),
            private: Some(false),
            deprecated: Some(DeprecationInfo::Boolean(true)),
            extend: Some("linux".to_string()),
            plugin: Some("linux_plugin".to_string()),
            watch: Some(TaskWatchOptions::Boolean(false)),
            condition: Some(TaskCondition {
                fail_message: None,
                profiles: Some(vec!["development".to_string()]),
                platforms: Some(vec!["linux".to_string()]),
                channels: Some(vec![
                    "nightly".to_string(),
                    "stable".to_string(),
                    "beta".to_string(),
                ]),
                env_set: None,
                env_not_set: None,
                env_true: None,
                env_false: None,
                env: None,
                env_contains: None,
                rust_version: None,
                files_exist: None,
                files_not_exist: None,
            }),
            condition_script: Some(vec!["echo test".to_string(), "exit 1".to_string()]),
            ignore_errors: Some(true),
            force: Some(true),
            env_files: Some(vec![EnvFile::Path("extended".to_string())]),
            env: Some(env),
            cwd: Some("cwd2".to_string()),
            install_crate_args: Some(vec!["c1".to_string(), "c2".to_string(), "c3".to_string()]),
            install_script: Some(ScriptValue::Text(vec![
                "A".to_string(),
                "B".to_string(),
                "C".to_string(),
                "D".to_string(),
            ])),
            args: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script: Some(ScriptValue::Text(vec![
                "a".to_string(),
                "b".to_string(),
                "c".to_string(),
            ])),
            script_runner: Some("sh2".to_string()),
            script_runner_args: Some(vec![
                "sr_a1".to_string(),
                "sr_a2".to_string(),
                "sr_a3".to_string(),
            ]),
            script_extension: Some("ext2".to_string()),
            run_task: Some(RunTaskInfo::Name("task2".to_string())),
            dependencies: Some(vec!["1".into(), "2".into()]),
            toolchain: Some("toolchain2".into()),
        }),
        windows: None,
        mac: None,
    };

    let normalized_task = task.get_normalized_task();

    assert!(normalized_task.clear.unwrap());
    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.command.is_some());
    assert!(normalized_task.description.is_some());
    assert!(normalized_task.category.is_some());
    assert!(normalized_task.workspace.is_some());
    assert!(normalized_task.plugin.is_some());
    assert!(normalized_task.disabled.is_some());
    assert!(normalized_task.private.is_some());
    assert!(normalized_task.deprecated.is_some());
    assert!(normalized_task.extend.is_some());
    assert!(normalized_task.watch.is_some());
    assert!(normalized_task.condition.is_some());
    assert!(normalized_task.condition_script.is_some());
    assert!(normalized_task.ignore_errors.is_some());
    assert!(normalized_task.force.is_some());
    assert!(normalized_task.env_files.is_some());
    assert!(normalized_task.env.is_some());
    assert!(normalized_task.cwd.is_some());
    assert!(normalized_task.alias.is_none());
    assert!(normalized_task.linux_alias.is_none());
    assert!(normalized_task.windows_alias.is_none());
    assert!(normalized_task.mac_alias.is_none());
    assert!(normalized_task.install_crate_args.is_some());
    assert!(normalized_task.install_script.is_some());
    assert!(normalized_task.args.is_some());
    assert!(normalized_task.script.is_some());
    assert!(normalized_task.script_runner.is_some());
    assert!(normalized_task.script_runner_args.is_some());
    assert!(normalized_task.script_extension.is_some());
    assert!(normalized_task.run_task.is_some());
    assert!(normalized_task.dependencies.is_some());
    assert!(normalized_task.toolchain.is_some());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(get_script_vec(&normalized_task).len(), 3);
    assert_eq!(
        normalized_task.install_crate.unwrap(),
        InstallCrate::Value("linux_crate".to_string())
    );
    assert_eq!(normalized_task.command.unwrap(), "linux_command");
    assert_eq!(normalized_task.description.unwrap(), "description");
    assert_eq!(normalized_task.category.unwrap(), "category");
    assert!(normalized_task.workspace.unwrap());
    assert_eq!(normalized_task.plugin.unwrap(), "linux_plugin");
    assert!(normalized_task.disabled.unwrap());
    assert!(!normalized_task.private.unwrap());
    assert_eq!(
        normalized_task.deprecated.unwrap(),
        DeprecationInfo::Boolean(true)
    );
    assert_eq!(normalized_task.extend.unwrap(), "linux");
    assert_eq!(
        normalized_task.watch.unwrap(),
        TaskWatchOptions::Boolean(false)
    );
    assert_eq!(normalized_task.condition_script.unwrap().len(), 2);
    assert!(normalized_task.ignore_errors.unwrap());
    assert!(normalized_task.force.unwrap());
    assert_eq!(normalized_task.env_files.unwrap().len(), 1);
    assert_eq!(normalized_task.env.unwrap().len(), 1);
    assert_eq!(normalized_task.cwd.unwrap(), "cwd2".to_string());
    assert_eq!(normalized_task.install_crate_args.unwrap().len(), 3);
    assert_eq!(get_script_as_vec(normalized_task.install_script).len(), 4);
    assert_eq!(normalized_task.args.unwrap().len(), 3);
    assert_eq!(normalized_task.script_runner.unwrap(), "sh2");
    assert_eq!(normalized_task.script_runner_args.unwrap().len(), 3);
    assert_eq!(normalized_task.script_extension.unwrap(), "ext2");
    let run_task_name = match normalized_task.run_task.unwrap() {
        RunTaskInfo::Name(name) => name,
        _ => panic!("Invalid run task value."),
    };
    assert_eq!(run_task_name, "task2".to_string());
    assert_eq!(normalized_task.dependencies.unwrap().len(), 2);
    assert_eq!(normalized_task.toolchain.unwrap(), "toolchain2".into());

    let condition = normalized_task.condition.unwrap();
    assert_eq!(condition.platforms.unwrap().len(), 1);
    assert_eq!(condition.channels.unwrap().len(), 3);
}

#[test]
#[cfg(target_os = "linux")]
fn task_get_normalized_task_with_override_clear_false_partial_override() {
    let mut task = Task {
        clear: Some(true),
        alias: Some("bad".to_string()),
        linux_alias: Some("bad".to_string()),
        windows_alias: Some("bad".to_string()),
        mac_alias: Some("bad".to_string()),
        install_crate: Some(InstallCrate::Value("install_crate".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("command".to_string()),
        disabled: Some(false),
        private: Some(true),
        deprecated: Some(DeprecationInfo::Boolean(true)),
        extend: Some("base".to_string()),
        watch: Some(TaskWatchOptions::Boolean(true)),
        condition: Some(TaskCondition {
            fail_message: None,
            profiles: Some(vec!["development".to_string()]),
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        ignore_errors: Some(false),
        force: Some(false),
        env_files: Some(vec![]),
        env: Some(IndexMap::new()),
        cwd: Some("cwd".to_string()),
        install_script: Some(ScriptValue::Text(vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
        ])),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(ScriptValue::Text(vec!["a".to_string(), "b".to_string()])),
        script_runner: Some("sh1".to_string()),
        script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
        script_extension: Some("ext1".to_string()),
        run_task: Some(RunTaskInfo::Name("task1".to_string())),
        dependencies: Some(vec!["1".into()]),
        toolchain: Some("toolchain1".into()),
        description: None,
        category: None,
        workspace: None,
        plugin: None,
        linux: Some(PlatformOverrideTask {
            clear: Some(false),
            install_crate: None,
            install_crate_args: None,
            command: None,
            disabled: None,
            private: None,
            deprecated: None,
            extend: None,
            plugin: None,
            watch: None,
            condition: None,
            condition_script: None,
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
        }),
        windows: None,
        mac: None,
    };

    let normalized_task = task.get_normalized_task();

    assert!(normalized_task.clear.unwrap());
    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.install_crate_args.is_some());
    assert!(normalized_task.command.is_some());
    assert!(normalized_task.disabled.is_some());
    assert!(normalized_task.private.is_some());
    assert!(normalized_task.deprecated.is_some());
    assert!(normalized_task.extend.is_some());
    assert!(normalized_task.watch.is_some());
    assert!(normalized_task.condition.is_some());
    assert!(normalized_task.condition_script.is_some());
    assert!(normalized_task.ignore_errors.is_some());
    assert!(normalized_task.force.is_some());
    assert!(normalized_task.env_files.is_some());
    assert!(normalized_task.env.is_some());
    assert!(normalized_task.cwd.is_some());
    assert!(normalized_task.alias.is_none());
    assert!(normalized_task.linux_alias.is_none());
    assert!(normalized_task.windows_alias.is_none());
    assert!(normalized_task.mac_alias.is_none());
    assert!(normalized_task.install_script.is_some());
    assert!(normalized_task.args.is_some());
    assert!(normalized_task.script.is_some());
    assert!(normalized_task.script_runner.is_some());
    assert!(normalized_task.script_runner_args.is_some());
    assert!(normalized_task.script_extension.is_some());
    assert!(normalized_task.run_task.is_some());
    assert!(normalized_task.dependencies.is_some());
    assert!(normalized_task.toolchain.is_some());
    assert!(normalized_task.description.is_none());
    assert!(normalized_task.category.is_none());
    assert!(normalized_task.workspace.is_none());
    assert!(normalized_task.plugin.is_none());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(get_script_vec(&normalized_task).len(), 2);
    assert_eq!(
        normalized_task.install_crate.unwrap(),
        InstallCrate::Value("install_crate".to_string())
    );
    assert_eq!(normalized_task.command.unwrap(), "command");
    assert!(!normalized_task.disabled.unwrap());
    assert!(normalized_task.private.unwrap());
    assert_eq!(
        normalized_task.deprecated.unwrap(),
        DeprecationInfo::Boolean(true)
    );
    assert_eq!(normalized_task.extend.unwrap(), "base");
    assert_eq!(
        normalized_task.watch.unwrap(),
        TaskWatchOptions::Boolean(true)
    );
    assert!(!normalized_task.ignore_errors.unwrap());
    assert!(!normalized_task.force.unwrap());
    assert_eq!(normalized_task.env_files.unwrap().len(), 0);
    assert_eq!(normalized_task.env.unwrap().len(), 0);
    assert_eq!(normalized_task.cwd.unwrap(), "cwd".to_string());
    assert_eq!(normalized_task.install_crate_args.unwrap().len(), 2);
    assert_eq!(get_script_as_vec(normalized_task.install_script).len(), 3);
    assert_eq!(normalized_task.args.unwrap().len(), 2);
    assert_eq!(normalized_task.script_runner.unwrap(), "sh1");
    assert_eq!(normalized_task.script_runner_args.unwrap().len(), 2);
    assert_eq!(normalized_task.script_extension.unwrap(), "ext1");
    let run_task_name = match normalized_task.run_task.unwrap() {
        RunTaskInfo::Name(name) => name,
        _ => panic!("Invalid run task value."),
    };
    assert_eq!(run_task_name, "task1".to_string());
    assert_eq!(normalized_task.dependencies.unwrap().len(), 1);
    assert_eq!(normalized_task.toolchain.unwrap(), "toolchain1".into());
}

#[test]
#[cfg(target_os = "linux")]
fn task_get_normalized_task_with_override_clear_true() {
    let mut task = Task {
        clear: Some(true),
        alias: Some("bad".to_string()),
        linux_alias: Some("bad".to_string()),
        windows_alias: Some("bad".to_string()),
        mac_alias: Some("bad".to_string()),
        install_crate: Some(InstallCrate::Value("install_crate".to_string())),
        install_crate_args: Some(vec!["c1".to_string(), "c2".to_string()]),
        command: Some("command".to_string()),
        disabled: Some(false),
        private: Some(true),
        deprecated: Some(DeprecationInfo::Boolean(true)),
        extend: Some("base".to_string()),
        watch: Some(TaskWatchOptions::Boolean(true)),
        condition: Some(TaskCondition {
            fail_message: None,
            profiles: Some(vec!["development".to_string()]),
            platforms: Some(vec!["linux".to_string(), "mac".to_string()]),
            channels: Some(vec!["nightly".to_string(), "stable".to_string()]),
            env_set: None,
            env_not_set: None,
            env_true: None,
            env_false: None,
            env: None,
            env_contains: None,
            rust_version: None,
            files_exist: None,
            files_not_exist: None,
        }),
        condition_script: Some(vec!["exit 0".to_string()]),
        ignore_errors: Some(false),
        force: Some(false),
        env_files: Some(vec![]),
        env: Some(IndexMap::new()),
        cwd: Some("cwd".to_string()),
        install_script: Some(ScriptValue::Text(vec![
            "A".to_string(),
            "B".to_string(),
            "C".to_string(),
        ])),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(ScriptValue::Text(vec!["a".to_string(), "b".to_string()])),
        script_runner: Some("sh1".to_string()),
        script_runner_args: Some(vec!["sr_a1".to_string(), "sr_a2".to_string()]),
        script_extension: Some("ext1".to_string()),
        run_task: Some(RunTaskInfo::Name("task1".to_string())),
        dependencies: Some(vec!["1".into()]),
        toolchain: Some("toolchain1".into()),
        description: Some("description".to_string()),
        category: Some("category".to_string()),
        workspace: Some(false),
        plugin: Some("plugin".to_string()),
        linux: Some(PlatformOverrideTask {
            clear: Some(true),
            install_crate: Some(InstallCrate::Value("linux_crate".to_string())),
            install_crate_args: None,
            command: None,
            disabled: None,
            private: None,
            deprecated: None,
            extend: None,
            plugin: None,
            watch: None,
            condition: None,
            condition_script: None,
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
        }),
        windows: None,
        mac: None,
    };

    let normalized_task = task.get_normalized_task();

    assert!(normalized_task.clear.unwrap());
    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.install_crate_args.is_none());
    assert!(normalized_task.command.is_none());
    assert!(normalized_task.disabled.is_none());
    assert!(normalized_task.private.is_none());
    assert!(normalized_task.deprecated.is_none());
    assert!(normalized_task.extend.is_none());
    assert!(normalized_task.plugin.is_none());
    assert!(normalized_task.watch.is_none());
    assert!(normalized_task.condition.is_none());
    assert!(normalized_task.condition_script.is_none());
    assert!(normalized_task.ignore_errors.is_none());
    assert!(normalized_task.force.is_none());
    assert!(normalized_task.env_files.is_none());
    assert!(normalized_task.env.is_none());
    assert!(normalized_task.cwd.is_none());
    assert!(normalized_task.alias.is_none());
    assert!(normalized_task.linux_alias.is_none());
    assert!(normalized_task.windows_alias.is_none());
    assert!(normalized_task.mac_alias.is_none());
    assert!(normalized_task.install_script.is_none());
    assert!(normalized_task.args.is_none());
    assert!(normalized_task.script.is_none());
    assert!(normalized_task.script_runner.is_none());
    assert!(normalized_task.script_runner_args.is_none());
    assert!(normalized_task.script_extension.is_none());
    assert!(normalized_task.run_task.is_none());
    assert!(normalized_task.dependencies.is_none());
    assert!(normalized_task.toolchain.is_none());
    assert!(normalized_task.description.is_some());
    assert!(normalized_task.category.is_some());
    assert!(normalized_task.workspace.is_some());
    assert!(normalized_task.plugin.is_none());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(
        normalized_task.install_crate.unwrap(),
        InstallCrate::Value("linux_crate".to_string())
    );
    assert_eq!(normalized_task.description.unwrap(), "description");
    assert_eq!(normalized_task.category.unwrap(), "category");
}

#[test]
fn task_is_valid_all_none() {
    let task = Task::new();

    assert!(task.is_valid());
}

#[test]
fn task_is_valid_only_run_task() {
    let mut task = Task::new();
    task.run_task = Some(RunTaskInfo::Name("test".to_string()));

    assert!(task.is_valid());
}

#[test]
fn task_is_valid_only_command() {
    let mut task = Task::new();
    task.command = Some("test".to_string());

    assert!(task.is_valid());
}

#[test]
fn task_is_valid_only_script() {
    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["test".to_string()]));

    assert!(task.is_valid());
}

#[test]
fn task_is_valid_both_run_task_and_command() {
    let mut task = Task::new();
    task.run_task = Some(RunTaskInfo::Name("test".to_string()));
    task.command = Some("test".to_string());

    assert!(!task.is_valid());
}

#[test]
fn task_is_valid_both_run_task_and_script() {
    let mut task = Task::new();
    task.run_task = Some(RunTaskInfo::Name("test".to_string()));
    task.script = Some(ScriptValue::Text(vec!["test".to_string()]));

    assert!(!task.is_valid());
}

#[test]
fn task_is_valid_both_command_and_script() {
    let mut task = Task::new();
    task.command = Some("test".to_string());
    task.script = Some(ScriptValue::Text(vec!["test".to_string()]));

    assert!(!task.is_valid());
}

#[test]
fn task_is_actionable_all_none() {
    let task = Task::new();

    assert!(!task.is_actionable());
}

#[test]
fn task_is_actionable_disabled() {
    let mut task = Task::new();
    task.disabled = Some(true);
    task.command = Some("test".to_string());

    assert!(!task.is_actionable());
}

#[test]
fn task_is_actionable_with_command() {
    let mut task = Task::new();
    task.command = Some("test".to_string());

    assert!(task.is_actionable());
}

#[test]
fn task_is_actionable_with_script() {
    let mut task = Task::new();
    task.script = Some(ScriptValue::Text(vec!["test".to_string()]));

    assert!(task.is_actionable());
}

#[test]
fn task_is_actionable_with_run_task() {
    let mut task = Task::new();
    task.run_task = Some(RunTaskInfo::Name("test".to_string()));

    assert!(task.is_actionable());
}

#[test]
fn task_is_actionable_with_install_crate() {
    let mut task = Task::new();
    task.install_crate = Some(InstallCrate::Value("test".to_string()));

    assert!(task.is_actionable());
}

#[test]
fn task_is_actionable_with_install_script() {
    let mut task = Task::new();
    task.install_script = Some(ScriptValue::SingleLine("test".to_string()));

    assert!(task.is_actionable());
}

#[test]
fn task_is_actionable_with_empty_env() {
    let mut task = Task::new();
    task.env = Some(IndexMap::new());

    assert!(!task.is_actionable());
}

#[test]
fn task_is_actionable_with_env() {
    let mut env = IndexMap::new();
    env.insert("test".to_string(), EnvValue::Value("test".to_string()));

    let mut task = Task::new();
    task.env = Some(env);

    assert!(task.is_actionable());
}

#[test]
fn task_is_actionable_with_empty_env_files() {
    let mut task = Task::new();
    task.env_files = Some(vec![]);

    assert!(!task.is_actionable());
}

#[test]
fn task_is_actionable_with_env_files() {
    let mut task = Task::new();
    task.env_files = Some(vec![EnvFile::Path("test".to_string())]);

    assert!(task.is_actionable());
}

#[test]
fn task_is_actionable_with_empty_dependencies() {
    let mut task = Task::new();
    task.dependencies = Some(vec![]);

    assert!(!task.is_actionable());
}

#[test]
fn task_is_actionable_with_dependencies() {
    let mut task = Task::new();
    task.dependencies = Some(vec!["test".into()]);

    assert!(task.is_actionable());
}

#[test]
fn task_is_actionable_with_watch_false() {
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Boolean(false));

    assert!(!task.is_actionable());
}

#[test]
fn task_is_actionable_with_watch_true() {
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Boolean(true));

    assert!(task.is_actionable());
}

#[test]
fn task_is_actionable_with_watch_options() {
    let mut task = Task::new();
    task.watch = Some(TaskWatchOptions::Options(WatchOptions {
        version: None,
        postpone: None,
        ignore_pattern: None,
        no_git_ignore: None,
        watch: None,
    }));

    assert!(task.is_actionable());
}

#[test]
fn config_section_new() {
    let config = ConfigSection::new();

    assert!(config.skip_core_tasks.is_none());
    assert!(config.modify_core_tasks.is_none());
    assert!(config.init_task.is_none());
    assert!(config.end_task.is_none());
    assert!(config.on_error_task.is_none());
    assert!(config.legacy_migration_task.is_none());
    assert!(config.additional_profiles.is_none());
    assert!(config.min_version.is_none());
    assert!(config.default_to_workspace.is_none());
    assert!(config.skip_git_env_info.is_none());
    assert!(config.skip_rust_env_info.is_none());
    assert!(config.skip_crate_env_info.is_none());
    assert!(config.reduce_output.is_none());
    assert!(config.time_summary.is_none());
    assert!(config.load_cargo_aliases.is_none());
    assert!(config.main_project_member.is_none());
    assert!(config.load_script.is_none());
    assert!(config.linux_load_script.is_none());
    assert!(config.windows_load_script.is_none());
    assert!(config.mac_load_script.is_none());
}

#[test]
fn config_section_extend_all_values() {
    let mut base = ConfigSection::new();
    let mut extended = ConfigSection::new();

    base.skip_core_tasks = Some(true);
    base.modify_core_tasks = Some(ModifyConfig {
        private: Some(true),
        namespace: Some("base".to_string()),
    });
    base.init_task = Some("base_init".to_string());
    base.end_task = Some("base_end".to_string());
    base.on_error_task = Some("base_err".to_string());
    base.legacy_migration_task = Some("base_legacy".to_string());
    base.additional_profiles = Some(vec!["b1".to_string(), "b2".to_string()]);
    base.min_version = Some("1.0.0".to_string());
    base.default_to_workspace = Some(true);
    base.skip_git_env_info = Some(true);
    base.skip_rust_env_info = Some(true);
    base.skip_crate_env_info = Some(true);
    base.reduce_output = Some(true);
    base.time_summary = Some(true);
    base.load_cargo_aliases = Some(true);
    base.load_script = Some(ScriptValue::Text(vec!["base_info".to_string()]));
    base.linux_load_script = Some(ScriptValue::Text(vec![
        "linux".to_string(),
        "base_info".to_string(),
    ]));
    base.windows_load_script = Some(ScriptValue::Text(vec![
        "windows".to_string(),
        "base_info".to_string(),
    ]));
    base.mac_load_script = Some(ScriptValue::Text(vec![
        "mac".to_string(),
        "base_info".to_string(),
    ]));

    extended.skip_core_tasks = Some(false);
    extended.modify_core_tasks = Some(ModifyConfig {
        private: Some(false),
        namespace: Some("extended".to_string()),
    });
    extended.init_task = Some("extended_init".to_string());
    extended.end_task = Some("extended_end".to_string());
    extended.on_error_task = Some("extended_err".to_string());
    extended.legacy_migration_task = Some("extended_legacy".to_string());
    extended.additional_profiles = Some(vec!["e1".to_string(), "e2".to_string()]);
    extended.min_version = Some("2.0.0".to_string());
    extended.default_to_workspace = Some(false);
    extended.skip_core_tasks = Some(false);
    extended.skip_git_env_info = Some(false);
    extended.skip_rust_env_info = Some(false);
    extended.skip_crate_env_info = Some(false);
    extended.reduce_output = Some(false);
    extended.time_summary = Some(false);
    extended.load_cargo_aliases = Some(false);
    extended.load_script = Some(ScriptValue::Text(vec![
        "extended_info".to_string(),
        "arg2".to_string(),
    ]));
    extended.linux_load_script = Some(ScriptValue::Text(vec!["extended_info".to_string()]));
    extended.windows_load_script = Some(ScriptValue::Text(vec!["extended_info".to_string()]));
    extended.mac_load_script = Some(ScriptValue::Text(vec!["extended_info".to_string()]));

    base.extend(&mut extended);

    assert!(!base.skip_core_tasks.unwrap());
    let modify_core_tasks = base.modify_core_tasks.unwrap();
    assert!(!modify_core_tasks.private.unwrap());
    assert_eq!(modify_core_tasks.namespace.unwrap(), "extended".to_string());
    assert_eq!(base.init_task.unwrap(), "extended_init".to_string());
    assert_eq!(base.end_task.unwrap(), "extended_end".to_string());
    assert_eq!(base.on_error_task.unwrap(), "extended_err".to_string());
    assert_eq!(
        base.legacy_migration_task.unwrap(),
        "extended_legacy".to_string()
    );
    assert_eq!(
        base.additional_profiles.unwrap(),
        vec!["e1".to_string(), "e2".to_string()]
    );
    assert_eq!(base.min_version.unwrap(), "2.0.0".to_string());
    assert!(!base.default_to_workspace.unwrap());
    assert!(!base.skip_git_env_info.unwrap());
    assert!(!base.skip_rust_env_info.unwrap());
    assert!(!base.skip_crate_env_info.unwrap());
    assert!(!base.reduce_output.unwrap());
    assert!(!base.time_summary.unwrap());
    assert!(!base.load_cargo_aliases.unwrap());
    assert_eq!(get_script_as_vec(base.load_script).len(), 2);
    assert_eq!(get_script_as_vec(base.linux_load_script).len(), 1);
    assert_eq!(get_script_as_vec(base.windows_load_script).len(), 1);
    assert_eq!(get_script_as_vec(base.mac_load_script).len(), 1);
}

#[test]
fn config_section_extend_no_values() {
    let mut base = ConfigSection::new();
    let mut extended = ConfigSection::new();

    base.skip_core_tasks = Some(true);
    base.modify_core_tasks = Some(ModifyConfig {
        private: Some(true),
        namespace: Some("base".to_string()),
    });
    base.init_task = Some("base_init".to_string());
    base.end_task = Some("base_end".to_string());
    base.on_error_task = Some("base_err".to_string());
    base.legacy_migration_task = Some("base_legacy".to_string());
    base.additional_profiles = Some(vec!["b1".to_string(), "b2".to_string()]);
    base.min_version = Some("1.0.0".to_string());
    base.default_to_workspace = Some(true);
    base.skip_git_env_info = Some(true);
    base.skip_rust_env_info = Some(true);
    base.skip_crate_env_info = Some(true);
    base.reduce_output = Some(true);
    base.time_summary = Some(true);
    base.load_cargo_aliases = Some(true);
    base.load_script = Some(ScriptValue::Text(vec![
        "base_info".to_string(),
        "arg2".to_string(),
    ]));
    base.linux_load_script = Some(ScriptValue::Text(vec![
        "linux".to_string(),
        "base_info".to_string(),
    ]));
    base.windows_load_script = Some(ScriptValue::Text(vec![
        "windows".to_string(),
        "base_info".to_string(),
    ]));
    base.mac_load_script = Some(ScriptValue::Text(vec![
        "mac".to_string(),
        "base_info".to_string(),
    ]));

    base.extend(&mut extended);

    assert!(base.skip_core_tasks.unwrap());
    let modify_core_tasks = base.modify_core_tasks.unwrap();
    assert!(modify_core_tasks.private.unwrap());
    assert_eq!(modify_core_tasks.namespace.unwrap(), "base".to_string());
    assert_eq!(base.init_task.unwrap(), "base_init".to_string());
    assert_eq!(base.end_task.unwrap(), "base_end".to_string());
    assert_eq!(base.on_error_task.unwrap(), "base_err".to_string());
    assert_eq!(
        base.legacy_migration_task.unwrap(),
        "base_legacy".to_string()
    );
    assert_eq!(
        base.additional_profiles.unwrap(),
        vec!["b1".to_string(), "b2".to_string()]
    );
    assert_eq!(base.min_version.unwrap(), "1.0.0".to_string());
    assert!(base.default_to_workspace.unwrap());
    assert!(base.skip_git_env_info.unwrap());
    assert!(base.skip_rust_env_info.unwrap());
    assert!(base.skip_crate_env_info.unwrap());
    assert!(base.reduce_output.unwrap());
    assert!(base.time_summary.unwrap());
    assert!(base.load_cargo_aliases.unwrap());
    assert_eq!(get_script_as_vec(base.load_script).len(), 2);
    assert_eq!(get_script_as_vec(base.linux_load_script).len(), 2);
    assert_eq!(get_script_as_vec(base.windows_load_script).len(), 2);
    assert_eq!(get_script_as_vec(base.mac_load_script).len(), 2);
}

#[test]
fn config_section_extend_some_values() {
    let mut base = ConfigSection::new();
    let mut extended = ConfigSection::new();

    base.skip_core_tasks = Some(true);
    base.modify_core_tasks = Some(ModifyConfig {
        private: Some(true),
        namespace: Some("base".to_string()),
    });
    base.init_task = Some("base_init".to_string());
    base.end_task = Some("base_end".to_string());
    base.on_error_task = Some("base_err".to_string());
    base.legacy_migration_task = Some("base_legacy".to_string());
    base.additional_profiles = Some(vec!["b1".to_string(), "b2".to_string()]);
    base.min_version = Some("1.0.0".to_string());
    base.default_to_workspace = Some(true);
    base.skip_git_env_info = Some(true);
    base.skip_rust_env_info = Some(true);
    base.skip_crate_env_info = Some(true);
    base.reduce_output = Some(true);
    base.time_summary = Some(true);
    base.load_cargo_aliases = Some(true);
    base.load_script = Some(ScriptValue::Text(vec![
        "base_info".to_string(),
        "arg2".to_string(),
    ]));
    base.linux_load_script = Some(ScriptValue::Text(vec![
        "linux".to_string(),
        "base_info".to_string(),
    ]));
    base.windows_load_script = Some(ScriptValue::Text(vec![
        "windows".to_string(),
        "base_info".to_string(),
    ]));
    base.mac_load_script = Some(ScriptValue::Text(vec![
        "mac".to_string(),
        "base_info".to_string(),
    ]));

    extended.skip_core_tasks = Some(false);
    extended.init_task = Some("extended_init".to_string());

    base.extend(&mut extended);

    assert!(!base.skip_core_tasks.unwrap());
    let modify_core_tasks = base.modify_core_tasks.unwrap();
    assert!(modify_core_tasks.private.unwrap());
    assert_eq!(modify_core_tasks.namespace.unwrap(), "base".to_string());
    assert_eq!(base.init_task.unwrap(), "extended_init".to_string());
    assert_eq!(base.end_task.unwrap(), "base_end".to_string());
    assert_eq!(base.on_error_task.unwrap(), "base_err".to_string());
    assert_eq!(
        base.legacy_migration_task.unwrap(),
        "base_legacy".to_string()
    );
    assert_eq!(
        base.additional_profiles.unwrap(),
        vec!["b1".to_string(), "b2".to_string()]
    );
    assert_eq!(base.min_version.unwrap(), "1.0.0".to_string());
    assert!(base.default_to_workspace.unwrap());
    assert!(base.skip_git_env_info.unwrap());
    assert!(base.skip_rust_env_info.unwrap());
    assert!(base.skip_crate_env_info.unwrap());
    assert!(base.reduce_output.unwrap());
    assert!(base.time_summary.unwrap());
    assert!(base.load_cargo_aliases.unwrap());
    assert_eq!(get_script_as_vec(base.load_script).len(), 2);
    assert_eq!(get_script_as_vec(base.linux_load_script).len(), 2);
    assert_eq!(get_script_as_vec(base.windows_load_script).len(), 2);
    assert_eq!(get_script_as_vec(base.mac_load_script).len(), 2);
}

#[test]
fn config_section_get_get_load_script_all_none() {
    let config = ConfigSection::new();

    let load_script = config.get_load_script();
    assert!(load_script.is_none());
}

#[test]
fn config_section_get_get_load_script_platform_none() {
    let mut config = ConfigSection::new();
    config.load_script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));

    let load_script = config.get_load_script();
    assert!(load_script.is_some());
}

#[test]
fn config_section_get_get_load_script_platform_some() {
    let mut config = ConfigSection::new();
    config.linux_load_script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    config.windows_load_script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));
    config.mac_load_script = Some(ScriptValue::Text(vec!["exit 0".to_string()]));

    let load_script = config.get_load_script();
    assert!(load_script.is_some());
}

#[test]
fn config_section_get_get_load_script_all_defined() {
    let mut config = ConfigSection::new();
    config.load_script = Some(ScriptValue::Text(vec!["base".to_string(), "0".to_string()]));
    config.linux_load_script = Some(ScriptValue::Text(vec!["linux".to_string()]));
    config.windows_load_script = Some(ScriptValue::Text(vec!["windows".to_string()]));
    config.mac_load_script = Some(ScriptValue::Text(vec!["mac".to_string()]));

    let load_script = config.get_load_script();
    assert!(load_script.is_some());

    let script = get_script_as_vec(load_script);
    assert_eq!(script.len(), 1);
    assert_eq!(script[0], get_platform_name());
}

#[test]
fn workspace_new() {
    let workspace = Workspace::new();

    assert!(workspace.members.is_none());
}

#[test]
fn get_namespaced_task_name_empty() {
    let output = get_namespaced_task_name("", "my_task");

    assert_eq!(output, "my_task");
}

#[test]
fn get_namespaced_task_name_with_value() {
    let output = get_namespaced_task_name("prefix", "my_task");

    assert_eq!(output, "prefix::my_task");
}

#[test]
fn task_apply_task_empty_modify_empty() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: None,
    };
    let mut task = Task::new();
    task.apply(&modify_config);

    assert!(task.private.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.run_task.is_none());
    assert!(task.dependencies.is_none());
}

#[test]
fn task_apply_task_empty_modify_private() {
    let modify_config = ModifyConfig {
        private: Some(true),
        namespace: None,
    };
    let mut task = Task::new();
    task.apply(&modify_config);

    assert!(task.private.unwrap());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.run_task.is_none());
    assert!(task.dependencies.is_none());
}

#[test]
fn task_apply_task_empty_modify_not_private() {
    let modify_config = ModifyConfig {
        private: Some(false),
        namespace: None,
    };
    let mut task = Task::new();
    task.apply(&modify_config);

    assert!(task.private.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.run_task.is_none());
    assert!(task.dependencies.is_none());
}

#[test]
fn task_apply_modify_empty() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: None,
    };
    let mut task = Task::new();
    task.private = Some(true);
    task.apply(&modify_config);

    assert!(task.private.unwrap());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.run_task.is_none());
    assert!(task.dependencies.is_none());
}

#[test]
fn task_apply_modify_private() {
    let modify_config = ModifyConfig {
        private: Some(true),
        namespace: None,
    };
    let mut task = Task::new();
    task.private = Some(false);
    task.apply(&modify_config);

    assert!(task.private.unwrap());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.run_task.is_none());
    assert!(task.dependencies.is_none());
}

#[test]
fn task_apply_modify_not_private() {
    let modify_config = ModifyConfig {
        private: Some(false),
        namespace: None,
    };
    let mut task = Task::new();
    task.private = Some(true);
    task.apply(&modify_config);

    assert!(task.private.unwrap());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.run_task.is_none());
    assert!(task.dependencies.is_none());
}

#[test]
fn task_apply_task_empty_modify_namespace() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: Some("default".to_string()),
    };
    let mut task = Task::new();
    task.apply(&modify_config);

    assert!(task.private.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    assert!(task.run_task.is_none());
    assert!(task.dependencies.is_none());
}

#[test]
fn task_apply_no_run_task_modify_namespace() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: Some("default".to_string()),
    };

    let mut task = Task::new();
    task.alias = Some("alias".to_string());
    task.linux_alias = Some("linux_alias".to_string());
    task.windows_alias = Some("windows_alias".to_string());
    task.mac_alias = Some("mac_alias".to_string());
    task.dependencies = Some(vec!["dep1".into(), "dep2".into()]);

    task.apply(&modify_config);

    assert!(task.private.is_none());
    assert_eq!(task.alias.unwrap(), "default::alias");
    assert_eq!(task.linux_alias.unwrap(), "default::linux_alias");
    assert_eq!(task.windows_alias.unwrap(), "default::windows_alias");
    assert_eq!(task.mac_alias.unwrap(), "default::mac_alias");
    assert!(task.run_task.is_none());

    let expected: Vec<DependencyIdentifier> = vec!["default::dep1".into(), "default::dep2".into()];
    assert_eq!(task.dependencies.unwrap(), expected);
}

#[test]
fn task_apply_run_task_name_modify_namespace() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: Some("default".to_string()),
    };

    let mut task = Task::new();
    task.run_task = Some(RunTaskInfo::Name("run_task1".to_string()));

    task.apply(&modify_config);

    assert!(task.private.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    let run_task_name = match task.run_task.unwrap() {
        RunTaskInfo::Name(name) => name,
        _ => panic!("Invalid run task value."),
    };
    assert_eq!(run_task_name, "default::run_task1");
    assert!(task.dependencies.is_none());
}

#[test]
fn task_apply_run_task_details_single_modify_namespace() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: Some("default".to_string()),
    };

    let mut task = Task::new();
    task.run_task = Some(RunTaskInfo::Details(RunTaskDetails {
        name: RunTaskName::Single("run_task1".to_string()),
        fork: None,
        parallel: None,
        cleanup_task: None,
    }));

    task.apply(&modify_config);

    assert!(task.private.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    let details = match task.run_task.unwrap() {
        RunTaskInfo::Details(ref mut details) => details.clone(),
        _ => panic!("Invalid run task value."),
    };
    assert_eq!(
        details.name,
        RunTaskName::Single("default::run_task1".to_string())
    );
    assert!(task.dependencies.is_none());
}

#[test]
fn task_apply_run_task_details_multiple_modify_namespace() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: Some("default".to_string()),
    };

    let mut task = Task::new();
    task.run_task = Some(RunTaskInfo::Details(RunTaskDetails {
        name: RunTaskName::Multiple(vec!["run_task1".to_string(), "run_task2".to_string()]),
        fork: None,
        parallel: None,
        cleanup_task: None,
    }));

    task.apply(&modify_config);

    assert!(task.private.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    let details = match task.run_task.unwrap() {
        RunTaskInfo::Details(ref mut details) => details.clone(),
        _ => panic!("Invalid run task value."),
    };
    assert_eq!(
        details.name,
        RunTaskName::Multiple(vec![
            "default::run_task1".to_string(),
            "default::run_task2".to_string()
        ])
    );
    assert!(task.dependencies.is_none());
}

#[test]
fn task_apply_run_task_routing_info_single_modify_namespace() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: Some("default".to_string()),
    };

    let mut task = Task::new();
    task.run_task = Some(RunTaskInfo::Routing(vec![RunTaskRoutingInfo {
        name: RunTaskName::Single("run_task1".to_string()),
        fork: None,
        parallel: None,
        cleanup_task: None,
        condition: None,
        condition_script: None,
    }]));

    task.apply(&modify_config);

    assert!(task.private.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    let routing_info = match task.run_task.unwrap() {
        RunTaskInfo::Routing(ref mut info) => info.pop(),
        _ => panic!("Invalid run task value."),
    };
    assert_eq!(
        routing_info.unwrap().name,
        RunTaskName::Single("default::run_task1".to_string())
    );
    assert!(task.dependencies.is_none());
}

#[test]
fn task_apply_run_task_routing_info_multiple_modify_namespace() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: Some("default".to_string()),
    };

    let mut task = Task::new();
    task.run_task = Some(RunTaskInfo::Routing(vec![RunTaskRoutingInfo {
        name: RunTaskName::Multiple(vec!["run_task1".to_string(), "run_task2".to_string()]),
        fork: None,
        parallel: None,
        cleanup_task: None,
        condition: None,
        condition_script: None,
    }]));

    task.apply(&modify_config);

    assert!(task.private.is_none());
    assert!(task.alias.is_none());
    assert!(task.linux_alias.is_none());
    assert!(task.windows_alias.is_none());
    assert!(task.mac_alias.is_none());
    let routing_info = match task.run_task.unwrap() {
        RunTaskInfo::Routing(ref mut info) => info.pop(),
        _ => panic!("Invalid run task value."),
    };
    assert_eq!(
        routing_info.unwrap().name,
        RunTaskName::Multiple(vec![
            "default::run_task1".to_string(),
            "default::run_task2".to_string()
        ])
    );
    assert!(task.dependencies.is_none());
}

#[test]
fn config_section_apply_config_empty_modify_empty() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: None,
    };
    let mut config_section = ConfigSection::new();
    config_section.apply(&modify_config);

    assert!(config_section.init_task.is_none());
    assert!(config_section.end_task.is_none());
    assert!(config_section.on_error_task.is_none());
    assert!(config_section.legacy_migration_task.is_none());
}

#[test]
fn config_section_apply_config_empty_modify_namespace() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: Some("default".to_string()),
    };
    let mut config_section = ConfigSection::new();
    config_section.apply(&modify_config);

    assert!(config_section.init_task.is_none());
    assert!(config_section.end_task.is_none());
    assert!(config_section.on_error_task.is_none());
    assert!(config_section.legacy_migration_task.is_none());
}

#[test]
fn config_section_apply_config_with_values_modify_empty() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: None,
    };
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    config_section.on_error_task = Some("error".to_string());
    config_section.legacy_migration_task = Some("legacy".to_string());
    config_section.apply(&modify_config);

    assert_eq!(config_section.init_task.unwrap(), "init");
    assert_eq!(config_section.end_task.unwrap(), "end");
    assert_eq!(config_section.on_error_task.unwrap(), "error");
    assert_eq!(config_section.legacy_migration_task.unwrap(), "legacy");
}

#[test]
fn config_section_apply_config_with_values_modify_namespace() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: Some("config_ns".to_string()),
    };
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    config_section.end_task = Some("end".to_string());
    config_section.on_error_task = Some("error".to_string());
    config_section.legacy_migration_task = Some("legacy".to_string());
    config_section.apply(&modify_config);

    assert_eq!(config_section.init_task.unwrap(), "config_ns::init");
    assert_eq!(config_section.end_task.unwrap(), "config_ns::end");
    assert_eq!(config_section.on_error_task.unwrap(), "config_ns::error");
    assert_eq!(
        config_section.legacy_migration_task.unwrap(),
        "config_ns::legacy"
    );
}

#[test]
fn config_apply_modify_empty() {
    let modify_config = ModifyConfig {
        private: None,
        namespace: None,
    };
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    let mut tasks = IndexMap::new();
    let mut task = Task::new();
    task.private = Some(false);
    tasks.insert("test".to_string(), task);
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };
    config.apply(&modify_config);

    assert_eq!(config.config.init_task.unwrap(), "init");
    assert_eq!(config.env.len(), 0);
    assert_eq!(config.tasks.len(), 1);
    assert!(!config.tasks.get("test").unwrap().private.unwrap());
}

#[test]
fn config_apply_modify_all() {
    let modify_config = ModifyConfig {
        private: Some(true),
        namespace: Some("all".to_string()),
    };
    let mut config_section = ConfigSection::new();
    config_section.init_task = Some("init".to_string());
    let mut tasks = IndexMap::new();
    let mut task = Task::new();
    task.private = Some(false);
    tasks.insert("test".to_string(), task);
    let mut config = Config {
        config: config_section,
        env_files: vec![],
        env: IndexMap::new(),
        env_scripts: vec![],
        tasks,
        plugins: None,
    };
    config.apply(&modify_config);

    assert_eq!(config.config.init_task.unwrap(), "all::init");
    assert_eq!(config.env.len(), 0);
    assert_eq!(config.tasks.len(), 1);
    assert!(config.tasks.get("all::test").unwrap().private.unwrap());
}

#[test]
fn deprecation_info_partial_eq_same_bool_true() {
    let value1 = DeprecationInfo::Boolean(true);
    let value2 = DeprecationInfo::Boolean(true);

    assert_eq!(value1, value2);
}

#[test]
fn deprecation_info_partial_eq_same_bool_false() {
    let value1 = DeprecationInfo::Boolean(false);
    let value2 = DeprecationInfo::Boolean(false);

    assert_eq!(value1, value2);
}

#[test]
fn deprecation_info_partial_eq_same_message() {
    let value1 = DeprecationInfo::Message("test".to_string());
    let value2 = DeprecationInfo::Message("test".to_string());

    assert_eq!(value1, value2);
}

#[test]
fn deprecation_info_partial_eq_diff_bool() {
    let value1 = DeprecationInfo::Boolean(true);
    let value2 = DeprecationInfo::Boolean(false);

    assert!(value1 != value2);
}

#[test]
fn deprecation_info_partial_eq_diff_message() {
    let value1 = DeprecationInfo::Message("test1".to_string());
    let value2 = DeprecationInfo::Message("test2".to_string());

    assert!(value1 != value2);
}

#[test]
fn deprecation_info_partial_eq_diff_type() {
    let value1 = DeprecationInfo::Boolean(true);
    let value2 = DeprecationInfo::Message("test2".to_string());

    assert!(value1 != value2);
}
