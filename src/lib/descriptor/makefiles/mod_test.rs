use crate::condition;
use crate::descriptor;
use crate::runner;
use crate::scriptengine;
use crate::scriptengine::EngineType;
use crate::test;
use crate::types::{Config, CrateInfo, EnvInfo, FlowInfo, FlowState, RunTaskInfo, Step, Task};
use ci_info;
use envmnt;
use fsio;
use git_info::types::GitInfo;
use rust_info::types::RustInfo;
use std::cell::RefCell;
use std::rc::Rc;
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
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    }
}

fn makefile_task_condition_test(name: &str, expect_enabled: bool, linux_only: bool, ci_only: bool) {
    if !linux_only || test::is_linux() {
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

    let output = scriptengine::get_engine_type(
        &task.script.unwrap(),
        &task.script_runner,
        &task.script_extension,
    );

    assert_eq!(output, engine);
}

#[test]
fn makefile_coverage_test() {
    if test::is_linux() {
        let config = load_descriptor();
        let task = get_task("coverage", &config);
        let run_task_info = task.run_task.unwrap();

        match run_task_info {
            RunTaskInfo::Routing(ref routing_info) => {
                let flow_info = create_flow_info(&config);
                let (task_name, fork, parallel, cleanup_task) =
                    runner::get_sub_task_info_for_routing_info(&flow_info, routing_info);
                let names = task_name.unwrap();
                assert_eq!(names.len(), 1);
                assert_eq!(names[0], "coverage-kcov");
                assert!(!fork);
                assert!(!parallel);
                assert!(cleanup_task.is_none());
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
    let enabled = if test::is_windows() { false } else { true };

    makefile_task_condition_test("codecov", enabled, false, false);
}

#[test]
fn makefile_coverage_kcov_test() {
    makefile_task_enabled_test("coverage-kcov", true, false);
}

#[test]
fn makefile_copy_apidocs_test() {
    makefile_task_script_engine_test("do-copy-apidocs", EngineType::Duckscript);
}

#[test]
fn makefile_do_on_members_test() {
    makefile_task_script_engine_test("do-on-members", EngineType::OS);
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

#[test]
fn makefile_build_file_increment_test() {
    makefile_task_disabled_test("build-file-increment", false);
}

#[test]
#[ignore]
fn makefile_build_file_increment_no_file_test() {
    let mut file = test::get_temp_test_directory();
    file.push("build_file_increment_test_no_file");
    fsio::directory::delete(&file).unwrap();
    file.push("buildnumber.txt");

    envmnt::set("CARGO_MAKE_BUILD_NUMBER_FILE", &file);

    let name = "build-file-increment";

    let config = load_descriptor();
    let task = get_task(name, &config);

    let flow_info = create_flow_info(&config);
    let step = Step {
        name: name.to_string(),
        config: task,
    };

    runner::run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);

    envmnt::remove("CARGO_MAKE_BUILD_NUMBER_FILE");

    let text = fsio::file::read_text_file(&file).unwrap();
    assert_eq!(text, "1");
    let number = envmnt::get_or_panic("CARGO_MAKE_BUILD_NUMBER");
    assert_eq!(number, "1");
}

#[test]
#[ignore]
fn makefile_build_file_increment_file_exists_test() {
    let mut file = test::get_temp_test_directory();
    file.push("build_file_increment_test_file_exists");
    fsio::directory::delete(&file).unwrap();
    file.push("buildnumber.txt");

    envmnt::set("CARGO_MAKE_BUILD_NUMBER_FILE", &file);
    envmnt::remove("CARGO_MAKE_BUILD_NUMBER");

    let name = "build-file-increment";

    let config = load_descriptor();
    let task = get_task(name, &config);

    let flow_info = create_flow_info(&config);
    let step = Step {
        name: name.to_string(),
        config: task,
    };

    runner::run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
    runner::run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
    runner::run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);

    envmnt::remove("CARGO_MAKE_BUILD_NUMBER_FILE");

    let text = fsio::file::read_text_file(&file).unwrap();
    assert_eq!(text, "3");
    let number = envmnt::get_or_panic("CARGO_MAKE_BUILD_NUMBER");
    assert_eq!(number, "3");
}

#[test]
#[ignore]
#[should_panic]
fn makefile_build_file_increment_panic_invalid_data_test() {
    let mut file = test::get_temp_test_directory();
    file.push("build_file_increment_test_invalid_data");
    fsio::directory::delete(&file).unwrap();
    file.push("buildnumber.txt");
    fsio::file::write_text_file(&file, "abc").unwrap();

    envmnt::set("CARGO_MAKE_BUILD_NUMBER_FILE", &file);
    envmnt::remove("CARGO_MAKE_BUILD_NUMBER");

    let name = "build-file-increment";

    let config = load_descriptor();
    let task = get_task(name, &config);

    let flow_info = create_flow_info(&config);
    let step = Step {
        name: name.to_string(),
        config: task,
    };

    runner::run_task(&flow_info, Rc::new(RefCell::new(FlowState::new())), &step);
}
