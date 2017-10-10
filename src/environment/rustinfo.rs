//! # rustinfo
//!
//! Loads rust compiler information.
//!

#[cfg(test)]
#[path = "./rustinfo_test.rs"]
mod rustinfo_test;

use rust_info;
use types::{RustChannel, RustInfo};

pub fn load() -> RustInfo {
    let info = rust_info::get();

    let mut rustinfo = RustInfo::new();

    if info.version.is_some() {
        rustinfo.version = Some(info.version.unwrap());
    }

    if info.channel.is_some() {
        let channel = info.channel.unwrap();

        match channel {
            rust_info::types::RustChannel::Stable => rustinfo.channel = Some(RustChannel::Stable),
            rust_info::types::RustChannel::Beta => rustinfo.channel = Some(RustChannel::Beta),
            rust_info::types::RustChannel::Nightly => rustinfo.channel = Some(RustChannel::Nightly),
        }
    }

    rustinfo.target_arch = Some(info.target_arch.unwrap_or("unknown".to_string()));
    rustinfo.target_env = Some(info.target_env.unwrap_or("unknown".to_string()));
    rustinfo.target_os = Some(info.target_os.unwrap_or("unknown".to_string()));
    rustinfo.target_pointer_width = Some(info.target_pointer_width.unwrap_or("unknown".to_string()));
    rustinfo.target_vendor = Some(info.target_vendor.unwrap_or("unknown".to_string()));

    rustinfo
}
