use super::*;
use crate::types::{CliArgs, GlobalConfig};
use std::env;
use std::path::Path;

#[test]
#[ignore]
#[should_panic]
fn run_makefile_not_found() {
    let global_config = GlobalConfig::new();

    run(
        CliArgs {
            command: "cargo make".to_string(),
            build_file: Some("bad.toml".to_string()),
            task: "empty".to_string(),
            profile: None,
            log_level: "error".to_string(),
            disable_color: true,
            cwd: None,
            env: None,
            env_file: None,
            disable_workspace: false,
            disable_on_error: false,
            allow_private: false,
            skip_init_end_tasks: false,
            disable_check_for_updates: true,
            print_only: false,
            list_all_steps: false,
            diff_execution_plan: false,
            experimental: false,
            arguments: None,
            output_format: "default".to_string(),
            output_file: None,
            print_time_summary: false,
        },
        &global_config,
    );
}

#[test]
#[ignore]
fn run_empty_task() {
    let global_config = GlobalConfig::new();

    run(
        CliArgs {
            command: "cargo make".to_string(),
            build_file: None,
            task: "empty".to_string(),
            profile: None,
            log_level: "error".to_string(),
            disable_color: true,
            cwd: None,
            env: None,
            env_file: None,
            disable_workspace: false,
            disable_on_error: false,
            allow_private: false,
            skip_init_end_tasks: false,
            disable_check_for_updates: true,
            print_only: false,
            list_all_steps: false,
            diff_execution_plan: false,
            experimental: false,
            arguments: None,
            output_format: "default".to_string(),
            output_file: None,
            print_time_summary: false,
        },
        &global_config,
    );
}

#[test]
#[ignore]
fn print_empty_task() {
    let global_config = GlobalConfig::new();

    run(
        CliArgs {
            command: "cargo make".to_string(),
            build_file: None,
            task: "empty".to_string(),
            profile: None,
            log_level: "error".to_string(),
            disable_color: true,
            cwd: None,
            env: None,
            env_file: None,
            disable_workspace: false,
            disable_on_error: false,
            allow_private: false,
            skip_init_end_tasks: false,
            disable_check_for_updates: true,
            print_only: true,
            list_all_steps: false,
            diff_execution_plan: false,
            experimental: false,
            arguments: None,
            output_format: "default".to_string(),
            output_file: None,
            print_time_summary: false,
        },
        &global_config,
    );
}

#[test]
#[ignore]
fn list_empty_task() {
    let global_config = GlobalConfig::new();

    run(
        CliArgs {
            command: "cargo make".to_string(),
            build_file: None,
            task: "empty".to_string(),
            profile: None,
            log_level: "error".to_string(),
            disable_color: true,
            cwd: None,
            env: None,
            env_file: None,
            disable_workspace: false,
            disable_on_error: false,
            allow_private: false,
            skip_init_end_tasks: false,
            disable_check_for_updates: true,
            print_only: false,
            list_all_steps: true,
            diff_execution_plan: false,
            experimental: false,
            arguments: None,
            output_format: "default".to_string(),
            output_file: None,
            print_time_summary: false,
        },
        &global_config,
    );
}

#[test]
#[ignore]
fn run_file_and_task() {
    let global_config = GlobalConfig::new();

    run(
        CliArgs {
            command: "cargo make".to_string(),
            build_file: Some("./examples/dependencies.toml".to_string()),
            task: "A".to_string(),
            profile: None,
            log_level: "error".to_string(),
            disable_color: true,
            cwd: None,
            env: None,
            env_file: None,
            disable_workspace: false,
            disable_on_error: false,
            allow_private: false,
            skip_init_end_tasks: false,
            disable_check_for_updates: true,
            print_only: false,
            list_all_steps: false,
            diff_execution_plan: false,
            experimental: false,
            arguments: None,
            output_format: "default".to_string(),
            output_file: None,
            print_time_summary: false,
        },
        &global_config,
    );
}

#[test]
#[ignore]
fn run_cwd_with_file() {
    let global_config = GlobalConfig::new();

    let directory = Path::new("./examples");
    assert!(env::set_current_dir(&directory).is_ok());

    run(
        CliArgs {
            command: "cargo make".to_string(),
            build_file: Some("./examples/dependencies.toml".to_string()),
            task: "A".to_string(),
            profile: None,
            log_level: "error".to_string(),
            disable_color: true,
            cwd: Some("..".to_string()),
            env: None,
            env_file: None,
            disable_workspace: false,
            disable_on_error: false,
            allow_private: false,
            skip_init_end_tasks: false,
            disable_check_for_updates: true,
            print_only: false,
            list_all_steps: false,
            diff_execution_plan: false,
            experimental: false,
            arguments: None,
            output_format: "default".to_string(),
            output_file: None,
            print_time_summary: false,
        },
        &global_config,
    );
}

#[test]
#[ignore]
fn run_file_not_go_to_project_root() {
    let mut global_config = GlobalConfig::new();
    global_config.search_project_root = Some(false);

    run(
        CliArgs {
            command: "cargo make".to_string(),
            build_file: Some("./examples/dependencies.toml".to_string()),
            task: "A".to_string(),
            profile: None,
            log_level: "error".to_string(),
            disable_color: true,
            cwd: None,
            env: None,
            env_file: None,
            disable_workspace: false,
            disable_on_error: false,
            allow_private: false,
            skip_init_end_tasks: false,
            disable_check_for_updates: true,
            print_only: false,
            list_all_steps: false,
            diff_execution_plan: false,
            experimental: false,
            arguments: None,
            output_format: "default".to_string(),
            output_file: None,
            print_time_summary: false,
        },
        &global_config,
    );
}

#[test]
#[ignore]
fn run_cwd_go_to_project_root_current_dir() {
    let mut global_config = GlobalConfig::new();
    global_config.search_project_root = Some(true);

    run(
        CliArgs {
            command: "cargo make".to_string(),
            build_file: Some("./examples/dependencies.toml".to_string()),
            task: "A".to_string(),
            profile: None,
            log_level: "error".to_string(),
            disable_color: true,
            cwd: None,
            env: None,
            env_file: None,
            disable_workspace: false,
            disable_on_error: false,
            allow_private: false,
            skip_init_end_tasks: false,
            disable_check_for_updates: true,
            print_only: false,
            list_all_steps: false,
            diff_execution_plan: false,
            experimental: false,
            arguments: None,
            output_format: "default".to_string(),
            output_file: None,
            print_time_summary: false,
        },
        &global_config,
    );
}

#[test]
#[ignore]
fn run_cwd_go_to_project_root_child_dir() {
    let mut global_config = GlobalConfig::new();
    global_config.search_project_root = Some(true);

    let directory = Path::new("./examples");
    assert!(env::set_current_dir(&directory).is_ok());

    run(
        CliArgs {
            command: "cargo make".to_string(),
            build_file: Some("./examples/dependencies.toml".to_string()),
            task: "A".to_string(),
            profile: None,
            log_level: "error".to_string(),
            disable_color: true,
            cwd: None,
            env: None,
            env_file: None,
            disable_workspace: false,
            disable_on_error: false,
            allow_private: false,
            skip_init_end_tasks: false,
            disable_check_for_updates: true,
            print_only: false,
            list_all_steps: false,
            diff_execution_plan: false,
            experimental: false,
            arguments: None,
            output_format: "default".to_string(),
            output_file: None,
            print_time_summary: false,
        },
        &global_config,
    );
}

#[test]
#[ignore]
#[should_panic]
fn run_cwd_task_not_found() {
    let global_config = GlobalConfig::new();

    let directory = Path::new("./examples");
    assert!(env::set_current_dir(&directory).is_ok());

    run(
        CliArgs {
            command: "cargo make".to_string(),
            build_file: Some("./dependencies.toml".to_string()),
            task: "A".to_string(),
            profile: None,
            log_level: "error".to_string(),
            disable_color: true,
            cwd: Some("..".to_string()),
            env: None,
            env_file: None,
            disable_workspace: false,
            disable_on_error: false,
            allow_private: false,
            skip_init_end_tasks: false,
            disable_check_for_updates: true,
            print_only: false,
            list_all_steps: false,
            diff_execution_plan: false,
            experimental: false,
            arguments: None,
            output_format: "default".to_string(),
            output_file: None,
            print_time_summary: false,
        },
        &global_config,
    );
}

#[test]
#[should_panic]
fn run_for_args_bad_subcommand() {
    let global_config = GlobalConfig::new();
    let app = create_cli(&global_config, &"make".to_string(), true);

    let matches = app.get_matches_from(vec!["bad"]);

    run_for_args(matches, &global_config, &"make".to_string(), true);
}

#[test]
#[ignore]
fn run_for_args_valid() {
    let global_config = GlobalConfig::new();
    let app = create_cli(&global_config, &"make".to_string(), true);

    let matches = app.get_matches_from(vec![
        "cargo",
        "make",
        "--makefile",
        "./examples/dependencies.toml",
        "-l",
        "error",
        "A",
        "arg1",
        "arg2",
        "arg3",
    ]);

    run_for_args(matches, &global_config, &"make".to_string(), true);
}

#[test]
#[ignore]
fn run_for_args_with_global_config() {
    let mut global_config = GlobalConfig::new();
    global_config.log_level = Some("info".to_string());
    global_config.default_task_name = Some("empty".to_string());
    global_config.disable_color = Some(true);
    let app = create_cli(&global_config, &"make".to_string(), true);

    let matches = app.get_matches_from(vec!["cargo", "make"]);

    run_for_args(matches, &global_config, &"make".to_string(), true);
}

#[test]
#[ignore]
fn run_for_args_log_level_override() {
    let global_config = GlobalConfig::new();
    let app = create_cli(&global_config, &"make".to_string(), true);

    let matches = app.get_matches_from(vec![
        "cargo",
        "make",
        "--makefile",
        "./examples/dependencies.toml",
        "-t",
        "A",
        "-l",
        "error",
        "-v",
    ]);

    run_for_args(matches, &global_config, &"make".to_string(), true);
}

#[test]
#[ignore]
fn run_for_args_set_env_values() {
    let global_config = GlobalConfig::new();
    let app = create_cli(&global_config, &"make".to_string(), true);

    envmnt::set("ENV1_TEST", "EMPTY");
    envmnt::set("ENV2_TEST", "EMPTY");
    envmnt::set("ENV3_TEST", "EMPTY");

    let matches = app.get_matches_from(vec![
        "cargo",
        "make",
        "--env",
        "ENV1_TEST=TEST1",
        "--env",
        "ENV2_TEST=TEST2a=TEST2b",
        "-e",
        "ENV3_TEST=TEST3",
        "--verbose",
        "--disable-check-for-updates",
        "-t",
        "empty",
    ]);

    run_for_args(matches, &global_config, &"make".to_string(), true);

    assert_eq!(envmnt::get_or_panic("ENV1_TEST"), "TEST1");
    assert_eq!(envmnt::get_or_panic("ENV2_TEST"), "TEST2a=TEST2b");
    assert_eq!(envmnt::get_or_panic("ENV3_TEST"), "TEST3");
}

#[test]
#[ignore]
fn run_for_args_set_env_via_file() {
    let global_config = GlobalConfig::new();
    let app = create_cli(&global_config, &"make".to_string(), true);

    envmnt::set("ENV1_TEST", "EMPTY");
    envmnt::set("ENV2_TEST", "EMPTY");
    envmnt::set("ENV3_TEST", "EMPTY");

    let matches = app.get_matches_from(vec![
        "cargo",
        "make",
        "--env-file=./examples/test.env",
        "--verbose",
        "--disable-check-for-updates",
        "-t",
        "empty",
    ]);

    run_for_args(matches, &global_config, &"make".to_string(), true);

    assert_eq!(envmnt::get_or_panic("ENV1_TEST"), "TEST1");
    assert_eq!(envmnt::get_or_panic("ENV2_TEST"), "TEST2");
    assert_eq!(envmnt::get_or_panic("ENV3_TEST"), "VALUE OF ENV2 IS: TEST2");
}

#[test]
#[ignore]
fn run_for_args_set_env_both() {
    let global_config = GlobalConfig::new();
    let app = create_cli(&global_config, &"make".to_string(), true);

    envmnt::set("ENV1_TEST", "EMPTY");
    envmnt::set("ENV2_TEST", "EMPTY");
    envmnt::set("ENV3_TEST", "EMPTY");
    envmnt::set("ENV4_TEST", "EMPTY");
    envmnt::set("ENV5_TEST", "EMPTY");
    envmnt::set("ENV6_TEST", "EMPTY");

    let matches = app.get_matches_from(vec![
        "cargo",
        "make",
        "--env-file=./examples/test.env",
        "--env",
        "ENV4_TEST=TEST4",
        "--env",
        "ENV5_TEST=TEST5",
        "-e",
        "ENV6_TEST=TEST6",
        "--loglevel=error",
        "--disable-check-for-updates",
        "-t",
        "empty",
    ]);

    run_for_args(matches, &global_config, &"make".to_string(), true);

    assert_eq!(envmnt::get_or_panic("ENV1_TEST"), "TEST1");
    assert_eq!(envmnt::get_or_panic("ENV2_TEST"), "TEST2");
    assert_eq!(envmnt::get_or_panic("ENV3_TEST"), "VALUE OF ENV2 IS: TEST2");
    assert_eq!(envmnt::get_or_panic("ENV4_TEST"), "TEST4");
    assert_eq!(envmnt::get_or_panic("ENV5_TEST"), "TEST5");
    assert_eq!(envmnt::get_or_panic("ENV6_TEST"), "TEST6");
}

#[test]
#[ignore]
fn run_for_args_print_only() {
    let global_config = GlobalConfig::new();
    let app = create_cli(&global_config, &"make".to_string(), true);

    let matches = app.get_matches_from(vec![
        "cargo",
        "make",
        "--makefile",
        "./examples/dependencies.toml",
        "-t",
        "A",
        "-l",
        "error",
        "--no-workspace",
        "--no-on-error",
        "--print-steps",
        "--experimental",
    ]);

    run_for_args(matches, &global_config, &"make".to_string(), true);
}

#[test]
#[ignore]
fn run_for_args_diff_steps() {
    let global_config = GlobalConfig::new();
    let app = create_cli(&global_config, &"make".to_string(), true);

    let matches = app.get_matches_from(vec![
        "cargo",
        "make",
        "--makefile",
        "./examples/dependencies.toml",
        "-t",
        "empty",
        "-l",
        "error",
        "--no-workspace",
        "--diff-steps",
    ]);

    run_for_args(matches, &global_config, &"make".to_string(), true);
}

#[test]
#[ignore]
#[should_panic]
fn run_protected_flow_example() {
    let global_config = GlobalConfig::new();
    let app = create_cli(&global_config, &"make".to_string(), true);

    let matches = app.get_matches_from(vec![
        "cargo",
        "make",
        "--makefile",
        "./examples/on_error.toml",
    ]);

    run_for_args(matches, &global_config, &"make".to_string(), true);
}

#[test]
#[ignore]
fn run_for_args_no_task_args() {
    let global_config = GlobalConfig::new();
    let app = create_cli(&global_config, &"make".to_string(), true);

    envmnt::set("CARGO_MAKE_TASK_ARGS", "EMPTY");

    let matches = app.get_matches_from(vec![
        "cargo",
        "make",
        "--disable-check-for-updates",
        "empty",
    ]);

    run_for_args(matches, &global_config, &"make".to_string(), true);

    assert_eq!(envmnt::get_or_panic("CARGO_MAKE_TASK_ARGS"), "");
}

#[test]
#[ignore]
fn run_for_args_set_task_args() {
    let global_config = GlobalConfig::new();
    let app = create_cli(&global_config, &"make".to_string(), true);

    envmnt::set("CARGO_MAKE_TASK_ARGS", "EMPTY");

    let matches = app.get_matches_from(vec![
        "cargo",
        "make",
        "--disable-check-for-updates",
        "empty",
        "arg1",
        "arg2",
        "arg3",
    ]);

    run_for_args(matches, &global_config, &"make".to_string(), true);

    assert_eq!(
        envmnt::get_or_panic("CARGO_MAKE_TASK_ARGS"),
        "arg1;arg2;arg3"
    );
}
