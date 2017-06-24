//! # types
//!
//! Defines the various types and aliases used by cargo-make.
//!

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds a single task configuration such as command and dependencies list
pub struct Task {
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
