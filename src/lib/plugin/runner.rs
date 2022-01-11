//! # runner
//!
//! Runs task plugins.
//!

#[cfg(test)]
#[path = "runner_test.rs"]
mod runner_test;

use crate::environment;
use crate::plugin::sdk;
use crate::plugin::types::Plugin;
use crate::scriptengine::duck_script;
use crate::types::{Config, DeprecationInfo, FlowInfo, FlowState, RunTaskOptions, Step};
use duckscript::runner::run_script;
use duckscript::types::command::Commands;
use duckscript::types::error::ScriptError;
use duckscript::types::runtime::Context;
use indexmap::IndexMap;
use serde_json::json;
use std::cell::RefCell;
use std::env;
use std::rc::Rc;

/// Resolve aliases to different plugins, checking for cycles
fn get_plugin_name_recursive(
    aliases: &IndexMap<String, String>,
    name: &str,
    seen: &mut Vec<String>,
) -> String {
    let name_string = name.to_string();
    if seen.contains(&name_string) {
        error!("Detected cycle while resolving plugin alias: {}", name);
    }
    seen.push(name_string);

    match aliases.get(name) {
        Some(target_name) => get_plugin_name_recursive(aliases, target_name, seen),
        None => name.to_string(),
    }
}

fn get_plugin(config: &Config, plugin_name: &str) -> Option<(String, Plugin)> {
    match &config.plugins {
        Some(plugins_config) => {
            let normalized_plugin_name = match plugins_config.aliases {
                Some(ref aliases) => {
                    let mut seen = vec![];
                    get_plugin_name_recursive(aliases, plugin_name, &mut seen)
                }
                None => plugin_name.to_string(),
            };

            match plugins_config.plugins.get(&normalized_plugin_name) {
                Some(plugin) => Some((normalized_plugin_name, plugin.clone())),
                None => None,
            }
        }
        None => None,
    }
}

fn run_plugin(
    flow_info: &FlowInfo,
    flow_state: Rc<RefCell<FlowState>>,
    step: &Step,
    plugin: Plugin,
    impl_plugin_name: String,
) {
    debug!(
        "Running Task: {} via plugin: {} script:\n{}",
        &step.name, &impl_plugin_name, &plugin.script
    );

    let cli_arguments = match flow_info.cli_arguments.clone() {
        Some(cli_arguments) => cli_arguments.clone(),
        None => vec![],
    };

    let mut context = duck_script::create_common_context(&cli_arguments);

    let mut script_text = "exit_on_error true\n".to_string();
    setup_script_globals(
        &mut context,
        flow_info,
        step,
        &impl_plugin_name,
        &mut script_text,
    );
    script_text.push_str(&plugin.script);

    match load_sdk(flow_info, flow_state, step, &mut context.commands) {
        Ok(_) => {
            let directory = env::current_dir();

            match run_script(&script_text, context) {
                Ok(_) => (),
                Err(error) => error!("Error while running plugin: {}", error),
            };

            // revert to originl working directory
            if let Ok(directory_path) = directory {
                let path = directory_path.to_string_lossy().into_owned();
                environment::setup_cwd(Some(&path));
            }
        }
        Err(error) => error!("Unable to load duckscript SDK: {}", error),
    };
}

fn setup_script_globals(
    context: &mut Context,
    flow_info: &FlowInfo,
    step: &Step,
    impl_plugin_name: &str,
    script: &mut String,
) {
    context
        .variables
        .insert("flow.task.name".to_string(), flow_info.task.to_string());
    script.push_str("flow.cli.args = array\n");
    if let Some(ref cli_arguments) = flow_info.cli_arguments {
        for arg in cli_arguments {
            script.push_str("array_push ${flow.cli.args} \"");
            script.push_str(&arg.replace("$", "\\$"));
            script.push_str("\"\n");
        }
    }
    context
        .variables
        .insert("plugin.impl.name".to_string(), impl_plugin_name.to_string());

    setup_script_globals_for_task(context, step, script);
}

fn setup_script_globals_for_task(context: &mut Context, step: &Step, script: &mut String) {
    // all task data as json
    let task = step.config.clone();
    let json_string = json!(task);
    context
        .variables
        .insert("task.as_json".to_string(), json_string.to_string());

    // meta info
    context.variables.insert(
        "task.has_condition".to_string(),
        (task.condition.is_some() || task.condition_script.is_some()).to_string(),
    );
    context.variables.insert(
        "task.has_env".to_string(),
        (task.env_files.is_some() || task.env.is_some()).to_string(),
    );
    context.variables.insert(
        "task.has_install_instructions".to_string(),
        (task.install_crate.is_some()
            || task.install_crate_args.is_some()
            || task.install_script.is_some())
        .to_string(),
    );
    context.variables.insert(
        "task.has_command".to_string(),
        task.command.is_some().to_string(),
    );
    context.variables.insert(
        "task.has_script".to_string(),
        task.script.is_some().to_string(),
    );
    context.variables.insert(
        "task.has_run_task".to_string(),
        task.run_task.is_some().to_string(),
    );
    context.variables.insert(
        "task.has_dependencies".to_string(),
        task.dependencies.is_some().to_string(),
    );
    context.variables.insert(
        "task.has_toolchain_specifier".to_string(),
        task.toolchain.is_some().to_string(),
    );

    context
        .variables
        .insert("task.name".to_string(), step.name.clone());

    context.variables.insert(
        "task.description".to_string(),
        task.description.unwrap_or("".to_string()),
    );
    context.variables.insert(
        "task.category".to_string(),
        task.category.unwrap_or("".to_string()),
    );
    context.variables.insert(
        "task.disabled".to_string(),
        task.disabled.unwrap_or(false).to_string(),
    );
    context.variables.insert(
        "task.private".to_string(),
        task.private.unwrap_or(false).to_string(),
    );
    let deprecated = match task.deprecated {
        Some(value) => match value {
            DeprecationInfo::Boolean(value) => value,
            DeprecationInfo::Message(_) => true,
        },
        None => false,
    };
    context
        .variables
        .insert("task.deprecated".to_string(), deprecated.to_string());
    context.variables.insert(
        "task.workspace".to_string(),
        task.workspace.unwrap_or(false).to_string(),
    );
    context.variables.insert(
        "task.plugin.name".to_string(),
        task.plugin.unwrap_or("".to_string()),
    );
    context
        .variables
        .insert("task.watch".to_string(), task.watch.is_some().to_string());
    context.variables.insert(
        "task.ignore_errors".to_string(),
        task.ignore_errors.unwrap_or(false).to_string(),
    );
    context.variables.insert(
        "task.cwd".to_string(),
        task.cwd.unwrap_or("".to_string()).to_string(),
    );
    context.variables.insert(
        "task.command".to_string(),
        task.command.unwrap_or("".to_string()).to_string(),
    );
    script.push_str("task.args = array\n");
    if let Some(args) = task.args {
        for arg in &args {
            script.push_str("array_push ${task.args} \"");
            script.push_str(&arg.replace("$", "\\$"));
            script.push_str("\"\n");
        }
    }
    context.variables.insert(
        "task.script_runner".to_string(),
        task.script_runner.unwrap_or("".to_string()).to_string(),
    );
    script.push_str("task.script_runner_args = array\n");
    if let Some(args) = task.script_runner_args {
        for arg in &args {
            script.push_str("array_push ${task.script_runner_args} \"");
            script.push_str(arg);
            script.push_str("\"\n");
        }
    }
    context.variables.insert(
        "task.script_extension".to_string(),
        task.script_extension.unwrap_or("".to_string()).to_string(),
    );
}

fn load_sdk(
    flow_info: &FlowInfo,
    flow_state: Rc<RefCell<FlowState>>,
    step: &Step,
    commands: &mut Commands,
) -> Result<(), ScriptError> {
    duck_script::load_sdk(commands, Some(flow_info), Some(flow_state.clone()))?;
    sdk::load(flow_info, flow_state, step, commands)?;

    Ok(())
}

pub(crate) fn run_task(
    flow_info: &FlowInfo,
    flow_state: Rc<RefCell<FlowState>>,
    step: &Step,
    options: &RunTaskOptions,
) -> bool {
    if !options.plugins_enabled {
        false
    } else {
        let plugin_name_option = match flow_state.borrow().forced_plugin {
            Some(ref value) => Some(value.clone()),
            None => match step.config.plugin {
                Some(ref value) => Some(value.clone()),
                None => None,
            },
        };

        match plugin_name_option {
            Some(ref plugin_name) => match get_plugin(&flow_info.config, plugin_name) {
                Some((normalized_plugin_name, plugin)) => {
                    debug!(
                        "Running Task: {} via plugin: {}",
                        &step.name, &normalized_plugin_name
                    );

                    run_plugin(flow_info, flow_state, step, plugin, normalized_plugin_name);

                    true
                }
                None => {
                    error!(
                        "Invalid task: {}, unknown plugin: {}",
                        &step.name, plugin_name
                    );
                    false
                }
            },
            None => false,
        }
    }
}
