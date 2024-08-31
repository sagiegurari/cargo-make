CREATE TABLE flow_info
(
    id                  INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    config              TEXT    NOT NULL,
    task                TEXT    NOT NULL,
    env_info            TEXT    NOT NULL,
    disable_workspace   BOOLEAN NOT NULL,
    disable_on_error    BOOLEAN NOT NULL,
    allow_private       BOOLEAN NOT NULL,
    skip_init_end_tasks BOOLEAN NOT NULL,
    skip_tasks_pattern  TEXT,
    cli_arguments       TEXT
);

CREATE TABLE flow_states
(
    id            INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    time_summary  TEXT NOT NULL,
    forced_plugin TEXT
);

CREATE TABLE modify_configs
(
    id        INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    private   BOOLEAN,
    namespace TEXT
);

CREATE TABLE config_sections
(
    id                    INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    skip_core_tasks       BOOLEAN,
    modify_core_tasks     TEXT,
    init_task             TEXT,
    end_task              TEXT,
    on_error_task         TEXT,
    legacy_migration_task TEXT,
    additional_profiles   TEXT,
    min_version           TEXT,
    default_to_workspace  BOOLEAN,
    skip_git_env_info     BOOLEAN,
    skip_rust_env_info    BOOLEAN,
    skip_crate_env_info   BOOLEAN,
    reduce_output         BOOLEAN,
    time_summary          BOOLEAN,
    load_cargo_aliases    BOOLEAN,
    main_project_member   TEXT,
    load_script           TEXT,
    linux_load_script     TEXT,
    windows_load_script   TEXT,
    mac_load_script       TEXT,
    unstable_features     TEXT
);

CREATE TABLE tasks
(
    id                           INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    clear                        BOOLEAN,
    description                  TEXT,
    category                     TEXT,
    disabled                     BOOLEAN,
    private                      BOOLEAN,
    deprecated                   TEXT,
    extend                       TEXT,
    workspace                    BOOLEAN,
    plugin                       TEXT,
    watch                        TEXT,
    condition                    TEXT,
    condition_script             TEXT,
    condition_script_runner_args TEXT,
    ignore_errors                BOOLEAN,
    force                        BOOLEAN,
    env_files                    TEXT,
    env                          TEXT,
    cwd                          TEXT,
    alias                        TEXT,
    linux_alias                  TEXT,
    windows_alias                TEXT,
    mac_alias                    TEXT,
    install_crate                TEXT,
    install_crate_args           TEXT,
    install_script               TEXT,
    command                      TEXT,
    args                         TEXT,
    script                       TEXT,
    script_runner                TEXT,
    script_runner_args           TEXT,
    script_extension             TEXT,
    run_task                     TEXT,
    dependencies                 TEXT,
    toolchain                    TEXT,
    linux                        TEXT,
    windows                      TEXT,
    mac                          TEXT
);

CREATE TABLE steps
(
    id     INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name   TEXT NOT NULL,
    config TEXT NOT NULL
);

CREATE TABLE execution_plans
(
    id           INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    steps        TEXT NOT NULL,
    steps_to_run TEXT NOT NULL,
    name         TEXT NOT NULL
);

