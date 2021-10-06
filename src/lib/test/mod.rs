use crate::logger;
use crate::logger::LoggerOptions;
use crate::types::{Config, ConfigSection, CrateInfo, EnvInfo, FlowInfo, ToolchainSpecifier};
use ci_info;
use ci_info::types::CiInfo;
use fsio;
use git_info::types::GitInfo;
use indexmap::IndexMap;
use rust_info;
use rust_info::types::{RustChannel, RustInfo};
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

pub(crate) fn should_test(panic_if_false: bool) -> bool {
    on_test_startup();

    let rustinfo = rust_info::get();
    let rust_channel = rustinfo.channel.unwrap();

    if (is_linux() && rust_channel == RustChannel::Nightly) || !ci_info::is_ci() {
        true
    } else if panic_if_false {
        panic!("Skipped");
    } else {
        false
    }
}

pub(crate) fn get_os_runner() -> String {
    on_test_startup();

    if cfg!(windows) {
        "powershell.exe".to_string()
    } else {
        "sh".to_string()
    }
}

pub(crate) fn get_os_extension() -> String {
    on_test_startup();

    if cfg!(windows) {
        "ps1".to_string()
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

pub(crate) fn get_toolchain() -> ToolchainSpecifier {
    on_test_startup();

    let rustinfo = rust_info::get();
    let rust_channel = rustinfo.channel.unwrap();
    let toolchain = match rust_channel {
        RustChannel::Stable => "stable",
        RustChannel::Beta => "beta",
        RustChannel::Nightly => "nightly",
    };

    toolchain.into()
}

pub(crate) fn create_empty_flow_info() -> FlowInfo {
    FlowInfo {
        config: Config {
            config: ConfigSection::new(),
            env_files: vec![],
            env: IndexMap::new(),
            env_scripts: vec![],
            tasks: IndexMap::new(),
        },
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: CiInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    }
}
