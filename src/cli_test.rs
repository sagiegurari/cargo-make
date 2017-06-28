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
