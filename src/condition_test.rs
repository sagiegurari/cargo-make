use super::*;
use indexmap::IndexMap;
use rust_info::types::{RustChannel, RustInfo};
use types::{
    Config, ConfigSection, CrateInfo, EnvInfo, FlowInfo, GitInfo, Step, Task, TaskCondition,
};

#[test]
fn validate_env_set_empty() {
    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
    };

    let enabled = validate_env_set(&condition);

    assert!(enabled);
}

#[test]
fn validate_env_set_valid() {
    env::set_var("ENV_SET1", "");
    env::set_var("ENV_SET2", "value");

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: Some(vec!["ENV_SET1".to_string(), "ENV_SET2".to_string()]),
        env_not_set: None,
        env: None,
    };

    let enabled = validate_env_set(&condition);

    assert!(enabled);
}

#[test]
fn validate_env_set_invalid() {
    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: Some(vec!["BAD_ENV_SET1".to_string(), "BAD_ENV_SET2".to_string()]),
        env_not_set: None,
        env: None,
    };

    let enabled = validate_env_set(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_set_invalid_partial_found() {
    env::set_var("ENV_SET1", "");
    env::set_var("ENV_SET2", "value");

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: Some(vec![
            "ENV_SET1".to_string(),
            "ENV_SET2".to_string(),
            "BAD_ENV_SET1".to_string(),
        ]),
        env_not_set: None,
        env: None,
    };

    let enabled = validate_env_set(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_not_set_empty() {
    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
    };

    let enabled = validate_env_not_set(&condition);

    assert!(enabled);
}

#[test]
fn validate_env_not_set_valid() {
    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: Some(vec!["BAD_ENV_SET1".to_string(), "BAD_ENV_SET2".to_string()]),
        env: None,
    };

    let enabled = validate_env_not_set(&condition);

    assert!(enabled);
}

#[test]
fn validate_env_not_set_invalid() {
    env::set_var("ENV_SET1", "");
    env::set_var("ENV_SET2", "value");

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: Some(vec!["ENV_SET1".to_string(), "ENV_SET2".to_string()]),
        env: None,
    };

    let enabled = validate_env_not_set(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_not_set_invalid_partial_found() {
    env::set_var("ENV_SET1", "");
    env::set_var("ENV_SET2", "value");

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: Some(vec![
            "ENV_SET1".to_string(),
            "ENV_SET2".to_string(),
            "BAD_ENV_SET1".to_string(),
        ]),
        env: None,
    };

    let enabled = validate_env_not_set(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_empty() {
    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
    };

    let enabled = validate_env(&condition);

    assert!(enabled);
}

#[test]
fn validate_env_valid() {
    env::set_var("ENV_SET1", "");
    env::set_var("ENV_SET2", "value");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "".to_string());
    env_values.insert("ENV_SET2".to_string(), "value".to_string());

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
    };

    let enabled = validate_env(&condition);

    assert!(enabled);
}

#[test]
fn validate_env_invalid_not_found() {
    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("BAD_ENV_SET1".to_string(), "".to_string());
    env_values.insert("BAD_ENV_SET2".to_string(), "value".to_string());

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
    };

    let enabled = validate_env(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_invalid_not_equal() {
    env::set_var("ENV_SET2", "value");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET2".to_string(), "value2".to_string());

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
    };

    let enabled = validate_env(&condition);

    assert!(!enabled);
}

#[test]
fn validate_env_invalid_partial_found() {
    env::set_var("ENV_SET1", "good");
    env::set_var("ENV_SET2", "good");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "good".to_string());
    env_values.insert("ENV_SET2".to_string(), "bad".to_string());

    let condition = TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
    };

    let enabled = validate_env(&condition);

    assert!(!enabled);
}

#[test]
fn validate_script_empty() {
    let task = Task::new();
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    let enabled = validate_script(&step);

    assert!(enabled);
}

#[test]
fn validate_script_valid() {
    let mut task = Task::new();
    task.condition_script = Some(vec!["exit 0".to_string()]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    let enabled = validate_script(&step);

    assert!(enabled);
}

#[test]
fn validate_script_invalid() {
    let mut task = Task::new();
    task.condition_script = Some(vec!["exit 1".to_string()]);
    let step = Step {
        name: "test".to_string(),
        config: task,
    };

    let enabled = validate_script(&step);

    assert!(!enabled);
}

#[test]
fn validate_platform_valid() {
    let condition = TaskCondition {
        platforms: Some(vec![
            "bad1".to_string(),
            types::get_platform_name(),
            "bad2".to_string(),
        ]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
    };

    let enabled = validate_platform(&condition);

    assert!(enabled);
}

#[test]
fn validate_platform_invalid() {
    let condition = TaskCondition {
        platforms: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
    };

    let enabled = validate_platform(&condition);

    assert!(!enabled);
}

#[test]
fn validate_channel_valid() {
    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    let mut condition = TaskCondition {
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "stable".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env: None,
    };
    let mut enabled = validate_channel(&condition, &flow_info);
    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Beta);
    condition = TaskCondition {
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "beta".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env: None,
    };
    enabled = validate_channel(&condition, &flow_info);

    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Nightly);
    condition = TaskCondition {
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "nightly".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env: None,
    };
    enabled = validate_channel(&condition, &flow_info);

    assert!(enabled);
}

#[test]
fn validate_channel_invalid() {
    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    let condition = TaskCondition {
        platforms: None,
        channels: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        env_set: None,
        env_not_set: None,
        env: None,
    };
    let enabled = validate_channel(&condition, &flow_info);

    assert!(!enabled);
}

#[test]
fn validate_criteria_empty() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
    });

    let enabled = validate_criteria(&flow_info, &step);

    assert!(enabled);
}

#[test]
fn validate_criteria_valid_platform() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    step.config.condition = Some(TaskCondition {
        platforms: Some(vec![
            "bad1".to_string(),
            types::get_platform_name(),
            "bad2".to_string(),
        ]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
    });

    let enabled = validate_criteria(&flow_info, &step);

    assert!(enabled);
}

#[test]
fn validate_criteria_invalid_platform() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    step.config.condition = Some(TaskCondition {
        platforms: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
    });

    let enabled = validate_criteria(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_criteria_valid_channel() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "stable".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env: None,
    });
    let mut enabled = validate_criteria(&flow_info, &step);

    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Beta);
    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "beta".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env: None,
    });
    enabled = validate_criteria(&flow_info, &step);

    assert!(enabled);

    flow_info.env_info.rust_info.channel = Some(RustChannel::Nightly);
    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: Some(vec![
            "bad1".to_string(),
            "nightly".to_string(),
            "bad2".to_string(),
        ]),
        env_set: None,
        env_not_set: None,
        env: None,
    });
    enabled = validate_criteria(&flow_info, &step);

    assert!(enabled);
}

#[test]
fn validate_criteria_invalid_channel() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let mut flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    flow_info.env_info.rust_info.channel = Some(RustChannel::Stable);
    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        env_set: None,
        env_not_set: None,
        env: None,
    });
    let enabled = validate_criteria(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_condition_both_valid() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    step.config.condition = Some(TaskCondition {
        platforms: Some(vec![
            "bad1".to_string(),
            types::get_platform_name(),
            "bad2".to_string(),
        ]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(enabled);
}

#[test]
fn validate_criteria_valid_script_invalid() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    step.config.condition = Some(TaskCondition {
        platforms: Some(vec![
            "bad1".to_string(),
            types::get_platform_name(),
            "bad2".to_string(),
        ]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
    });
    step.config.condition_script = Some(vec!["exit 1".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_criteria_invalid_script_valid() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    step.config.condition = Some(TaskCondition {
        platforms: Some(vec!["bad1".to_string(), "bad2".to_string()]),
        channels: None,
        env_set: None,
        env_not_set: None,
        env: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_criteria_invalid_env_set() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: Some(vec!["BAD_ENV_SET1".to_string()]),
        env_not_set: None,
        env: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_criteria_invalid_env_not_set() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    env::set_var("ENV_SET1", "bad");

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: Some(vec!["ENV_SET1".to_string()]),
        env: None,
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_criteria_valid_env() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    env::set_var("ENV_SET1", "good1");
    env::set_var("ENV_SET2", "good2");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "good1".to_string());
    env_values.insert("ENV_SET2".to_string(), "good2".to_string());

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(enabled);
}

#[test]
fn validate_criteria_invalid_env_not_found() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("BAD_ENV_SET1".to_string(), "good".to_string());
    env_values.insert("BAD_ENV_SET2".to_string(), "bad".to_string());

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(!enabled);
}

#[test]
fn validate_criteria_invalid_env_not_equal() {
    let mut step = Step {
        name: "test".to_string(),
        config: Task::new(),
    };

    let config = Config {
        config: ConfigSection::new(),
        env: IndexMap::new(),
        tasks: IndexMap::new(),
    };
    let flow_info = FlowInfo {
        config,
        task: "test".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
        },
        disable_workspace: false,
        disable_on_error: false,
    };

    env::set_var("ENV_SET1", "good");
    env::set_var("ENV_SET2", "good");

    let mut env_values = IndexMap::<String, String>::new();
    env_values.insert("ENV_SET1".to_string(), "good".to_string());
    env_values.insert("ENV_SET2".to_string(), "bad".to_string());

    step.config.condition = Some(TaskCondition {
        platforms: None,
        channels: None,
        env_set: None,
        env_not_set: None,
        env: Some(env_values),
    });
    step.config.condition_script = Some(vec!["exit 0".to_string()]);

    let enabled = validate_condition(&flow_info, &step);

    assert!(!enabled);
}
