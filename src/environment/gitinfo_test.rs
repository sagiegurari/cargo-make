use super::*;

use log;

#[test]
fn git_info_new() {
    let git_info = GitInfo::new();

    assert!(git_info.branch.is_none());
    assert!(git_info.user_name.is_none());
    assert!(git_info.user_email.is_none());
}

#[test]
fn load_with_values() {
    let logger = log::create("error");
    let git_info = load(&logger);

    assert!(git_info.branch.is_some());
}
