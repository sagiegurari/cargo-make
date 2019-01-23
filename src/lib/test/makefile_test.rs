use crate::condition;
use crate::descriptor;
use crate::runner;
use crate::scriptengine;
use crate::scriptengine::EngineType;
use crate::types::{Config, CrateInfo, EnvInfo, FlowInfo, GitInfo, RunTaskInfo, Step, Task};
use ci_info;
use rust_info::types::RustInfo;

fn load_descriptor() -> Config {
    descriptor::load_internal_descriptors(true, false)
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

#[test]
#[cfg(target_os = "linux")]
fn makefile_coverage_test() {
    let config = load_descriptor();
    let task = get_task("coverage", &config);
    let run_task_info = task.run_task.unwrap();

    match run_task_info {
        RunTaskInfo::Routing(ref routing_info) => {
            let flow_info = create_flow_info(&config);
            let task_name = runner::get_sub_task_name_for_routing_info(&flow_info, routing_info);
            assert_eq!(task_name.unwrap(), "coverage-kcov");
        }
        _ => panic!("makefile error"),
    };
}

#[test]
#[cfg(target_os = "linux")]
fn makefile_ci_coverage_flow_test() {
    let config = load_descriptor();
    let task = get_task("ci-coverage-flow", &config);
    let flow_info = create_flow_info(&config);
    let step = Step {
        name: "ci-coverage-flow".to_string(),
        config: task,
    };

    let enabled = condition::validate_condition_for_step(&flow_info, &step);

    assert_eq!(flow_info.env_info.ci_info.ci, enabled);
}

#[test]
fn makefile_codecov_test() {
    let config = load_descriptor();
    let task = get_task("codecov", &config);

    let output = scriptengine::get_engine_type(&task);

    assert_eq!(output, EngineType::OS);
}
