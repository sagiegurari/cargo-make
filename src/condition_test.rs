use super::*;
use log;
use std::collections::HashMap;
use types::{Config, ConfigSection, CrateInfo, EnvInfo, FlowInfo, GitInfo, RustChannel, RustInfo, Step, Task, TaskCondition};

#[test]
fn validate_script_empty() {
    let logger = log::create("error");

    let task = Task::new();
    let step = Step { name: "test".to_string(), config: task };

    let enabled = validate_script(&logger, &step);

    assert!(enabled);
}

#[test]
fn validate_script_valid() {
    let logger = log::create("error");

    let mut task = Task::new();
    task.condition_script = Some(vec!["exit 0".to_string()]);
    let step = Step { name: "test".to_string(), config: task };

    let enabled = validate_script(&logger, &step);

    assert!(enabled);
}

#[test]
fn validate_script_invalid() {
    let logger = log::create("error");

    let mut task = Task::new();
    task.condition_script = Some(vec!["exit 1".to_string()]);
    let step = Step { name: "test".to_string(), config: task };

    let enabled = validate_script(&logger, &step);

    assert!(!enabled);
}

#[test]
fn valdiate_criteria_empty() {
    let logger = log::create("error");
    let mut step = Step { name: "test".to_string(), config: Task::new() };

    let config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo { rust_info: RustInfo::new(), crate_info: CrateInfo::new(), git_info: GitInfo::new() },
        disable_workspace: false
    };

    step.config.condition = Some(TaskCondition { platforms: None, channels: None });

    let enabled = valdiate_criteria(&logger, &flow_info, &step);

    assert!(enabled);
}

#[test]
fn valdiate_criteria_valid_platform() {
    let logger = log::create("error");
    let mut step = Step { name: "test".to_string(), config: Task::new() };

    let config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo { rust_info: RustInfo::new(), crate_info: CrateInfo::new(), git_info: GitInfo::new() },
        disable_workspace: false
    };

    step.config.condition = Some(TaskCondition {
        platforms: Some(vec!["bad1".to_string(), types::get_platform_name(), "bad2".to_string()]),
        channels: None
    });

    let enabled = valdiate_criteria(&logger, &flow_info, &step);

    assert!(enabled);
}

#[test]
fn valdiate_criteria_invalid_platform() {
    let logger = log::create("error");
    let mut step = Step { name: "test".to_string(), config: Task::new() };

    let config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo { rust_info: RustInfo::new(), crate_info: CrateInfo::new(), git_info: GitInfo::new() },
        disable_workspace: false
    };

    step.config.condition = Some(TaskCondition { platforms: Some(vec!["bad1".to_string(), "bad2".to_string()]), channels: None });

    let enabled = valdiate_criteria(&logger, &flow_info, &step);

    assert!(!enabled);
}

#[test]
fn valdiate_criteria_valid_channel() {
    let logger = log::create("error");
    let mut step = Step { name: "test".to_string(), config: Task::new() };

    let config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo { rust_info: RustInfo::new(), crate_info: CrateInfo::new(), git_info: GitInfo::new() },
        disable_workspace: false
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    step.config.condition = Some(TaskCondition { platforms: None, channels: Some(vec!["bad1".to_string(), "stable".to_string(), "bad2".to_string()]) });
    let mut enabled = valdiate_criteria(&logger, &flow_info, &step);

    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Beta);
    step.config.condition = Some(TaskCondition { platforms: None, channels: Some(vec!["bad1".to_string(), "beta".to_string(), "bad2".to_string()]) });
    enabled = valdiate_criteria(&logger, &flow_info, &step);

    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Nightly);
    step.config.condition = Some(TaskCondition { platforms: None, channels: Some(vec!["bad1".to_string(), "nightly".to_string(), "bad2".to_string()]) });
    enabled = valdiate_criteria(&logger, &flow_info, &step);

    assert!(enabled);
}

#[test]
fn valdiate_criteria_invalid_channel() {
    let logger = log::create("error");
    let mut step = Step { name: "test".to_string(), config: Task::new() };

    let config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo { rust_info: RustInfo::new(), crate_info: CrateInfo::new(), git_info: GitInfo::new() },
        disable_workspace: false
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    step.config.condition = Some(TaskCondition { platforms: None, channels: Some(vec!["bad1".to_string(), "bad2".to_string()]) });
    let enabled = valdiate_criteria(&logger, &flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_condition_both_valid() {
    let logger = log::create("error");
    let mut step = Step { name: "test".to_string(), config: Task::new() };

    let config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo { rust_info: RustInfo::new(), crate_info: CrateInfo::new(), git_info: GitInfo::new() },
        disable_workspace: false
    };

    step.config.condition = Some(TaskCondition {
        platforms: Some(vec!["bad1".to_string(), types::get_platform_name(), "bad2".to_string()]),
        channels: None
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&logger, &flow_info, &step);

    assert!(enabled);
}

#[test]
fn valdiate_criteria_valid_script_invalid() {
    let logger = log::create("error");
    let mut step = Step { name: "test".to_string(), config: Task::new() };

    let config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo { rust_info: RustInfo::new(), crate_info: CrateInfo::new(), git_info: GitInfo::new() },
        disable_workspace: false
    };

    step.config.condition = Some(TaskCondition {
        platforms: Some(vec!["bad1".to_string(), types::get_platform_name(), "bad2".to_string()]),
        channels: None
    });
    step.config.condition_script = Some(vec!["exit 1".to_string()]);

    let enabled = validate_condition(&logger, &flow_info, &step);

    assert!(!enabled);
}

#[test]
fn valdiate_criteria_invalid_script_valid() {
    let logger = log::create("error");
    let mut step = Step { name: "test".to_string(), config: Task::new() };

    let config = Config { config: ConfigSection::new(), env: HashMap::new(), tasks: HashMap::new() };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo { rust_info: RustInfo::new(), crate_info: CrateInfo::new(), git_info: GitInfo::new() },
        disable_workspace: false
    };

    step.config.condition = Some(TaskCondition { platforms: Some(vec!["bad1".to_string(), "bad2".to_string()]), channels: None });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&logger, &flow_info, &step);

    assert!(!enabled);
}
