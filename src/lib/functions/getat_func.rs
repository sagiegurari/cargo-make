//! # getat_func
//!
//! Split function which returns only the requested item from the created array.
//!

#[cfg(test)]
#[path = "getat_func_test.rs"]
mod getat_func_test;

use envmnt;

pub(crate) fn invoke(function_args: &Vec<String>) -> Vec<String> {
    if function_args.len() != 3 {
        error!(
            "split expects only 3 arguments (environment variable name, split by character, index)"
        );
    }

    let env_key = function_args[0].clone();
    let split_by = function_args[1].clone();
    let index: usize = match function_args[2].parse() {
        Ok(value) => value,
        Err(error) => {
            error!("Invalid index value: {}", &error);
            return vec![]; // should not get here
        }
    };

    if split_by.len() != 1 {
        error!("split expects a single character separator");
    }

    let split_by_char = split_by.chars().next().unwrap();

    let value = envmnt::get_or(&env_key, "");

    if value.len() > index {
        let splitted = value.split(split_by_char);

        let splitted_vec: Vec<String> = splitted.map(|str_value| str_value.to_string()).collect();
        let value = splitted_vec[index].clone();

        vec![value]
    } else {
        vec![]
    }
}
