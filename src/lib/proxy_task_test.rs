#[cfg(target_os = "linux")]
use super::*;

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn create_proxy_task_no_makefile() {
    let makefile = envmnt::get_or("CARGO_MAKE_MAKEFILE_PATH", "EMPTY");
    envmnt::remove("CARGO_MAKE_MAKEFILE_PATH");
    let task = create_proxy_task("some_task", false, false, None);
    envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &makefile);

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut log_level_arg = "--loglevel=".to_string();
    log_level_arg.push_str(&log_level);

    let mut profile_arg = "--profile=".to_string();
    profile_arg.push_str(&profile::get());

    let args = task.args.unwrap();
    assert_eq!(args.len(), 6);
    assert_eq!(args[0], "make");
    assert_eq!(args[1], "--disable-check-for-updates");
    assert_eq!(args[2], "--no-on-error");
    assert_eq!(args[3], log_level_arg);
    assert_eq!(args[4], profile_arg);
    assert_eq!(args[5], "some_task");
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn create_proxy_task_with_makefile() {
    let makefile = envmnt::get_or("CARGO_MAKE_MAKEFILE_PATH", "EMPTY");
    envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &makefile);
    let task = create_proxy_task("some_task", false, false, None);

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut log_level_arg = "--loglevel=".to_string();
    log_level_arg.push_str(&log_level);

    let mut profile_arg = "--profile=".to_string();
    profile_arg.push_str(&profile::get());

    let args = task.args.unwrap();
    assert_eq!(args.len(), 8);
    assert_eq!(args[0], "make");
    assert_eq!(args[1], "--disable-check-for-updates");
    assert_eq!(args[2], "--no-on-error");
    assert_eq!(args[3], log_level_arg);
    assert_eq!(args[4], profile_arg);
    assert_eq!(args[5], "--makefile");
    assert_eq!(args[6], makefile);
    assert_eq!(args[7], "some_task");
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn create_proxy_task_with_makefile_argument() {
    let makefile = envmnt::get_or("CARGO_MAKE_MAKEFILE_PATH", "EMPTY");
    envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &makefile);
    let task = create_proxy_task("some_task", false, false, Some("external.toml".to_string()));

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut log_level_arg = "--loglevel=".to_string();
    log_level_arg.push_str(&log_level);

    let mut profile_arg = "--profile=".to_string();
    profile_arg.push_str(&profile::get());

    let args = task.args.unwrap();
    assert_eq!(args.len(), 8);
    assert_eq!(args[0], "make");
    assert_eq!(args[1], "--disable-check-for-updates");
    assert_eq!(args[2], "--no-on-error");
    assert_eq!(args[3], log_level_arg);
    assert_eq!(args[4], profile_arg);
    assert_eq!(args[5], "--makefile");
    assert_eq!(args[6], "external.toml");
    assert_eq!(args[7], "some_task");
}

#[test]
#[ignore]
#[cfg(target_os = "linux")]
fn create_proxy_task_allow_private() {
    let makefile = envmnt::get_or("CARGO_MAKE_MAKEFILE_PATH", "EMPTY");
    envmnt::remove("CARGO_MAKE_MAKEFILE_PATH");
    let task = create_proxy_task("some_task", true, false, None);
    envmnt::set("CARGO_MAKE_MAKEFILE_PATH", &makefile);

    assert_eq!(task.command.unwrap(), "cargo".to_string());

    let log_level = logger::get_log_level();
    let mut log_level_arg = "--loglevel=".to_string();
    log_level_arg.push_str(&log_level);

    let mut profile_arg = "--profile=".to_string();
    profile_arg.push_str(&profile::get());

    let args = task.args.unwrap();
    assert_eq!(args.len(), 7);
    assert_eq!(args[0], "make");
    assert_eq!(args[1], "--disable-check-for-updates");
    assert_eq!(args[2], "--no-on-error");
    assert_eq!(args[3], log_level_arg);
    assert_eq!(args[4], profile_arg);
    assert_eq!(args[5], "--allow-private");
    assert_eq!(args[6], "some_task");
}
