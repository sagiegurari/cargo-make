//! # script_utils
//!
//! Helper functions for script invocations.
//!

#[cfg(test)]
#[path = "script_utils_test.rs"]
mod script_utils_test;

use crate::io;

pub(crate) fn create_script_file(script_text: &Vec<String>, extension: &str) -> String {
    let text = script_text.join("\n");

    io::create_text_file(&text, &extension)
}
