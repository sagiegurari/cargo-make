use super::*;
use ci_info;
use rust_info;
use rust_info::types::RustChannel;

fn should_test(panic_if_false: bool) -> bool {
    let rustinfo = rust_info::get();
    let rust_channel = rustinfo.channel.unwrap();

    if (cfg!(target_os = "linux") && rust_channel == RustChannel::Nightly) || !ci_info::is_ci() {
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
