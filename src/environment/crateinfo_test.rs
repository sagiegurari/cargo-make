use super::*;
use log;

#[test]
fn crate_info_load() {
    let logger = log::create("error");
    let crate_info = load(&logger);

    assert!(crate_info.package.is_some());
    assert!(crate_info.workspace.is_none());

    let package = crate_info.package.unwrap();
    assert_eq!(package.name.unwrap(), "cargo-make");
}
