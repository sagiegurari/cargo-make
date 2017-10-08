use super::*;
use environment::rustinfo;
use types::RustChannel;

#[test]
#[cfg(target_os = "linux")]
fn execute_valid() {
    let rust_info = rustinfo::load();

    if rust_info.channel.unwrap() == RustChannel::Nightly {
        execute(&vec!["fn main() {println!(\"test\");}".to_string()]);
    }
}

#[test]
#[cfg(target_os = "linux")]
#[should_panic]
fn execute_not_compile() {
    let rust_info = rustinfo::load();

    if rust_info.channel.unwrap() == RustChannel::Nightly {
        execute(&vec!["fn main() {donotcompile();}".to_string()]);
    } else {
        panic!("skipped");
    }
}

#[test]
#[cfg(target_os = "linux")]
#[should_panic]
fn execute_runtime_panic() {
    let rust_info = rustinfo::load();

    if rust_info.channel.unwrap() == RustChannel::Nightly {
        execute(&vec!["fn main() {panic!(\"error\");}".to_string()]);
    } else {
        panic!("skipped");
    }
}
