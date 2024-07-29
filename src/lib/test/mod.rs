use crate::installer::crate_version_check::is_min_version_valid_for_versions;
use crate::logger;
use crate::logger::LoggerOptions;
use crate::types::{Config, ConfigSection, CrateInfo, EnvInfo, FlowInfo, ToolchainSpecifier};
use ci_info::types::CiInfo;
use git_info::types::GitInfo;
use indexmap::IndexMap;
use rust_info::types::{RustChannel, RustInfo};
use semver::Version;
use std::env;
use std::path::PathBuf;

pub(crate) fn on_test_startup() {
    logger::init(&LoggerOptions {
        name: String::from(env!("CARGO_PKG_NAME")),
        level: "error".to_string(),
        color: true,
    });
}

pub(crate) fn is_linux() -> bool {
    on_test_startup();

    cfg!(target_os = "linux")
}

pub(crate) fn is_windows() -> bool {
    on_test_startup();

    cfg!(windows)
}

pub(crate) fn is_rust_channel(rust_channel: RustChannel) -> bool {
    let rustinfo = rust_info::get();
    let current_rust_channel = rustinfo.channel.unwrap();

    current_rust_channel == rust_channel
}

pub(crate) fn is_min_rust_version(version: &str) -> bool {
    let rustinfo = rust_info::get();
    let rust_version = rustinfo.version.unwrap();

    let version_struct = Version::parse(version).unwrap();
    let rust_version_struct = Version::parse(&rust_version).unwrap();

    is_min_version_valid_for_versions(&version_struct, &rust_version_struct)
}

pub(crate) fn should_test(panic_if_false: bool) -> bool {
    on_test_startup();

    if (is_linux() && is_rust_channel(RustChannel::Nightly)) || !ci_info::is_ci() {
        true
    } else if panic_if_false {
        panic!("Skipped");
    } else {
        false
    }
}

pub(crate) fn should_test_unstable() -> bool {
    return !ci_info::is_ci();
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

pub(crate) fn get_temp_test_directory(subdir: &str) -> PathBuf {
    on_test_startup();

    let path = env::current_dir().unwrap();
    let mut directory = path.join("target/_cargo_make_temp/test");
    directory = directory.join(subdir);

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
            plugins: None,
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
