use super::*;
use test;

#[test]
fn execute_valid() {
    if test::should_test(false) {
        execute(&vec!["fn main() {println!(\"test\");}".to_string()]);
    }
}

#[test]
#[should_panic]
fn execute_not_compile() {
    if test::should_test(true) {
        execute(&vec!["fn main() {donotcompile();}".to_string()]);
    }
}

#[test]
#[should_panic]
fn execute_runtime_panic() {
    if test::should_test(true) {
        execute(&vec!["fn main() {panic!(\"error\");}".to_string()]);
    }
}
