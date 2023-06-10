//! # split_func
//!
//! Split function which takes an environment variable name and a split by string.
//!

#[cfg(test)]
#[path = "split_func_test.rs"]
mod split_func_test;

use envmnt;

pub(crate) fn invoke(function_args: &Vec<String>) -> Vec<String> {
    let args_count = function_args.len();
    if args_count < 2 || args_count > 3 {
        error!("split expects two or three arguments (environment variable name, split by character, optional mode: default, remove-empty)");
    }

    let env_key = function_args[0].clone();
    let split_by = function_args[1].clone();
    let mode_name = if args_count == 3 {
        &function_args[2]
    } else {
        "default"
    };
    let remove_empty = mode_name == "remove-empty";

    if split_by.len() != 1 {
        error!("split expects a single character separator");
    }

    let split_by_char = split_by.chars().next().unwrap();

    let value = envmnt::get_or(&env_key, "");

    if value.len() > 0 {
        let splitted = value.split(split_by_char);

        splitted
            .map(|str_value| str_value.to_string())
            .filter(|string_value| {
                if remove_empty && string_value.is_empty() {
                    false
                } else {
                    true
                }
            })
            .collect()
    } else {
        vec![]
    }
}
