use super::*;
use crate::test;

#[test]
#[ignore]
fn get_script_runner_default() {
    envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");
    let provider = get_script_runner();

    assert_eq!(provider, ScriptRunner::CargoScript);
}

#[test]
#[ignore]
fn get_script_runner_cargo_script() {
    envmnt::set("CARGO_MAKE_RUST_SCRIPT_PROVIDER", "cargo-script");
    let provider = get_script_runner();
    envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");

    assert_eq!(provider, ScriptRunner::CargoScript);
}

#[test]
#[ignore]
fn get_script_runner_cargo_play() {
    envmnt::set("CARGO_MAKE_RUST_SCRIPT_PROVIDER", "cargo-play");
    let provider = get_script_runner();
    envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");

    assert_eq!(provider, ScriptRunner::CargoPlay);
}

#[test]
#[ignore]
fn get_script_runner_cargo_other() {
    envmnt::set("CARGO_MAKE_RUST_SCRIPT_PROVIDER", "bad value");
    let provider = get_script_runner();
    envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");

    assert_eq!(provider, ScriptRunner::CargoScript);
}

#[test]
#[ignore]
fn execute_default_valid() {
    if test::should_test(false) {
        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");

        execute(
            &vec!["fn main() {println!(\"test\");}".to_string()],
            &vec![],
            true,
        );
    }
}

#[test]
#[ignore]
#[should_panic]
fn execute_default_not_compile() {
    if test::should_test(true) {
        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");

        execute(
            &vec!["fn main() {donotcompile();}".to_string()],
            &vec![],
            true,
        );

        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");
    }
}

#[test]
#[ignore]
#[should_panic]
fn execute_default_runtime_panic() {
    if test::should_test(true) {
        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");

        execute(
            &vec!["fn main() {panic!(\"error\");}".to_string()],
            &vec![],
            true,
        );

        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");
    }
}

#[test]
#[ignore]
fn execute_default_runtime_panic_no_validate() {
    if test::should_test(false) {
        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");

        execute(
            &vec!["fn main() {panic!(\"error\");}".to_string()],
            &vec![],
            false,
        );

        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");
    }
}

#[test]
#[ignore]
fn execute_cargo_script_valid() {
    if test::should_test(false) {
        envmnt::set("CARGO_MAKE_RUST_SCRIPT_PROVIDER", "cargo-script");

        execute(
            &vec!["fn main() {println!(\"test\");}".to_string()],
            &vec![],
            true,
        );

        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");
    }
}

#[test]
#[ignore]
#[should_panic]
fn execute_cargo_script_not_compile() {
    if test::should_test(true) {
        envmnt::set("CARGO_MAKE_RUST_SCRIPT_PROVIDER", "cargo-script");

        execute(
            &vec!["fn main() {donotcompile();}".to_string()],
            &vec![],
            true,
        );

        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");
    }
}

#[test]
#[ignore]
#[should_panic]
fn execute_cargo_script_runtime_panic() {
    if test::should_test(true) {
        envmnt::set("CARGO_MAKE_RUST_SCRIPT_PROVIDER", "cargo-script");

        execute(
            &vec!["fn main() {panic!(\"error\");}".to_string()],
            &vec![],
            true,
        );

        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");
    }
}

#[test]
#[ignore]
fn execute_cargo_script_runtime_panic_no_validate() {
    if test::should_test(false) {
        envmnt::set("CARGO_MAKE_RUST_SCRIPT_PROVIDER", "cargo-script");

        execute(
            &vec!["fn main() {panic!(\"error\");}".to_string()],
            &vec![],
            false,
        );

        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");
    }
}

#[test]
#[ignore]
fn execute_cargo_play_valid() {
    if test::should_test(false) {
        envmnt::set("CARGO_MAKE_RUST_SCRIPT_PROVIDER", "cargo-play");

        execute(
            &vec!["fn main() {println!(\"test\");}".to_string()],
            &vec![],
            true,
        );

        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");
    }
}

#[test]
#[ignore]
#[should_panic]
fn execute_cargo_play_not_compile() {
    if test::should_test(true) {
        envmnt::set("CARGO_MAKE_RUST_SCRIPT_PROVIDER", "cargo-play");

        execute(
            &vec!["fn main() {donotcompile();}".to_string()],
            &vec![],
            true,
        );

        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");
    }
}

#[test]
#[ignore]
#[should_panic]
fn execute_cargo_play_runtime_panic() {
    if test::should_test(true) {
        envmnt::set("CARGO_MAKE_RUST_SCRIPT_PROVIDER", "cargo-play");

        execute(
            &vec!["fn main() {panic!(\"error\");}".to_string()],
            &vec![],
            true,
        );

        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");
    }
}

#[test]
#[ignore]
fn execute_cargo_play_runtime_panic_no_validate() {
    if test::should_test(false) {
        envmnt::set("CARGO_MAKE_RUST_SCRIPT_PROVIDER", "cargo-play");

        execute(
            &vec!["fn main() {panic!(\"error\");}".to_string()],
            &vec![],
            false,
        );

        envmnt::remove("CARGO_MAKE_RUST_SCRIPT_PROVIDER");
    }
}
