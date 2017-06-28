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
fn create_verbose() {
    let logger = create("verbose");

    assert!(logger.is_verbose_enabled());
    assert!(logger.is_info_enabled());

    logger.verbose::<()>("test", &["test"], None);
}

#[test]
fn create_info() {
    let logger = create("info");

    assert!(!logger.is_verbose_enabled());
    assert!(logger.is_info_enabled());

    logger.info::<()>("test", &["test"], None);
}

#[test]
#[should_panic]
fn create_error() {
    let logger = create("error");

    assert!(!logger.is_verbose_enabled());
    assert!(!logger.is_info_enabled());

    logger.error::<()>("test", &["test"], None);
}
