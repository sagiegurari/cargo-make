#[path = "./makefile_test.rs"]
mod makefile_test;

use ci_info;
use rust_info;
use rust_info::types::RustChannel;
use std::env;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::PathBuf;

fn is_travis_ci() -> bool {
    match env::var("TRAVIS") {
        Ok(value) => value == "true",
        _ => false,
    }
}

pub(crate) fn is_windows_on_travis_ci() -> bool {
    if cfg!(windows) {
        is_travis_ci()
    } else {
        false
    }
}

pub(crate) fn should_test(panic_if_false: bool) -> bool {
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

pub(crate) fn get_os_runner() -> String {
    if cfg!(windows) {
        if is_travis_ci() {
            "sh".to_string()
        } else {
            "powershell.exe".to_string()
        }
    } else {
        "sh".to_string()
    }
}

pub(crate) fn get_os_extension() -> String {
    if cfg!(windows) {
        if is_travis_ci() {
            "sh".to_string()
        } else {
            "ps1".to_string()
        }
    } else {
        "sh".to_string()
    }
}

pub(crate) fn get_temp_test_directory() -> PathBuf {
    let path = env::current_dir().unwrap();
    let directory = path.join("target/_cargo_make_temp/test");

    if directory.exists() {
        remove_dir_all(&directory).unwrap();
    }
    create_dir_all(&directory).unwrap();

    directory
}

pub(crate) fn is_not_rust_stable() -> bool {
    let rustinfo = rust_info::get();
    let rust_channel = rustinfo.channel.unwrap();
    match rust_channel {
        RustChannel::Stable => false,
        RustChannel::Beta => true,
        RustChannel::Nightly => true,
    }
}

pub(crate) fn get_toolchain() -> String {
    let rustinfo = rust_info::get();
    let rust_channel = rustinfo.channel.unwrap();
    let toolchain = match rust_channel {
        RustChannel::Stable => "stable",
        RustChannel::Beta => "beta",
        RustChannel::Nightly => "nightly",
    };

    toolchain.to_string()
}
