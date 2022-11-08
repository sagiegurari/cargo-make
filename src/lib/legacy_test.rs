use super::*;
use crate::test;

#[test]
#[cfg(target_os = "linux")]
fn get_legacy_cargo_make_home_linux() {
    let mut home = envmnt::get_or_panic("HOME");
    home.push_str("/.cargo-make");
    let cargo_make_home = get_legacy_cargo_make_home().unwrap();

    assert_eq!(home, cargo_make_home.to_str().unwrap());
}

#[test]
#[ignore]
fn get_cargo_make_home_no_env() {
    envmnt::remove("CARGO_MAKE_HOME");

    let cargo_make_home = get_legacy_cargo_make_home().unwrap();
    let home = get_cargo_make_home().unwrap();
    assert_eq!(home, cargo_make_home);
}

#[test]
#[ignore]
fn get_cargo_make_home_with_env() {
    let path = env::current_dir().unwrap();
    let directory = path.join("examples/cargo-make");
    envmnt::set("CARGO_MAKE_HOME", directory.to_str().unwrap());

    let home = get_cargo_make_home().unwrap();

    envmnt::remove("CARGO_MAKE_HOME");

    assert_eq!(home, directory);
}

#[test]
#[ignore]
fn migrate_from_directory_not_exists() {
    let path = env::current_dir().unwrap();
    let legacy_directory = path.join("legacy_bad");
    let target_directory = path.join("target_bad");

    let done = migrate_from_directory(target_directory, "test.txt", &legacy_directory);

    assert!(done);
}

#[test]
#[ignore]
fn migrate_from_directory_dir_exists_file_not_exists() {
    let path = env::current_dir().unwrap();
    let target_directory = path.join("target_bad");

    let legacy_directory = test::get_temp_test_directory(None);

    let done = migrate_from_directory(target_directory, "test.txt", &legacy_directory);

    assert!(done);
}

#[test]
#[ignore]
fn migrate_from_directory_delete_legacy_directory() {
    let test_directory = test::get_temp_test_directory(None);
    let legacy_directory = test_directory.join("legacy");
    let target_directory = test_directory.join("target");

    let legacy_file = legacy_directory.join("test.txt");
    fsio::file::write_text_file(&legacy_file, "test 123").unwrap();

    let done = migrate_from_directory(target_directory.clone(), "test.txt", &legacy_directory);

    assert!(done);

    assert!(target_directory.exists());

    #[cfg(target_os = "linux")]
    assert!(!legacy_directory.exists());

    let target_file = target_directory.join("test.txt");
    let file_text = fsio::file::read_text_file(&target_file).unwrap();

    assert_eq!(&file_text, "test 123");
}

#[test]
fn show_deprecated_attriute_warning_valid() {
    show_deprecated_attriute_warning("old", "new");
}
