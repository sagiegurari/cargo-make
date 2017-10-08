use super::*;
use environment::is_ci;
use environment::rustinfo;
use types::RustChannel;

fn should_test(panic_if_false: bool) -> bool {
    let rust_info = rustinfo::load();
    let rust_channel = rust_info.channel.unwrap();

    if (cfg!(target_os = "linux") && rust_channel == RustChannel::Nightly) || !is_ci() {
        true
    } else if panic_if_false {
        panic!("Skippied");
    } else {
        false
    }
}

#[test]
fn execute_valid() {
    if should_test(false) {
        execute(&vec!["fn main() {println!(\"test\");}".to_string()]);
    }
}

#[test]
#[should_panic]
fn execute_not_compile() {
    if should_test(true) {
        execute(&vec!["fn main() {donotcompile();}".to_string()]);
    }
}

#[test]
#[should_panic]
fn execute_runtime_panic() {
    if should_test(true) {
        execute(&vec!["fn main() {panic!(\"error\");}".to_string()]);
    }
}
