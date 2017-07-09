use super::*;

use log;

#[test]
fn load_with_values() {
    let logger = log::create("error");
    let git_info = load(&logger);

    assert!(git_info.branch.is_some());
}
