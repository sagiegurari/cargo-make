pub mod custom_to_fro;

use diesel::table;

table! {
    // Task table stores individual task configurations
    tasks (id) {
        id -> Integer,
        clear -> Nullable<Bool>,
        description -> Nullable<Text>,
        category -> Nullable<Text>,
        disabled -> Nullable<Bool>,
        private -> Nullable<Bool>,
        deprecated -> Nullable<Jsonb>,  // Assuming DeprecationInfo is serialized to JSONB
        extend -> Nullable<Text>,
        workspace -> Nullable<Bool>,
        plugin -> Nullable<Text>,
        watch -> Nullable<Jsonb>,  // Assuming TaskWatchOptions is serialized to JSONB
        condition -> Nullable<Jsonb>,  // Assuming TaskCondition is serialized to JSONB
        condition_script -> Nullable<Jsonb>,  // Assuming ConditionScriptValue is serialized to JSONB
        condition_script_runner_args -> Nullable<Jsonb>,  // Assuming Vec<String> is serialized to JSONB
        ignore_errors -> Nullable<Bool>,
        force -> Nullable<Bool>,
        env_files -> Nullable<Jsonb>,  // Assuming Vec<EnvFile> is serialized to JSONB
        env -> Nullable<Jsonb>,  // Assuming IndexMap<String, EnvValue> is serialized to JSONB
        cwd -> Nullable<Text>,
        alias -> Nullable<Text>,
        linux_alias -> Nullable<Text>,
        windows_alias -> Nullable<Text>,
        mac_alias -> Nullable<Text>,
        install_crate -> Nullable<Jsonb>,  // Assuming InstallCrate is serialized to JSONB
        install_crate_args -> Nullable<Jsonb>,  // Assuming Vec<String> is serialized to JSONB
        install_script -> Nullable<Jsonb>,  // Assuming ScriptValue is serialized to JSONB
        command -> Nullable<Text>,
        args -> Nullable<Jsonb>,  // Assuming Vec<String> is serialized to JSONB
        script -> Nullable<Jsonb>,  // Assuming ScriptValue is serialized to JSONB
        script_runner -> Nullable<Text>,
        script_runner_args -> Nullable<Jsonb>,  // Assuming Vec<String> is serialized to JSONB
        script_extension -> Nullable<Text>,
        run_task -> Nullable<Jsonb>,  // Assuming RunTaskInfo is serialized to JSONB
        dependencies -> Nullable<Jsonb>,  // Assuming Vec<DependencyIdentifier> is serialized to JSONB
        toolchain -> Nullable<Jsonb>,  // Assuming ToolchainSpecifier is serialized to JSONB
        linux -> Nullable<Jsonb>,  // Assuming PlatformOverrideTask is serialized to JSONB
        windows -> Nullable<Jsonb>,
        mac -> Nullable<Jsonb>,
    }
}

table! {
    // Step table stores execution steps
    steps (id) {
        id -> Integer,
        name -> Text,
        config -> Jsonb,  // Assuming Task struct is serialized to JSONB
    }
}

table! {
    // ExecutionPlan table stores a full execution plan
    execution_plans (id) {
        id -> Integer,
        steps -> Jsonb,  // Assuming Vec<Step> is serialized to JSONB
        steps_to_run -> Jsonb,  // Assuming std::ops::Range<usize> is serialized to JSONB
        name -> Text,
    }
}
