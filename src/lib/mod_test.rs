use super::*;

#[test]
#[should_panic]
fn run_cli_panic() {
    run_cli("make".to_string(), true);
}
