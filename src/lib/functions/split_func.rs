//! # split_func
//!
//! Split function which takes an environment variable name and a split by string.
//!

#[cfg(test)]
#[path = "./split_func_test.rs"]
mod split_func_test;

use envmnt;

pub(crate) fn invoke(function_args: &Vec<String>) -> Vec<String> {
    if function_args.len() != 2 {
        error!("split expects only 2 arguments (environment variable name, split by character)");
    }

    let env_key = function_args[0].clone();
    let split_by = function_args[1].clone();

    if split_by.len() != 1 {
        error!("split expects a single character separator");
    }

    let split_by_char = split_by.chars().next().unwrap();

    let value = envmnt::get_or(&env_key, "");

    if value.len() > 0 {
        let splitted = value.split(split_by_char);

        splitted.map(|str_value| str_value.to_string()).collect()
    } else {
        vec![]
    }
}
