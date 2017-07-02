use super::*;

#[test]
fn run_empty_task() {
    run("bad.toml", "empty", "error");
}

#[test]
fn run_file_and_task() {
    run("./examples/dependencies.toml", "A", "error");
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
