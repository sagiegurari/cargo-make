| Date        | Version | Description |
| ----------- | ------- | ----------- |
| 2017-12-06  | v0.7.10 | Workspace support fixes for windows  |
| 2017-12-03  | v0.7.5  | Support conditional skipping of workspace members from flow (#42)  |
| 2017-10-25  | v0.7.4  | kcov support for excluding lines/blocks from coverage report  |
| 2017-10-23  | v0.7.2  | Added workspace level coverage task without the need to run full CI flow (workspace-coverage) |
| 2017-10-21  | v0.7.1  | Added support for cross platform scripts using script runner @shell |
| 2017-10-20  | v0.7.0  | Refactored how rust code based scripts are defined to enable future expansion of this feature. |
| 2017-10-12  | v0.6.5  | ci-flow and build-flow will build/run benches and examples based on new environment variables: CARGO_MAKE_BUILD_EXAMPLES, CARGO_MAKE_BUILD_BENCH and CARGO_MAKE_RUN_BENCH |
| 2017-10-08  | v0.6.1  | Tasks can now run rust code using rust_script attribute and tasks can no longer hold multiple different action definitions. |
| 2017-09-29  | v0.5.3  | Added support to evaluate env var values from script output and define cwd on task level |
| 2017-09-24  | v0.5.2  | Added various check and build tasks |
| 2017-09-21  | v0.5.0  | Disable by default coverage flow on mac |
| 2017-09-18  | v0.4.1  | Added watch-flow task |
| 2017-09-17  | v0.3.77 | Automatically define install_crate and enable cargo install arguments via install_crate_args |
| 2017-09-16  | v0.3.76 | Build failure will not cause panic but will exit |
| 2017-09-12  | v0.3.75 | By default build and test should work with "--all-features" flag |
| 2017-08-23  | v0.3.66 | Add support for workspace glob members |
| 2017-08-22  | v0.3.64 | Cargo.toml parsing fix |
| 2017-08-21  | v0.3.63 | Add support for Cargo.toml workspace.exclude |
| 2017-08-21  | v0.3.62 | Add workspace member detection based on dependency paths |
| 2017-08-20  | v0.3.59 | Support load_script platform overrides |
| 2017-08-19  | v0.3.58 | Added load_script capability |
| 2017-08-18  | v0.3.56 | Set environment variables during task invocation |
| 2017-08-09  | v0.3.53 | Added new condition types: env, env_set and env_not_set |
| 2017-08-09  | v0.3.51 | Added experimental cli arg to enable access unsupported experimental predefined tasks |
| 2017-08-08  | v0.3.49 | Added condition attribute |
| 2017-08-06  | v0.3.46 | Added bintray upload task |
| 2017-08-02  | v0.3.43 | Added --env/-e cli args to set environment variables via command line |
| 2017-08-01  | v0.3.41 | Added github-publish task |
| 2017-07-28  | v0.3.38 | Added run_script which allows executing sub tasks |
| 2017-07-25  | v0.3.37 | Added condition script capability for tasks |
| 2017-07-22  | v0.3.36 | Added coverage-lcov task (not fully tested) |
| 2017-07-21  | v0.3.34 | Added coverage-tarpaulin task |
| 2017-07-21  | v0.3.33 | Added more environment variables for workspace support |
| 2017-07-20  | v0.3.32 | Added --list-all-steps cli option |
| 2017-07-17  | v0.3.28 | workspace level ci flow |
| 2017-07-16  | v0.3.27 | cargo make ci-flow on travis now automatically runs code coverage and uploads to codecov |
| 2017-07-16  | v0.3.25 | New --no-workspace cli arg |
| 2017-07-15  | v0.3.24 | Workspace support |
| 2017-07-14  | v0.3.23 | Added codecov task in default toml |
| 2017-07-14  | v0.3.20 | Added coverage task in default toml |
| 2017-07-14  | v0.3.16 | Added more environment variables based on target environment and rust compiler |
| 2017-07-13  | v0.3.15 | Added common init and end tasks |
| 2017-07-10  | v0.3.13 | cargo-make now defines rust version env vars |
| 2017-07-09  | v0.3.11 | cargo-make now defines env vars based on project git repo information |
| 2017-07-06  | v0.3.10 | cargo-make now defines env vars based on project Cargo.toml |
| 2017-07-05  | v0.3.6  | Added --cwd cli arg to enable setting working directory |
| 2017-07-04  | v0.3.5  | Added clippy task |
| 2017-07-03  | v0.3.4  | Added --print-steps cli arg |
| 2017-07-02  | v0.3.1  | Added CARGO_MAKE_TASK env var holding the main task name |
| 2017-07-02  | v0.3.0  | Renamed few cli options |
| 2017-07-02  | v0.2.20 | Added -v and --verbose cli arg |
| 2017-07-01  | v0.2.19 | Added extend config level attribute |
| 2017-06-30  | v0.2.17 | Added force task attribute |
| 2017-06-28  | v0.2.12 | Published website |
| 2017-06-28  | v0.2.8  | Platform specific task override |
| 2017-06-26  | v0.2.7  | Platform specific alias |
| 2017-06-26  | v0.2.6  | Enable task attributes override |
| 2017-06-25  | v0.2.3  | Added disabled task attribute support |
| 2017-06-24  | v0.2.0  | Internal fixes (renamed dependencies attribute) |
| 2017-06-24  | v0.1.2  | Print build time, added internal docs, unit tests and coverage |
| 2017-06-24  | v0.1.1  | Added support for env vars, task alias and crate installation |
| 2017-06-23  | v0.1.0  | Initial release. |