//! # types
//!
//! Defines all the common types.
//!

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub install_crate: Option<String>,
    pub install_script: Option<Vec<String>>,
    pub script: Option<Vec<String>>,
    pub command: Option<String>,
    pub args: Option<Vec<String>>,
    pub depedencies: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub tasks: HashMap<String, Task>
}

#[derive(Debug)]
pub struct Step {
    pub name: String,
    pub config: Task
}

#[derive(Debug)]
pub struct ExecutionPlan {
    pub steps: Vec<Step>
}
