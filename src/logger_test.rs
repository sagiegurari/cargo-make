use super::*;

#[test]
fn get_level_verbose() {
    let level = get_level("verbose");
    assert_eq!(level, Level::VERBOSE);
}

#[test]
fn get_level_info() {
    let level = get_level("info");
    assert_eq!(level, Level::INFO);
}

#[test]
fn get_level_error() {
    let level = get_level("error");
    assert_eq!(level, Level::ERROR);
}

#[test]
fn get_level_other() {
    let level = get_level("test123");
    assert_eq!(level, Level::INFO);
}

#[test]
#[should_panic]
fn create_error() {
    init("error");

    error!("test");
}
