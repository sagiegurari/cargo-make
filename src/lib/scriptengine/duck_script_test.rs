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
#[should_panic]
fn execute_duckscript_error_with_validate() {
    execute(&vec!["badcommand".to_string()], &vec![], true);
}

#[test]
fn execute_duckscript_cli_arguments() {
    execute(
        &vec!["assert ${1}".to_string()],
        &vec!["true".to_string()],
        true,
    );
}

#[test]
#[should_panic]
fn execute_duckscript_cli_arguments2() {
    execute(
        &vec!["assert ${1}".to_string()],
        &vec!["false".to_string()],
        true,
    );
}

#[test]
#[should_panic]
fn execute_duckscript_crash() {
    execute(&vec!["function test".to_string()], &vec![], true);
}

#[test]
#[should_panic]
fn execute_duckscript_crash2() {
    execute(&vec!["assert false".to_string()], &vec![], true);
}
