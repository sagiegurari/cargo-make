CREATE TABLE tasks
(
    id                           INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    clear                        BOOLEAN,
    description                  TEXT,
    category                     TEXT,
    disabled                     BOOLEAN,
    private                      BOOLEAN,
    deprecated                   JSONB,
    extend                       TEXT,
    workspace                    BOOLEAN,
    plugin                       TEXT,
    watch                        JSONB,
    condition                    JSONB,
    condition_script             JSONB,
    condition_script_runner_args JSONB,
    ignore_errors                BOOLEAN,
    force                        BOOLEAN,
    env_files                    JSONB,
    env                          JSONB,
    cwd                          TEXT,
    alias                        TEXT,
    linux_alias                  TEXT,
    windows_alias                TEXT,
    mac_alias                    TEXT,
    install_crate                JSONB,
    install_crate_args           JSONB,
    install_script               JSONB,
    command                      TEXT,
    args                         JSONB,
    script                       JSONB,
    script_runner                TEXT,
    script_runner_args           JSONB,
    script_extension             TEXT,
    run_task                     JSONB,
    dependencies                 JSONB,
    toolchain                    JSONB,
    linux                        JSONB,
    windows                      JSONB,
    mac                          JSONB
);

CREATE TABLE steps
(
    id     INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name   TEXT  NOT NULL,
    config JSONB NOT NULL
);

CREATE TABLE execution_plans
(
    id           INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    steps        JSONB NOT NULL,
    steps_to_run JSONB NOT NULL,
    name         TEXT  NOT NULL
);
