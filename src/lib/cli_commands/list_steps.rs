//! # list_steps
//!
//! Lists all known tasks in multiple formats.
//!

#[cfg(test)]
#[path = "./list_steps_test.rs"]
mod list_steps_test;

use crate::types::Config;
use crate::execution_plan;
use std::collections::BTreeMap;

pub(crate) fn run(config: &Config, output_format: &str) -> u32 {
    let mut count = 0;

    let markdown = output_format == "markdown";

    let mut categories = BTreeMap::new();

    for key in config.tasks.keys() {
        let task = execution_plan::get_normalized_task(&config, &key, true);

        let is_private = match task.private {
            Some(private) => private,
            None => false,
        };

        if !is_private {
            count = count + 1;

            let category = match task.category {
                Some(value) => value,
                None => "No Category".to_string(),
            };

            let description = match task.description {
                Some(value) => value,
                None => "No Description.".to_string(),
            };

            let mut tasks_map = BTreeMap::new();
            match categories.get_mut(&category) {
                Some(value) => tasks_map.append(value),
                _ => (),
            };

            tasks_map.insert(key.clone(), description.clone());
            categories.insert(category, tasks_map);
        }
    }

    let post_key = if markdown { "**" } else { "" };
    for (category, tasks) in &categories {
        if markdown {
            println!("##### {}\n", category);
        } else {
            println!("{}\n----------", category);
        }

        for (key, description) in tasks {
            if markdown {
                print!("* **");
            }
            println!("{}{} - {} ", &key, &post_key, &description);
        }
        println!("");
    }

    count
}
