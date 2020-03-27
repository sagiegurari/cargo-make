use crate::logger;
use crate::logger::LoggerOptions;
use ci_info;
use ci_info::types::Vendor;
use fsio;
use rust_info;
use rust_info::types::RustChannel;
use std::env;
use std::path::PathBuf;

pub(crate) fn on_test_startup() {
    logger::init(&LoggerOptions {
        level: "error".to_string(),
        color: true,
    });
}

pub(crate) fn is_linux() -> bool {
    on_test_startup();

    if cfg!(target_os = "linux") {
        true
    } else {
        false
    }
}

pub(crate) fn is_windows() -> bool {
    on_test_startup();

    if cfg!(windows) {
        true
    } else {
        false
    }
}

fn is_travis_ci() -> bool {
    on_test_startup();

    envmnt::is_or("TRAVIS", false)
}

pub(crate) fn is_appveyor_ci() -> bool {
    let info = ci_info::get();

    if !info.ci {
        false
    } else {
        info.vendor.unwrap() == Vendor::AppVeyor
    }
}

pub(crate) fn is_windows_on_travis_ci() -> bool {
    on_test_startup();

    if is_windows() {
        is_travis_ci()
    } else {
        false
    }
}

pub(crate) fn is_local_or_travis_ci() -> bool {
    on_test_startup();

    !ci_info::is_ci() || is_travis_ci()
}

pub(crate) fn should_test(panic_if_false: bool) -> bool {
    on_test_startup();

    let rustinfo = rust_info::get();
    let rust_channel = rustinfo.channel.unwrap();

    if (is_linux() && rust_channel == RustChannel::Nightly) || !ci_info::is_ci() {
        true
    } else if panic_if_false {
        panic!("Skippied");
    } else {
        false
    }
}

pub(crate) fn get_os_runner() -> String {
    on_test_startup();

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
    on_test_startup();

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
    on_test_startup();

    let path = env::current_dir().unwrap();
    let directory = path.join("target/_cargo_make_temp/test");

    if directory.exists() {
        fsio::directory::delete(&directory).unwrap();
    }
    fsio::directory::create(&directory).unwrap();

    directory
}

pub(crate) fn is_not_rust_stable() -> bool {
    on_test_startup();

    let rustinfo = rust_info::get();
    let rust_channel = rustinfo.channel.unwrap();
    match rust_channel {
        RustChannel::Stable => false,
        RustChannel::Beta => true,
        RustChannel::Nightly => true,
    }
}

pub(crate) fn get_toolchain() -> String {
    on_test_startup();

    let rustinfo = rust_info::get();
    let rust_channel = rustinfo.channel.unwrap();
    let toolchain = match rust_channel {
        RustChannel::Stable => "stable",
        RustChannel::Beta => "beta",
        RustChannel::Nightly => "nightly",
    };

    toolchain.to_string()
}
