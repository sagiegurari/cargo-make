use super::*;
use semver::Version;

#[test]
fn print_notification_simple() {
    print_notification("1");
}

#[test]
fn get_version_from_output_empty() {
    let version = get_version_from_output("");

    assert!(version.is_none());
}

#[test]
fn get_version_from_output_few_args() {
    let version = get_version_from_output("1 2");

    assert!(version.is_none());
}

#[test]
fn get_version_from_output_found() {
    let version = get_version_from_output("1 2 \"1.2.3\" 4 5 6");

    assert_eq!(version.unwrap(), "1.2.3");
}

#[test]
fn is_newer_found_same() {
    let current = env!("CARGO_PKG_VERSION");
    let newer = is_newer_found(current);

    assert!(!newer);
}

#[test]
fn is_newer_found_newer_major() {
    let current = env!("CARGO_PKG_VERSION");
    let version = Version::parse(current).unwrap();
    let version_string = (version.major + 1).to_string() + "." + &version.minor.to_string() + "." + &version.patch.to_string();

    let newer = is_newer_found(&version_string);

    assert!(newer);
}

#[test]
fn is_newer_found_newer_minor() {
    let current = env!("CARGO_PKG_VERSION");
    let version = Version::parse(current).unwrap();
    let version_string = version.major.to_string() + "." + &(version.minor + 1).to_string() + "." + &version.patch.to_string();

    let newer = is_newer_found(&version_string);

    assert!(newer);
}

#[test]
fn is_newer_found_newer_patch() {
    let current = env!("CARGO_PKG_VERSION");
    let version = Version::parse(current).unwrap();
    let version_string = version.major.to_string() + "." + &version.minor.to_string() + "." + &(version.patch + 1).to_string();

    let newer = is_newer_found(&version_string);

    assert!(newer);
}

#[test]
fn is_newer_found_older_major_newer_minor() {
    let current = env!("CARGO_PKG_VERSION");
    let version = Version::parse(current).unwrap();
    let version_string = (version.major as i64 - 1).to_string() + "." + &(version.minor + 1).to_string() + "." + &version.patch.to_string();

    let newer = is_newer_found(&version_string);

    assert!(!newer);
}

#[test]
fn is_newer_found_older_major_newer_patch() {
    let current = env!("CARGO_PKG_VERSION");
    let version = Version::parse(current).unwrap();
    let version_string = (version.major as i64 - 1).to_string() + "." + &version.minor.to_string() + "." + &(version.patch + 1).to_string();

    let newer = is_newer_found(&version_string);

    assert!(!newer);
}

#[test]
fn is_newer_found_older_minor_newer_patch() {
    let current = env!("CARGO_PKG_VERSION");
    let version = Version::parse(current).unwrap();
    let version_string = version.major.to_string() + "." + &(version.minor as i64 - 1).to_string() + "." + &(version.patch + 1).to_string();

    let newer = is_newer_found(&version_string);

    assert!(!newer);
}

#[test]
fn check_full() {
    check();
}
