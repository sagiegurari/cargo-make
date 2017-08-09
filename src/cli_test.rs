use super::*;
use std::env;
use std::path::Path;
use types::CliArgs;

#[test]
fn run_empty_task() {
    run(CliArgs {
        build_file: "bad.toml".to_string(),
        task: "empty".to_string(),
        log_level: "error".to_string(),
        cwd: None,
        env: None,
        disable_workspace: false,
        print_only: false,
        list_all_steps: false,
        experimental: false
    });
}

#[test]
fn print_empty_task() {
    run(CliArgs {
        build_file: "bad.toml".to_string(),
        task: "empty".to_string(),
        log_level: "error".to_string(),
        cwd: None,
        env: None,
        disable_workspace: false,
        print_only: true,
        list_all_steps: false,
        experimental: false
    });
}

#[test]
fn list_empty_task() {
    run(CliArgs {
        build_file: "bad.toml".to_string(),
        task: "empty".to_string(),
        log_level: "error".to_string(),
        cwd: None,
        env: None,
        disable_workspace: false,
        print_only: false,
        list_all_steps: true,
        experimental: false
    });
}

#[test]
fn run_file_and_task() {
    run(CliArgs {
        build_file: "./examples/dependencies.toml".to_string(),
        task: "A".to_string(),
        log_level: "error".to_string(),
        cwd: None,
        env: None,
        disable_workspace: false,
        print_only: false,
        list_all_steps: false,
        experimental: false
    });
}

#[test]
fn run_cwd_with_file() {
    let directory = Path::new("./examples");
    assert!(env::set_current_dir(&directory).is_ok());

    run(CliArgs {
        build_file: "./examples/dependencies.toml".to_string(),
        task: "A".to_string(),
        log_level: "error".to_string(),
        cwd: Some("..".to_string()),
        env: None,
        disable_workspace: false,
        print_only: false,
        list_all_steps: false,
        experimental: false
    });
}

#[test]
#[should_panic]
fn run_cwd_task_not_found() {
    let directory = Path::new("./examples");
    assert!(env::set_current_dir(&directory).is_ok());

    run(CliArgs {
        build_file: "./dependencies.toml".to_string(),
        task: "A".to_string(),
        log_level: "error".to_string(),
        cwd: Some("..".to_string()),
        env: None,
        disable_workspace: false,
        print_only: false,
        list_all_steps: false,
        experimental: false
    });
}

#[test]
#[should_panic]
fn run_cli_panic() {
    run_cli();
}

#[test]
#[should_panic]
fn run_for_args_bad_subcommand() {
    let app = create_cli();

    let matches = app.get_matches_from(vec!["bad"]);

    run_for_args(matches);
}

#[test]
fn run_for_args_valid() {
    let app = create_cli();

    let matches = app.get_matches_from(vec!["cargo", "make", "--makefile", "./examples/dependencies.toml", "-t", "A", "-l", "error"]);

    run_for_args(matches);
}

#[test]
fn run_for_args_log_level_override() {
    let app = create_cli();

    let matches = app.get_matches_from(vec!["cargo", "make", "--makefile", "./examples/dependencies.toml", "-t", "A", "-l", "error", "-v"]);

    run_for_args(matches);
}

#[test]
fn run_for_args_set_env() {
    let app = create_cli();

    env::set_var("ENV1_TEST", "EMPTY");
    env::set_var("ENV2_TEST", "EMPTY");
    env::set_var("ENV3_TEST", "EMPTY");

    let matches = app.get_matches_from(vec![
        "cargo",
        "make",
        "--env",
        "ENV1_TEST=TEST1",
        "--env",
        "ENV2_TEST=TEST2",
        "-e",
        "ENV3_TEST=TEST3",
        "--verbose",
        "-t",
        "empty",
    ]);

    run_for_args(matches);

    assert_eq!(env::var("ENV1_TEST").unwrap(), "TEST1");
    assert_eq!(env::var("ENV2_TEST").unwrap(), "TEST2");
    assert_eq!(env::var("ENV3_TEST").unwrap(), "TEST3");
}

#[test]
fn run_for_args_print_only() {
    let app = create_cli();

    let matches = app.get_matches_from(vec![
        "cargo",
        "make",
        "--makefile",
        "./examples/dependencies.toml",
        "-t",
        "A",
        "-l",
        "error",
        "--no-workspace",
        "--print-steps",
        "--experimental",
    ]);

    run_for_args(matches);
}
