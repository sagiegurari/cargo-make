// @generated automatically by Diesel CLI.

diesel::table! {
    execution_plans (id) {
        id -> Int4,
        steps -> Jsonb,
        steps_to_run -> Jsonb,
        name -> Text,
    }
}

diesel::table! {
    steps (id) {
        id -> Int4,
        name -> Text,
        config -> Jsonb,
    }
}

diesel::table! {
    tasks (id) {
        id -> Int4,
        clear -> Nullable<Bool>,
        description -> Nullable<Text>,
        category -> Nullable<Text>,
        disabled -> Nullable<Bool>,
        private -> Nullable<Bool>,
        deprecated -> Nullable<Jsonb>,
        extend -> Nullable<Text>,
        workspace -> Nullable<Bool>,
        plugin -> Nullable<Text>,
        watch -> Nullable<Jsonb>,
        condition -> Nullable<Jsonb>,
        condition_script -> Nullable<Jsonb>,
        condition_script_runner_args -> Nullable<Jsonb>,
        ignore_errors -> Nullable<Bool>,
        force -> Nullable<Bool>,
        env_files -> Nullable<Jsonb>,
        env -> Nullable<Jsonb>,
        cwd -> Nullable<Text>,
        alias -> Nullable<Text>,
        linux_alias -> Nullable<Text>,
        windows_alias -> Nullable<Text>,
        mac_alias -> Nullable<Text>,
        install_crate -> Nullable<Jsonb>,
        install_crate_args -> Nullable<Jsonb>,
        install_script -> Nullable<Jsonb>,
        command -> Nullable<Text>,
        args -> Nullable<Jsonb>,
        script -> Nullable<Jsonb>,
        script_runner -> Nullable<Text>,
        script_runner_args -> Nullable<Jsonb>,
        script_extension -> Nullable<Text>,
        run_task -> Nullable<Jsonb>,
        dependencies -> Nullable<Jsonb>,
        toolchain -> Nullable<Jsonb>,
        linux -> Nullable<Jsonb>,
        windows -> Nullable<Jsonb>,
        mac -> Nullable<Jsonb>,
    }
}

diesel::allow_tables_to_appear_in_same_query!(execution_plans, steps, tasks,);
