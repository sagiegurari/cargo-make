use super::*;

#[test]
fn execute_duckscript() {
    execute(&vec!["echo test".to_string()], &vec![], true);
}

#[test]
#[should_panic]
fn execute_duckscript_error() {
    execute(&vec!["badcommand".to_string()], &vec![], true);
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
fn execute_duckscript_cli_arguments_error() {
    execute(
        &vec!["assert ${1}".to_string()],
        &vec!["false".to_string()],
        true,
    );
}
