//! # trim_func
//!
//! Takes an environment variable name and returns its value trimmed.
//! The value will be removed if empty.
//!

#[cfg(test)]
#[path = "trim_func_test.rs"]
mod trim_func_test;

pub(crate) fn invoke(function_args: &Vec<String>) -> Vec<String> {
    if function_args.len() > 2 {
        error!("trim expects up to 2 arguments (environment variable name and optionally start/end trim flag)");
    }

    let env_key = function_args[0].clone();

    let value = envmnt::get_or(&env_key, "");

    let trimmed_value = if function_args.len() == 1 {
        value.trim().to_string()
    } else {
        let trim_type = function_args[1].clone();

        match trim_type.as_ref() {
            "start" => value.trim_start().to_string(),
            "end" => value.trim_end().to_string(),
            _ => {
                error!("Invalid trim type provided, only start or end are supported.");
                panic!("Invalid trim type provided, only start or end are supported.");
            }
        }
    };

    if trimmed_value.len() > 0 {
        vec![trimmed_value]
    } else {
        vec![]
    }
}
