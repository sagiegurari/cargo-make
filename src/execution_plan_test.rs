use super::*;
use crate::types::{ConfigSection, CrateInfo, Task, Workspace};
use indexmap::IndexMap;
use std::env;

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

fn update_member_path_get_expected() -> String {
    if cfg!(windows) {
        ".\\member\\".to_string()
    } else {
        "./member/".to_string()
    }
}

#[test]
fn update_member_path_unix() {
    let output = update_member_path("./member/");
    assert_eq!(output, update_member_path_get_expected());
}

#[test]
fn update_member_path_windows() {
    let output = update_member_path(".\\member\\");
    assert_eq!(output, update_member_path_get_expected());
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
cargo make --disable-check-for-updates --no-on-error --loglevel=LEVEL_NAME some_task
cd -
cd ./member2
cargo make --disable-check-for-updates --no-on-error --loglevel=LEVEL_NAME some_task
cd -
cd ./dir1/member3
cargo make --disable-check-for-updates --no-on-error --loglevel=LEVEL_NAME some_task
cd -"#
        .to_string();

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
fn is_workspace_flow_true_default() {
    let crate_info = CrateInfo::new();

    let task = Task::new();

    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", false, &crate_info);

    assert!(!workspace_flow);
}

#[test]
fn is_workspace_flow_true_in_task() {
    let crate_info = CrateInfo::new();

    let mut task = Task::new();
    task.workspace = Some(true);

    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", false, &crate_info);

    assert!(!workspace_flow);
}

#[test]
fn is_workspace_flow_no_workspace() {
    let crate_info = CrateInfo::new();

    let mut task = Task::new();
    task.workspace = Some(true);

    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", false, &crate_info);

    assert!(!workspace_flow);
}

#[test]
fn is_workspace_flow_disabled_via_cli() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
    });

    let mut task = Task::new();
    task.workspace = Some(true);

    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", true, &crate_info);

    assert!(!workspace_flow);
}

#[test]
fn is_workspace_flow_disabled_via_task() {
    let mut crate_info = CrateInfo::new();
    let members = vec![];
    crate_info.workspace = Some(Workspace {
        members: Some(members),
        exclude: None,
    });

    let mut task = Task::new();
    task.workspace = Some(false);

    let mut config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    config.tasks.insert("test".to_string(), task);

    let workspace_flow = is_workspace_flow(&config, "test", false, &crate_info);

    assert!(!workspace_flow);
}
