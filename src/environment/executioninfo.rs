//! # executioninfo
//!
//! Gets information about the execution environment.
//!

use ci_info;
use ci_info::types::Vendor;

pub(crate) fn current_ci_vendor() -> Option<Vendor> {
    let info = ci_info::get();
    info.vendor
}

pub(crate) fn is_ci() -> bool {
    ci_info::is_ci()
}
