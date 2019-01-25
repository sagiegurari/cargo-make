use super::*;
use crate::types::{CrateDependencyInfo, Workspace};
use indexmap::IndexMap;

#[test]
fn crate_info_load() {
    let crate_info = load();

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
    add_members(
        &mut crate_info,
        vec!["test1".to_string(), "test2".to_string()],
    );

    assert!(crate_info.workspace.is_none());
}

#[test]
fn add_members_workspace_new_members_with_data() {
    let mut crate_info = CrateInfo::new();
    crate_info.workspace = Some(Workspace::new());
    add_members(
        &mut crate_info,
        vec!["test1".to_string(), "test2".to_string()],
    );

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
    add_members(
        &mut crate_info,
        vec!["test1".to_string(), "test2".to_string()],
    );

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
    add_members(
        &mut crate_info,
        vec!["test1".to_string(), "test2".to_string()],
    );

    assert!(crate_info.workspace.is_some());
    let workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    assert_eq!(workspace.members.unwrap().len(), 4);
}

#[test]
fn add_members_workspace_with_data_members_with_data_with_duplicates() {
    let mut crate_info = CrateInfo::new();
    let mut workspace = Workspace::new();
    workspace.members = Some(vec![
        "member1".to_string(),
        "member2".to_string(),
        "test1".to_string(),
    ]);
    crate_info.workspace = Some(workspace);
    add_members(
        &mut crate_info,
        vec!["test1".to_string(), "test2".to_string()],
    );

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
    crate_info.dependencies = Some(IndexMap::new());
    let members = get_members_from_dependencies(&crate_info);

    assert_eq!(members.len(), 0);
}

#[test]
fn get_members_from_dependencies_only_versions() {
    let mut dependencies = IndexMap::new();
    dependencies.insert(
        "test1".to_string(),
        CrateDependency::Version("1".to_string()),
    );
    dependencies.insert(
        "test2".to_string(),
        CrateDependency::Version("2".to_string()),
    );

    let mut crate_info = CrateInfo::new();
    crate_info.dependencies = Some(dependencies);
    let members = get_members_from_dependencies(&crate_info);

    assert_eq!(members.len(), 0);
}

#[test]
fn get_members_from_dependencies_no_paths() {
    let mut dependencies = IndexMap::new();
    dependencies.insert(
        "test1".to_string(),
        CrateDependency::Version("1".to_string()),
    );
    dependencies.insert(
        "test2".to_string(),
        CrateDependency::Version("2".to_string()),
    );
    dependencies.insert(
        "test3".to_string(),
        CrateDependency::Info(CrateDependencyInfo { path: None }),
    );

    let mut crate_info = CrateInfo::new();
    crate_info.dependencies = Some(dependencies);
    let members = get_members_from_dependencies(&crate_info);

    assert_eq!(members.len(), 0);
}

#[test]
fn get_members_from_dependencies_no_workspace_paths() {
    let mut info = IndexMap::new();
    info.insert("path".to_string(), "somepath".to_string());

    let mut dependencies = IndexMap::new();
    dependencies.insert(
        "test1".to_string(),
        CrateDependency::Version("1".to_string()),
    );
    dependencies.insert(
        "test2".to_string(),
        CrateDependency::Version("2".to_string()),
    );
    dependencies.insert(
        "test3".to_string(),
        CrateDependency::Info(CrateDependencyInfo { path: None }),
    );

    let mut crate_info = CrateInfo::new();
    crate_info.dependencies = Some(dependencies);
    let members = get_members_from_dependencies(&crate_info);

    assert_eq!(members.len(), 0);
}

#[test]
fn get_members_from_dependencies_workspace_paths() {
    let mut dependencies = IndexMap::new();
    dependencies.insert(
        "test1".to_string(),
        CrateDependency::Version("1".to_string()),
    );
    dependencies.insert(
        "test2".to_string(),
        CrateDependency::Version("2".to_string()),
    );
    dependencies.insert(
        "test3".to_string(),
        CrateDependency::Info(CrateDependencyInfo {
            path: Some("somepath".to_string()),
        }),
    );
    dependencies.insert(
        "valid1".to_string(),
        CrateDependency::Info(CrateDependencyInfo {
            path: Some("./member1".to_string()),
        }),
    );
    dependencies.insert(
        "valid2".to_string(),
        CrateDependency::Info(CrateDependencyInfo {
            path: Some("./member2".to_string()),
        }),
    );

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
    workspace.exclude = Some(vec![
        "test0".to_string(),
        "test2".to_string(),
        "test3".to_string(),
        "test6".to_string(),
    ]);
    workspace.members = Some(vec![
        "test1".to_string(),
        "test2".to_string(),
        "test3".to_string(),
        "test4".to_string(),
    ]);
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

#[test]
fn expand_glob_members_empty() {
    let members = expand_glob_members("examples/*/*.bad");

    assert_eq!(members.len(), 0);
}

#[test]
fn expand_glob_members_found() {
    let mut members = expand_glob_members("examples/*.toml");

    assert!(members.len() > 0);
    assert!(members
        .iter()
        .position(|member| member == "examples/env.toml")
        .is_some());

    members = expand_glob_members("examples/*/*.toml");

    assert!(members.len() > 0);
    assert!(members
        .iter()
        .position(|member| member == "examples/workspace2/Makefile.toml")
        .is_some());

    members = expand_glob_members("examples/workspace/member*");

    assert!(members.len() > 0);
    assert!(members
        .iter()
        .position(|member| member == "examples/workspace/member1")
        .is_some());
    assert!(members
        .iter()
        .position(|member| member == "examples/workspace/member2")
        .is_some());
}

#[test]
fn normalize_members_no_workspace() {
    let mut crate_info = CrateInfo::new();
    normalize_members(&mut crate_info);

    assert!(crate_info.workspace.is_none());
}

#[test]
fn normalize_members_no_members() {
    let mut crate_info = CrateInfo::new();
    crate_info.workspace = Some(Workspace::new());
    normalize_members(&mut crate_info);

    assert!(crate_info.workspace.is_some());
    let workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_none());
}

#[test]
fn normalize_members_empty_members() {
    let mut crate_info = CrateInfo::new();
    let mut workspace = Workspace::new();
    workspace.members = Some(vec![]);
    crate_info.workspace = Some(workspace);
    normalize_members(&mut crate_info);

    assert!(crate_info.workspace.is_some());
    workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    let members = workspace.members.unwrap();
    assert_eq!(members.len(), 0);
}

#[test]
fn normalize_members_no_glob() {
    let mut crate_info = CrateInfo::new();
    let mut workspace = Workspace::new();
    workspace.members = Some(vec!["member1".to_string(), "member2".to_string()]);
    crate_info.workspace = Some(workspace);
    normalize_members(&mut crate_info);

    assert!(crate_info.workspace.is_some());
    workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    let members = workspace.members.unwrap();
    assert_eq!(members.len(), 2);
    assert_eq!(members, vec!["member1".to_string(), "member2".to_string()]);
}

#[test]
fn normalize_members_mixed() {
    let mut crate_info = CrateInfo::new();
    let mut workspace = Workspace::new();
    workspace.members = Some(vec![
        "member1".to_string(),
        "member2".to_string(),
        "examples/workspace/mem*".to_string(),
        "member3".to_string(),
        "member4".to_string(),
    ]);
    crate_info.workspace = Some(workspace);
    normalize_members(&mut crate_info);

    assert!(crate_info.workspace.is_some());
    workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    let members = workspace.members.unwrap();
    assert!(members
        .iter()
        .position(|member| member == "member1")
        .is_some());
    assert!(members
        .iter()
        .position(|member| member == "member2")
        .is_some());
    assert!(members
        .iter()
        .position(|member| member == "member3")
        .is_some());
    assert!(members
        .iter()
        .position(|member| member == "member4")
        .is_some());
    assert!(members
        .iter()
        .position(|member| member == "examples/workspace/member1")
        .is_some());
    assert!(members
        .iter()
        .position(|member| member == "examples/workspace/member2")
        .is_some());
}

#[test]
fn load_workspace_members_no_workspace() {
    let mut crate_info = CrateInfo::new();
    load_workspace_members(&mut crate_info);

    assert!(crate_info.workspace.is_none());
}

#[test]
fn load_workspace_members_mixed() {
    let mut crate_info = CrateInfo::new();

    let mut dependencies = IndexMap::new();
    dependencies.insert(
        "test1".to_string(),
        CrateDependency::Version("1".to_string()),
    );
    dependencies.insert(
        "test2".to_string(),
        CrateDependency::Version("2".to_string()),
    );
    dependencies.insert(
        "test3".to_string(),
        CrateDependency::Info(CrateDependencyInfo {
            path: Some("somepath".to_string()),
        }),
    );
    dependencies.insert(
        "valid1".to_string(),
        CrateDependency::Info(CrateDependencyInfo {
            path: Some("./path1".to_string()),
        }),
    );
    dependencies.insert(
        "valid2".to_string(),
        CrateDependency::Info(CrateDependencyInfo {
            path: Some("./path2".to_string()),
        }),
    );
    dependencies.insert(
        "valid3".to_string(),
        CrateDependency::Info(CrateDependencyInfo {
            path: Some("./member1".to_string()),
        }),
    );
    crate_info.dependencies = Some(dependencies);

    let mut workspace = Workspace::new();
    workspace.members = Some(vec![
        "member1".to_string(),
        "member2".to_string(),
        "examples/workspace/mem*".to_string(),
        "member3".to_string(),
        "member4".to_string(),
    ]);
    workspace.exclude = Some(vec![
        "bad1".to_string(),
        "member3".to_string(),
        "examples/workspace/member2".to_string(),
        "bad2".to_string(),
    ]);

    crate_info.workspace = Some(workspace);
    load_workspace_members(&mut crate_info);

    assert!(crate_info.workspace.is_some());
    workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    let members = workspace.members.unwrap();
    assert!(members
        .iter()
        .position(|member| member == "path1")
        .is_some());
    assert!(members
        .iter()
        .position(|member| member == "path2")
        .is_some());
    assert!(members
        .iter()
        .position(|member| member == "member1")
        .is_some());
    assert!(members
        .iter()
        .position(|member| member == "member2")
        .is_some());
    assert!(members
        .iter()
        .position(|member| member == "member4")
        .is_some());
    assert!(members
        .iter()
        .position(|member| member == "examples/workspace/member1")
        .is_some());
    assert_eq!(members.len(), 7);
}
