//! # types
//!
//! Defines the various types and aliases used by cargo-make.
//!

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds a single task configuration such as command and dependencies list
pub struct Task {
    /// if true, the command/script of this task will not be invoked, depedencies however will be
    pub disabled: Option<bool>,
    /// if defined, task points to another task and all other properties are ignored
    pub alias: Option<String>,
    /// if defined, the provided crate will be installed (if needed) before running the task
    pub install_crate: Option<String>,
    /// if defined, the provided script will be executed before running the task
    pub install_script: Option<Vec<String>>,
    /// The command to execute
    pub command: Option<String>,
    /// The command args
    pub args: Option<Vec<String>>,
    /// If command is not defined, and script is defined, the provided script will be executed
    pub script: Option<Vec<String>>,
    /// A list of tasks to execute before this task
    pub dependencies: Option<Vec<String>>
}

impl Task {
    pub fn extend(
        self: &mut Task,
        task: &Task,
    ) {
        if task.disabled.is_some() {
            self.disabled = task.disabled.clone();
        }

        if task.alias.is_some() {
            self.alias = task.alias.clone();
        }

        if task.install_crate.is_some() {
            self.install_crate = task.install_crate.clone();
        }

        if task.install_script.is_some() {
            self.install_script = task.install_script.clone();
        }

        if task.command.is_some() {
            self.command = task.command.clone();
        }

        if task.args.is_some() {
            self.args = task.args.clone();
        }

        if task.script.is_some() {
            self.script = task.script.clone();
        }

        if task.dependencies.is_some() {
            self.dependencies = task.dependencies.clone();
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
/// Holds the entire configuration such as task definitions and env vars
pub struct Config {
    /// The env vars to setup before running the tasks
    pub env: HashMap<String, String>,
    /// All task definitions
    pub tasks: HashMap<String, Task>
}

#[derive(Serialize, Deserialize, Debug)]
/// Same as the config struct but all memebers are optional
pub struct ExternalConfig {
    /// The env vars to setup before running the tasks
    pub env: Option<HashMap<String, String>>,
    /// All task definitions
    pub tasks: Option<HashMap<String, Task>>
}

#[derive(Debug)]
/// Execution plan step to execute
pub struct Step {
    /// The task name
    pub name: String,
    /// The task config
    pub config: Task
}

#[derive(Debug)]
/// Execution plan which defines all steps to run and the order to run them
pub struct ExecutionPlan {
    /// A list of steps to execute
    pub steps: Vec<Step>
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extend_both_have_misc_data() {
        let mut base = Task {
            install_crate: Some("my crate1".to_string()),
            command: Some("test1".to_string()),
            disabled: Some(false),
            alias: None,
            install_script: None,
            args: None,
            script: Some(vec!["1".to_string(), "2".to_string()]),
            dependencies: None
        };
        let extended = Task {
            install_crate: Some("my crate2".to_string()),
            command: None,
            disabled: Some(true),
            alias: Some("alias2".to_string()),
            install_script: None,
            args: None,
            script: None,
            dependencies: None
        };

        base.extend(&extended);

        assert!(base.install_crate.is_some());
        assert!(base.command.is_some());
        assert!(base.disabled.is_some());
        assert!(base.alias.is_some());
        assert!(base.install_script.is_none());
        assert!(base.args.is_none());
        assert!(base.script.is_some());
        assert!(base.dependencies.is_none());

        assert_eq!(base.install_crate.unwrap(), "my crate2");
        assert_eq!(base.command.unwrap(), "test1");
        assert!(base.disabled.unwrap());
        assert_eq!(base.alias.unwrap(), "alias2");
        assert_eq!(base.script.unwrap().len(), 2);
    }

    #[test]
    fn extend_extended_have_all_fields() {
        let mut base = Task {
            install_crate: Some("my crate1".to_string()),
            command: Some("test1".to_string()),
            disabled: Some(false),
            alias: None,
            install_script: None,
            args: None,
            script: Some(vec!["1".to_string(), "2".to_string()]),
            dependencies: None
        };
        let extended = Task {
            install_crate: Some("my crate2".to_string()),
            command: Some("test2".to_string()),
            disabled: Some(true),
            alias: Some("alias2".to_string()),
            install_script: Some(vec!["i1".to_string(), "i2".to_string()]),
            args: Some(vec!["a1".to_string(), "a2".to_string()]),
            script: Some(vec!["1".to_string(), "2".to_string(), "3".to_string()]),
            dependencies: Some(vec!["A".to_string()])
        };

        base.extend(&extended);

        assert!(base.install_crate.is_some());
        assert!(base.command.is_some());
        assert!(base.disabled.is_some());
        assert!(base.alias.is_some());
        assert!(base.install_script.is_some());
        assert!(base.args.is_some());
        assert!(base.script.is_some());
        assert!(base.dependencies.is_some());

        assert_eq!(base.install_crate.unwrap(), "my crate2");
        assert_eq!(base.command.unwrap(), "test2");
        assert!(base.disabled.unwrap());
        assert_eq!(base.alias.unwrap(), "alias2");
        assert_eq!(base.install_script.unwrap().len(), 2);
        assert_eq!(base.args.unwrap().len(), 2);
        assert_eq!(base.script.unwrap().len(), 3);
        assert_eq!(base.dependencies.unwrap().len(), 1);
    }
}
