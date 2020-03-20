use super::*;
use envmnt;

#[test]
fn recursion_level_changes() {
    // backup environment to avoid having conflicts if
    // the test is run within `cargo-make`.
    let rec_lvl = envmnt::get_or(RECURSION_ENV_VAR_NAME, "0");
    envmnt::remove(RECURSION_ENV_VAR_NAME);

    assert!(is_first_level());
    assert_eq!(recursion_level(), 0);

    increase_level(); // explicitly set to 0
    increase_level();
    assert!(!is_first_level());
    assert_eq!(recursion_level(), 1);

    increase_level();
    assert_eq!(recursion_level(), 2);

    envmnt::set(RECURSION_ENV_VAR_NAME, rec_lvl);
}
