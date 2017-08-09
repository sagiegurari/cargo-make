//! # types
//!
//! Defines the various types and aliases used by cargo-make.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

use log::Logger;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use toml;

/// Returns the platform name
pub fn get_platform_name() -> String {
    if cfg!(windows) {
        "windows".to_string()
    } else if cfg!(target_os = "macos") || cfg!(target_os = "ios") {
        "mac".to_string()
    } else {
        "linux".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds CLI args
pub struct CliArgs {
    /// The external Makefile.toml path
    pub build_file: String,
    /// The task to invoke
    pub task: String,
    /// Log level name
    pub log_level: String,
    /// Current working directory
    pub cwd: Option<String>,
    /// Environment variables
    pub env: Option<Vec<String>>,
    /// Prevent workspace support
    pub disable_workspace: bool,
    /// Only print the execution plan
    pub print_only: bool,
    /// List all known steps
    pub list_all_steps: bool,
    /// Allows access unsupported experimental predefined tasks
    pub experimental: bool
}

impl CliArgs {
    /// Creates and returns a new instance.
    pub fn new() -> CliArgs {
        CliArgs {
            build_file: "Makefile.toml".to_string(),
            task: "default".to_string(),
            log_level: "info".to_string(),
            cwd: None,
            env: None,
            disable_workspace: false,
            print_only: false,
            list_all_steps: false,
            experimental: false
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Copy)]
/// Rust channel type
pub enum RustChannel {
    /// Rust stable channel
    Stable,
    /// Rust beta channel
    Beta,
    /// Rust nightly channel
    Nightly
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds rust info for the current runtime
pub struct RustInfo {
    /// version
    pub version: Option<String>,
    /// channel
    pub channel: Option<RustChannel>,
    /// target arch cfg value
    pub target_arch: Option<String>,
    /// target env cfg value
    pub target_env: Option<String>,
    /// target OS cfg value
    pub target_os: Option<String>,
    /// target pointer width cfg value
    pub target_pointer_width: Option<String>,
    /// target vendor cfg value
    pub target_vendor: Option<String>
}

impl RustInfo {
    /// Returns new instasnce
    pub fn new() -> RustInfo {
        RustInfo {
            version: None,
            channel: None,
            target_arch: None,
            target_env: None,
            target_os: None,
            target_pointer_width: None,
            target_vendor: None
        }
    }
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
    /// Returns new instasnce
    pub fn new() -> GitInfo {
        GitInfo { branch: None, user_name: None, user_email: None }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds crate workspace info.
pub struct Workspace {
    /// members paths
    pub members: Option<Vec<String>>
}

impl Workspace {
    /// Creates and returns a new instance.
    pub fn new() -> Workspace {
        Workspace { members: None }
    }
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
    /// Creates and returns a new instance.
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
    pub package: Option<PackageInfo>,
    /// workspace info
    pub workspace: Option<Workspace>
}

impl CrateInfo {
    /// Creates and returns a new instance.
    pub fn new() -> CrateInfo {
        CrateInfo { package: None, workspace: None }
    }

    /// Loads the crate info based on the Cargo.toml found in the current working directory.
    ///
    /// # Arguments
    ///
    /// * `logger` - Logger instance
    pub fn load(logger: &Logger) -> CrateInfo {
        // load crate info
        let file_path = Path::new("Cargo.toml");

        if file_path.exists() {
            logger.verbose("Opening file:", &[], Some(&file_path));
            let mut file = match File::open(&file_path) {
                Ok(value) => value,
                Err(error) => panic!("Unable to open Cargo.toml, error: {}", error),
            };
            let mut crate_info_string = String::new();
            file.read_to_string(&mut crate_info_string).unwrap();

            let crate_info: CrateInfo = match toml::from_str(&crate_info_string) {
                Ok(value) => value,
                Err(error) => panic!("Unable to parse Cargo.toml, {}", error),
            };
            logger.verbose("Loaded Cargo.toml:", &[], Some(&crate_info));

            crate_info
        } else {
            CrateInfo::new()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds env information
pub struct EnvInfo {
    /// Rust info
    pub rust_info: RustInfo,
    /// Crate info
    pub crate_info: CrateInfo,
    /// Git info
    pub git_info: GitInfo
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds flow information
pub struct FlowInfo {
    /// The flow config object
    pub config: Config,
    /// The main task of the flow
    pub task: String,
    /// The env info
    pub env_info: EnvInfo,
    /// Prevent workspace support
    pub disable_workspace: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds condition attributes
pub struct TaskCondition {
    /// Platform names (linux, windows, mac)
    pub platforms: Option<Vec<String>>,
    /// Channel names (stable, beta, nightly)
    pub channels: Option<Vec<String>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds a single task configuration such as command and dependencies list
pub struct Task {
    /// Task description
    pub description: Option<String>,
    /// if true, the command/script of this task will not be invoked, dependencies however will be
    pub disabled: Option<bool>,
    /// if provided all condition values must be met in order for the task to be invoked (will not stop dependencies)
    pub condition: Option<TaskCondition>,
    /// if script exit code is not 0, the command/script of this task will not be invoked, dependencies however will be
    pub condition_script: Option<Vec<String>>,
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
    /// The script runner (defaults to cmd in windows and sh for other platforms)
    pub script_runner: Option<String>,
    /// The task name to execute
    pub run_task: Option<String>,
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
    /// Creates and returns a new instance.
    pub fn new() -> Task {
        Task {
            description: None,
            disabled: None,
            condition: None,
            condition_script: None,
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
            script_runner: None,
            run_task: None,
            dependencies: None,
            linux: None,
            windows: None,
            mac: None
        }
    }

    /// Copies values from the task into self.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to copy from
    pub fn extend(
        self: &mut Task,
        task: &Task,
    ) {
        if task.description.is_some() {
            self.description = task.description.clone();
        }

        if task.disabled.is_some() {
            self.disabled = task.disabled.clone();
        }

        if task.condition.is_some() {
            self.condition = task.condition.clone();
        }

        if task.condition_script.is_some() {
            self.condition_script = task.condition_script.clone();
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

        if task.script_runner.is_some() {
            self.script_runner = task.script_runner.clone();
        }

        if task.run_task.is_some() {
            self.run_task = task.run_task.clone();
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

    /// Returns true if the task force attribute is defined and true
    pub fn is_force(self: &Task) -> bool {
        self.force.unwrap_or(false)
    }

    /// Returns the override task definition based on the current platform.
    fn get_override(self: &Task) -> Option<PlatformOverrideTask> {
        let platform_name = get_platform_name();
        if platform_name == "windows" {
            match self.windows {
                Some(ref value) => Some(value.clone()),
                _ => None,
            }
        } else if platform_name == "mac" {
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

    /// Returns a new task based on the override information and current platform.
    pub fn get_normalized_task(self: &mut Task) -> Task {
        match self.get_override() {
            Some(ref mut override_task) => {
                override_task.extend(self);

                Task {
                    description: self.description.clone(),
                    disabled: override_task.disabled.clone(),
                    condition: override_task.condition.clone(),
                    condition_script: override_task.condition_script.clone(),
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
                    script_runner: override_task.script_runner.clone(),
                    run_task: override_task.run_task.clone(),
                    dependencies: override_task.dependencies.clone(),
                    linux: None,
                    windows: None,
                    mac: None
                }
            }
            None => self.clone(),
        }
    }

    /// Returns the alias value based on the current platform and task definition.
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
    /// if provided all condition values must be met in order for the task to be invoked (will not stop dependencies)
    pub condition: Option<TaskCondition>,
    /// if script exit code is not 0, the command/script of this task will not be invoked, dependencies however will be
    pub condition_script: Option<Vec<String>>,
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
    /// The script runner (defaults to cmd in windows and sh for other platforms)
    pub script_runner: Option<String>,
    /// The task name to execute
    pub run_task: Option<String>,
    /// A list of tasks to execute before this task
    pub dependencies: Option<Vec<String>>
}

impl PlatformOverrideTask {
    /// Copies values from the task into self.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to copy from
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

            if self.condition.is_none() && task.condition.is_some() {
                self.condition = task.condition.clone();
            }

            if self.condition_script.is_none() && task.condition_script.is_some() {
                self.condition_script = task.condition_script.clone();
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

            if self.script_runner.is_none() && task.script_runner.is_some() {
                self.script_runner = task.script_runner.clone();
            }

            if self.run_task.is_none() && task.run_task.is_some() {
                self.run_task = task.run_task.clone();
            }

            if self.dependencies.is_none() && task.dependencies.is_some() {
                self.dependencies = task.dependencies.clone();
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds the configuration found in the makefile toml config section.
pub struct ConfigSection {
    /// Init task name which will be invoked at the start of every run
    pub init_task: Option<String>,
    /// End task name which will be invoked at the end of every run
    pub end_task: Option<String>
}

impl ConfigSection {
    /// Creates and returns a new instance.
    pub fn new() -> ConfigSection {
        ConfigSection { init_task: None, end_task: None }
    }

    /// Copies values from the config section into self.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to copy from
    pub fn extend(
        self: &mut ConfigSection,
        extended: &mut ConfigSection,
    ) {
        if extended.init_task.is_some() {
            self.init_task = extended.init_task.clone();
        }

        if extended.end_task.is_some() {
            self.end_task = extended.end_task.clone();
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds the entire configuration such as task definitions and env vars
pub struct Config {
    /// Runtime config
    pub config: ConfigSection,
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
    /// Runtime config
    pub config: Option<ConfigSection>,
    /// The env vars to setup before running the tasks
    pub env: Option<HashMap<String, String>>,
    /// All task definitions
    pub tasks: Option<HashMap<String, Task>>
}

impl ExternalConfig {
    /// Creates and returns a new instance.
    pub fn new() -> ExternalConfig {
        ExternalConfig { extend: None, config: None, env: None, tasks: None }
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
