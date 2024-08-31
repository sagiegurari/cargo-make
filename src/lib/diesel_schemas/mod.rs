pub mod custom_to_fro;

use diesel::table;

table! {
    // FlowInfo to store various flow-related information
    flow_info (id) {
        id -> Integer,
        config -> Text,  // Assuming Config can be serialized into a JSON-like text format
        task -> Text,
        env_info -> Text,  // Assuming EnvInfo can be stored as JSON-like text
        disable_workspace -> Bool,
        disable_on_error -> Bool,
        allow_private -> Bool,
        skip_init_end_tasks -> Bool,
        skip_tasks_pattern -> Nullable<Text>,
        cli_arguments -> Nullable<Text>,  // Optional vector serialized to JSON/text
    }
}

table! {
    // Stores the state of a flow
    flow_states (id) {
        id -> Integer,
        time_summary -> Jsonb,
        forced_plugin -> Nullable<Text>,
    }
}

table! {
    // ModifyConfig for modifying core tasks
    modify_configs (id) {
        id -> Integer,
        private -> Nullable<Bool>,
        namespace -> Nullable<Text>,
    }
}

table! {
    // ConfigSection, the main configuration store
    config_sections (id) {
        id -> Integer,
        skip_core_tasks -> Nullable<Bool>,
        modify_core_tasks -> Nullable<Text>,  // Assuming ModifyConfig is serialized to JSON-like format
        init_task -> Nullable<Text>,
        end_task -> Nullable<Text>,
        on_error_task -> Nullable<Text>,
        legacy_migration_task -> Nullable<Text>,
        additional_profiles -> Nullable<Text>,  // Assuming Vec<String> is serialized to JSON-like format
        min_version -> Nullable<Text>,
        default_to_workspace -> Nullable<Bool>,
        skip_git_env_info -> Nullable<Bool>,
        skip_rust_env_info -> Nullable<Bool>,
        skip_crate_env_info -> Nullable<Bool>,
        reduce_output -> Nullable<Bool>,
        time_summary -> Nullable<Bool>,
        load_cargo_aliases -> Nullable<Bool>,
        main_project_member -> Nullable<Text>,
        load_script -> Nullable<Text>,  // Assuming ScriptValue can be serialized to JSON-like format
        linux_load_script -> Nullable<Text>,
        windows_load_script -> Nullable<Text>,
        mac_load_script -> Nullable<Text>,
        unstable_features -> Nullable<Text>,  // Assuming IndexSet<UnstableFeature> can be serialized to JSON-like format
    }
}

table! {
    // Task, stores a single task's configuration
    tasks (id) {
        id -> Integer,
        clear -> Nullable<Bool>,
        description -> Nullable<Text>,
        category -> Nullable<Text>,
        disabled -> Nullable<Bool>,
        private -> Nullable<Bool>,
        deprecated -> Nullable<Text>,  // Assuming DeprecationInfo serialized to text
        extend -> Nullable<Text>,
        workspace -> Nullable<Bool>,
        plugin -> Nullable<Text>,
        watch -> Nullable<Text>,  // Assuming TaskWatchOptions serialized to text
        condition -> Nullable<Text>,  // Assuming TaskCondition serialized to text
        condition_script -> Nullable<Text>,
        condition_script_runner_args -> Nullable<Text>,
        ignore_errors -> Nullable<Bool>,
        force -> Nullable<Bool>,
        env_files -> Nullable<Text>,  // Assuming Vec<EnvFile> serialized to text
        env -> Nullable<Text>,  // Assuming IndexMap<String, EnvValue> serialized to text
        cwd -> Nullable<Text>,
        alias -> Nullable<Text>,
        linux_alias -> Nullable<Text>,
        windows_alias -> Nullable<Text>,
        mac_alias -> Nullable<Text>,
        install_crate -> Nullable<Text>,  // Assuming InstallCrate serialized to text
        install_crate_args -> Nullable<Text>,
        install_script -> Nullable<Text>,  // Assuming ScriptValue serialized to text
        command -> Nullable<Text>,
        args -> Nullable<Text>,
        script -> Nullable<Text>,  // Assuming ScriptValue serialized to text
        script_runner -> Nullable<Text>,
        script_runner_args -> Nullable<Text>,
        script_extension -> Nullable<Text>,
        run_task -> Nullable<Text>,  // Assuming RunTaskInfo serialized to text
        dependencies -> Nullable<Text>,  // Assuming Vec<DependencyIdentifier> serialized to text
        toolchain -> Nullable<Text>,
        linux -> Nullable<Text>,  // Assuming PlatformOverrideTask serialized to text
        windows -> Nullable<Text>,
        mac -> Nullable<Text>,
    }
}

table! {
    steps (id) {
        id -> Integer,
        name -> Text,
        config -> Jsonb,  // Assuming Task serialized to text
    }
}

table! {
    execution_plans (id) {
        id -> Integer,
        steps -> Jsonb,  // Assuming Vec<Step> serialized to JSON/text
        steps_to_run -> Jsonb,  // Assuming std::ops::Range<usize> serialized to JSON/text
        name -> Text,
    }
}
