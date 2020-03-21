//! # trim_func
//!
//! Takes an environment variable name and returns its mapped value.
//! In case no mapped value if found, the default value will be returned if provided as last argument or if no default value is provided, the original value is returned.
//! The value will be removed if empty.
//!

#[cfg(test)]
#[path = "./decode_func_test.rs"]
mod decode_func_test;

use crate::environment;
use envmnt;

pub(crate) fn invoke(function_args: &Vec<String>) -> Vec<String> {
    if function_args.len() == 0 {
        error!("decode expects at least one argument.");
    }

    let env_key = function_args[0].clone();
    let env_value = envmnt::get_or(&env_key, "");

    let mut mapped_value = None;
    let mut found = false;
    let mut skip = true;
    for item in function_args.iter() {
        if skip {
            skip = false;
        } else if found {
            mapped_value = Some(item.to_string());
            break;
        } else if item.to_string() == env_value {
            found = true;
        } else {
            skip = true;
        }
    }

    // if no mapped value found and default value provided
    let mut output_value = match mapped_value {
        Some(value) => value.clone(),
        None => {
            if function_args.len() % 2 == 0 && function_args.len() > 1 {
                function_args[function_args.len() - 1].to_string()
            } else {
                env_value.clone()
            }
        }
    };

    output_value = environment::expand_value(&output_value);

    if output_value.len() > 0 {
        vec![output_value]
    } else {
        vec![]
    }
}
