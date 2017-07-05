use super::*;
use std::env;
use std::path::Path;

#[test]
fn run_empty_task() {
    run("bad.toml", "empty", "error", None, false);
}

#[test]
fn print_empty_task() {
    run("bad.toml", "empty", "error", None, true);
}

#[test]
fn run_file_and_task() {
    run("./examples/dependencies.toml", "A", "error", None, false);
}

#[test]
fn run_cwd_with_file() {
    let directory = Path::new("./examples");
    assert!(env::set_current_dir(&directory).is_ok());

    run("./examples/dependencies.toml", "A", "error", Some(".."), false);
}

#[test]
#[should_panic]
fn run_cwd_task_not_found() {
    let directory = Path::new("./examples");
    assert!(env::set_current_dir(&directory).is_ok());

    run("./dependencies.toml", "A", "error", Some(".."), false);
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
        "--print-steps",
    ]);

    run_for_args(matches);
}
