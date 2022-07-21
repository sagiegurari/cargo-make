use super::*;

fn default_parse_cli_args(mut args: Vec<&str>) -> CliArgs {
    let global_config = GlobalConfig::new();
    args.insert(0, "makers");
    parse_args(&global_config, "makers", false, Some(args))
}

fn default_parsed_cli_args() -> CliArgs {
    let mut cli_args = CliArgs::new();

    cli_args.command = "makers".to_string();
    cli_args.task = "default".to_string();
    cli_args.profile = Some("development".to_string());

    if ci_info::is_ci() {
        cli_args.print_time_summary = true;
    }

    cli_args
}

fn assert_cli_args(cli_args_ref1: &CliArgs, cli_args_ref2: &CliArgs) {
    let cli_args1 = cli_args_ref1.clone();
    let cli_args2 = cli_args_ref2.clone();

    assert_eq!(cli_args1.command, cli_args2.command);
    assert_eq!(cli_args1.build_file, cli_args2.build_file);
    assert_eq!(cli_args1.task, cli_args2.task);
    assert_eq!(cli_args1.profile.unwrap(), cli_args2.profile.unwrap());
    assert_eq!(cli_args1.log_level, cli_args2.log_level);
    assert_eq!(cli_args1.disable_color, cli_args2.disable_color);
    assert_eq!(cli_args1.cwd, cli_args2.cwd);
    assert_eq!(cli_args1.env, cli_args2.env);
    assert_eq!(cli_args1.env_file, cli_args2.env_file);
    assert_eq!(cli_args1.disable_workspace, cli_args2.disable_workspace);
    assert_eq!(cli_args1.disable_on_error, cli_args2.disable_on_error);
    assert_eq!(cli_args1.allow_private, cli_args2.allow_private);
    assert_eq!(cli_args1.skip_init_end_tasks, cli_args2.skip_init_end_tasks);
    assert_eq!(cli_args1.skip_tasks_pattern, cli_args2.skip_tasks_pattern);
    assert_eq!(
        cli_args1.disable_check_for_updates,
        cli_args2.disable_check_for_updates
    );
    assert_eq!(cli_args1.print_only, cli_args2.print_only);
    assert_eq!(cli_args1.list_all_steps, cli_args2.list_all_steps);
    assert_eq!(cli_args1.diff_execution_plan, cli_args2.diff_execution_plan);
    assert_eq!(cli_args1.experimental, cli_args2.experimental);
    assert_eq!(cli_args1.arguments, cli_args2.arguments);
    assert_eq!(cli_args1.output_format, cli_args2.output_format);
    assert_eq!(cli_args1.output_file, cli_args2.output_file);
    assert_eq!(cli_args1.print_time_summary, cli_args2.print_time_summary);
}

#[test]
fn parse_args_makers() {
    let global_config = GlobalConfig::new();
    let cli_args = parse_args(&global_config, "makers", false, Some(vec!["makers"]));

    let expected = default_parsed_cli_args();

    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_cargo_make() {
    let global_config = GlobalConfig::new();
    let cli_args = parse_args(&global_config, "make", true, Some(vec!["cargo", "make"]));

    let mut expected = default_parsed_cli_args();
    expected.command = "cargo make".to_string();

    assert_cli_args(&cli_args, &expected);
}

#[test]
#[should_panic]
fn parse_args_help_long() {
    default_parse_cli_args(vec!["--help"]);
}

#[test]
#[should_panic]
fn parse_args_help_short() {
    default_parse_cli_args(vec!["-h"]);
}

#[test]
#[should_panic]
fn parse_args_version_long() {
    default_parse_cli_args(vec!["--version"]);
}

#[test]
#[should_panic]
fn parse_args_version_short() {
    default_parse_cli_args(vec!["-V"]);
}

#[test]
fn parse_args_makefile() {
    let mut cli_args = default_parse_cli_args(vec!["--makefile", "./mymakefile.toml"]);

    let mut expected = default_parsed_cli_args();
    expected.build_file = Some("./mymakefile.toml".to_string());

    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["--makefile", "./mymakefile.toml", "taskname"]);
    expected.task = "taskname".to_string();
    expected.arguments = Some(vec![]);
    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_task() {
    let mut cli_args = default_parse_cli_args(vec!["--task", "sometask"]);

    let mut expected = default_parsed_cli_args();
    expected.task = "sometask".to_string();

    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["-t", "sometask"]);
    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["sometask"]);
    expected.arguments = Some(vec![]);
    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_profile() {
    let mut cli_args = default_parse_cli_args(vec!["--profile", "someprofile"]);

    let mut expected = default_parsed_cli_args();
    expected.profile = Some("someprofile".to_string());

    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["-p", "someprofile"]);
    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["--profile", "someprofile", "taskname"]);
    expected.task = "taskname".to_string();
    expected.arguments = Some(vec![]);
    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_cwd() {
    let cli_args = default_parse_cli_args(vec!["--cwd", "./mydir/subdir/"]);

    let mut expected = default_parsed_cli_args();
    expected.cwd = Some("./mydir/subdir/".to_string());

    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_no_workspace() {
    let cli_args = default_parse_cli_args(vec!["--no-workspace"]);

    let mut expected = default_parsed_cli_args();
    expected.disable_workspace = true;

    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_allow_private() {
    let cli_args = default_parse_cli_args(vec!["--allow-private"]);

    let mut expected = default_parsed_cli_args();
    expected.allow_private = true;

    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_skip_init_end_tasks() {
    let cli_args = default_parse_cli_args(vec!["--skip-init-end-tasks"]);

    let mut expected = default_parsed_cli_args();
    expected.skip_init_end_tasks = true;

    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_skip_tasks() {
    let mut cli_args = default_parse_cli_args(vec!["--skip-tasks", "pre-.*"]);

    let mut expected = default_parsed_cli_args();
    expected.skip_tasks_pattern = Some("pre-.*".to_string());

    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["--skip-tasks", "pre-.*", "taskname"]);
    expected.task = "taskname".to_string();
    expected.arguments = Some(vec![]);
    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_env_file() {
    let mut cli_args = default_parse_cli_args(vec!["--env-file", "./.env"]);

    let mut expected = default_parsed_cli_args();
    expected.env_file = Some("./.env".to_string());

    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["--env-file=./.env"]);
    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["--env-file", "./.env", "taskname"]);
    expected.task = "taskname".to_string();
    expected.arguments = Some(vec![]);
    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["--env-file=./.env", "taskname"]);
    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_env() {
    let mut cli_args = default_parse_cli_args(vec!["--env", "K=V"]);

    let mut expected = default_parsed_cli_args();
    expected.env = Some(vec!["K=V".to_string()]);

    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["-e", "K=V"]);
    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["-e", "K1=V1", "-e", "K2=V2", "--env", "K3=V3"]);
    expected.env = Some(vec![
        "K1=V1".to_string(),
        "K2=V2".to_string(),
        "K3=V3".to_string(),
    ]);
    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec![
        "-e", "K1=V1", "-e", "K2=V2", "--env", "K3=V3", "taskname",
    ]);
    expected.task = "taskname".to_string();
    expected.arguments = Some(vec![]);
    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_loglevel() {
    let mut cli_args = default_parse_cli_args(vec!["--loglevel", "verbose"]);

    let mut expected = default_parsed_cli_args();
    expected.log_level = "verbose".to_string();

    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["-l", "verbose"]);
    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["--loglevel", "verbose", "taskname"]);
    expected.task = "taskname".to_string();
    expected.arguments = Some(vec![]);
    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_verbose() {
    let mut cli_args = default_parse_cli_args(vec!["--verbose"]);

    let mut expected = default_parsed_cli_args();
    expected.log_level = "verbose".to_string();

    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["-v"]);
    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_no_color() {
    let cli_args = default_parse_cli_args(vec!["--no-color"]);

    let mut expected = default_parsed_cli_args();
    expected.disable_color = true;

    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_time_summary() {
    let cli_args = default_parse_cli_args(vec!["--time-summary"]);

    let mut expected = default_parsed_cli_args();
    expected.print_time_summary = true;

    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_experimental() {
    let cli_args = default_parse_cli_args(vec!["--experimental"]);

    let mut expected = default_parsed_cli_args();
    expected.experimental = true;

    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_disable_check_for_updates() {
    let cli_args = default_parse_cli_args(vec!["--disable-check-for-updates"]);

    let mut expected = default_parsed_cli_args();
    expected.disable_check_for_updates = true;

    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_output_format() {
    let mut cli_args = default_parse_cli_args(vec!["--output-format", "autocomplete"]);

    let mut expected = default_parsed_cli_args();
    expected.output_format = "autocomplete".to_string();

    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["--output-format", "autocomplete", "taskname"]);
    expected.task = "taskname".to_string();
    expected.arguments = Some(vec![]);
    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_output_file() {
    let mut cli_args = default_parse_cli_args(vec!["--output-file", "./out"]);

    let mut expected = default_parsed_cli_args();
    expected.output_file = Some("./out".to_string());

    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["--output-file", "./out", "taskname"]);
    expected.task = "taskname".to_string();
    expected.arguments = Some(vec![]);
    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_print_steps() {
    let cli_args = default_parse_cli_args(vec!["--print-steps"]);

    let mut expected = default_parsed_cli_args();
    expected.print_only = true;

    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_list_all_steps() {
    let cli_args = default_parse_cli_args(vec!["--list-all-steps"]);

    let mut expected = default_parsed_cli_args();
    expected.list_all_steps = true;

    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_list_category_steps() {
    let mut cli_args = default_parse_cli_args(vec!["--list-category-steps", "build"]);

    let mut expected = default_parsed_cli_args();
    expected.list_category_steps = Some("build".to_string());

    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["--list-category-steps", "build", "taskname"]);
    expected.task = "taskname".to_string();
    expected.arguments = Some(vec![]);
    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_diff_steps() {
    let cli_args = default_parse_cli_args(vec!["--diff-steps"]);

    let mut expected = default_parsed_cli_args();
    expected.diff_execution_plan = true;

    assert_cli_args(&cli_args, &expected);
}

#[test]
fn parse_args_task_cmd() {
    let mut cli_args = default_parse_cli_args(vec!["task1"]);

    let mut expected = default_parsed_cli_args();
    expected.task = "task1".to_string();
    expected.arguments = Some(vec![]);

    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["--", "task1"]);
    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["task1", "arg1", "arg2"]);
    expected.arguments = Some(vec!["arg1".to_string(), "arg2".to_string()]);
    assert_cli_args(&cli_args, &expected);

    cli_args = default_parse_cli_args(vec!["--", "task1", "arg1", "arg2"]);
    assert_cli_args(&cli_args, &expected);
}
