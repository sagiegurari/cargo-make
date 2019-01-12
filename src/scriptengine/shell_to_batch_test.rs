use super::*;

#[test]
fn execute_and_update_env_valid() {
    execute_and_update_env(
        "shell2batch_test",
        &vec!["echo test".to_string()],
        &vec!["test".to_string()],
    );
}

#[test]
#[should_panic]
fn execute_and_update_env_error() {
    execute_and_update_env("shell2batch_test", &vec!["exit 1".to_string()], &vec![]);
}
