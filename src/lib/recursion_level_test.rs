use super::*;

#[test]
#[ignore]
fn recursion_level_changes() {
    // backup environment to avoid having conflicts if
    // the test is run within `cargo-make`.
    let rec_lvl = envmnt::get_or(RECURSION_ENV_VAR_NAME, "0");
    envmnt::remove(RECURSION_ENV_VAR_NAME);

    assert!(is_top());
    assert_eq!(get(), 0);

    increment(); // explicitly set to 0
    increment();
    assert!(!is_top());
    assert_eq!(get(), 1);

    increment();
    assert_eq!(get(), 2);

    envmnt::set(RECURSION_ENV_VAR_NAME, rec_lvl);
}
