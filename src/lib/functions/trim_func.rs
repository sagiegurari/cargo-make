//! # trim_func
//!
//! Takes an environment variable name and returns its value trimmed.
//! The value will be removed if empty.
//!

#[cfg(test)]
#[path = "./trim_func_test.rs"]
mod trim_func_test;

use crate::environment;

pub(crate) fn invoke(function_args: &Vec<String>) -> Vec<String> {
    if function_args.len() != 1 {
        error!("trim expects only 1 argument (environment variable name)");
        panic!("trim expects only 1 argument (environment variable name)");
    }

    let env_key = function_args[0].clone();

    let value = environment::get_env(&env_key, "");

    let trimmed_value = value.trim().to_string();

    if trimmed_value.len() > 0 {
        vec![trimmed_value]
    } else {
        vec![]
    }
}
