use super::*;

#[test]
#[cfg(target_os = "linux")]
fn execute_valid() {
    execute(&vec!["fn main() {println!(\"test\");}".to_string()]);
}

#[test]
#[should_panic]
fn execute_not_compile() {
    execute(&vec!["fn main() {donotcompile();}".to_string()]);
}

#[test]
#[should_panic]
fn execute_runtime_panic() {
    execute(&vec!["fn main() {panic!(\"error\");}".to_string()]);
}
