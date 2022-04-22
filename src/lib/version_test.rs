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
fn is_newer_same_full_no_partial() {
    let newer = is_newer("1.60.0", "1.60.0", false, true);
    assert!(!newer);
}

#[test]
fn is_newer_same_no_partial_no_patch() {
    let newer = is_newer("1.60", "1.60", false, true);
    assert!(newer);
}

#[test]
fn is_newer_same_no_partial_no_patch_for_newer() {
    let newer = is_newer("1.60.0", "1.60", false, true);
    assert!(newer);
}

#[test]
fn is_newer_same_no_partial_no_patch_for_older() {
    let newer = is_newer("1.60", "1.60.0", false, true);
    assert!(newer);
}

#[test]
fn is_newer_same_full_allow_partial() {
    let newer = is_newer("1.60.0", "1.60.0", true, true);
    assert!(!newer);
}

#[test]
fn is_newer_same_allow_partial_no_patch() {
    let newer = is_newer("1.60", "1.60", true, true);
    assert!(!newer);
}

#[test]
fn is_newer_same_allow_partial_no_patch_for_newer() {
    let newer = is_newer("1.60.0", "1.60", true, true);
    assert!(!newer);
}

#[test]
fn is_newer_same_allow_partial_no_patch_for_older() {
    let newer = is_newer("1.60", "1.60.0", true, true);
    assert!(!newer);
}

#[test]
fn is_same_equal_allow_partial_no_patch_for_older() {
    let newer = is_same("1.60", "1.60.0", true, false);
    assert!(newer);
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
    let version_string = (version.major + 1).to_string()
        + "."
        + &version.minor.to_string()
        + "."
        + &version.patch.to_string();

    let newer = is_newer_found(&version_string);

    assert!(newer);
}

#[test]
fn is_newer_found_newer_minor() {
    let current = env!("CARGO_PKG_VERSION");
    let version = Version::parse(current).unwrap();
    let version_string = version.major.to_string()
        + "."
        + &(version.minor + 1).to_string()
        + "."
        + &version.patch.to_string();

    let newer = is_newer_found(&version_string);

    assert!(newer);
}

#[test]
fn is_newer_found_newer_patch() {
    let current = env!("CARGO_PKG_VERSION");
    let version = Version::parse(current).unwrap();
    let version_string = version.major.to_string()
        + "."
        + &version.minor.to_string()
        + "."
        + &(version.patch + 1).to_string();

    let newer = is_newer_found(&version_string);

    assert!(newer);
}

#[test]
fn is_newer_found_older_major_newer_minor() {
    let current = env!("CARGO_PKG_VERSION");
    let version = Version::parse(current).unwrap();
    let version_string = (version.major as i64 - 1).to_string()
        + "."
        + &(version.minor + 1).to_string()
        + "."
        + &version.patch.to_string();

    let newer = is_newer_found(&version_string);

    assert!(!newer);
}

#[test]
fn is_newer_found_older_major_newer_patch() {
    let current = env!("CARGO_PKG_VERSION");
    let version = Version::parse(current).unwrap();
    let version_string = (version.major as i64 - 1).to_string()
        + "."
        + &version.minor.to_string()
        + "."
        + &(version.patch + 1).to_string();

    let newer = is_newer_found(&version_string);

    assert!(!newer);
}

#[test]
fn is_newer_found_older_minor_newer_patch() {
    let current = env!("CARGO_PKG_VERSION");
    let version = Version::parse(current).unwrap();
    let version_string = version.major.to_string()
        + "."
        + &(version.minor as i64 - 1).to_string()
        + "."
        + &(version.patch + 1).to_string();

    let newer = is_newer_found(&version_string);

    assert!(!newer);
}

#[test]
fn check_full() {
    check();
}

#[test]
fn get_now_as_seconds_valid() {
    let now = get_now_as_seconds();
    assert!(now > 0);
}

#[test]
fn has_amount_of_days_passed_from_last_check_zero_days() {
    let passed = has_amount_of_days_passed_from_last_check(0, 1);
    assert!(passed);
}

#[test]
fn has_amount_of_days_passed_from_last_check_false() {
    let last_check = get_now_as_seconds();
    let passed = has_amount_of_days_passed_from_last_check(1, last_check);
    assert!(!passed);
}

#[test]
fn has_amount_of_days_passed_from_last_check_true_by_day() {
    let last_check = get_now_as_seconds() - (4 * 24 * 60 * 60);
    let passed = has_amount_of_days_passed_from_last_check(3, last_check);
    assert!(passed);
}

#[test]
fn has_amount_of_days_passed_from_last_check_true_by_second() {
    let last_check = get_now_as_seconds() - (24 * 60 * 60) - 1;
    let passed = has_amount_of_days_passed_from_last_check(1, last_check);
    assert!(passed);
}

#[test]
fn has_amount_of_days_passed_from_last_check_false_by_second() {
    let last_check = get_now_as_seconds() - (24 * 60 * 60) + 1;
    let passed = has_amount_of_days_passed_from_last_check(1, last_check);
    assert!(!passed);
}

#[test]
fn has_amount_of_days_passed_none() {
    let cache = Cache::new();
    let passed = has_amount_of_days_passed(300, &cache);
    assert!(passed);
}

#[test]
fn has_amount_of_days_passed_zero_days() {
    let last_check = get_now_as_seconds();
    let mut cache = Cache::new();
    cache.last_update_check = Some(last_check);
    let passed = has_amount_of_days_passed(0, &cache);
    assert!(passed);
}

#[test]
fn has_amount_of_days_passed_false() {
    let last_check = get_now_as_seconds();
    let mut cache = Cache::new();
    cache.last_update_check = Some(last_check);
    let passed = has_amount_of_days_passed(1, &cache);
    assert!(!passed);
}

#[test]
fn has_amount_of_days_passed_true_by_day() {
    let last_check = get_now_as_seconds() - (4 * 24 * 60 * 60);
    let mut cache = Cache::new();
    cache.last_update_check = Some(last_check);
    let passed = has_amount_of_days_passed(3, &cache);
    assert!(passed);
}

#[test]
fn has_amount_of_days_passed_true_by_second() {
    let last_check = get_now_as_seconds() - (24 * 60 * 60) - 1;
    let mut cache = Cache::new();
    cache.last_update_check = Some(last_check);
    let passed = has_amount_of_days_passed(1, &cache);
    assert!(passed);
}

#[test]
fn has_amount_of_days_passed_false_by_second() {
    let last_check = get_now_as_seconds() - (24 * 60 * 60) + 1;
    let mut cache = Cache::new();
    cache.last_update_check = Some(last_check);
    let passed = has_amount_of_days_passed(1, &cache);
    assert!(!passed);
}

#[test]
fn get_days_none() {
    let global_config = GlobalConfig::new();
    let days = get_days(&global_config);
    assert_eq!(days, 7);
}

#[test]
fn get_days_always() {
    let mut global_config = GlobalConfig::new();
    global_config.update_check_minimum_interval = Some("always".to_string());
    let days = get_days(&global_config);
    assert_eq!(days, 0);
}

#[test]
fn get_days_daily() {
    let mut global_config = GlobalConfig::new();
    global_config.update_check_minimum_interval = Some("daily".to_string());
    let days = get_days(&global_config);
    assert_eq!(days, 1);
}

#[test]
fn get_days_weekly() {
    let mut global_config = GlobalConfig::new();
    global_config.update_check_minimum_interval = Some("weekly".to_string());
    let days = get_days(&global_config);
    assert_eq!(days, 7);
}

#[test]
fn get_days_monthly() {
    let mut global_config = GlobalConfig::new();
    global_config.update_check_minimum_interval = Some("monthly".to_string());
    let days = get_days(&global_config);
    assert_eq!(days, 30);
}

#[test]
fn get_days_unknown() {
    let mut global_config = GlobalConfig::new();
    global_config.update_check_minimum_interval = Some("bad123".to_string());
    let days = get_days(&global_config);
    assert_eq!(days, 7);
}

#[test]
fn should_check_none() {
    let global_config = GlobalConfig::new();
    should_check(&global_config);
}

#[test]
fn should_check_always() {
    let mut global_config = GlobalConfig::new();
    global_config.update_check_minimum_interval = Some("always".to_string());
    let check = should_check(&global_config);
    assert!(check);
}

#[test]
fn should_check_other() {
    let mut global_config = GlobalConfig::new();
    global_config.update_check_minimum_interval = Some("weekly".to_string());
    should_check(&global_config);
}
