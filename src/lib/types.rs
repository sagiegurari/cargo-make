//! # types
//!
//! Defines the various types and aliases used by cargo-make.
//!

#[cfg(test)]
#[path = "types_test.rs"]
mod types_test;

use crate::legacy;
use crate::plugin::types::Plugins;
use ci_info::types::CiInfo;
use git_info::types::GitInfo;
use indexmap::{IndexMap, IndexSet};
use regex::Regex;
use rust_info::types::RustInfo;
use std::collections::HashMap;

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

fn get_namespaced_task_name(namespace: &str, task: &str) -> String {
    let mut namespaced_task = String::new();

    if namespace.len() > 0 {
        namespaced_task.push_str(namespace);
        namespaced_task.push_str("::");
    }
    namespaced_task.push_str(task);

    namespaced_task
}

fn extend_script_value(
    current_script_value: Option<ScriptValue>,
    new_script_value: Option<ScriptValue>,
) -> Option<ScriptValue> {
    match current_script_value {
        Some(ref current_value) => match new_script_value {
            Some(ref new_value) => match current_value {
                ScriptValue::Sections(current_sections) => match new_value {
                    ScriptValue::Sections(new_sections) => {
                        let pre = if new_sections.pre.is_some() {
                            new_sections.pre.clone()
                        } else {
                            current_sections.pre.clone()
                        };
                        let main = if new_sections.main.is_some() {
                            new_sections.main.clone()
                        } else {
                            current_sections.main.clone()
                        };
                        let post = if new_sections.post.is_some() {
                            new_sections.post.clone()
                        } else {
                            current_sections.post.clone()
                        };

                        Some(ScriptValue::Sections(ScriptSections { pre, main, post }))
                    }
                    _ => current_script_value,
                },
                _ => new_script_value,
            },
            None => current_script_value,
        },
        None => new_script_value,
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
    /// The profile name
    pub profile: Option<String>,
    /// Log level name
    pub log_level: String,
    /// Disables colorful output
    pub disable_color: bool,
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
    /// Allow invocation of private tasks
    pub allow_private: bool,
    /// If true, the init and end tasks are skipped
    pub skip_init_end_tasks: bool,
    /// Skip tasks that match the provided pattern
    pub skip_tasks_pattern: Option<String>,
    /// Only print the execution plan
    pub print_only: bool,
    /// List all known steps
    pub list_all_steps: bool,
    /// List steps for a given category
    pub list_category_steps: Option<String>,
    /// Diff flows
    pub diff_execution_plan: bool,
    /// Disables the update check during startup
    pub disable_check_for_updates: bool,
    /// Allows access unsupported experimental predefined tasks
    pub experimental: bool,
    /// additional command line arguments
    pub arguments: Option<Vec<String>>,
    /// Output format
    pub output_format: String,
    /// Output file name
    pub output_file: Option<String>,
    /// Print time summary at end of the flow
    pub print_time_summary: bool,
    /// Hide any minor tasks such as pre/post hooks
    pub hide_uninteresting: bool,
}

impl CliArgs {
    /// Creates and returns a new instance.
    pub fn new() -> CliArgs {
        CliArgs {
            command: "".to_string(),
            build_file: None,
            task: "default".to_string(),
            profile: None,
            log_level: "info".to_string(),
            disable_color: false,
            cwd: None,
            env: None,
            env_file: None,
            disable_workspace: false,
            disable_on_error: false,
            allow_private: false,
            skip_init_end_tasks: false,
            skip_tasks_pattern: None,
            print_only: false,
            list_all_steps: false,
            list_category_steps: None,
            diff_execution_plan: false,
            disable_check_for_updates: false,
            experimental: false,
            arguments: None,
            output_format: "default".to_string(),
            output_file: None,
            print_time_summary: false,
            hide_uninteresting: false,
        }
    }
}

#[derive(Debug)]
pub(crate) struct RunTaskOptions {
    pub(crate) plugins_enabled: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
        Default::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// Holds configuration info for cargo-make
pub struct GlobalConfig {
    /// File from which the global config was loaded from
    #[serde(skip)]
    pub file_name: Option<String>,
    /// Default log level
    pub log_level: Option<String>,
    /// Default output coloring
    pub disable_color: Option<bool>,
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
            search_project_root: Some(false),
            ..Default::default()
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// Holds crate workspace info, see <http://doc.crates.io/manifest.html#the-workspace-section>
pub struct Workspace {
    /// members paths
    pub members: Option<Vec<String>>,
    /// exclude paths
    pub exclude: Option<Vec<String>>,
    /// workspace level dependencies
    pub dependencies: Option<IndexMap<String, CrateDependency>>,
    /// root package
    pub package: Option<PackageInfo>,
}

impl Workspace {
    /// Creates and returns a new instance.
    pub fn new() -> Workspace {
        Default::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
        Default::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds crate dependency info.
pub struct CrateDependencyInfo {
    /// Holds the dependency path
    pub path: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Holds crate dependency info.
pub enum CrateDependency {
    /// Holds the dependency version
    Version(String),
    /// Hold dependency info
    Info(CrateDependencyInfo),
}

#[derive(Deserialize, Debug, Clone, Default)]
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
        Default::default()
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
    /// Allow invocation of private tasks
    pub allow_private: bool,
    /// If true, the init and end tasks are skipped
    pub skip_init_end_tasks: bool,
    /// Skip tasks that match the provided pattern
    pub skip_tasks_pattern: Option<Regex>,
    /// additional command line arguments
    pub cli_arguments: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default)]
/// Holds mutable flow state
pub struct FlowState {
    /// timing info for summary
    pub time_summary: Vec<(String, u128)>,
    /// forced plugin name
    pub forced_plugin: Option<String>,
}

impl FlowState {
    /// Creates and returns a new instance.
    pub fn new() -> FlowState {
        Default::default()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Rust version condition structure
pub struct RustVersionCondition {
    /// min version number
    pub min: Option<String>,
    /// max version number
    pub max: Option<String>,
    /// specific version number
    pub equal: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Files modified (input/output) condition structure
pub struct FilesFilesModifiedCondition {
    /// input files
    pub input: Vec<String>,
    /// output files
    pub output: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
/// Control how condition checks are evaluated
pub enum ConditionType {
    /// All conditions must pass
    And,
    /// Any condition must pass
    Or,
    /// Any condition group must pass, but each group will be validated as an AND
    GroupOr,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// Holds condition attributes
pub struct TaskCondition {
    /// condition type (AND/OR) by default AND
    pub condition_type: Option<ConditionType>,
    /// Failure message
    pub fail_message: Option<String>,
    /// Profile names (development, ...)
    pub profiles: Option<Vec<String>>,
    /// As defined in the cfg target_os
    pub os: Option<Vec<String>>,
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
    /// Environment variables which are defined as true
    pub env_true: Option<Vec<String>>,
    /// Environment variables which are defined as false
    pub env_false: Option<Vec<String>>,
    /// Environment variables and the values which they are required to contain
    pub env_contains: Option<IndexMap<String, String>>,
    /// Rust version condition
    pub rust_version: Option<RustVersionCondition>,
    /// Files exist
    pub files_exist: Option<Vec<String>>,
    /// Files which do not exist
    pub files_not_exist: Option<Vec<String>>,
    /// Files modified since last execution
    pub files_modified: Option<FilesFilesModifiedCondition>,
}

impl TaskCondition {
    pub fn get_condition_type(&self) -> ConditionType {
        match self.condition_type {
            Some(ref value) => value.clone(),
            None => ConditionType::And,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Env file path and attributes
pub struct EnvFileInfo {
    /// The file path as string
    pub path: String,
    /// The path base directory (relative paths are from this base path)
    pub base_path: Option<String>,
    /// The profile name this file is relevant to
    pub profile: Option<String>,
    /// If true, only set the env vars if not already defined
    pub defaults_only: Option<bool>,
}

impl EnvFileInfo {
    /// Creates and returns a new instance.
    pub fn new(path: String) -> EnvFileInfo {
        EnvFileInfo {
            path,
            base_path: None,
            profile: None,
            defaults_only: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Holds the env file path and attributes
pub enum EnvFile {
    /// The file path as string
    Path(String),
    /// Extended info object for env file
    Info(EnvFileInfo),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Env value provided by a script
pub struct EnvValueScript {
    /// The script to execute to get the env value
    pub script: Vec<String>,
    /// True/False to enable multi line env values
    pub multi_line: Option<bool>,
    /// The condition to validate
    pub condition: Option<TaskCondition>,
    /// The explicit environment variables this script depends on
    pub depends_on: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Env value provided by decoding other values
pub struct EnvValueDecode {
    /// The source value (can be an env expression)
    pub source: String,
    /// The default value in case no decode mapping was found, if not provided it will default to the source value
    pub default_value: Option<String>,
    /// The decoding mapping
    pub mapping: HashMap<String, String>,
    /// The condition to validate
    pub condition: Option<TaskCondition>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
/// Enables to unset env variables
pub struct EnvValueUnset {
    /// If true, the env variable will be unset, else ignored
    pub unset: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Env value set if condition is met
pub struct EnvValueConditioned {
    /// The value to set (can be an env expression)
    pub value: String,
    /// The condition to validate
    pub condition: Option<TaskCondition>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Env value holding a list of paths based on given glob definitions
pub struct EnvValuePathGlob {
    /// The glob used to fetch all paths
    pub glob: String,
    /// True to include files (default is true if undefined)
    pub include_files: Option<bool>,
    /// True to include directories (default is true if undefined)
    pub include_dirs: Option<bool>,
    /// Enables to respect ignore files
    pub ignore_type: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Holds the env value or script
pub enum EnvValue {
    /// The value as string
    Value(String),
    /// The value as boolean
    Boolean(bool),
    /// The value as number
    Number(isize),
    /// The value as a list of strings
    List(Vec<String>),
    /// Unset env
    Unset(EnvValueUnset),
    /// Script which will return the value
    Script(EnvValueScript),
    /// Env decoding info
    Decode(EnvValueDecode),
    /// Conditional env value
    Conditional(EnvValueConditioned),
    /// Path glob
    PathGlob(EnvValuePathGlob),
    /// Profile env
    Profile(IndexMap<String, EnvValue>),
}

/// Arguments used to check whether a crate or rustup component is installed.
///
/// Deserialize into an array of strings. Allows both a single string (which will
/// become a single-element array) or a sequence of strings.
#[derive(Debug, Serialize, Clone, PartialEq, Eq)]
#[serde(transparent)]
pub struct TestArg {
    /// Content of the arguments
    pub inner: Vec<String>,
}

impl std::ops::Deref for TestArg {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl std::ops::DerefMut for TestArg {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<'de> serde::de::Deserialize<'de> for TestArg {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::de::Deserializer<'de>,
    {
        struct StringVecVisitor;
        impl<'de> serde::de::Visitor<'de> for StringVecVisitor {
            type Value = TestArg;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("A string or an array of strings")
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Ok(TestArg {
                    inner: vec![s.to_string()],
                })
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let mut v = Vec::with_capacity(seq.size_hint().unwrap_or(0));
                while let Some(s) = seq.next_element()? {
                    v.push(s);
                }

                Ok(TestArg { inner: v })
            }
        }
        deserializer.deserialize_any(StringVecVisitor)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds instructions how to install the cargo plugin
pub struct InstallCargoPluginInfo {
    /// The provided crate to install
    pub crate_name: Option<String>,
    /// Minimal version
    pub min_version: Option<String>,
    /// Optional alternate 'install' command
    pub install_command: Option<String>,
    /// Optional add force flag (if needed), default is true
    pub force: Option<bool>,
}

impl PartialEq for InstallCargoPluginInfo {
    fn eq(&self, other: &InstallCargoPluginInfo) -> bool {
        let mut same = match self.crate_name {
            Some(ref crate_name) => match other.crate_name {
                Some(ref other_crate_name) => crate_name == other_crate_name,
                None => false,
            },
            None => match other.crate_name {
                None => true,
                _ => false,
            },
        };
        if !same {
            return false;
        }

        same = match self.min_version {
            Some(ref min_version) => match other.min_version {
                Some(ref other_min_version) => min_version == other_min_version,
                None => false,
            },
            None => match other.min_version {
                None => true,
                _ => false,
            },
        };
        if !same {
            return false;
        }

        same = match self.install_command {
            Some(ref install_command) => match other.install_command {
                Some(ref other_install_command) => install_command == other_install_command,
                None => false,
            },
            None => match other.install_command {
                None => true,
                _ => false,
            },
        };
        if !same {
            return false;
        }

        match self.force {
            Some(ref force) => match other.force {
                Some(ref other_force) => force == other_force,
                None => false,
            },
            None => match other.force {
                None => true,
                _ => false,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds instructions how to install the crate
pub struct InstallCrateInfo {
    /// The provided crate to install
    pub crate_name: String,
    /// If defined, the component to install via rustup
    pub rustup_component_name: Option<String>,
    /// The binary file name to be used to test if the crate is already installed
    pub binary: String,
    /// Test arguments that will be used to check that the crate is installed.
    pub test_arg: TestArg,
    /// Minimal version
    pub min_version: Option<String>,
    /// Exact version
    pub version: Option<String>,
    /// Optional alternate 'install' command
    pub install_command: Option<String>,
    /// Optional add force flag (if needed), default is true
    pub force: Option<bool>,
}

impl PartialEq for InstallCrateInfo {
    fn eq(&self, other: &InstallCrateInfo) -> bool {
        if self.crate_name != other.crate_name
            || self.binary != other.binary
            || self.test_arg != other.test_arg
        {
            return false;
        }

        let mut same = match self.rustup_component_name {
            Some(ref rustup_component_name) => match other.rustup_component_name {
                Some(ref other_rustup_component_name) => {
                    if rustup_component_name == other_rustup_component_name {
                        true
                    } else {
                        false
                    }
                }
                None => false,
            },
            None => match other.rustup_component_name {
                None => true,
                _ => false,
            },
        };
        if !same {
            return false;
        }

        same = match self.min_version {
            Some(ref min_version) => match other.min_version {
                Some(ref other_min_version) => min_version == other_min_version,
                None => false,
            },
            None => match other.min_version {
                None => true,
                _ => false,
            },
        };
        if !same {
            return false;
        }

        same = match self.version {
            Some(ref version) => match other.version {
                Some(ref other_version) => version == other_version,
                None => false,
            },
            None => match other.version {
                None => true,
                _ => false,
            },
        };
        if !same {
            return false;
        }

        same = match self.install_command {
            Some(ref install_command) => match other.install_command {
                Some(ref other_install_command) => install_command == other_install_command,
                None => false,
            },
            None => match other.install_command {
                None => true,
                _ => false,
            },
        };
        if !same {
            return false;
        }

        match self.force {
            Some(ref force) => match other.force {
                Some(ref other_force) => force == other_force,
                None => false,
            },
            None => match other.force {
                None => true,
                _ => false,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds instructions how to install a rustup component
pub struct InstallRustupComponentInfo {
    /// The component to install via rustup
    pub rustup_component_name: String,
    /// The binary file name to be used to test if the crate is already installed
    pub binary: Option<String>,
    /// Test argument that will be used to check that the crate is installed
    pub test_arg: Option<TestArg>,
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
                self.test_arg == other.test_arg
            } else {
                false
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Install crate name or params
pub enum InstallCrate {
    /// Enables to prevent installation flow
    Enabled(bool),
    /// The value as string
    Value(String),
    /// Install crate params
    CrateInfo(InstallCrateInfo),
    /// Install rustup component params
    RustupComponentInfo(InstallRustupComponentInfo),
    /// Install cargo plugin info
    CargoPluginInfo(InstallCargoPluginInfo),
}

impl PartialEq for InstallCrate {
    fn eq(&self, other: &InstallCrate) -> bool {
        match self {
            InstallCrate::Enabled(value) => match other {
                InstallCrate::Enabled(other_value) => value == other_value,
                _ => false,
            },
            InstallCrate::Value(value) => match other {
                InstallCrate::Value(other_value) => value == other_value,
                _ => false,
            },
            InstallCrate::CargoPluginInfo(info) => match other {
                InstallCrate::CargoPluginInfo(other_info) => info == other_info,
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(untagged)]
/// Holds the run task name/s
pub enum RunTaskName {
    /// Single task name
    Single(String),
    /// Multiple task names
    Multiple(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds the run task information
pub struct RunTaskDetails {
    /// The task name
    pub name: RunTaskName,
    /// True to fork the task to a new sub process
    pub fork: Option<bool>,
    /// True to run all tasks in parallel (default false)
    pub parallel: Option<bool>,
    /// Cleanup task name
    pub cleanup_task: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds the run task routing information
pub struct RunTaskRoutingInfo {
    /// The task name
    pub name: RunTaskName,
    /// True to fork the task to a new sub process
    pub fork: Option<bool>,
    /// True to run all tasks in parallel (default false)
    pub parallel: Option<bool>,
    /// Cleanup task name
    pub cleanup_task: Option<String>,
    /// if provided all condition values must be met in order for the task to be invoked
    pub condition: Option<TaskCondition>,
    /// if script exit code is not 0, the task will not be invoked
    pub condition_script: Option<ConditionScriptValue>,
    /// The script runner arguments before the script file path
    pub condition_script_runner_args: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Run task info
pub enum RunTaskInfo {
    /// Task name
    Name(String),
    /// Run Task Info
    Details(RunTaskDetails),
    /// Task conditional selector
    Routing(Vec<RunTaskRoutingInfo>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds watch options
pub struct WatchOptions {
    /// Watch version to install if not already installed
    pub version: Option<String>,
    /// Postpone first run until a file changes
    pub postpone: Option<bool>,
    /// Ignore a glob/gitignore-style pattern
    pub ignore_pattern: Option<MaybeArray<String>>,
    /// Do not use .gitignore files
    pub no_git_ignore: Option<bool>,
    /// Show paths that changed
    pub why: Option<bool>,
    /// Select which files/folders to watch
    pub watch: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(untagged)]
/// Could be an array or single value
pub enum MaybeArray<T> {
    /// Single value
    Single(T),
    /// Multiple values
    Multiple(Vec<T>),
}

impl PartialEq for WatchOptions {
    fn eq(&self, other: &WatchOptions) -> bool {
        let mut same = match self.version {
            Some(ref value) => match other.version {
                Some(ref other_value) => value == other_value,
                None => false,
            },
            None => match other.version {
                None => true,
                _ => false,
            },
        };

        same = if same {
            match self.postpone {
                Some(ref value) => match other.postpone {
                    Some(ref other_value) => value == other_value,
                    None => false,
                },
                None => match other.postpone {
                    None => true,
                    _ => false,
                },
            }
        } else {
            false
        };

        same = if same {
            match self.ignore_pattern {
                Some(ref value) => match other.ignore_pattern {
                    Some(ref other_value) => value == other_value,
                    None => false,
                },
                None => match other.ignore_pattern {
                    None => true,
                    _ => false,
                },
            }
        } else {
            false
        };

        same = if same {
            match self.no_git_ignore {
                Some(ref value) => match other.no_git_ignore {
                    Some(ref other_value) => value == other_value,
                    None => false,
                },
                None => match other.no_git_ignore {
                    None => true,
                    _ => false,
                },
            }
        } else {
            false
        };

        if same {
            match self.watch {
                Some(ref value) => match other.watch {
                    Some(ref other_value) => value == other_value,
                    None => false,
                },
                None => match other.watch {
                    None => true,
                    _ => false,
                },
            }
        } else {
            false
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Holds watch options or simple true/false value
pub enum TaskWatchOptions {
    /// True/False to enable/disable watch
    Boolean(bool),
    /// Extended configuration for watch
    Options(WatchOptions),
}

impl PartialEq for TaskWatchOptions {
    fn eq(&self, other: &TaskWatchOptions) -> bool {
        match self {
            TaskWatchOptions::Boolean(value) => match other {
                TaskWatchOptions::Boolean(other_value) => value == other_value,
                _ => false,
            },
            TaskWatchOptions::Options(info) => match other {
                TaskWatchOptions::Options(other_info) => info == other_info,
                _ => false,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Holds deprecation info such as true/false/message
pub enum DeprecationInfo {
    /// True/False flag (true is deprecated)
    Boolean(bool),
    /// Deprecation message
    Message(String),
}

impl PartialEq for DeprecationInfo {
    fn eq(&self, other: &DeprecationInfo) -> bool {
        match self {
            DeprecationInfo::Boolean(value) => match other {
                DeprecationInfo::Boolean(other_value) => value == other_value,
                _ => false,
            },
            DeprecationInfo::Message(message) => match other {
                DeprecationInfo::Message(other_message) => message == other_message,
                _ => false,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Script file name
pub struct FileScriptValue {
    /// Script file name
    pub file: String,
    /// True for absolute path (default false)
    pub absolute_path: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Script content split to parts to enable a more fine tuned extension capability
pub struct ScriptSections {
    /// Script section
    pub pre: Option<String>,
    /// Script section
    pub main: Option<String>,
    /// Script section
    pub post: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Script value (text, file name, ...)
pub enum ScriptValue {
    /// The script text as single line
    SingleLine(String),
    /// The script text lines
    Text(Vec<String>),
    /// Script file name
    File(FileScriptValue),
    /// Script content split to multiple parts to enable fine tuned extension
    Sections(ScriptSections),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Condition script value (not as advanced as normal script value)
pub enum ConditionScriptValue {
    /// The script text as single line
    SingleLine(String),
    /// The script text lines
    Text(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
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
    /// if not false, this task is defined as deprecated
    pub deprecated: Option<DeprecationInfo>,
    /// Extend any task based on the defined name
    pub extend: Option<String>,
    /// set to false to notify cargo-make that this is not a workspace and should not call task for every member (same as --no-workspace CLI flag)
    pub workspace: Option<bool>,
    /// Optional plugin used to execute the task
    pub plugin: Option<String>,
    /// set to true to watch for file changes and invoke the task operation
    pub watch: Option<TaskWatchOptions>,
    /// if provided all condition values must be met in order for the task to be invoked (will not stop dependencies)
    pub condition: Option<TaskCondition>,
    /// if script exit code is not 0, the command/script of this task will not be invoked, dependencies however will be
    pub condition_script: Option<ConditionScriptValue>,
    /// The script runner arguments before the script file path
    pub condition_script_runner_args: Option<Vec<String>>,
    /// if true, any error while executing the task will be printed but will not break the build
    pub ignore_errors: Option<bool>,
    /// DEPRECATED, replaced with ignore_errors
    pub force: Option<bool>,
    /// The env files to setup before running the task commands
    pub env_files: Option<Vec<EnvFile>>,
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
    pub install_script: Option<ScriptValue>,
    /// The command to execute
    pub command: Option<String>,
    /// The command args
    pub args: Option<Vec<String>>,
    /// If command is not defined, and script is defined, the provided script will be executed
    pub script: Option<ScriptValue>,
    /// The script runner (defaults to cmd in windows and sh for other platforms)
    pub script_runner: Option<String>,
    /// The script runner arguments before the script file path
    pub script_runner_args: Option<Vec<String>>,
    /// The script file extension
    pub script_extension: Option<String>,
    /// The task name to execute
    pub run_task: Option<RunTaskInfo>,
    /// A list of tasks to execute before this task
    pub dependencies: Option<Vec<DependencyIdentifier>>,
    /// The rust toolchain used to invoke the command or install the needed crates/components
    pub toolchain: Option<ToolchainSpecifier>,
    /// override task if runtime OS is Linux (takes precedence over alias)
    pub linux: Option<PlatformOverrideTask>,
    /// override task if runtime OS is Windows (takes precedence over alias)
    pub windows: Option<PlatformOverrideTask>,
    /// override task if runtime OS is Mac (takes precedence over alias)
    pub mac: Option<PlatformOverrideTask>,
}

/// A toolchain, defined either as a string (following the rustup syntax)
/// or a ToolchainBoundedSpecifier.
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum ToolchainSpecifier {
    /// A string specifying the channel name of the toolchain
    Simple(String),
    /// A toolchain with a minimum version bound
    Bounded(ToolchainBoundedSpecifier),
}

impl From<String> for ToolchainSpecifier {
    fn from(channel: String) -> Self {
        Self::Simple(channel)
    }
}

impl From<&str> for ToolchainSpecifier {
    fn from(channel: &str) -> Self {
        channel.to_string().into()
    }
}

impl std::fmt::Display for ToolchainSpecifier {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(ref channel) => write!(formatter, "{}", channel),
            Self::Bounded(ref spec) => write!(formatter, "{}", spec),
        }
    }
}

impl ToolchainSpecifier {
    /// Return the channel of the toolchain to look for
    pub fn channel(&self) -> &str {
        match self {
            Self::Simple(ref channel) => &channel,
            Self::Bounded(ToolchainBoundedSpecifier { ref channel, .. }) => channel,
        }
    }

    /// Return the minimal version, if any, to look for
    pub fn min_version(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Bounded(ToolchainBoundedSpecifier {
                ref min_version, ..
            }) => Some(min_version),
        }
    }
}

/// A toolchain with a minimum version bound
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct ToolchainBoundedSpecifier {
    /// The channel of the toolchain to use
    pub channel: String,
    /// The minimum version to match
    pub min_version: String,
}

impl std::fmt::Display for ToolchainBoundedSpecifier {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{} >= {}", self.channel, self.min_version)
    }
}

/// A dependency, defined either as a string or as a Dependency object
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum DependencyIdentifier {
    /// A full dependency definition (potentially in a different file)
    Definition(TaskIdentifier),
    /// A string dependency definition (its name in the current file)
    Name(String),
}

impl From<&str> for DependencyIdentifier {
    fn from(name: &str) -> Self {
        DependencyIdentifier::Name(name.to_string())
    }
}

impl DependencyIdentifier {
    /// Get the name of a dependency
    pub fn name(&self) -> &str {
        match self {
            DependencyIdentifier::Definition(identifier) => &identifier.name,
            DependencyIdentifier::Name(name) => name,
        }
    }

    /// Adorn the TaskIdentifier with a namespace
    pub fn with_namespace(self, namespace: &str) -> Self {
        match self {
            DependencyIdentifier::Definition(mut identifier) => {
                identifier.name = get_namespaced_task_name(namespace, &identifier.name);
                DependencyIdentifier::Definition(identifier)
            }
            DependencyIdentifier::Name(name) => {
                DependencyIdentifier::Name(get_namespaced_task_name(namespace, &name))
            }
        }
    }
}

/// An identifier for a task
#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq)]
pub struct TaskIdentifier {
    /// The task name to execute
    pub name: String,
    /// The path to the makefile the task resides in
    pub path: Option<String>,
}

impl std::fmt::Display for TaskIdentifier {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.path {
            Some(path) => write!(formatter, "{}:{}", path, self.name),
            None => write!(formatter, "{}", self.name),
        }
    }
}

impl TaskIdentifier {
    /// Create a new TaskIdentifier referencing a task in the current file
    pub fn from_name(name: &str) -> Self {
        Self {
            name: name.to_string(),
            path: None,
        }
    }
}

impl Into<TaskIdentifier> for DependencyIdentifier {
    fn into(self) -> TaskIdentifier {
        match self {
            DependencyIdentifier::Definition(identifier) => identifier,
            DependencyIdentifier::Name(name) => TaskIdentifier { name, path: None },
        }
    }
}

impl Task {
    /// Creates and returns a new instance.
    pub fn new() -> Task {
        Default::default()
    }

    /// Apply modifications
    pub fn apply(self: &mut Task, modify_config: &ModifyConfig) {
        match modify_config.private {
            Some(value) => {
                if value {
                    self.private = Some(true);
                }
            }
            None => (),
        };

        match modify_config.namespace {
            Some(ref namespace) => {
                if namespace.len() > 0 {
                    if self.extend.is_some() {
                        self.extend = Some(get_namespaced_task_name(
                            namespace,
                            &self.extend.clone().unwrap(),
                        ));
                    }

                    if self.alias.is_some() {
                        self.alias = Some(get_namespaced_task_name(
                            namespace,
                            &self.alias.clone().unwrap(),
                        ));
                    }

                    if self.linux_alias.is_some() {
                        self.linux_alias = Some(get_namespaced_task_name(
                            namespace,
                            &self.linux_alias.clone().unwrap(),
                        ));
                    }

                    if self.windows_alias.is_some() {
                        self.windows_alias = Some(get_namespaced_task_name(
                            namespace,
                            &self.windows_alias.clone().unwrap(),
                        ));
                    }

                    if self.mac_alias.is_some() {
                        self.mac_alias = Some(get_namespaced_task_name(
                            namespace,
                            &self.mac_alias.clone().unwrap(),
                        ));
                    }

                    if self.run_task.is_some() {
                        let mut run_task = self.run_task.clone().unwrap();

                        run_task = match run_task {
                            RunTaskInfo::Name(value) => {
                                RunTaskInfo::Name(get_namespaced_task_name(namespace, &value))
                            }
                            RunTaskInfo::Details(mut run_task_details) => {
                                match run_task_details.name {
                                    RunTaskName::Single(ref name) => {
                                        run_task_details.name = RunTaskName::Single(
                                            get_namespaced_task_name(namespace, name),
                                        )
                                    }
                                    RunTaskName::Multiple(ref names) => {
                                        let mut updated_names = vec![];
                                        for name in names {
                                            updated_names
                                                .push(get_namespaced_task_name(namespace, name));
                                        }

                                        run_task_details.name =
                                            RunTaskName::Multiple(updated_names);
                                    }
                                };

                                RunTaskInfo::Details(run_task_details)
                            }
                            RunTaskInfo::Routing(mut routing_info_vector) => {
                                for routing_info in &mut routing_info_vector {
                                    match routing_info.name {
                                        RunTaskName::Single(ref name) => {
                                            routing_info.name = RunTaskName::Single(
                                                get_namespaced_task_name(namespace, name),
                                            )
                                        }
                                        RunTaskName::Multiple(ref names) => {
                                            let mut updated_names = vec![];
                                            for name in names {
                                                updated_names.push(get_namespaced_task_name(
                                                    namespace, name,
                                                ));
                                            }

                                            routing_info.name =
                                                RunTaskName::Multiple(updated_names);
                                        }
                                    };
                                }

                                RunTaskInfo::Routing(routing_info_vector)
                            }
                        };

                        self.run_task = Some(run_task);
                    }

                    if let Some(dependencies) = &self.dependencies {
                        self.dependencies = Some(
                            dependencies
                                .iter()
                                .map(|identifier| identifier.to_owned().with_namespace(namespace))
                                .collect(),
                        );
                    }
                }
            }
            None => (),
        };
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

        if task.deprecated.is_some() {
            self.deprecated = task.deprecated.clone();
        } else if override_values {
            self.deprecated = None;
        }

        if task.extend.is_some() {
            self.extend = task.extend.clone();
        } else if override_values {
            self.extend = None;
        }

        if task.workspace.is_some() {
            self.workspace = task.workspace.clone();
        } else if override_values {
            self.workspace = None;
        }

        if task.plugin.is_some() {
            self.plugin = task.plugin.clone();
        } else if override_values {
            self.plugin = None;
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

        if task.condition_script_runner_args.is_some() {
            self.condition_script_runner_args = task.condition_script_runner_args.clone();
        } else if override_values {
            self.condition_script_runner_args = None;
        }

        if task.ignore_errors.is_some() {
            self.ignore_errors = task.ignore_errors.clone();
        } else if override_values {
            self.ignore_errors = None;
        }

        if task.force.is_some() {
            self.force = task.force.clone();
        } else if override_values {
            self.force = None;
        }

        if task.env_files.is_some() {
            self.env_files = task.env_files.clone();
        } else if override_values {
            self.env_files = None;
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
            self.install_script =
                extend_script_value(self.install_script.clone(), task.install_script.clone());
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
            self.script = extend_script_value(self.script.clone(), task.script.clone());
        } else if override_values {
            self.script = None;
        }

        if task.script_runner.is_some() {
            self.script_runner = task.script_runner.clone();
        } else if override_values {
            self.script_runner = None;
        }

        if task.script_runner_args.is_some() {
            self.script_runner_args = task.script_runner_args.clone();
        } else if override_values {
            self.script_runner_args = None;
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

    /// Returns true if the task ignore_errors attribute is defined and true
    pub fn should_ignore_errors(self: &Task) -> bool {
        match self.ignore_errors {
            Some(value) => value,
            None => match self.force {
                Some(value) => {
                    legacy::show_deprecated_attriute_warning("force", "ignore_errors");

                    value
                }
                None => false,
            },
        }
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
                    deprecated: override_task.deprecated.clone(),
                    extend: override_task.extend.clone(),
                    workspace: self.workspace.clone(),
                    plugin: override_task.plugin.clone(),
                    watch: override_task.watch.clone(),
                    condition: override_task.condition.clone(),
                    condition_script: override_task.condition_script.clone(),
                    condition_script_runner_args: override_task
                        .condition_script_runner_args
                        .clone(),
                    ignore_errors: override_task.ignore_errors.clone(),
                    force: override_task.force.clone(),
                    env_files: override_task.env_files.clone(),
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
                    script_runner_args: override_task.script_runner_args.clone(),
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

    /// Returns the amount of actions defined on the task
    pub fn get_actions_count(self: &Task) -> u8 {
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

        actions_count
    }

    /// Returns true if the task has any actions on its own
    /// or if it modifies the environment in any way.
    pub fn is_actionable(self: &Task) -> bool {
        if self.disabled.unwrap_or(false) {
            return false;
        }

        let actions_count = self.get_actions_count();
        if actions_count > 0 {
            return true;
        }

        if self.install_crate.is_some() || self.install_script.is_some() {
            return true;
        }

        let mut actionable = match self.env {
            Some(ref value) => value.len() > 0,
            None => false,
        };
        if actionable {
            return true;
        }

        actionable = match self.env_files {
            Some(ref value) => value.len() > 0,
            None => false,
        };
        if actionable {
            return true;
        }

        actionable = match self.dependencies {
            Some(ref value) => value.len() > 0,
            None => false,
        };
        if actionable {
            return true;
        }

        actionable = match self.watch {
            Some(ref options) => match options {
                TaskWatchOptions::Boolean(value) => *value,
                _ => true,
            },
            None => false,
        };

        actionable
    }

    /// Returns true if the task is valid
    pub fn is_valid(self: &Task) -> bool {
        let actions_count = self.get_actions_count();

        if actions_count <= 1 {
            true
        } else {
            false
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
    /// if true, the task is hidden from the list of available tasks and also cannot be invoked directly from cli
    pub private: Option<bool>,
    /// if not false, this task is defined as deprecated
    pub deprecated: Option<DeprecationInfo>,
    /// Extend any task based on the defined name
    pub extend: Option<String>,
    /// Optional plugin used to execute the task
    pub plugin: Option<String>,
    /// set to true to watch for file changes and invoke the task operation
    pub watch: Option<TaskWatchOptions>,
    /// if provided all condition values must be met in order for the task to be invoked (will not stop dependencies)
    pub condition: Option<TaskCondition>,
    /// if script exit code is not 0, the command/script of this task will not be invoked, dependencies however will be
    pub condition_script: Option<ConditionScriptValue>,
    /// The script runner arguments before the script file path
    pub condition_script_runner_args: Option<Vec<String>>,
    /// if true, any error while executing the task will be printed but will not break the build
    pub ignore_errors: Option<bool>,
    /// DEPRECATED, replaced with ignore_errors
    pub force: Option<bool>,
    /// The env files to setup before running the task commands
    pub env_files: Option<Vec<EnvFile>>,
    /// The env vars to setup before running the task commands
    pub env: Option<IndexMap<String, EnvValue>>,
    /// The working directory for the task to execute its command/script
    pub cwd: Option<String>,
    /// if defined, the provided crate will be installed (if needed) before running the task
    pub install_crate: Option<InstallCrate>,
    /// additional cargo install arguments
    pub install_crate_args: Option<Vec<String>>,
    /// if defined, the provided script will be executed before running the task
    pub install_script: Option<ScriptValue>,
    /// The command to execute
    pub command: Option<String>,
    /// The command args
    pub args: Option<Vec<String>>,
    /// If command is not defined, and script is defined, the provided script will be executed
    pub script: Option<ScriptValue>,
    /// The script runner (defaults to cmd in windows and sh for other platforms)
    pub script_runner: Option<String>,
    /// The script runner arguments before the script file path
    pub script_runner_args: Option<Vec<String>>,
    /// The script file extension
    pub script_extension: Option<String>,
    /// The task name to execute
    pub run_task: Option<RunTaskInfo>,
    /// A list of tasks to execute before this task
    pub dependencies: Option<Vec<DependencyIdentifier>>,
    /// The rust toolchain used to invoke the command or install the needed crates/components
    pub toolchain: Option<ToolchainSpecifier>,
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

            if self.deprecated.is_none() && task.deprecated.is_some() {
                self.deprecated = task.deprecated.clone();
            }

            if self.extend.is_none() && task.extend.is_some() {
                self.extend = task.extend.clone();
            }

            if self.plugin.is_none() && task.plugin.is_some() {
                self.plugin = task.plugin.clone();
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

            if self.condition_script_runner_args.is_none()
                && task.condition_script_runner_args.is_some()
            {
                self.condition_script_runner_args = task.condition_script_runner_args.clone();
            }

            if self.ignore_errors.is_none() && task.ignore_errors.is_some() {
                self.ignore_errors = task.ignore_errors.clone();
            }

            if self.force.is_none() && task.force.is_some() {
                self.force = task.force.clone();
            }

            if self.env_files.is_none() && task.env_files.is_some() {
                self.env_files = task.env_files.clone();
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
                self.install_script =
                    extend_script_value(self.install_script.clone(), task.install_script.clone());
            }

            if self.command.is_none() && task.command.is_some() {
                self.command = task.command.clone();
            }

            if self.args.is_none() && task.args.is_some() {
                self.args = task.args.clone();
            }

            if self.script.is_none() && task.script.is_some() {
                self.script = extend_script_value(None, task.script.clone());
            }

            if self.script_runner.is_none() && task.script_runner.is_some() {
                self.script_runner = task.script_runner.clone();
            }

            if self.script_runner_args.is_none() && task.script_runner_args.is_some() {
                self.script_runner_args = task.script_runner_args.clone();
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

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Extend with more fine tuning options
pub struct ExtendOptions {
    /// Path to another makefile
    pub path: String,
    /// Enable optional extend (default to false)
    pub optional: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
/// Holds makefile extend value
pub enum Extend {
    /// Path to another makefile
    Path(String),
    /// Extend options for more fine tune control
    Options(ExtendOptions),
    /// Multiple extends list
    List(Vec<ExtendOptions>),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds properties to modify the core tasks
pub struct ModifyConfig {
    /// If true, all core tasks will be set to private (default false)
    pub private: Option<bool>,
    /// If set to some value, all core tasks are modified to: namespace::name for example default::build
    pub namespace: Option<String>,
}

impl ModifyConfig {
    /// Returns true if config modifications is needed based on the current state
    pub fn is_modifications_defined(self: &ModifyConfig) -> bool {
        if self.private.unwrap_or(false) {
            true
        } else {
            match self.namespace {
                Some(ref value) => value.len() > 0,
                None => false,
            }
        }
    }

    /// Returns the namespace prefix for task names
    pub fn get_namespace_prefix(self: &ModifyConfig) -> String {
        match self.namespace {
            Some(ref value) => get_namespaced_task_name(value, ""),
            None => "".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
/// Unstable cargo-make feature
pub enum UnstableFeature {
    /// Gracefully shutdown and then kill the running command on Ctrl+C signal
    CtrlCHandling,
}

impl UnstableFeature {
    /// Creates the env. variable name associated to the feature
    pub fn to_env_name(&self) -> String {
        let mut feature = serde_json::to_string(&self).unwrap();
        feature = feature.replace("\"", "");
        format!("CARGO_MAKE_UNSTABLE_FEATURE_{feature}", feature = feature)
    }

    /// Is the corresponding env. variable set?
    pub fn is_env_set(&self) -> bool {
        envmnt::is(self.to_env_name())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// Holds the configuration found in the makefile toml config section.
pub struct ConfigSection {
    /// If true, the default core tasks will not be loaded
    pub skip_core_tasks: Option<bool>,
    /// Modify core tasks config
    pub modify_core_tasks: Option<ModifyConfig>,
    /// Init task name which will be invoked at the start of every run
    pub init_task: Option<String>,
    /// End task name which will be invoked at the end of every run
    pub end_task: Option<String>,
    /// before_each task name which will be invoked before each task (except `init_task`)
    pub before_each_task: Option<String>,
    /// after_each task name which will be invoked after each task (except `end_task`)
    pub after_each_task: Option<String>,
    /// The name of the task to run in case of any error during the invocation of the flow
    pub on_error_task: Option<String>,
    /// The name of the task which runs legacy migration flows
    pub legacy_migration_task: Option<String>,
    /// Additional profile names to load
    pub additional_profiles: Option<Vec<String>>,
    /// Minimum cargo-make/makers version
    pub min_version: Option<String>,
    /// The task.workspace default value
    pub default_to_workspace: Option<bool>,
    /// do not load git env info (save on perf)
    pub skip_git_env_info: Option<bool>,
    /// do not load rust env info (save on perf)
    pub skip_rust_env_info: Option<bool>,
    /// do not load current crate env info (save on perf)
    pub skip_crate_env_info: Option<bool>,
    /// True to reduce console output for non CI execution
    pub reduce_output: Option<bool>,
    /// True to print time summary at the end of the flow
    pub time_summary: Option<bool>,
    /// Automatically load cargo aliases as cargo-make tasks
    pub load_cargo_aliases: Option<bool>,
    /// The project information member (used by workspaces)
    pub main_project_member: Option<String>,
    /// Invoked while loading the descriptor file but before loading any extended descriptor
    pub load_script: Option<ScriptValue>,
    /// acts like load_script if runtime OS is Linux (takes precedence over load_script)
    pub linux_load_script: Option<ScriptValue>,
    /// acts like load_script if runtime OS is Windows (takes precedence over load_script)
    pub windows_load_script: Option<ScriptValue>,
    /// acts like load_script if runtime OS is Mac (takes precedence over load_script)
    pub mac_load_script: Option<ScriptValue>,
    /// Enables unstable cargo-make features
    pub unstable_features: Option<IndexSet<UnstableFeature>>,
}

impl ConfigSection {
    /// Creates and returns a new instance.
    pub fn new() -> ConfigSection {
        Default::default()
    }

    /// Apply modifications
    pub fn apply(self: &mut ConfigSection, modify_config: &ModifyConfig) {
        match modify_config.namespace {
            Some(ref namespace) => {
                if self.init_task.is_some() {
                    self.init_task = Some(get_namespaced_task_name(
                        namespace,
                        &self.init_task.clone().unwrap(),
                    ));
                }

                if self.end_task.is_some() {
                    self.end_task = Some(get_namespaced_task_name(
                        namespace,
                        &self.end_task.clone().unwrap(),
                    ));
                }

                if self.before_each_task.is_some() {
                    self.before_each_task = Some(get_namespaced_task_name(
                        namespace,
                        &self.before_each_task.clone().unwrap(),
                    ));
                }

                if self.after_each_task.is_some() {
                    self.after_each_task = Some(get_namespaced_task_name(
                        namespace,
                        &self.after_each_task.clone().unwrap(),
                    ));
                }

                if self.on_error_task.is_some() {
                    self.on_error_task = Some(get_namespaced_task_name(
                        namespace,
                        &self.on_error_task.clone().unwrap(),
                    ));
                }

                if self.legacy_migration_task.is_some() {
                    self.legacy_migration_task = Some(get_namespaced_task_name(
                        namespace,
                        &self.legacy_migration_task.clone().unwrap(),
                    ));
                }
            }
            None => (),
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

        if extended.modify_core_tasks.is_some() {
            self.modify_core_tasks = extended.modify_core_tasks.clone();
        }

        if extended.init_task.is_some() {
            self.init_task = extended.init_task.clone();
        }

        if extended.end_task.is_some() {
            self.end_task = extended.end_task.clone();
        }

        if let Some(before_each_task) = &extended.before_each_task {
            self.before_each_task = Some(before_each_task.clone());
        }

        if let Some(after_each_task) = &extended.after_each_task {
            self.after_each_task = Some(after_each_task.clone());
        }

        if extended.on_error_task.is_some() {
            self.on_error_task = extended.on_error_task.clone();
        }

        if extended.legacy_migration_task.is_some() {
            self.legacy_migration_task = extended.legacy_migration_task.clone();
        }

        if extended.additional_profiles.is_some() {
            self.additional_profiles = extended.additional_profiles.clone();
        }

        if extended.min_version.is_some() {
            self.min_version = extended.min_version.clone();
        }

        if extended.default_to_workspace.is_some() {
            self.default_to_workspace = extended.default_to_workspace.clone();
        }

        if extended.skip_git_env_info.is_some() {
            self.skip_git_env_info = extended.skip_git_env_info.clone();
        }

        if extended.skip_rust_env_info.is_some() {
            self.skip_rust_env_info = extended.skip_rust_env_info.clone();
        }

        if extended.skip_crate_env_info.is_some() {
            self.skip_crate_env_info = extended.skip_crate_env_info.clone();
        }

        if extended.reduce_output.is_some() {
            self.reduce_output = extended.reduce_output.clone();
        }

        if extended.time_summary.is_some() {
            self.time_summary = extended.time_summary.clone();
        }

        if extended.load_cargo_aliases.is_some() {
            self.load_cargo_aliases = extended.load_cargo_aliases.clone();
        }

        if extended.main_project_member.is_some() {
            self.main_project_member = extended.main_project_member.clone();
        }

        if extended.load_script.is_some() {
            self.load_script =
                extend_script_value(self.load_script.clone(), extended.load_script.clone());
        }

        if extended.linux_load_script.is_some() {
            self.linux_load_script = extend_script_value(
                self.linux_load_script.clone(),
                extended.linux_load_script.clone(),
            );
        }

        if extended.windows_load_script.is_some() {
            self.windows_load_script = extend_script_value(
                self.windows_load_script.clone(),
                extended.windows_load_script.clone(),
            );
        }

        if extended.mac_load_script.is_some() {
            self.mac_load_script = extend_script_value(
                self.mac_load_script.clone(),
                extended.mac_load_script.clone(),
            );
        }

        if let Some(extended_unstable_features) = extended.unstable_features.clone() {
            if let Some(unstable_features) = &mut self.unstable_features {
                unstable_features.extend(extended_unstable_features);
            } else {
                self.unstable_features = Some(extended_unstable_features);
            }
        }
    }

    /// Returns the load script based on the current platform
    pub fn get_load_script(self: &ConfigSection) -> Option<ScriptValue> {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds the entire configuration such as task definitions and env vars
pub struct Config {
    /// Runtime config
    pub config: ConfigSection,
    /// The env files to setup before running the flow
    pub env_files: Vec<EnvFile>,
    /// The env vars to setup before running the flow
    pub env: IndexMap<String, EnvValue>,
    /// The env scripts to execute before running the flow
    pub env_scripts: Vec<String>,
    /// All task definitions
    pub tasks: IndexMap<String, Task>,
    /// All plugin definitions
    pub plugins: Option<Plugins>,
}

impl Config {
    /// Apply modifications
    pub fn apply(self: &mut Config, modify_config: &ModifyConfig) {
        self.config.apply(&modify_config);

        let namespace = match modify_config.namespace {
            Some(ref namespace) => namespace,
            None => "",
        };

        let mut modified_tasks = IndexMap::<String, Task>::new();

        for (key, value) in self.tasks.iter() {
            let namespaced_task = get_namespaced_task_name(namespace, &key);
            let mut task = value.clone();

            task.apply(&modify_config);

            modified_tasks.insert(namespaced_task, task);
        }

        self.tasks = modified_tasks;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
/// Holds the entire externally read configuration such as task definitions and env vars where all values are optional
pub struct ExternalConfig {
    /// Path to another toml file to extend
    pub extend: Option<Extend>,
    /// Runtime config
    pub config: Option<ConfigSection>,
    /// The env files to setup before running the flow
    pub env_files: Option<Vec<EnvFile>>,
    /// The env vars to setup before running the flow
    pub env: Option<IndexMap<String, EnvValue>>,
    /// The env scripts to execute before running the flow
    pub env_scripts: Option<Vec<String>>,
    /// All task definitions
    pub tasks: Option<IndexMap<String, Task>>,
    /// All plugin definitions
    pub plugins: Option<Plugins>,
}

impl ExternalConfig {
    /// Creates and returns a new instance.
    pub fn new() -> ExternalConfig {
        Default::default()
    }
}

#[derive(Serialize, Clone, Debug)]
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
