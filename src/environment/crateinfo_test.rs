use super::*;
use log;
use std::collections::HashMap;
use types::Workspace;

#[test]
fn crate_info_load() {
    let logger = log::create("error");
    let crate_info = load(&logger);

    assert!(crate_info.package.is_some());
    assert!(crate_info.workspace.is_none());

    let package = crate_info.package.unwrap();
    assert_eq!(package.name.unwrap(), "cargo-make");
}

#[test]
fn add_members_workspace_none_members_empty() {
    let mut crate_info = CrateInfo::new();
    add_members(&mut crate_info, vec![]);

    assert!(crate_info.workspace.is_none());
}

#[test]
fn add_members_workspace_none_members_with_data() {
    let mut crate_info = CrateInfo::new();
    add_members(&mut crate_info, vec!["test1".to_string(), "test2".to_string()]);

    assert!(crate_info.workspace.is_none());
}

#[test]
fn add_members_workspace_new_members_with_data() {
    let mut crate_info = CrateInfo::new();
    crate_info.workspace = Some(Workspace::new());
    add_members(&mut crate_info, vec!["test1".to_string(), "test2".to_string()]);

    assert!(crate_info.workspace.is_some());
    let workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    assert_eq!(workspace.members.unwrap().len(), 2);
}

#[test]
fn add_members_workspace_empty_members_with_data() {
    let mut crate_info = CrateInfo::new();
    let mut workspace = Workspace::new();
    workspace.members = Some(vec![]);
    crate_info.workspace = Some(workspace);
    add_members(&mut crate_info, vec!["test1".to_string(), "test2".to_string()]);

    assert!(crate_info.workspace.is_some());
    let workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    assert_eq!(workspace.members.unwrap().len(), 2);
}

#[test]
fn add_members_workspace_with_data_members_with_data_no_duplicates() {
    let mut crate_info = CrateInfo::new();
    let mut workspace = Workspace::new();
    workspace.members = Some(vec!["member1".to_string(), "member2".to_string()]);
    crate_info.workspace = Some(workspace);
    add_members(&mut crate_info, vec!["test1".to_string(), "test2".to_string()]);

    assert!(crate_info.workspace.is_some());
    let workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    assert_eq!(workspace.members.unwrap().len(), 4);
}

#[test]
fn add_members_workspace_with_data_members_with_data_with_duplicates() {
    let mut crate_info = CrateInfo::new();
    let mut workspace = Workspace::new();
    workspace.members = Some(vec!["member1".to_string(), "member2".to_string(), "test1".to_string()]);
    crate_info.workspace = Some(workspace);
    add_members(&mut crate_info, vec!["test1".to_string(), "test2".to_string()]);

    assert!(crate_info.workspace.is_some());
    let workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    assert_eq!(workspace.members.unwrap().len(), 4);
}

#[test]
fn get_members_from_dependencies_none() {
    let crate_info = CrateInfo::new();
    let members = get_members_from_dependencies(&crate_info);

    assert_eq!(members.len(), 0);
}

#[test]
fn get_members_from_dependencies_empty() {
    let mut crate_info = CrateInfo::new();
    crate_info.dependencies = Some(HashMap::new());
    let members = get_members_from_dependencies(&crate_info);

    assert_eq!(members.len(), 0);
}

#[test]
fn get_members_from_dependencies_only_versions() {
    let mut dependencies = HashMap::new();
    dependencies.insert("test1".to_string(), CrateDependency::Version("1".to_string()));
    dependencies.insert("test2".to_string(), CrateDependency::Version("2".to_string()));

    let mut crate_info = CrateInfo::new();
    crate_info.dependencies = Some(dependencies);
    let members = get_members_from_dependencies(&crate_info);

    assert_eq!(members.len(), 0);
}

#[test]
fn get_members_from_dependencies_no_paths() {
    let mut dependencies = HashMap::new();
    dependencies.insert("test1".to_string(), CrateDependency::Version("1".to_string()));
    dependencies.insert("test2".to_string(), CrateDependency::Version("2".to_string()));
    dependencies.insert("test3".to_string(), CrateDependency::Info(HashMap::new()));

    let mut crate_info = CrateInfo::new();
    crate_info.dependencies = Some(dependencies);
    let members = get_members_from_dependencies(&crate_info);

    assert_eq!(members.len(), 0);
}

#[test]
fn get_members_from_dependencies_no_workspace_paths() {
    let mut info = HashMap::new();
    info.insert("path".to_string(), "somepath".to_string());

    let mut dependencies = HashMap::new();
    dependencies.insert("test1".to_string(), CrateDependency::Version("1".to_string()));
    dependencies.insert("test2".to_string(), CrateDependency::Version("2".to_string()));
    dependencies.insert("test3".to_string(), CrateDependency::Info(HashMap::new()));

    let mut crate_info = CrateInfo::new();
    crate_info.dependencies = Some(dependencies);
    let members = get_members_from_dependencies(&crate_info);

    assert_eq!(members.len(), 0);
}

#[test]
fn get_members_from_dependencies_workspace_paths() {
    let mut dependencies = HashMap::new();
    dependencies.insert("test1".to_string(), CrateDependency::Version("1".to_string()));
    dependencies.insert("test2".to_string(), CrateDependency::Version("2".to_string()));

    let mut invalid = HashMap::new();
    invalid.insert("path".to_string(), "somepath".to_string());
    dependencies.insert("test3".to_string(), CrateDependency::Info(invalid));

    let mut valid1 = HashMap::new();
    valid1.insert("path".to_string(), "./member1".to_string());
    dependencies.insert("valid1".to_string(), CrateDependency::Info(valid1));

    let mut valid2 = HashMap::new();
    valid2.insert("path".to_string(), "./member2".to_string());
    dependencies.insert("valid2".to_string(), CrateDependency::Info(valid2));

    let mut crate_info = CrateInfo::new();
    crate_info.dependencies = Some(dependencies);
    let members = get_members_from_dependencies(&crate_info);

    assert_eq!(members.len(), 2);
    let value1 = &members[0];
    assert!((value1 == "member1") || value1 == "member2");
    let value2 = &members[1];
    assert!((value2 == "member1") || value2 == "member2");
    assert!(value1 != value2);
}

#[test]
fn remove_excludes_no_workspace() {
    let mut crate_info = CrateInfo::new();
    remove_excludes(&mut crate_info);

    assert!(crate_info.workspace.is_none());
}

#[test]
fn remove_excludes_workspace_no_members_with_excludes() {
    let mut crate_info = CrateInfo::new();
    let mut workspace = Workspace::new();
    workspace.exclude = Some(vec!["test".to_string()]);
    crate_info.workspace = Some(workspace);

    remove_excludes(&mut crate_info);

    assert!(crate_info.workspace.is_some());
    workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_none());
}

#[test]
fn remove_excludes_workspace_empty_members_with_excludes() {
    let mut crate_info = CrateInfo::new();
    let mut workspace = Workspace::new();
    workspace.exclude = Some(vec!["test".to_string()]);
    workspace.members = Some(vec![]);
    crate_info.workspace = Some(workspace);

    remove_excludes(&mut crate_info);

    assert!(crate_info.workspace.is_some());
    workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    assert_eq!(workspace.members.unwrap().len(), 0);
}

#[test]
fn remove_excludes_workspace_with_members_no_excludes() {
    let mut crate_info = CrateInfo::new();
    let mut workspace = Workspace::new();
    workspace.members = Some(vec!["test".to_string()]);
    crate_info.workspace = Some(workspace);

    remove_excludes(&mut crate_info);

    assert!(crate_info.workspace.is_some());
    workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    assert_eq!(workspace.members.unwrap().len(), 1);
}

#[test]
fn remove_excludes_workspace_with_members_empty_excludes() {
    let mut crate_info = CrateInfo::new();
    let mut workspace = Workspace::new();
    workspace.exclude = Some(vec![]);
    workspace.members = Some(vec!["test".to_string()]);
    crate_info.workspace = Some(workspace);

    remove_excludes(&mut crate_info);

    assert!(crate_info.workspace.is_some());
    workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    assert_eq!(workspace.members.unwrap().len(), 1);
}

#[test]
fn remove_excludes_workspace_with_members_with_excludes() {
    let mut crate_info = CrateInfo::new();
    let mut workspace = Workspace::new();
    workspace.exclude = Some(vec!["test0".to_string(), "test2".to_string(), "test3".to_string(), "test6".to_string()]);
    workspace.members = Some(vec!["test1".to_string(), "test2".to_string(), "test3".to_string(), "test4".to_string()]);
    crate_info.workspace = Some(workspace);

    remove_excludes(&mut crate_info);

    assert!(crate_info.workspace.is_some());
    workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    let members = workspace.members.unwrap();
    assert_eq!(members.len(), 2);
    assert_eq!(&members[0], "test1");
    assert_eq!(&members[1], "test4");
}
