use super::*;
use crate::test;
use envmnt;
use rust_info::types::RustChannel;

#[test]
#[ignore]
fn get_cargo_home_env() {
    let directory_path = env::current_dir().unwrap();
    let directory = directory_path.to_str().unwrap();
    envmnt::set("CARGO_HOME", directory);

    let output = get_cargo_home().unwrap();
    envmnt::remove("CARGO_HOME");

    assert_eq!(output, directory);
}

#[test]
#[ignore]
fn get_cargo_home_no_env() {
    envmnt::remove("CARGO_HOME");

    let output = get_cargo_home().unwrap();

    assert!(output.contains(".cargo"));
}

#[test]
#[ignore]
fn get_cargo_home_file() {
    let directory_path = env::current_dir().unwrap();
    let directory = directory_path.to_str().unwrap();
    let file_path = Path::new(directory).join("Cargo.toml");
    envmnt::set("CARGO_HOME", file_path);

    let output = get_cargo_home();
    envmnt::remove("CARGO_HOME");

    assert!(output.is_none());
}

#[test]
fn load_crates_toml_valid() {
    let cwd = envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY");
    let file_path = Path::new(&cwd).join("src/lib/test/test_files");
    let directory = file_path.to_str().unwrap();
    let info_option = load_crates_toml(&directory);

    assert!(info_option.is_some());
    let info = info_option.unwrap();
    assert!(info.v1.is_some());
    assert_eq!(info.v1.unwrap().len(), 3);
}

#[test]
fn load_crates_toml_file_not_found() {
    let cwd = envmnt::get_or_panic("CARGO_MAKE_WORKING_DIRECTORY");
    let info_option = load_crates_toml(&cwd);

    assert!(info_option.is_none());
}

#[test]
fn get_crate_version_from_info_no_info() {
    let info = CratesRegistryInfo { v1: None };
    let version = get_crate_version_from_info("cargo-make", &info);

    assert!(version.is_none());
}

#[test]
fn get_crate_version_from_info_empty_info() {
    let info = CratesRegistryInfo {
        v1: Some(HashMap::new()),
    };
    let version = get_crate_version_from_info("cargo-make", &info);

    assert!(version.is_none());
}

#[test]
fn get_crate_version_from_info_not_found() {
    let mut map = HashMap::new();
    map.insert("1 1 1".to_string(), vec![]);
    map.insert("2 2 2".to_string(), vec![]);
    let info = CratesRegistryInfo { v1: Some(map) };
    let version = get_crate_version_from_info("cargo-make", &info);

    assert!(version.is_none());
}

#[test]
fn get_crate_version_from_info_found_invalid_line() {
    let mut map = HashMap::new();
    map.insert("1 1 1".to_string(), vec![]);
    map.insert("cargo-make".to_string(), vec![]);
    map.insert("2 2 2".to_string(), vec![]);
    let info = CratesRegistryInfo { v1: Some(map) };
    let version = get_crate_version_from_info("cargo-make", &info);

    assert!(version.is_none());
}

#[test]
fn get_crate_version_from_info_found_invalid_version() {
    let mut map = HashMap::new();
    map.insert("1 1 1".to_string(), vec![]);
    map.insert("cargo-make abc 2".to_string(), vec![]);
    map.insert("2 2 2".to_string(), vec![]);
    let info = CratesRegistryInfo { v1: Some(map) };
    let version = get_crate_version_from_info("cargo-make", &info);

    assert!(version.is_none());
}

#[test]
fn get_crate_version_from_info_valid() {
    let mut map = HashMap::new();
    map.insert("1 1 1".to_string(), vec![]);
    map.insert("cargo-make 1.2.3 2".to_string(), vec![]);
    map.insert("2 2 2".to_string(), vec![]);
    let info = CratesRegistryInfo { v1: Some(map) };
    let version = get_crate_version_from_info("cargo-make", &info);

    let semver_version = version.unwrap();
    assert_eq!(semver_version.major, 1);
    assert_eq!(semver_version.minor, 2);
    assert_eq!(semver_version.patch, 3);
}

#[test]
#[cfg(target_os = "linux")]
fn get_crate_version_for_rustup_component() {
    if test::is_rust_channel(RustChannel::Stable) {
        let mut version = get_crate_version("rustfmt", None);
        assert!(version.is_none());

        version = get_crate_version("rustfmt", Some("rustfmt"));
        assert!(version.is_some());
    }
}

#[test]
fn is_min_version_valid_for_versions_equal() {
    let valid = is_min_version_valid_for_versions(
        &Version::parse("1.2.3").unwrap(),
        &Version::parse("1.2.3").unwrap(),
    );

    assert!(valid);
}

#[test]
fn is_min_version_valid_for_versions_false_major() {
    let valid = is_min_version_valid_for_versions(
        &Version::parse("2.2.3").unwrap(),
        &Version::parse("1.2.3").unwrap(),
    );

    assert!(!valid);
}

#[test]
fn is_min_version_valid_for_versions_false_minor() {
    let valid = is_min_version_valid_for_versions(
        &Version::parse("1.3.3").unwrap(),
        &Version::parse("1.2.3").unwrap(),
    );

    assert!(!valid);
}

#[test]
fn is_min_version_valid_for_versions_false_patch() {
    let valid = is_min_version_valid_for_versions(
        &Version::parse("1.2.4").unwrap(),
        &Version::parse("1.2.3").unwrap(),
    );

    assert!(!valid);
}

#[test]
fn is_min_version_valid_for_versions_true_major() {
    let valid = is_min_version_valid_for_versions(
        &Version::parse("1.2.3").unwrap(),
        &Version::parse("2.2.3").unwrap(),
    );

    assert!(valid);
}

#[test]
fn is_min_version_valid_for_versions_true_minor() {
    let valid = is_min_version_valid_for_versions(
        &Version::parse("1.2.3").unwrap(),
        &Version::parse("1.3.3").unwrap(),
    );

    assert!(valid);
}

#[test]
fn is_min_version_valid_for_versions_true_patch() {
    let valid = is_min_version_valid_for_versions(
        &Version::parse("1.2.3").unwrap(),
        &Version::parse("1.2.4").unwrap(),
    );

    assert!(valid);
}

#[test]
fn is_min_version_valid_not_found() {
    let valid = is_min_version_valid("bad_crate", "1.2.3", None);

    assert!(valid);
}

#[test]
fn is_min_version_valid_invalid_version() {
    let valid = is_min_version_valid("cargo-make", "bad_version", None);

    assert!(valid);
}

#[test]
fn is_min_version_valid_old_version() {
    let valid = is_min_version_valid("cargo-make", "0.0.1", None);

    assert!(valid);
}

#[test]
fn is_min_version_valid_newer_version() {
    let valid = is_min_version_valid("cargo-make", "10000.0.0", None);

    assert!(!valid);
}

#[test]
fn is_min_version_valid_same_version() {
    let version = get_crate_version("cargo-make", None).unwrap();
    let mut version_string = String::new();
    version_string.push_str(&version.major.to_string());
    version_string.push_str(".");
    version_string.push_str(&version.minor.to_string());
    version_string.push_str(".");
    version_string.push_str(&version.patch.to_string());

    let valid = is_min_version_valid("cargo-make", &version_string, None);

    assert!(valid);
}

#[test]
fn is_version_valid_for_versions_equal() {
    let valid = is_version_valid_for_versions(
        &Version::parse("1.2.3").unwrap(),
        &Version::parse("1.2.3").unwrap(),
    );

    assert!(valid);
}

#[test]
fn is_version_valid_for_versions_false_major() {
    let valid = is_version_valid_for_versions(
        &Version::parse("2.2.3").unwrap(),
        &Version::parse("1.2.3").unwrap(),
    );

    assert!(!valid);
}

#[test]
fn is_version_valid_for_versions_false_minor() {
    let valid = is_version_valid_for_versions(
        &Version::parse("1.3.3").unwrap(),
        &Version::parse("1.2.3").unwrap(),
    );

    assert!(!valid);
}

#[test]
fn is_version_valid_for_versions_false_patch() {
    let valid = is_version_valid_for_versions(
        &Version::parse("1.2.4").unwrap(),
        &Version::parse("1.2.3").unwrap(),
    );

    assert!(!valid);
}

#[test]
fn is_version_valid_not_found() {
    let valid = is_version_valid("bad_crate", "1.2.3", None);

    assert!(valid);
}

#[test]
fn is_version_valid_invalid_version() {
    let valid = is_version_valid("cargo-make", "bad_version", None);

    assert!(valid);
}

#[test]
fn is_version_valid_old_version() {
    let valid = is_version_valid("cargo-make", "0.0.1", None);

    assert!(!valid);
}

#[test]
fn is_version_valid_newer_version() {
    let valid = is_version_valid("cargo-make", "10000.0.0", None);

    assert!(!valid);
}

#[test]
fn is_version_valid_same_version() {
    let version = get_crate_version("cargo-make", None).unwrap();
    let mut version_string = String::new();
    version_string.push_str(&version.major.to_string());
    version_string.push_str(".");
    version_string.push_str(&version.minor.to_string());
    version_string.push_str(".");
    version_string.push_str(&version.patch.to_string());

    let valid = is_min_version_valid("cargo-make", &version_string, None);

    assert!(valid);
}
