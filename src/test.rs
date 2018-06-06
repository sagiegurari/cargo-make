use ci_info;
use rust_info;
use rust_info::types::RustChannel;

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
        "cmd.exe".to_string()
    } else {
        "sh".to_string()
    }
}

pub(crate) fn get_os_extension() -> String {
    if cfg!(windows) {
        "bat".to_string()
    } else {
        "sh".to_string()
    }
}
