//! # list_steps
//!
//! Lists all known tasks in multiple formats.
//! Or can list tasks based on a category
//!

#[cfg(test)]
#[path = "list_steps_test.rs"]
mod list_steps_test;

use crate::execution_plan;
use crate::io;
use crate::types::{Config, DeprecationInfo};
use std::collections::{BTreeMap, BTreeSet};

pub fn run(
    config: &Config,
    output_format: &str,
    output_file: &Option<String>,
    category: &Option<String>,
    hide_uninteresting: bool,
) {
    let output = create_list(&config, output_format, category, hide_uninteresting);

    match output_file {
        Some(file) => {
            io::write_text_file(&file, &output);
            ()
        }
        None => print!("{}", output),
    }
}

/// Panics if task does not exist.
pub(crate) fn create_list(
    config: &Config,
    output_format: &str,
    category_filter: &Option<String>,
    hide_uninteresting: bool,
) -> String {
    // category -> actual_task -> description
    let mut categories: BTreeMap<String, BTreeMap<String, String>> = BTreeMap::new();
    // actual_task -> aliases
    let mut aliases: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();

    // iterate over all tasks to build categories and aliases
    for key in config.tasks.keys() {
        let actual_task_name =
            execution_plan::get_actual_task_name(&config, &key).unwrap_or_else(|| {
                error!("Task {} not found", &key);
                panic!("Task {} not found", &key);
            });

        let task = execution_plan::get_normalized_task(&config, &actual_task_name, true);

        let is_private = match task.private {
            Some(private) => private,
            None => false,
        };

        let skip_task = if is_private {
            true
        } else if hide_uninteresting {
            key.contains("pre-")
                || key.contains("post-")
                || key == "init"
                || key == "end"
                || key == "empty"
        } else {
            false
        };

        if !skip_task {
            let category = match task.category {
                Some(value) => value,
                None => "No Category".to_string(),
            };

            if category_filter
                .as_ref()
                .map_or(false, |value| value != &category)
            {
                continue;
            }

            if &actual_task_name != key {
                aliases
                    .entry(actual_task_name)
                    .or_default()
                    .insert(key.clone());
                continue;
            }

            let description = match task.description {
                Some(value) => value,
                None => "No Description.".to_string(),
            };

            let deprecated_message = match task.deprecated {
                Some(deprecated) => match deprecated {
                    DeprecationInfo::Boolean(value) => {
                        if value {
                            " (deprecated)".to_string()
                        } else {
                            "".to_string()
                        }
                    }
                    DeprecationInfo::Message(ref message) => {
                        let mut buffer = " (deprecated - ".to_string();
                        buffer.push_str(message);
                        buffer.push_str(")");

                        buffer
                    }
                },
                None => "".to_string(),
            };

            let mut text = String::from(description);
            text.push_str(&deprecated_message);

            categories
                .entry(category)
                .or_default()
                .insert(key.clone(), text);
        }
    }

    // build the task list output string
    let single_page_markdown = output_format == "markdown-single-page";
    let markdown = single_page_markdown
        || output_format == "markdown"
        || output_format == "markdown-sub-section";
    let just_task_name = output_format == "autocomplete";

    let mut buffer = String::new();
    if single_page_markdown {
        buffer.push_str(&format!("# Task List\n\n"));
    }

    let post_key = if markdown { "**" } else { "" };
    for (category, tasks) in &categories {
        if category_filter
            .as_ref()
            .map_or(false, |value| value != category)
        {
            continue;
        }

        if !just_task_name {
            if single_page_markdown {
                buffer.push_str(&format!("## {}\n\n", category));
            } else if markdown {
                buffer.push_str(&format!("#### {}\n\n", category));
            } else {
                buffer.push_str(&format!("{}\n----------\n", category));
            }
        }

        for (key, description) in tasks {
            if markdown {
                buffer.push_str(&format!("* **"));
            }

            let aliases = if let Some(aliases) = aliases.remove(key) {
                if just_task_name {
                    aliases.into_iter().collect::<Vec<String>>().join(" ")
                } else {
                    format!(
                        " [aliases: {}]",
                        aliases.into_iter().collect::<Vec<String>>().join(", ")
                    )
                }
            } else {
                "".to_string()
            };

            if just_task_name {
                buffer.push_str(&format!("{} {} ", &key, aliases));
            } else {
                buffer.push_str(&format!(
                    "{}{} - {}{}\n",
                    &key, &post_key, &description, aliases
                ));
            }
        }

        if !just_task_name {
            buffer.push('\n');
        }
    }

    buffer
}
