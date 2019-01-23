use super::*;

#[test]
fn load_with_values() {
    let git_info = load();

    assert!(git_info.branch.is_some());
}
