use super::*;
use crate::test::is_min_rust_version;
use crate::types::CrateDependencyInfo;
use cargo_metadata::camino::Utf8Path;

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
fn add_members_workspace_non_alpha_ordered() {
    let mut crate_info = CrateInfo::new();

    crate_info.workspace = Some(Workspace::new());
    add_members(
        &mut crate_info,
        vec!["test2".to_string(), "test1".to_string()],
    );

    load_workspace_members(&mut crate_info);

    let members = crate_info.workspace.unwrap().members.unwrap();
    assert_eq!(members.len(), 2);
    assert_eq!(members[0], "test2".to_owned());
    assert_eq!(members[1], "test1".to_owned());
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

    assert_eq!(members.len(), 3);
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
        .position(|member| member == "somepath")
        .is_some());
}

#[test]
fn get_members_from_workspace_dependencies_workspace_paths() {
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
    dependencies.insert(
        "invalid1".to_string(),
        CrateDependency::Info(CrateDependencyInfo {
            path: Some("../outside_workspace".to_string()),
        }),
    );

    let mut workspace = Workspace::new();
    workspace.dependencies = Some(dependencies);
    let mut crate_info = CrateInfo::new();
    crate_info.workspace = Some(workspace);
    let members = get_members_from_workspace_dependencies(&crate_info);

    assert_eq!(members.len(), 3);
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
        .position(|member| member == "somepath")
        .is_some());
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
    let mut members = expand_glob_members("examples/workspace/*");

    assert!(members.len() > 0);
    assert!(members
        .iter()
        .position(|member| member == "examples/workspace/member1")
        .is_some());

    members = expand_glob_members("examples/workspace2/mem*");

    assert!(members.len() > 0);
    assert!(members
        .iter()
        .position(|member| member == "examples/workspace2/member2")
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
fn load_workspace_members_no_members_with_package() {
    let mut crate_info = CrateInfo::new();
    crate_info.workspace = Some(Workspace::new());
    crate_info.package = Some(PackageInfo::new());
    load_workspace_members(&mut crate_info);

    assert!(crate_info.workspace.is_some());
    let workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    let members = workspace.members.unwrap();
    assert_eq!(members.len(), 1);
    assert_eq!(members, vec![".".to_string()]);
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
    crate_info.package = Some(PackageInfo::new());
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
    assert!(members.iter().position(|member| member == ".").is_some());
    assert_eq!(members.len(), 9);
}

#[test]
fn load_workspace_members_mixed_members_and_paths() {
    let mut crate_info = CrateInfo::new();

    let mut dependencies = IndexMap::new();
    dependencies.insert(
        "my_package2".to_string(),
        CrateDependency::Info(CrateDependencyInfo {
            path: Some("path/to/my_package2".to_string()),
        }),
    );
    crate_info.dependencies = Some(dependencies);

    let mut workspace = Workspace::new();
    workspace.members = Some(vec!["path/to/my_package1".to_string()]);

    dependencies = IndexMap::new();
    dependencies.insert(
        "my_package3".to_string(),
        CrateDependency::Info(CrateDependencyInfo {
            path: Some("path/to/my_package3".to_string()),
        }),
    );
    workspace.dependencies = Some(dependencies);

    crate_info.workspace = Some(workspace);
    load_workspace_members(&mut crate_info);

    assert!(crate_info.workspace.is_some());
    workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    let members = workspace.members.unwrap();
    assert_eq!(members.len(), 3);
    assert!(members
        .iter()
        .position(|member| member == "path/to/my_package1")
        .is_some());
    assert!(members
        .iter()
        .position(|member| member == "path/to/my_package2")
        .is_some());
    assert!(members
        .iter()
        .position(|member| member == "path/to/my_package3")
        .is_some());
}

#[test]
#[ignore]
fn get_crate_target_triple() {
    let old_current_dir = env::current_dir().unwrap();

    assert_eq!(crate_target_triple(None, None), None);

    env::set_current_dir("src/lib/test/workspace2").unwrap();
    assert_eq!(crate_target_triple(None, None), None);

    let target_triple = rust_info::get().target_triple;
    assert_eq!(
        crate_target_triple(target_triple.clone(), None),
        target_triple
    );

    env::set_current_dir("member2").unwrap();
    assert_eq!(
        crate_target_triple(target_triple.clone(), None),
        Some("wasm32-unknown-unknown".into())
    );

    env::set_current_dir("../member/member3").unwrap();
    assert_eq!(
        crate_target_triple(target_triple.clone(), None),
        Some("aarch64-linux-android".into())
    );

    env::set_current_dir("../../member4").unwrap();
    assert_eq!(
        crate_target_triple(target_triple.clone(), None),
        Some("x86_64-pc-windows-msvc".into())
    );

    env::set_current_dir(old_current_dir).unwrap();
}

#[test]
#[ignore]
fn get_crate_target_dir() {
    let old_cwd = env::current_dir().unwrap();

    macro_rules! assert_dirs {
        ($host:literal $( , $custom:expr )?) => {{
            let dirs = crate_target_dirs(None);
            assert!(dirs.host.ends_with($host), "{} doesn't end with {}", dirs.host, $host);
            $(
                assert!(
                    dirs.custom.as_ref().map(|custom| custom.ends_with($custom)).unwrap_or(false),
                    "{:?} doesn't end with {}", dirs.custom, $custom
                );
            )?
        }};
    }

    assert_dirs!("target");

    env::set_current_dir("src/lib/test/workspace2").unwrap();
    assert_dirs!("target");

    env::set_var("CARGO_TARGET_DIR", "my_custom_dir");
    assert_dirs!("my_custom_dir");
    env::set_current_dir("env_target_dir_and_triple").unwrap();
    assert_dirs!(
        "my_custom_dir",
        Utf8Path::new("my_custom_dir").join("x86_64-pc-windows-msvc")
    );
    env::remove_var("CARGO_TARGET_DIR");

    env::set_current_dir("../target_dir").unwrap();
    assert_dirs!("my_custom_dir");

    env::set_current_dir("../target_dir_and_triple").unwrap();
    assert_dirs!(
        "my_custom_dir",
        Utf8Path::new("my_custom_dir").join("x86_64-pc-windows-msvc")
    );

    env::set_current_dir(old_cwd).unwrap();
}

#[test]
fn load_from_inherit_from_workspace_toml() {
    if is_min_rust_version("1.64.0") {
        let crate_info =
            load_from(Path::new("src/lib/test/workspace-inherit/member1/Cargo.toml").to_path_buf());

        let package_info = crate_info.package.unwrap();
        assert_eq!(package_info.name.unwrap(), "member1");
        assert_eq!(package_info.version.unwrap(), "1.2.3");
        assert_eq!(package_info.description.unwrap(), "test description");
        assert_eq!(package_info.documentation.unwrap(), "test docs");
        assert_eq!(package_info.license.unwrap(), "test license");
        assert_eq!(package_info.homepage.unwrap(), "https://testpage.com");
        assert_eq!(package_info.repository.unwrap(), "https://repotest.com");
    }
}

#[test]
fn dedup_members_with_duplicates() {
    let mut crate_info = CrateInfo::new();
    crate_info.workspace = Some(Workspace::new());
    add_members(
        &mut crate_info,
        vec![
            "test4".to_string(),
            "test1".to_string(),
            "test2".to_string(),
            "test1".to_string(),
            "test3".to_string(),
        ],
    );

    dedup_members(&mut crate_info);

    assert!(crate_info.workspace.is_some());
    let workspace = crate_info.workspace.unwrap();
    assert!(workspace.members.is_some());
    let members = workspace.members.unwrap();
    assert_eq!(members.len(), 4);
    assert_eq!(
        members,
        vec![
            "test4".to_string(),
            "test1".to_string(),
            "test2".to_string(),
            "test3".to_string()
        ]
    );
}
