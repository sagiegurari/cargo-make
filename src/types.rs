//! # types
//!
//! Defines the various types and aliases used by cargo-make.
//!

#[cfg(test)]
#[path = "./types_test.rs"]
mod types_test;

use ci_info::types::CiInfo;
use indexmap::IndexMap;
use rust_info::types::RustInfo;

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

#[derive(Debug, Clone)]
/// Holds CLI args
pub struct CliArgs {
    /// The command name
    pub command: String,
    /// The external Makefile.toml path
    pub build_file: Option<String>,
    /// The task to invoke
    pub task: String,
    /// Log level name
    pub log_level: String,
    /// Current working directory
    pub cwd: Option<String>,
    /// Environment variables
    pub env: Option<Vec<String>>,
    /// Environment variables file
    pub env_file: Option<String>,
    /// Prevent workspace support
    pub disable_workspace: bool,
    /// Prevent on error flow even if defined in config section
    pub disable_on_error: bool,
    /// Only print the execution plan
    pub print_only: bool,
    /// List all known steps
    pub list_all_steps: bool,
    /// Disables the update check during startup
    pub disable_check_for_updates: bool,
    /// Allows access unsupported experimental predefined tasks
    pub experimental: bool,
    /// additional command line arguments
    pub arguments: Option<Vec<String>>,
    /// Output format
    pub output_format: String,
}

impl CliArgs {
    /// Creates and returns a new instance.
    pub fn new() -> CliArgs {
        CliArgs {
            command: "".to_string(),
            build_file: None,
            task: "default".to_string(),
            log_level: "info".to_string(),
            cwd: None,
            env: None,
            env_file: None,
            disable_workspace: false,
            disable_on_error: false,
            print_only: false,
            list_all_steps: false,
            disable_check_for_updates: false,
            experimental: false,
            arguments: None,
            output_format: "default".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds persisted data used by cargo-make
pub struct Cache {
    /// File from which the cache file was loaded from
    #[serde(skip)]
    pub file_name: Option<String>,
    /// Holds last update check with returned no updates result
    pub last_update_check: Option<u64>,
}

impl Cache {
    /// Returns new instance
    pub fn new() -> Cache {
        Cache {
            file_name: None,
            last_update_check: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds configuration info for cargo-make
pub struct GlobalConfig {
    /// File from which the global config was loaded from
    #[serde(skip)]
    pub file_name: Option<String>,
    /// Default log level
    pub log_level: Option<String>,
    /// Default task name
    pub default_task_name: Option<String>,
    /// Update check minimum time from the previous check (always, daily, weekly, monthly)
    pub update_check_minimum_interval: Option<String>,
    /// True to search for project root in parent directories if current cwd is not a project root
    pub search_project_root: Option<bool>,
}

impl GlobalConfig {
    /// Returns new instance
    pub fn new() -> GlobalConfig {
        GlobalConfig {
            file_name: None,
            log_level: None,
            default_task_name: None,
            update_check_minimum_interval: None,
            search_project_root: Some(false),
        }
    }
}

#[derive(Debug, Clone)]
/// Holds git info for the given repo directory
pub struct GitInfo {
    /// branch name
    pub branch: Option<String>,
    /// user.name
    pub user_name: Option<String>,
    /// user.email
    pub user_email: Option<String>,
}

impl GitInfo {
    /// Returns new instance
    pub fn new() -> GitInfo {
        GitInfo {
            branch: None,
            user_name: None,
            user_email: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Holds crate workspace info, see http://doc.crates.io/manifest.html#the-workspace-section
pub struct Workspace {
    /// members paths
    pub members: Option<Vec<String>>,
    /// exclude paths
    pub exclude: Option<Vec<String>>,
}

impl Workspace {
    /// Creates and returns a new instance.
    pub fn new() -> Workspace {
        Workspace {
            members: None,
            exclude: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
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
    pub repository: Option<String>,
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
            repository: None,
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Holds crate dependency info.
pub struct CrateDependencyInfo {
    /// Holds the dependency path
    pub path: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Holds crate dependency info.
pub enum CrateDependency {
    /// Holds the dependency version
    Version(String),
    /// Hold dependency info
    Info(CrateDependencyInfo),
}

#[derive(Deserialize, Debug, Clone)]
/// Holds crate information loaded from the Cargo.toml file.
pub struct CrateInfo {
    /// package info
    pub package: Option<PackageInfo>,
    /// workspace info
    pub workspace: Option<Workspace>,
    /// crate dependencies
    pub dependencies: Option<IndexMap<String, CrateDependency>>,
}

impl CrateInfo {
    /// Creates and returns a new instance.
    pub fn new() -> CrateInfo {
        CrateInfo {
            package: None,
            workspace: None,
            dependencies: None,
        }
    }
}

#[derive(Debug, Clone)]
/// Holds env information
pub struct EnvInfo {
    /// Rust info
    pub rust_info: RustInfo,
    /// Crate info
    pub crate_info: CrateInfo,
    /// Git info
    pub git_info: GitInfo,
    /// CI info
    pub ci_info: CiInfo,
}

#[derive(Debug, Clone)]
/// Holds flow information
pub struct FlowInfo {
    /// The flow config object
    pub config: Config,
    /// The main task of the flow
    pub task: String,
    /// The env info
    pub env_info: EnvInfo,
    /// Prevent workspace support
    pub disable_workspace: bool,
    /// Prevent on error flow even if defined in config section
    pub disable_on_error: bool,
    /// additional command line arguments
    pub cli_arguments: Option<Vec<String>>,
}

#[derive(Deserialize, Debug, Clone)]
/// Rust version condition structure
pub struct RustVersionCondition {
    /// min version number
    pub min: Option<String>,
    /// max version number
    pub max: Option<String>,
    /// specific version number
    pub equal: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
/// Holds condition attributes
pub struct TaskCondition {
    /// Platform names (linux, windows, mac)
    pub platforms: Option<Vec<String>>,
    /// Channel names (stable, beta, nightly)
    pub channels: Option<Vec<String>>,
    /// Environment variables which must be defined
    pub env_set: Option<Vec<String>>,
    /// Environment variables which must not be defined
    pub env_not_set: Option<Vec<String>>,
    /// Environment variables and their values
    pub env: Option<IndexMap<String, String>>,
    /// Rust version condition
    pub rust_version: Option<RustVersionCondition>,
}

#[derive(Deserialize, Debug, Clone)]
/// Holds a single task configuration such as command and dependencies list
pub struct EnvValueInfo {
    /// The script to execute to get the env value
    pub script: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Holds the env value or script
pub enum EnvValue {
    /// The value as string
    Value(String),
    /// Script which will return the value
    Info(EnvValueInfo),
}

#[derive(Deserialize, Debug, Clone)]
/// Holds instructions how to install the crate
pub struct InstallCrateInfo {
    /// The provided crate to install
    pub crate_name: String,
    /// If defined, the component to install via rustup
    pub rustup_component_name: Option<String>,
    /// The binary file name to be used to test if the crate is already installed
    pub binary: String,
    /// Test argument that will be used to check that the crate is installed
    pub test_arg: String,
}

impl PartialEq for InstallCrateInfo {
    fn eq(&self, other: &InstallCrateInfo) -> bool {
        if self.crate_name != other.crate_name
            || self.binary != other.binary
            || self.test_arg != other.test_arg
        {
            false
        } else {
            match self.rustup_component_name {
                Some(ref value) => match other.rustup_component_name {
                    Some(ref other_value) => value == other_value,
                    None => false,
                },
                None => match other.rustup_component_name {
                    None => true,
                    _ => false,
                },
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Holds instructions how to install a rustup component
pub struct InstallRustupComponentInfo {
    /// The component to install via rustup
    pub rustup_component_name: String,
    /// The binary file name to be used to test if the crate is already installed
    pub binary: Option<String>,
    /// Test argument that will be used to check that the crate is installed
    pub test_arg: Option<String>,
}

impl PartialEq for InstallRustupComponentInfo {
    fn eq(&self, other: &InstallRustupComponentInfo) -> bool {
        if self.rustup_component_name != other.rustup_component_name {
            false
        } else {
            let same = match self.binary {
                Some(ref value) => match other.binary {
                    Some(ref other_value) => value == other_value,
                    None => false,
                },
                None => match other.binary {
                    None => true,
                    _ => false,
                },
            };

            if same {
                match self.test_arg {
                    Some(ref value) => match other.test_arg {
                        Some(ref other_value) => value == other_value,
                        None => false,
                    },
                    None => match other.test_arg {
                        None => true,
                        _ => false,
                    },
                }
            } else {
                false
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Install crate name or params
pub enum InstallCrate {
    /// The value as string
    Value(String),
    /// Install crate params
    CrateInfo(InstallCrateInfo),
    /// Install rustup component params
    RustupComponentInfo(InstallRustupComponentInfo),
}

impl PartialEq for InstallCrate {
    fn eq(&self, other: &InstallCrate) -> bool {
        match self {
            InstallCrate::Value(value) => match other {
                InstallCrate::Value(other_value) => value == other_value,
                _ => false,
            },
            InstallCrate::CrateInfo(info) => match other {
                InstallCrate::CrateInfo(other_info) => info == other_info,
                _ => false,
            },
            InstallCrate::RustupComponentInfo(info) => match other {
                InstallCrate::RustupComponentInfo(other_info) => info == other_info,
                _ => false,
            },
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Holds a single task configuration such as command and dependencies list
pub struct Task {
    /// if true, it should ignore all data in base task
    pub clear: Option<bool>,
    /// Task description
    pub description: Option<String>,
    /// Category name used to document the task
    pub category: Option<String>,
    /// if true, the command/script of this task will not be invoked, dependencies however will be
    pub disabled: Option<bool>,
    /// if true, the task is hidden from the list of available tasks and also cannot be invoked directly from cli
    pub private: Option<bool>,
    /// set to false to notify cargo-make that this is not a workspace and should not call task for every member (same as --no-workspace CLI flag)
    pub workspace: Option<bool>,
    /// set to true to watch for file changes and invoke the task operation
    pub watch: Option<bool>,
    /// if provided all condition values must be met in order for the task to be invoked (will not stop dependencies)
    pub condition: Option<TaskCondition>,
    /// if script exit code is not 0, the command/script of this task will not be invoked, dependencies however will be
    pub condition_script: Option<Vec<String>>,
    /// if true, any error while executing the task will be printed but will not break the build
    pub force: Option<bool>,
    /// The env vars to setup before running the task commands
    pub env: Option<IndexMap<String, EnvValue>>,
    /// The working directory for the task to execute its command/script
    pub cwd: Option<String>,
    /// if defined, task points to another task and all other properties are ignored
    pub alias: Option<String>,
    /// acts like alias if runtime OS is Linux (takes precedence over alias)
    pub linux_alias: Option<String>,
    /// acts like alias if runtime OS is Windows (takes precedence over alias)
    pub windows_alias: Option<String>,
    /// acts like alias if runtime OS is Mac (takes precedence over alias)
    pub mac_alias: Option<String>,
    /// if defined, the provided crate will be installed (if needed) before running the task
    pub install_crate: Option<InstallCrate>,
    /// additional cargo install arguments
    pub install_crate_args: Option<Vec<String>>,
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
    /// The script file extension
    pub script_extension: Option<String>,
    /// The task name to execute
    pub run_task: Option<String>,
    /// A list of tasks to execute before this task
    pub dependencies: Option<Vec<String>>,
    /// The rust toolchain used to invoke the command or install the needed crates/components
    pub toolchain: Option<String>,
    /// override task if runtime OS is Linux (takes precedence over alias)
    pub linux: Option<PlatformOverrideTask>,
    /// override task if runtime OS is Windows (takes precedence over alias)
    pub windows: Option<PlatformOverrideTask>,
    /// override task if runtime OS is Mac (takes precedence over alias)
    pub mac: Option<PlatformOverrideTask>,
}

impl Task {
    /// Creates and returns a new instance.
    pub fn new() -> Task {
        Task {
            clear: None,
            description: None,
            category: None,
            disabled: None,
            private: None,
            workspace: None,
            watch: None,
            condition: None,
            condition_script: None,
            force: None,
            env: None,
            cwd: None,
            alias: None,
            linux_alias: None,
            windows_alias: None,
            mac_alias: None,
            install_crate: None,
            install_crate_args: None,
            install_script: None,
            command: None,
            args: None,
            script: None,
            script_runner: None,
            script_extension: None,
            run_task: None,
            dependencies: None,
            toolchain: None,
            linux: None,
            windows: None,
            mac: None,
        }
    }

    /// Copies values from the task into self.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to copy from
    pub fn extend(self: &mut Task, task: &Task) {
        let override_values = match task.clear {
            Some(value) => value,
            None => false,
        };

        if task.clear.is_some() {
            self.clear = task.clear.clone();
        }

        if task.description.is_some() {
            self.description = task.description.clone();
        } else if override_values {
            self.description = None;
        }

        if task.category.is_some() {
            self.category = task.category.clone();
        } else if override_values {
            self.category = None;
        }

        if task.disabled.is_some() {
            self.disabled = task.disabled.clone();
        } else if override_values {
            self.disabled = None;
        }

        if task.private.is_some() {
            self.private = task.private.clone();
        } else if override_values {
            self.private = None;
        }

        if task.workspace.is_some() {
            self.workspace = task.workspace.clone();
        } else if override_values {
            self.workspace = None;
        }

        if task.watch.is_some() {
            self.watch = task.watch.clone();
        } else if override_values {
            self.watch = None;
        }

        if task.condition.is_some() {
            self.condition = task.condition.clone();
        } else if override_values {
            self.condition = None;
        }

        if task.condition_script.is_some() {
            self.condition_script = task.condition_script.clone();
        } else if override_values {
            self.condition_script = None;
        }

        if task.force.is_some() {
            self.force = task.force.clone();
        } else if override_values {
            self.force = None;
        }

        if task.env.is_some() {
            self.env = task.env.clone();
        } else if override_values {
            self.env = None;
        }

        if task.cwd.is_some() {
            self.cwd = task.cwd.clone();
        } else if override_values {
            self.cwd = None;
        }

        if task.alias.is_some() {
            self.alias = task.alias.clone();
        } else if override_values {
            self.alias = None;
        }

        if task.linux_alias.is_some() {
            self.linux_alias = task.linux_alias.clone();
        } else if override_values {
            self.linux_alias = None;
        }

        if task.windows_alias.is_some() {
            self.windows_alias = task.windows_alias.clone();
        } else if override_values {
            self.windows_alias = None;
        }

        if task.mac_alias.is_some() {
            self.mac_alias = task.mac_alias.clone();
        } else if override_values {
            self.mac_alias = None;
        }

        if task.install_crate.is_some() {
            self.install_crate = task.install_crate.clone();
        } else if override_values {
            self.install_crate = None;
        }

        if task.install_crate_args.is_some() {
            self.install_crate_args = task.install_crate_args.clone();
        } else if override_values {
            self.install_crate_args = None;
        }

        if task.install_script.is_some() {
            self.install_script = task.install_script.clone();
        } else if override_values {
            self.install_script = None;
        }

        if task.command.is_some() {
            self.command = task.command.clone();
        } else if override_values {
            self.command = None;
        }

        if task.args.is_some() {
            self.args = task.args.clone();
        } else if override_values {
            self.args = None;
        }

        if task.script.is_some() {
            self.script = task.script.clone();
        } else if override_values {
            self.script = None;
        }

        if task.script_runner.is_some() {
            self.script_runner = task.script_runner.clone();
        } else if override_values {
            self.script_runner = None;
        }

        if task.script_extension.is_some() {
            self.script_extension = task.script_extension.clone();
        } else if override_values {
            self.script_extension = None;
        }

        if task.run_task.is_some() {
            self.run_task = task.run_task.clone();
        } else if override_values {
            self.run_task = None;
        }

        if task.dependencies.is_some() {
            self.dependencies = task.dependencies.clone();
        } else if override_values {
            self.dependencies = None;
        }

        if task.toolchain.is_some() {
            self.toolchain = task.toolchain.clone();
        } else if override_values {
            self.toolchain = None;
        }

        if task.linux.is_some() {
            self.linux = task.linux.clone();
        } else if override_values {
            self.linux = None;
        }

        if task.windows.is_some() {
            self.windows = task.windows.clone();
        } else if override_values {
            self.windows = None;
        }

        if task.mac.is_some() {
            self.mac = task.mac.clone();
        } else if override_values {
            self.mac = None;
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
                    clear: self.clear.clone(),
                    description: self.description.clone(),
                    category: self.category.clone(),
                    disabled: override_task.disabled.clone(),
                    private: override_task.private.clone(),
                    workspace: self.workspace.clone(),
                    watch: override_task.watch.clone(),
                    condition: override_task.condition.clone(),
                    condition_script: override_task.condition_script.clone(),
                    force: override_task.force.clone(),
                    env: override_task.env.clone(),
                    cwd: override_task.cwd.clone(),
                    alias: None,
                    linux_alias: None,
                    windows_alias: None,
                    mac_alias: None,
                    install_crate: override_task.install_crate.clone(),
                    install_crate_args: override_task.install_crate_args.clone(),
                    install_script: override_task.install_script.clone(),
                    command: override_task.command.clone(),
                    args: override_task.args.clone(),
                    script: override_task.script.clone(),
                    script_runner: override_task.script_runner.clone(),
                    script_extension: override_task.script_extension.clone(),
                    run_task: override_task.run_task.clone(),
                    dependencies: override_task.dependencies.clone(),
                    toolchain: override_task.toolchain.clone(),
                    linux: None,
                    windows: None,
                    mac: None,
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
            _ => match self.alias {
                Some(ref alias) => Some(alias.clone()),
                _ => None,
            },
        }
    }

    /// Returns true if the task is valid
    pub fn is_valid(self: &Task) -> bool {
        let mut actions_count = 0;

        if self.run_task.is_some() {
            actions_count = actions_count + 1;
        }
        if self.command.is_some() {
            actions_count = actions_count + 1;
        }
        if self.script.is_some() {
            actions_count = actions_count + 1;
        }

        if actions_count <= 1 {
            true
        } else {
            false
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Holds a single task configuration for a specific platform as an override of another task
pub struct PlatformOverrideTask {
    /// if true, it should ignore all data in base task
    pub clear: Option<bool>,
    /// if true, the command/script of this task will not be invoked, dependencies however will be
    pub disabled: Option<bool>,
    /// if true, the task is hidden from the list of available tasks and also cannot be invoked directly from cli
    pub private: Option<bool>,
    /// set to true to watch for file changes and invoke the task operation
    pub watch: Option<bool>,
    /// if provided all condition values must be met in order for the task to be invoked (will not stop dependencies)
    pub condition: Option<TaskCondition>,
    /// if script exit code is not 0, the command/script of this task will not be invoked, dependencies however will be
    pub condition_script: Option<Vec<String>>,
    /// if true, any error while executing the task will be printed but will not break the build
    pub force: Option<bool>,
    /// The env vars to setup before running the task commands
    pub env: Option<IndexMap<String, EnvValue>>,
    /// The working directory for the task to execute its command/script
    pub cwd: Option<String>,
    /// if defined, the provided crate will be installed (if needed) before running the task
    pub install_crate: Option<InstallCrate>,
    /// additional cargo install arguments
    pub install_crate_args: Option<Vec<String>>,
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
    /// The script file extension
    pub script_extension: Option<String>,
    /// The task name to execute
    pub run_task: Option<String>,
    /// A list of tasks to execute before this task
    pub dependencies: Option<Vec<String>>,
    /// The rust toolchain used to invoke the command or install the needed crates/components
    pub toolchain: Option<String>,
}

impl PlatformOverrideTask {
    /// Copies values from the task into self.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to copy from
    pub fn extend(self: &mut PlatformOverrideTask, task: &mut Task) {
        let copy_values = match self.clear {
            Some(value) => !value,
            None => true,
        };

        if copy_values {
            if self.disabled.is_none() && task.disabled.is_some() {
                self.disabled = task.disabled.clone();
            }

            if self.private.is_none() && task.private.is_some() {
                self.private = task.private.clone();
            }

            if self.watch.is_none() && task.watch.is_some() {
                self.watch = task.watch.clone();
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

            if self.env.is_none() && task.env.is_some() {
                self.env = task.env.clone();
            }

            if self.cwd.is_none() && task.cwd.is_some() {
                self.cwd = task.cwd.clone();
            }

            if self.install_crate.is_none() && task.install_crate.is_some() {
                self.install_crate = task.install_crate.clone();
            }

            if self.install_crate_args.is_none() && task.install_crate_args.is_some() {
                self.install_crate_args = task.install_crate_args.clone();
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

            if self.script_extension.is_none() && task.script_extension.is_some() {
                self.script_extension = task.script_extension.clone();
            }

            if self.run_task.is_none() && task.run_task.is_some() {
                self.run_task = task.run_task.clone();
            }

            if self.dependencies.is_none() && task.dependencies.is_some() {
                self.dependencies = task.dependencies.clone();
            }

            if self.toolchain.is_none() && task.toolchain.is_some() {
                self.toolchain = task.toolchain.clone();
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Holds the configuration found in the makefile toml config section.
pub struct ConfigSection {
    /// If true, the default core tasks will not be loaded
    pub skip_core_tasks: Option<bool>,
    /// Init task name which will be invoked at the start of every run
    pub init_task: Option<String>,
    /// End task name which will be invoked at the end of every run
    pub end_task: Option<String>,
    /// The name of the task to run in case of any error during the invocation of the flow
    pub on_error_task: Option<String>,
    /// Invoked while loading the descriptor file but before loading any extended descriptor
    pub load_script: Option<Vec<String>>,
    /// acts like load_script if runtime OS is Linux (takes precedence over load_script)
    pub linux_load_script: Option<Vec<String>>,
    /// acts like load_script if runtime OS is Windows (takes precedence over load_script)
    pub windows_load_script: Option<Vec<String>>,
    /// acts like load_script if runtime OS is Mac (takes precedence over load_script)
    pub mac_load_script: Option<Vec<String>>,
}

impl ConfigSection {
    /// Creates and returns a new instance.
    pub fn new() -> ConfigSection {
        ConfigSection {
            skip_core_tasks: None,
            init_task: None,
            end_task: None,
            on_error_task: None,
            load_script: None,
            linux_load_script: None,
            windows_load_script: None,
            mac_load_script: None,
        }
    }

    /// Copies values from the config section into self.
    ///
    /// # Arguments
    ///
    /// * `task` - The task to copy from
    pub fn extend(self: &mut ConfigSection, extended: &mut ConfigSection) {
        if extended.skip_core_tasks.is_some() {
            self.skip_core_tasks = extended.skip_core_tasks.clone();
        }

        if extended.init_task.is_some() {
            self.init_task = extended.init_task.clone();
        }

        if extended.end_task.is_some() {
            self.end_task = extended.end_task.clone();
        }

        if extended.on_error_task.is_some() {
            self.on_error_task = extended.on_error_task.clone();
        }

        if extended.load_script.is_some() {
            self.load_script = extended.load_script.clone();
        }

        if extended.linux_load_script.is_some() {
            self.linux_load_script = extended.linux_load_script.clone();
        }

        if extended.windows_load_script.is_some() {
            self.windows_load_script = extended.windows_load_script.clone();
        }

        if extended.mac_load_script.is_some() {
            self.mac_load_script = extended.mac_load_script.clone();
        }
    }

    /// Returns the load script based on the current platform
    pub fn get_load_script(self: &ConfigSection) -> Option<Vec<String>> {
        let platform_name = get_platform_name();

        if platform_name == "windows" {
            if self.windows_load_script.is_some() {
                self.windows_load_script.clone()
            } else {
                self.load_script.clone()
            }
        } else if platform_name == "mac" {
            if self.mac_load_script.is_some() {
                self.mac_load_script.clone()
            } else {
                self.load_script.clone()
            }
        } else {
            if self.linux_load_script.is_some() {
                self.linux_load_script.clone()
            } else {
                self.load_script.clone()
            }
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
/// Holds the entire configuration such as task definitions and env vars
pub struct Config {
    /// Runtime config
    pub config: ConfigSection,
    /// The env vars to setup before running the tasks
    pub env: IndexMap<String, EnvValue>,
    /// All task definitions
    pub tasks: IndexMap<String, Task>,
}

#[derive(Deserialize, Debug, Clone)]
/// Holds the entire externally read configuration such as task definitions and env vars where all values are optional
pub struct ExternalConfig {
    /// Path to another toml file to extend
    pub extend: Option<String>,
    /// Runtime config
    pub config: Option<ConfigSection>,
    /// The env vars to setup before running the tasks
    pub env: Option<IndexMap<String, EnvValue>>,
    /// All task definitions
    pub tasks: Option<IndexMap<String, Task>>,
}

impl ExternalConfig {
    /// Creates and returns a new instance.
    pub fn new() -> ExternalConfig {
        ExternalConfig {
            extend: None,
            config: None,
            env: None,
            tasks: None,
        }
    }
}

#[derive(Debug)]
/// Execution plan step to execute
pub struct Step {
    /// The task name
    pub name: String,
    /// The task config
    pub config: Task,
}

#[derive(Debug)]
/// Execution plan which defines all steps to run and the order to run them
pub struct ExecutionPlan {
    /// A list of steps to execute
    pub steps: Vec<Step>,
}

#[derive(Debug)]
/// Command info
pub struct CommandSpec {
    /// The command to execute
    pub command: String,
    /// The command args
    pub args: Option<Vec<String>>,
}
