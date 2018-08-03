use super::*;
use std::fs::File;
use std::io::{Read, Write};
use test;

#[test]
#[cfg(target_os = "linux")]
fn get_legacy_cargo_make_home_linux() {
    let mut home = env::var("HOME").unwrap();
    home.push_str("/.cargo-make");
    let cargo_make_home = get_legacy_cargo_make_home().unwrap();

    assert_eq!(home, cargo_make_home.to_str().unwrap());
}

#[test]
fn get_cargo_make_home_no_env() {
    env::remove_var("CARGO_MAKE_HOME");

    let cargo_make_home = get_legacy_cargo_make_home().unwrap();
    let home = get_cargo_make_home().unwrap();
    assert_eq!(home, cargo_make_home);
}

#[test]
fn get_cargo_make_home_with_env() {
    let path = env::current_dir().unwrap();
    let directory = path.join("examples/cargo-make");
    env::set_var("CARGO_MAKE_HOME", directory.to_str().unwrap());

    let home = get_cargo_make_home().unwrap();

    env::remove_var("CARGO_MAKE_HOME");

    assert_eq!(home, directory);
}

#[test]
fn migrate_from_directory_not_exists() {
    let path = env::current_dir().unwrap();
    let legacy_directory = path.join("legacy_bad");
    let target_directory = path.join("target_bad");

    let done = migrate_from_directory(target_directory, "test.txt", &legacy_directory);

    assert!(done);
}

#[test]
fn migrate_from_directory_dir_exists_file_not_exists() {
    let path = env::current_dir().unwrap();
    let target_directory = path.join("target_bad");

    let legacy_directory = test::get_temp_test_directory();

    let done = migrate_from_directory(target_directory, "test.txt", &legacy_directory);

    assert!(done);
}

#[test]
fn migrate_from_directory_delete_legacy_directory() {
    let test_directory = test::get_temp_test_directory();
    let legacy_directory = test_directory.join("legacy");
    let target_directory = test_directory.join("target");
    create_dir_all(&legacy_directory).unwrap();

    let legacy_file = legacy_directory.join("test.txt");
    let mut legacy_file_pt = File::create(&legacy_file).unwrap();
    legacy_file_pt.write_all("test 123".as_bytes()).unwrap();

    let done = migrate_from_directory(target_directory.clone(), "test.txt", &legacy_directory);

    assert!(done);

    assert!(target_directory.exists());

    #[cfg(target_os = "linux")]
    assert!(!legacy_directory.exists());

    let target_file = target_directory.join("test.txt");
    let mut target_file_pt = File::open(&target_file).unwrap();
    let mut file_text = String::new();
    target_file_pt.read_to_string(&mut file_text).unwrap();

    assert_eq!(&file_text, "test 123");
}
