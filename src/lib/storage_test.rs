use super::*;

#[test]
fn get_storage_directory_no_env_no_os_dir() {
    envmnt::remove("CARGO_MAKE_HOME");

    let directory = get_storage_directory(None, "test.txt", false).unwrap();
    let legacy_directory = get_legacy_directory().unwrap();

    assert_eq!(directory, legacy_directory);
}

#[test]
fn get_storage_directory_with_env() {
    let env_directory = env::current_dir().unwrap();
    envmnt::set("CARGO_MAKE_HOME", env_directory.to_str().unwrap());

    let directory = get_storage_directory(None, "test.txt", false).unwrap();

    envmnt::remove("CARGO_MAKE_HOME");

    assert_eq!(directory, env_directory);
}

#[test]
fn get_storage_directory_no_env_with_os_dir_file_exists() {
    envmnt::remove("CARGO_MAKE_HOME");

    let path = env::current_dir().unwrap();
    let mut os_directory = path.join("examples");
    let directory = get_storage_directory(Some(os_directory.clone()), "cache.toml", true).unwrap();

    os_directory = os_directory.join("cargo-make");
    assert_eq!(directory, os_directory);
}
