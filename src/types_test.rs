use super::*;

#[test]
fn extend_both_have_misc_data() {
    let mut base = Task {
        install_crate: Some("my crate1".to_string()),
        command: Some("test1".to_string()),
        disabled: Some(false),
        alias: None,
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        install_script: None,
        args: None,
        script: Some(vec!["1".to_string(), "2".to_string()]),
        dependencies: None,
        linux: None,
        windows: None,
        mac: None
    };
    let extended = Task {
        install_crate: Some("my crate2".to_string()),
        command: None,
        disabled: Some(true),
        alias: Some("alias2".to_string()),
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        install_script: None,
        args: None,
        script: None,
        dependencies: None,
        linux: None,
        windows: None,
        mac: None
    };

    base.extend(&extended);

    assert!(base.install_crate.is_some());
    assert!(base.command.is_some());
    assert!(base.disabled.is_some());
    assert!(base.alias.is_some());
    assert!(base.linux_alias.is_none());
    assert!(base.windows_alias.is_none());
    assert!(base.mac_alias.is_none());
    assert!(base.install_script.is_none());
    assert!(base.args.is_none());
    assert!(base.script.is_some());
    assert!(base.dependencies.is_none());
    assert!(base.linux.is_none());
    assert!(base.windows.is_none());
    assert!(base.mac.is_none());

    assert_eq!(base.install_crate.unwrap(), "my crate2");
    assert_eq!(base.command.unwrap(), "test1");
    assert!(base.disabled.unwrap());
    assert_eq!(base.alias.unwrap(), "alias2");
    assert_eq!(base.script.unwrap().len(), 2);
}

#[test]
fn extend_extended_have_all_fields() {
    let mut base = Task {
        install_crate: Some("my crate1".to_string()),
        command: Some("test1".to_string()),
        disabled: Some(false),
        alias: None,
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        install_script: None,
        args: None,
        script: Some(vec!["1".to_string(), "2".to_string()]),
        dependencies: None,
        linux: None,
        windows: None,
        mac: None
    };
    let extended = Task {
        install_crate: Some("my crate2".to_string()),
        command: Some("test2".to_string()),
        disabled: Some(true),
        alias: Some("alias2".to_string()),
        linux_alias: Some("linux".to_string()),
        windows_alias: Some("windows".to_string()),
        mac_alias: Some("mac".to_string()),
        install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
        args: Some(vec!["a1".to_string(), "a2".to_string()]),
        script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
        dependencies: Some(vec!["A".to_string()]),
        linux: Some(PlatformOverrideTask {
            clear: Some(true),
            install_crate: Some("my crate2".to_string()),
            command: Some("test2".to_string()),
            disabled: Some(true),
            install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            dependencies: Some(vec!["A".to_string()])
        }),
        windows: Some(PlatformOverrideTask {
            clear: Some(false),
            install_crate: Some("my crate2".to_string()),
            command: Some("test2".to_string()),
            disabled: Some(true),
            install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            dependencies: Some(vec!["A".to_string()])
        }),
        mac: Some(PlatformOverrideTask {
            clear: None,
            install_crate: Some("my crate2".to_string()),
            command: Some("test2".to_string()),
            disabled: Some(true),
            install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            dependencies: Some(vec!["A".to_string()])
        })
    };

    base.extend(&extended);

    assert!(base.install_crate.is_some());
    assert!(base.command.is_some());
    assert!(base.disabled.is_some());
    assert!(base.alias.is_some());
    assert!(base.linux_alias.is_some());
    assert!(base.windows_alias.is_some());
    assert!(base.mac_alias.is_some());
    assert!(base.install_script.is_some());
    assert!(base.args.is_some());
    assert!(base.script.is_some());
    assert!(base.dependencies.is_some());
    assert!(base.linux.is_some());
    assert!(base.windows.is_some());
    assert!(base.mac.is_some());

    assert_eq!(base.install_crate.unwrap(), "my crate2");
    assert_eq!(base.command.unwrap(), "test2");
    assert!(base.disabled.unwrap());
    assert_eq!(base.alias.unwrap(), "alias2");
    assert_eq!(base.linux_alias.unwrap(), "linux");
    assert_eq!(base.windows_alias.unwrap(), "windows");
    assert_eq!(base.mac_alias.unwrap(), "mac");
    assert_eq!(base.install_script.unwrap().len(), 2);
    assert_eq!(base.args.unwrap().len(), 2);
    assert_eq!(base.script.unwrap().len(), 3);
    assert_eq!(base.dependencies.unwrap().len(), 1);
    assert!(base.linux.unwrap().clear.unwrap());
    assert!(!base.windows.unwrap().clear.unwrap());
    assert!(base.mac.unwrap().clear.is_none());
}

#[test]
fn get_alias_all_none() {
    let task = Task {
        alias: None,
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        install_crate: None,
        command: None,
        disabled: None,
        install_script: None,
        args: None,
        script: None,
        dependencies: None,
        linux: None,
        windows: None,
        mac: None
    };

    let alias = task.get_alias();
    assert!(alias.is_none());
}

#[test]
fn get_alias_common_defined() {
    let task = Task {
        alias: Some("other".to_string()),
        linux_alias: None,
        windows_alias: None,
        mac_alias: None,
        install_crate: None,
        command: None,
        disabled: None,
        install_script: None,
        args: None,
        script: None,
        dependencies: None,
        linux: None,
        windows: None,
        mac: None
    };

    let alias = task.get_alias();
    assert_eq!(alias.unwrap(), "other");
}

#[test]
fn get_alias_platform_defined() {
    let task = Task {
        alias: Some("other".to_string()),
        linux_alias: Some("linux".to_string()),
        windows_alias: Some("windows".to_string()),
        mac_alias: Some("mac".to_string()),
        install_crate: None,
        command: None,
        disabled: None,
        install_script: None,
        args: None,
        script: None,
        dependencies: None,
        linux: None,
        windows: None,
        mac: None
    };

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
fn get_normalized_task_undefined() {
    let mut task = Task {
        alias: Some("alias".to_string()),
        linux_alias: Some("linux".to_string()),
        windows_alias: Some("windows".to_string()),
        mac_alias: Some("mac".to_string()),
        install_crate: Some("install_crate".to_string()),
        command: Some("command".to_string()),
        disabled: Some(false),
        install_script: Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(vec!["a".to_string(), "b".to_string()]),
        dependencies: Some(vec!["1".to_string()]),
        linux: None,
        windows: None,
        mac: None
    };

    let normalized_task = task.get_normalized_task();

    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.command.is_some());
    assert!(normalized_task.disabled.is_some());
    assert!(normalized_task.alias.is_some());
    assert!(normalized_task.linux_alias.is_some());
    assert!(normalized_task.windows_alias.is_some());
    assert!(normalized_task.mac_alias.is_some());
    assert!(normalized_task.install_script.is_some());
    assert!(normalized_task.args.is_some());
    assert!(normalized_task.script.is_some());
    assert!(normalized_task.dependencies.is_some());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(normalized_task.install_crate.unwrap(), "install_crate");
    assert_eq!(normalized_task.command.unwrap(), "command");
    assert!(!normalized_task.disabled.unwrap());
    assert_eq!(normalized_task.alias.unwrap(), "alias");
    assert_eq!(normalized_task.linux_alias.unwrap(), "linux");
    assert_eq!(normalized_task.windows_alias.unwrap(), "windows");
    assert_eq!(normalized_task.mac_alias.unwrap(), "mac");
    assert_eq!(normalized_task.install_script.unwrap().len(), 3);
    assert_eq!(normalized_task.args.unwrap().len(), 2);
    assert_eq!(normalized_task.script.unwrap().len(), 2);
    assert_eq!(normalized_task.dependencies.unwrap().len(), 1);
}

#[test]
#[cfg(target_os = "linux")]
fn get_normalized_task_with_override_no_clear() {
    let mut task = Task {
        alias: Some("bad".to_string()),
        linux_alias: Some("bad".to_string()),
        windows_alias: Some("bad".to_string()),
        mac_alias: Some("bad".to_string()),
        install_crate: Some("install_crate".to_string()),
        command: Some("command".to_string()),
        disabled: Some(false),
        install_script: Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(vec!["a".to_string(), "b".to_string()]),
        dependencies: Some(vec!["1".to_string()]),
        linux: Some(PlatformOverrideTask {
            clear: None,
            install_crate: Some("linux_crate".to_string()),
            command: Some("linux_command".to_string()),
            disabled: Some(true),
            install_script: Some(vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()]),
            args: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script: Some(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
            dependencies: Some(vec!["1".to_string(), "2".to_string()])
        }),
        windows: None,
        mac: None
    };

    let normalized_task = task.get_normalized_task();

    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.command.is_some());
    assert!(normalized_task.disabled.is_some());
    assert!(normalized_task.alias.is_none());
    assert!(normalized_task.linux_alias.is_none());
    assert!(normalized_task.windows_alias.is_none());
    assert!(normalized_task.mac_alias.is_none());
    assert!(normalized_task.install_script.is_some());
    assert!(normalized_task.args.is_some());
    assert!(normalized_task.script.is_some());
    assert!(normalized_task.dependencies.is_some());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(normalized_task.install_crate.unwrap(), "linux_crate");
    assert_eq!(normalized_task.command.unwrap(), "linux_command");
    assert!(normalized_task.disabled.unwrap());
    assert_eq!(normalized_task.install_script.unwrap().len(), 4);
    assert_eq!(normalized_task.args.unwrap().len(), 3);
    assert_eq!(normalized_task.script.unwrap().len(), 3);
    assert_eq!(normalized_task.dependencies.unwrap().len(), 2);
}

#[test]
#[cfg(target_os = "linux")]
fn get_normalized_task_with_override_clear_false() {
    let mut task = Task {
        alias: Some("bad".to_string()),
        linux_alias: Some("bad".to_string()),
        windows_alias: Some("bad".to_string()),
        mac_alias: Some("bad".to_string()),
        install_crate: Some("install_crate".to_string()),
        command: Some("command".to_string()),
        disabled: Some(false),
        install_script: Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(vec!["a".to_string(), "b".to_string()]),
        dependencies: Some(vec!["1".to_string()]),
        linux: Some(PlatformOverrideTask {
            clear: Some(false),
            install_crate: Some("linux_crate".to_string()),
            command: Some("linux_command".to_string()),
            disabled: Some(true),
            install_script: Some(vec!["A".to_string(), "B".to_string(), "C".to_string(), "D".to_string()]),
            args: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            script: Some(vec!["a".to_string(), "b".to_string(), "c".to_string()]),
            dependencies: Some(vec!["1".to_string(), "2".to_string()])
        }),
        windows: None,
        mac: None
    };

    let normalized_task = task.get_normalized_task();

    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.command.is_some());
    assert!(normalized_task.disabled.is_some());
    assert!(normalized_task.alias.is_none());
    assert!(normalized_task.linux_alias.is_none());
    assert!(normalized_task.windows_alias.is_none());
    assert!(normalized_task.mac_alias.is_none());
    assert!(normalized_task.install_script.is_some());
    assert!(normalized_task.args.is_some());
    assert!(normalized_task.script.is_some());
    assert!(normalized_task.dependencies.is_some());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(normalized_task.install_crate.unwrap(), "linux_crate");
    assert_eq!(normalized_task.command.unwrap(), "linux_command");
    assert!(normalized_task.disabled.unwrap());
    assert_eq!(normalized_task.install_script.unwrap().len(), 4);
    assert_eq!(normalized_task.args.unwrap().len(), 3);
    assert_eq!(normalized_task.script.unwrap().len(), 3);
    assert_eq!(normalized_task.dependencies.unwrap().len(), 2);
}

#[test]
#[cfg(target_os = "linux")]
fn get_normalized_task_with_override_clear_false_partial_override() {
    let mut task = Task {
        alias: Some("bad".to_string()),
        linux_alias: Some("bad".to_string()),
        windows_alias: Some("bad".to_string()),
        mac_alias: Some("bad".to_string()),
        install_crate: Some("install_crate".to_string()),
        command: Some("command".to_string()),
        disabled: Some(false),
        install_script: Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(vec!["a".to_string(), "b".to_string()]),
        dependencies: Some(vec!["1".to_string()]),
        linux: Some(PlatformOverrideTask {
            clear: Some(false),
            install_crate: None,
            command: None,
            disabled: None,
            install_script: None,
            args: None,
            script: None,
            dependencies: None
        }),
        windows: None,
        mac: None
    };

    let normalized_task = task.get_normalized_task();

    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.command.is_some());
    assert!(normalized_task.disabled.is_some());
    assert!(normalized_task.alias.is_none());
    assert!(normalized_task.linux_alias.is_none());
    assert!(normalized_task.windows_alias.is_none());
    assert!(normalized_task.mac_alias.is_none());
    assert!(normalized_task.install_script.is_some());
    assert!(normalized_task.args.is_some());
    assert!(normalized_task.script.is_some());
    assert!(normalized_task.dependencies.is_some());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(normalized_task.install_crate.unwrap(), "install_crate");
    assert_eq!(normalized_task.command.unwrap(), "command");
    assert!(!normalized_task.disabled.unwrap());
    assert_eq!(normalized_task.install_script.unwrap().len(), 3);
    assert_eq!(normalized_task.args.unwrap().len(), 2);
    assert_eq!(normalized_task.script.unwrap().len(), 2);
    assert_eq!(normalized_task.dependencies.unwrap().len(), 1);
}

#[test]
#[cfg(target_os = "linux")]
fn get_normalized_task_with_override_clear_true() {
    let mut task = Task {
        alias: Some("bad".to_string()),
        linux_alias: Some("bad".to_string()),
        windows_alias: Some("bad".to_string()),
        mac_alias: Some("bad".to_string()),
        install_crate: Some("install_crate".to_string()),
        command: Some("command".to_string()),
        disabled: Some(false),
        install_script: Some(vec!["A".to_string(), "B".to_string(), "C".to_string()]),
        args: Some(vec!["1".to_string(), "2".to_string()]),
        script: Some(vec!["a".to_string(), "b".to_string()]),
        dependencies: Some(vec!["1".to_string()]),
        linux: Some(PlatformOverrideTask {
            clear: Some(true),
            install_crate: Some("linux_crate".to_string()),
            command: None,
            disabled: None,
            install_script: None,
            args: None,
            script: None,
            dependencies: None
        }),
        windows: None,
        mac: None
    };

    let normalized_task = task.get_normalized_task();

    assert!(normalized_task.install_crate.is_some());
    assert!(normalized_task.command.is_none());
    assert!(normalized_task.disabled.is_none());
    assert!(normalized_task.alias.is_none());
    assert!(normalized_task.linux_alias.is_none());
    assert!(normalized_task.windows_alias.is_none());
    assert!(normalized_task.mac_alias.is_none());
    assert!(normalized_task.install_script.is_none());
    assert!(normalized_task.args.is_none());
    assert!(normalized_task.script.is_none());
    assert!(normalized_task.dependencies.is_none());
    assert!(normalized_task.linux.is_none());
    assert!(normalized_task.windows.is_none());
    assert!(normalized_task.mac.is_none());

    assert_eq!(normalized_task.install_crate.unwrap(), "linux_crate");
}
