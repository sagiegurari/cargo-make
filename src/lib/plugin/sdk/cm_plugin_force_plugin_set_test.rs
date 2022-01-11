use super::*;
use crate::descriptor::descriptor_deserializer;
use crate::runner;
use crate::types::{CrateInfo, EnvInfo, FlowInfo};
use git_info::types::GitInfo;
use rust_info::types::RustInfo;

#[test]
fn force_plugin_set_and_clear_flow_test() {
    let makefile_string = r#"

env_files = []
env_scripts = []

[env]

[config]
skip_core_tasks = true
skip_git_env_info = true
skip_rust_env_info = true
skip_crate_env_info = true

[plugins.impl.force]
script = '''
plugin_force_set = get_env FORCE_PLUGIN_SET_AND_CLEAR_FLOW_TEST_SET
plugin_force_set = eq "${plugin_force_set}" 1

if eq ${task.name} force_3
    cm_plugin_force_plugin_clear
elif ${plugin_force_set}
    set_env FORCE_PLUGIN_SET_AND_CLEAR_FLOW_TEST_SET_ALREADY 1
else
    cm_plugin_force_plugin_set
    set_env FORCE_PLUGIN_SET_AND_CLEAR_FLOW_TEST_SET 1
end
'''

[tasks.force_flow]
dependencies = ["force_1", "force_2", "force_3", "force_4"]

[tasks.force_1]
plugin = "force"
command = "exit"
args = ["1"]

[tasks.force_2]
command = "exit"
args = ["1"]

[tasks.force_3]
command = "exit"
args = ["1"]

[tasks.force_4]
script_runner = "@duckscript"
script = '''
set_env FORCE_PLUGIN_SET_AND_CLEAR_FLOW_TEST_SET_4 1
'''
"#;

    let config = descriptor_deserializer::load_config(&makefile_string, false);

    let flow_info = FlowInfo {
        config,
        task: "force_flow".to_string(),
        env_info: EnvInfo {
            rust_info: RustInfo::new(),
            crate_info: CrateInfo::new(),
            git_info: GitInfo::new(),
            ci_info: ci_info::get(),
        },
        disable_workspace: false,
        disable_on_error: false,
        allow_private: false,
        skip_init_end_tasks: false,
        skip_tasks_pattern: None,
        cli_arguments: None,
    };

    assert!(!envmnt::exists("FORCE_PLUGIN_SET_AND_CLEAR_FLOW_TEST_SET"));
    assert!(!envmnt::exists(
        "FORCE_PLUGIN_SET_AND_CLEAR_FLOW_TEST_SET_ALREADY"
    ));
    assert!(!envmnt::exists(
        "FORCE_PLUGIN_SET_AND_CLEAR_FLOW_TEST_SET_4"
    ));

    runner::run_flow(&flow_info, Rc::new(RefCell::new(FlowState::new())), false);

    assert!(envmnt::is_equal(
        "FORCE_PLUGIN_SET_AND_CLEAR_FLOW_TEST_SET",
        "1"
    ));
    assert!(envmnt::is_equal(
        "FORCE_PLUGIN_SET_AND_CLEAR_FLOW_TEST_SET_ALREADY",
        "1"
    ));
    assert!(envmnt::is_equal(
        "FORCE_PLUGIN_SET_AND_CLEAR_FLOW_TEST_SET_4",
        "1"
    ));
}
