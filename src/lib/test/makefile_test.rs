use super::*;
use crate::condition;
use crate::descriptor;
use crate::runner;
use crate::scriptengine;
use crate::scriptengine::EngineType;
use crate::types::{Config, CrateInfo, EnvInfo, FlowInfo, GitInfo, RunTaskInfo, Step, Task};
use ci_info;
use rust_info::types::RustInfo;

fn load_descriptor() -> Config {
    descriptor::load_internal_descriptors(true, false, None)
}

fn get_task(name: &str, config: &Config) -> Task {
    let task_name = name.to_string();
    let task = config.tasks.get(&task_name).unwrap();

    task.clone()
}

fn create_flow_info(config: &Config) -> FlowInfo {
    FlowInfo {
        config: config.clone(),
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        cli_arguments: None,
    }
}

fn makefile_task_condition_test(name: &str, expect_enabled: bool, linux_only: bool, ci_only: bool) {
    if !linux_only || is_linux() {
        let config = load_descriptor();
        let task = get_task(name, &config);
        let flow_info = create_flow_info(&config);
        let step = Step {
            name: name.to_string(),
            config: task,
        };

        let enabled = condition::validate_condition_for_step(&flow_info, &step);

        let should_be_enabled = if expect_enabled {
            if ci_only {
                flow_info.env_info.ci_info.ci
            } else {
                true
            }
        } else {
            false
        };

        assert_eq!(should_be_enabled, enabled);
    }
}

fn makefile_task_enabled_test(name: &str, linux_only: bool, ci_only: bool) {
    makefile_task_condition_test(name, true, linux_only, ci_only);
}

fn makefile_task_disabled_test(name: &str, linux_only: bool) {
    makefile_task_condition_test(name, false, linux_only, false);
}

fn makefile_task_script_engine_test(name: &str, engine: EngineType) {
    let config = load_descriptor();
    let task = get_task(name, &config);

    let output = scriptengine::get_engine_type(&task);

    assert_eq!(output, engine);
}

#[test]
fn makefile_coverage_test() {
    if is_linux() {
        let config = load_descriptor();
        let task = get_task("coverage", &config);
        let run_task_info = task.run_task.unwrap();

        match run_task_info {
            RunTaskInfo::Routing(ref routing_info) => {
                let flow_info = create_flow_info(&config);
                let task_name =
                    runner::get_sub_task_name_for_routing_info(&flow_info, routing_info);
                assert_eq!(task_name.unwrap(), "coverage-kcov");
            }
            _ => panic!("makefile error"),
        };
    }
}

#[test]
fn makefile_ci_coverage_flow_test() {
    makefile_task_enabled_test("ci-coverage-flow", true, true);
}

#[test]
fn makefile_codecov_test() {
    makefile_task_script_engine_test("codecov", EngineType::OS);
    makefile_task_enabled_test("codecov", false, false);
}

#[test]
fn makefile_coverage_kcov_test() {
    makefile_task_enabled_test("coverage-kcov", true, false);
}

#[test]
fn makefile_copy_apidocs_test() {
    makefile_task_script_engine_test("copy-apidocs", EngineType::Shell2Batch);
}

#[test]
fn makefile_do_on_members_test() {
    makefile_task_script_engine_test("do-on-members", EngineType::Shell2Batch);
    makefile_task_disabled_test("do-on-members", false);
}

#[test]
fn makefile_audit_test() {
    makefile_task_enabled_test("audit", false, false);
}

#[test]
fn makefile_outdated_test() {
    makefile_task_enabled_test("outdated", false, false);
}
