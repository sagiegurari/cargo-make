use super::*;

#[test]
fn execute_duckscript() {
    execute(&vec!["echo test".to_string()], &vec![], true);
}

#[test]
fn execute_duckscript_error_no_validate() {
    execute(&vec!["badcommand".to_string()], &vec![], false);
}

#[test]
fn execute_duckscript_cli_arguments() {
    execute(
        &vec!["get_env ${1}".to_string()],
        &vec!["CARGO_MAKE".to_string()],
        true,
    );
}

#[test]
#[should_panic]
fn execute_duckscript_crash() {
    execute(&vec!["function test".to_string()], &vec![], true);
}
