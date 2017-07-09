//! # types
//!
//! Defines the various types and aliases used by cargo-make.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds a single task configuration such as command and dependencies list
pub struct Task {
    /// if true, the command/script of this task will not be invoked, dependencies however will be
    pub disabled: Option<bool>,
    /// if true, any error while executing the task will be printed but will not break the build
    pub force: Option<bool>,
    /// if defined, task points to another task and all other properties are ignored
    pub alias: Option<String>,
    /// acts like alias if runtime OS is Linux (takes precedence over alias)
    pub linux_alias: Option<String>,
    /// acts like alias if runtime OS is Windows (takes precedence over alias)
    pub windows_alias: Option<String>,
    /// acts like alias if runtime OS is Mac (takes precedence over alias)
    pub mac_alias: Option<String>,
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
    pub dependencies: Option<Vec<String>>,
    /// override task if runtime OS is Linux (takes precedence over alias)
    pub linux: Option<PlatformOverrideTask>,
    /// override task if runtime OS is Windows (takes precedence over alias)
    pub windows: Option<PlatformOverrideTask>,
    /// override task if runtime OS is Mac (takes precedence over alias)
    pub mac: Option<PlatformOverrideTask>
}

impl Task {
    pub fn new() -> Task {
        Task {
            disabled: None,
            force: None,
            alias: None,
            linux_alias: None,
            windows_alias: None,
            mac_alias: None,
            install_crate: None,
            install_script: None,
            command: None,
            args: None,
            script: None,
            dependencies: None,
            linux: None,
            windows: None,
            mac: None
        }
    }

    pub fn extend(
        self: &mut Task,
        task: &Task,
    ) {
        if task.disabled.is_some() {
            self.disabled = task.disabled.clone();
        }

        if task.force.is_some() {
            self.force = task.force.clone();
        }

        if task.alias.is_some() {
            self.alias = task.alias.clone();
        }

        if task.linux_alias.is_some() {
            self.linux_alias = task.linux_alias.clone();
        }

        if task.windows_alias.is_some() {
            self.windows_alias = task.windows_alias.clone();
        }

        if task.mac_alias.is_some() {
            self.mac_alias = task.mac_alias.clone();
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

        if task.linux.is_some() {
            self.linux = task.linux.clone();
        }

        if task.windows.is_some() {
            self.windows = task.windows.clone();
        }

        if task.mac.is_some() {
            self.mac = task.mac.clone();
        }
    }

    pub fn is_force(self: &Task) -> bool {
        self.force.unwrap_or(false)
    }

    fn get_override(self: &Task) -> Option<PlatformOverrideTask> {
        if cfg!(windows) {
            match self.windows {
                Some(ref value) => Some(value.clone()),
                _ => None,
            }
        } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
            match self.mac {
                Some(ref value) => Some(value.clone()),
                _ => None,
            }
        } else {
            match self.linux {
                Some(ref value) => Some(value.clone()),
                _ => None,
            }
        }
    }

    pub fn get_normalized_task(self: &mut Task) -> Task {
        match self.get_override() {
            Some(ref mut override_task) => {
                override_task.extend(self);

                Task {
                    disabled: override_task.disabled.clone(),
                    force: override_task.force.clone(),
                    alias: None,
                    linux_alias: None,
                    windows_alias: None,
                    mac_alias: None,
                    install_crate: override_task.install_crate.clone(),
                    install_script: override_task.install_script.clone(),
                    command: override_task.command.clone(),
                    args: override_task.args.clone(),
                    script: override_task.script.clone(),
                    dependencies: override_task.dependencies.clone(),
                    linux: None,
                    windows: None,
                    mac: None
                }
            }
            None => self.clone(),
        }
    }

    pub fn get_alias(self: &Task) -> Option<String> {
        let alias = if cfg!(windows) {
            match self.windows_alias {
                Some(ref value) => Some(value),
                _ => None,
            }
        } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
            match self.mac_alias {
                Some(ref value) => Some(value),
                _ => None,
            }
        } else {
            match self.linux_alias {
                Some(ref value) => Some(value),
                _ => None,
            }
        };

        match alias {
            Some(os_alias) => Some(os_alias.clone()),
            _ => {
                match self.alias {
                    Some(ref alias) => Some(alias.clone()),
                    _ => None,
                }
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds a single task configuration for a specific platform as an override of another task
pub struct PlatformOverrideTask {
    /// if true, it should ignore all data in base task
    pub clear: Option<bool>,
    /// if true, the command/script of this task will not be invoked, dependencies however will be
    pub disabled: Option<bool>,
    /// if true, any error while executing the task will be printed but will not break the build
    pub force: Option<bool>,
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

impl PlatformOverrideTask {
    pub fn extend(
        self: &mut PlatformOverrideTask,
        task: &mut Task,
    ) {
        let copy_values = match self.clear {
            Some(value) => !value,
            None => true,
        };

        if copy_values {
            if self.disabled.is_none() && task.disabled.is_some() {
                self.disabled = task.disabled.clone();
            }

            if self.force.is_none() && task.force.is_some() {
                self.force = task.force.clone();
            }

            if self.install_crate.is_none() && task.install_crate.is_some() {
                self.install_crate = task.install_crate.clone();
            }

            if self.install_script.is_none() && task.install_script.is_some() {
                self.install_script = task.install_script.clone();
            }

            if self.command.is_none() && task.command.is_some() {
                self.command = task.command.clone();
            }

            if self.args.is_none() && task.args.is_some() {
                self.args = task.args.clone();
            }

            if self.script.is_none() && task.script.is_some() {
                self.script = task.script.clone();
            }

            if self.dependencies.is_none() && task.dependencies.is_some() {
                self.dependencies = task.dependencies.clone();
            }
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
/// Holds the entire externally read configuration such as task definitions and env vars where all values are optional
pub struct ExternalConfig {
    /// Path to another toml file to extend
    pub extend: Option<String>,
    /// The env vars to setup before running the tasks
    pub env: Option<HashMap<String, String>>,
    /// All task definitions
    pub tasks: Option<HashMap<String, Task>>
}

impl ExternalConfig {
    pub fn new() -> ExternalConfig {
        ExternalConfig { extend: None, env: None, tasks: None }
    }
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

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds crate package information loaded from the Cargo.toml file package section.
pub struct PackageInfo {
    /// name
    pub name: Option<String>,
    /// version
    pub version: Option<String>,
    /// description
    pub description: Option<String>,
    /// license
    pub license: Option<String>,
    /// documentation link
    pub documentation: Option<String>,
    /// homepage link
    pub homepage: Option<String>,
    /// repository link
    pub repository: Option<String>
}

impl PackageInfo {
    pub fn new() -> PackageInfo {
        PackageInfo {
            name: None,
            version: None,
            description: None,
            license: None,
            documentation: None,
            homepage: None,
            repository: None
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds crate information loaded from the Cargo.toml file.
pub struct CrateInfo {
    /// package info
    pub package: Option<PackageInfo>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds git info for the given repo directory
pub struct GitInfo {
    /// branch name
    pub branch: Option<String>,
    /// user.name
    pub user_name: Option<String>,
    /// user.email
    pub user_email: Option<String>
}

impl GitInfo {
    pub fn new() -> GitInfo {
        GitInfo { branch: None, user_name: None, user_email: None }
    }
}
