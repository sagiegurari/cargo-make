use std::env;

use crate::{logger, profile, types::Task};

#[cfg(test)]
#[path = "./proxy_task_test.rs"]
mod proxy_task_test;

pub(crate) fn create_proxy_task(
    task: &str,
    allow_private: bool,
    skip_init_end_tasks: bool,
    makefile: Option<String>,
) -> Task {
    //get log level name
    let log_level = logger::get_log_level();

    let mut log_level_arg = "--loglevel=".to_string();
    log_level_arg.push_str(&log_level);

    //get profile
    let profile_name = profile::get();

    let mut profile_arg = "--profile=".to_string();
    profile_arg.push_str(&profile_name);

    //setup common args
    let mut args = vec![
        "make".to_string(),
        "--disable-check-for-updates".to_string(),
        "--no-on-error".to_string(),
        log_level_arg.to_string(),
        profile_arg.to_string(),
    ];

    if allow_private {
        args.push("--allow-private".to_string());
    }

    if skip_init_end_tasks {
        args.push("--skip-init-end-tasks".to_string());
    }

    //get makefile location
    let makefile_path_option = match makefile {
        Some(makefile_path) => Some(makefile_path),
        None => match env::var("CARGO_MAKE_MAKEFILE_PATH") {
            Ok(makefile_path) => Some(makefile_path),
            _ => None,
        },
    };
    if let Some(makefile_path) = makefile_path_option {
        if makefile_path.len() > 0 {
            args.push("--makefile".to_string());
            args.push(makefile_path);
        }
    };

    args.push(task.to_string());

    let mut proxy_task = Task::new();
    proxy_task.command = Some("cargo".to_string());
    proxy_task.args = Some(args);

    proxy_task.get_normalized_task()
}
