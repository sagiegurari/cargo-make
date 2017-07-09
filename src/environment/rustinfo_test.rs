use super::*;

use log;

#[test]
fn rust_info_new() {
    let rust_info = RustInfo::new();

    assert!(rust_info.version.is_none());
    assert!(rust_info.channel.is_none());
}

#[test]
fn load_with_values() {
    let logger = log::create("error");
    let rust_info = load(&logger);

    assert!(rust_info.version.is_some());
    assert!(rust_info.channel.is_some());
}
