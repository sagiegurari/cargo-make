## CHANGELOG

### v0.32.5 (2020-09-11)

* Allow for explicit workspace flow in sub flows #455 (thanks @epontan)

### v0.32.4 (2020-08-28)

* Prevent multiple calls to cleanup task

### v0.32.3 (2020-08-27)

* Support cleanup task for forked sub flow #452
* Upgrade duckscript SDK to 0.6.7

### v0.32.2 (2020-08-20)

* Fix watch invocation in case makefile file path contains spaces #445
* Clear RUST_RECURSION_COUNT at start of flow #447
* Fix do-copy-apidocs issue with triple folder #444 (thanks @haraldh)
* Fix workspace detection for sub flows
* Add task time summary via new cli flag **--time-summary** or config **time_summary=true** #446
* Reduce console output for non CI environment (can be disabled via new reduce_output config attribute)
* Workaround critical bug in rust nightly which breaks shebang based script invocations
* Upgrade duckscript SDK to 0.6.6

### v0.32.1 (2020-07-31)

* Support numeric environment variables
* Upgrade duckscript SDK to 0.6.5

### v0.32.0 (2020-07-03)

* Upgrade duckscript SDK to 0.6

### v0.31.1 (2020-06-26)

* Support CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY in workspace emulation mode #434
* New readme-include-files markdown-include-files tasks to modify markdown files based on content from other files #426

### v0.31.0 (2020-06-06)

* Upgrade duckscript to 0.5
* Upgrade cargo-watch to 7.4.1

### v0.30.8 (2020-05-25)

* Fix rustfmt nightly installation issue due to rust env issue: rust-lang/rust#72423

### v0.30.7 (2020-05-07)

* New cm_run_task duckscript command which enables to run cargo-make tasks from within duckscript #417
* New CARGO_MAKE_GIT_HEAD_LAST_COMMIT_HASH and CARGO_MAKE_GIT_HEAD_LAST_COMMIT_HASH_PREFIX environment variables.
* Bug Fix - no need to define member task names in workspace makefile #420
* rust_info upgrade.
* duckscript 0.4 upgrade.

### v0.30.6 (2020-04-24)

* Workspace emulation support #415
* Support array definition of environment variable values.
* New wasm related tasks for built in wasm support.
* Fixed env extension bug when extending a core task.
* Enable adding custom hooks into print-env flow.

### v0.30.5 (2020-04-15)

* Fixed default core task init/end task names pickup #407
* New readme-set-crate-version task which modifies README with crate version.
* Enable to provide custom arguments to git-push task #408
* build-publish flow will skip git hooks while publishing #408
* Improve clippy installation flow

### v0.30.4 (2020-04-07)

* Added clippy pre/post flow hook tasks.
* Added check format pre/post flow hook tasks.
* clippy and check-format CI flow tasks will only run on nightly linux.

### v0.30.3 (2020-04-04)

* Fix linux release build.

### v0.30.2 (2020-04-04)

* New getat function #402
* Update openssl version to 1.1.1f

### v0.30.1 (2020-03-28)

* New build-file-increment task and flow #399
* Display recursion level like `make(1)` #389 (thanks @Ma27)
* Document github actions CI instructions.
* Migrate CI to github actions.

### v0.30.0 (2020-03-23)

* Ensure that the specified rustup toolchain exists before using it #388 (thanks @Ma27)
* kcov coverage flow support for rustc >= 1.44 #393
* Enable to timeout kcov test execution for each executable via new CARGO_MAKE_KCOV_TEST_TIMEOUT_VALUE environment variable #396
* New multi-phase-tests profile to enable splitting the tests to multiple phases (thread safe, multi threaded, custom).
* Ignoring non actionable tasks #390
* Default build and test tasks support verbose flag on CI environment.
* Pager disabled for diff command.
* Split internal cargo-make tests to thread safe and single threaded to improve testing perf

### v0.29.0 (2020-03-19)

* Provide accurate CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY when starting build from member crate #382
* Support locked automatically for all crate installations via CARGO_MAKE_CRATE_INSTALLATION_LOCKED environment variable #381
* Added ability to disable sudo invocation from cargo-make scripts via new environment variable CARGO_MAKE_SUDO_DISABLE #387
* Improve toml loading performance.
* Split core makefiles to multiple files.
* Upgrade duckscript to 0.3

### v0.28.0 (2020-02-21)

* Added github hub cli support #376
* Use fsio crate for file system apis.

### v0.27.0 (2020-02-07)

* Enable to pipe stdin for scripts #355 (thanks @dakom)
* Upgrade to duckscript 0.2

### v0.26.2 (2020-01-24)

* Print duckscript version info env task #371
* Duckscript upgrade
* Migrate several task scripts from shell to duckscript to improve performance

### v0.26.1 (2020-01-17)

* Enable custom project binary executable name for binary release zip
* Support kcov mac installation #340 (thanks @bluejekyll)
* Migrate several task scripts from shell to duckscript to improve performance
* duckscript should exit on any error like shell scripts
* Update kcov to version 38 #367
* New wait task to enable sleep as part of the flow
* Add sleep before member publish during workspace publish flow #368

### v0.26.0 (2020-01-10)

* New env_scripts to enable custom environment setup #360
* Added new CARGO_MAKE_RUST_TARGET_TRIPLE environment variable #356 (thanks @daxpedda)
* Added new CARGO_MAKE_CRATE_TARGET_TRIPLE environment variable (get default build target including from '.cargo/Config.toml') #357 (thanks @daxpedda)
* Enable custom scripts via shebang line for load and install scripts.
* Enable @script type detection with shebang line #359
* Upgraded duckscript version
* Increased minimum rust version to 1.39.0
* Added condition for build-publish-flow to restrict only to master branch #364

### v0.25.1 (2020-01-03)

* Bug Fix - Fixed CARGO_MAKE_PROJECT_VERSION when main_project_member is set
* Ensure cargo-make working directory is not modified after duckscript execution.

### v0.25.0 (2020-01-01)

* \[**backward compatability break**\] Ability to set multiple watch paths in watch options #354

### v0.24.3 (2019-12-31)

* Add support for duckscript #348
* Add conditional validation support for environment variables #344
* New CARGO_MAKE_PROJECT_NAME and CARGO_MAKE_PROJECT_VERSION environment variables #349
* Ability to set watch path in watch options #350
* New zip-release-ci-flow task to enable zip creation for binary release publishing
* New workspace publish flow #347
* Upgrading run_script crate

### v0.24.2 (2019-12-13)

* New env_contains condition #335
* New custom condition failure message attribute: **fail_message** #332
* New print-env-flow task which prints out entire env info (rust, git, cargo, crate, ci, ...) #333
* New CARGO_MAKE_CI_BRANCH_NAME environment variable #334
* New CARGO_MAKE_CI_VENDOR environment variable #337
* Outdated dependencies validation as part of CI flow to be limited to CI master branch only #323

### v0.24.1 (2019-12-06)

* Support invocation of multiple tasks via run_task #324
* Support invocation of multiple tasks in parallel via run_task #321
* New CARGO_MAKE_CURRENT_TASK_NAME, CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE, CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE_DIRECTORY environment variables #322
* Info printout to state if task is running or skipped #326
* New CARGO_MAKE_CARGO_HOME environment variable #331 (thanks @daxpedda)
* Generate task list documentation file via new cli argument (output-file) #325
* Outdated dependencies validation as part of CI flow to be limited to master branch only #323
* Use new git_info crate to fetch git info and setup environment #320

### v0.24.0 (2019-11-22)

* Support defining and loading env files in makefile via new env_files attribute #306
* \[**backward compatability break**\] Workspace profile now passed on to members #309 (thanks @daxpedda)
* New CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY environment variable #311 (thanks @daxpedda)
* Fix CARGO_MAKE_TASK_ARGS not being passed down to workspace members #314 (thanks @daxpedda)
* \[**backward compatability break**\] Task cwd attribute supports environment variables expansion #318
* Guard against self-referential aliases #305 (thanks @phi-gamma)
* Migrate to envmnt::expand #308
* New CARGO_MAKE_WORKSPACE_INCLUDE_MEMBERS environment variable #316

### v0.23.0 (2019-10-16)

* Enable/Disable color output child process support #299 (thanks @emakryo)
* Main profile should override additional profiles env blocks #300
* Print project name being built #301

### v0.22.2 (2019-10-01)

* Ability to unset environment variables #294
* New environment variable to hold current workspace member name #285
* Support script file path for task script attribute #286
* New files_exist and files_not_exist conditions #287
* Added new curl based github release task #293
* Add cargo-udeps task #284
* New CARGO_MAKE_PR environment variable #297
* Add outdated check as part of CI flow

### v0.22.1 (2019-08-20)

* Support decoding ability for environment variables initialization #275 #277
* Codecov and tarpaulin integration fix for CI flow #274 #275 (thanks @vtavernier)
* New @@decode function for command arguments #278
* Move link-dead-code definition to coverage task #279
* Enable decode to mirror source value as default value #280
* Add env mapping between cargo make profile and cargo.toml profile #281

### v0.22.0 (2019-08-14)

* Specify minimum version for tools/dependencies #265
* Globally disable workspace support via makefile config #264
* Support additional rust script runners #216
* Ability to mark a task deprecated #270
* \[**backward compatability break**\] Use task cwd in condition and installation scripts #259
* Check descriptor min_version before deserializing #268 (thanks @roblabla)
* Profile name passed incorrectly via forked sub tasks #263
* Add support for deleting lock file before publishing
* Add support for dirty cargo publish

### v0.21.0 (2019-06-26)

* Add support for multiple test_args when checking for installed crate #252 (thanks @roblabla)
* \[**backward compatability break**\] Upgrade shell2batch for improved windows integration

### v0.20.0 (2019-06-16)

* Control kcov installation directory #98
* Caching kcov documentation #238
* Upgrade default kcov version to 36
* \[**backward compatability break**\] Rename KCOV_VERSION to CARGO_MAKE_KCOV_VERSION

### v0.19.5 (2019-06-09)

* Specify cargo-make minimal version #243
* Add ability to remove --all-features flag from build/test/bench tasks #248
* Add ability to modify clippy arguments #246
* Documentation - installing `cargo-make` when caching `cargo` #249 (thanks @elpiel)

### v0.19.4 (2019-06-01)

* Support additional profiles env blocks #240
* Prevent reinstallation of clippy on nightly #239
* Upgrade cargo-watch #241 (thanks @David-OConnor)

### v0.19.3 (2019-05-27)

* Fix clippy installation on nightly #236

### v0.19.2 (2019-05-25)

* Allow to modify kcov include pattern #229
* Improve kcov coverage binary filter pattern #230
* Prevent init/end task invocations on forked run tasks #227
* Migrate env file parsing and loading to envmnt

### v0.19.1 (2019-05-10)

* Support setting boolean environment variables #223
* Added new env_true/env_false conditions #221
* \[**backward compatability break**\] Normalize boolean env vars #220

### v0.18.0 (2019-04-30)

* Allow run_task to invoke the task as a sub process #214
* \[**backward compatability break**\] Upgrade shell2batch to fix path separator issue

### v0.17.1 (2019-04-04)

* Fix broken workspace support #210

### v0.17.0 (2019-04-03)

* Add extend capability for tasks #206
* Support internal core tasks modifications (private and namespacing) #201
* Support splitting command line argument to multiple arguments using functions #202
* New remove_empty function #205
* New trim function #208
* Add support for namespaces for workspace builds #204
* Add kcov version validation #203

### v0.16.10 (2019-03-01)

* Fix docs

### v0.16.9 (2019-03-01)

* Colorful output and a new cli argument to disable color (--no-color) #191
* Support multiple extend #192
* Support optional extend #193
* New test-with-args task to enable testing specific tests only
* Support multi line environment variables from script output #196
* Reducing cargo-make info level output #194
* Locking default cargo watch version and supporting user defined version #195

### v0.16.8 (2019-02-09)

* Support additional watch options #188
* Support glob paths in CARGO_MAKE_WORKSPACE_SKIP_MEMBERS #187
* Add install-rust-src and install-rls to internal makefile #189
* Renamed task 'force' attribute to 'ignore_errors' #120
* Split CARGO_MAKE_TEST_COVERAGE_BINARY_FILTER to use CARGO_MAKE_TEST_COVERAGE_DEFAULT_BINARY_FILTER for easier customization

### v0.16.7 (2019-01-25)

* Shebang line support #130 (thanks @SamuelMarks)
* Prevent multiple calls for init/end #184
* Test internal makefile coverage tasks #183

### v0.16.6 (2019-01-21)

* Fix broken coverage task

### v0.16.3 (2019-01-21)

* Enable coverage implementation selection via env #180
* Fix on error and watch proxy tasks #179

### v0.16.2 (2019-01-20)

* Profile support #174
* Task routing #175
* Set travis deploy overwrite=true
* Fix broken links in documentation

### v0.16.1 (2019-01-17)

* Enable coverage for every Linux based CI system
* Support task watch #118
* Command --list-all-steps now supports markdown output format
* New command --diff-steps to enable diff changes to flow based on custom makefile #121
* Document Azure Pipelines config #161
* New diff-files task which enables to diff 2 provided files

### v0.16.0 (2019-01-04)

* \[**backward compatability break**\] Should abort on non existent makefile if provided on cli or on extends property #143
* \[**backward compatability break**\] Update format task for stable rust #160
* Add format check and clippy to ci-flow #161 (thanks @D4nte)
* Add new print steps output format 'short-description' #121
* \[**backward compatability break**\] Remove preview from clippy and rustfmt components #162
* Add new CARGO_MAKE_CI environment variable

### v0.15.3 (2018-12-07)

* Package additional executable without cargo dependency #69
* Add CircleCI docs #152
* Delete merged branches as part of github release flow #154
* Fixed uploading windows binaries to GitHub releases #157

### v0.15.2 (2018-11-28)

* Rustup integration for non-binary/non-cargo dependencies #139
* Rust toolchain support #132
* Add fmt check task #138
* Support absolute path for CARGO_MAKE_WORKING_DIRECTORY on windows #142

### v0.15.1 (2018-11-12)

* Fix github release as part of travis build

### v0.15.0 (2018-11-12)

* rustup integration #122
* add clippy rustup installation config #119
* prevent script output if log level is error #117
* Support uploading binaries to GitHub releases #127 (thanks @LegNeato)

### v0.14.0 (2018-09-05)

* Condition support for rust version (min,max,eq) #110
* Upgrade dependencies with possible backward compatability break

### v0.13.0 (2018-08-10)

* Upgrade rustfmt support

### v0.12.1 (2018-08-09)

* Support additional default coverage test patterns #107

### v0.12.0 (2018-08-02)

* Defining --no-workspace cli flag via task attribute #99
* Support unsetting task attributes in extended Makefile #100
* Support accepting arguments and passing them to commands and scripts #101
* Follow XDG Base Directory Specification #105
* Doc category for task list #102

### v0.11.3 (2018-06-24)

* Fix workspace coverage not being generated #97

### v0.11.2 (2018-06-22)

* Fix automatic workspace makefile extending #96

### v0.11.1 (2018-06-06)

* Support writing tasks in various scripting languages #90
* Internal private tasks #94

### v0.11.0 (2018-05-29)

* Support environment variable substitution in commands and arguments #92

### v0.10.8 (2018-05-18)

* Add support for a catch/cleanup task in case of any errors #89
* Load env vars from simple text files #88
* Fix workspace member paths on windows #87

### v0.10.7 (2018-05-11)

* Support automatic workspace makefile extend #84
* New task to rebuild lock file with most updated dependencies #83

### v0.10.6 (2018-03-20)

* Fix script issues due to file permissions (#81)

### v0.10.5 (2018-02-27)

* cargo make audit will only be invoked if a Cargo.lock file exists (#74)

### v0.10.4 (2018-02-26)

* Fix composite environment variables resolution order (#78)

### v0.10.3 (2018-02-24)

* update_check_minimum_interval default changed be weekly (#76)
* Added test_ files to kcov test coverage invocation
* Update dependencies documentation (#72)

### v0.10.2 (2018-02-08)

* Check for update minimum interval definition in global configuration (#68)

### v0.10.0 (2018-02-02)

* Search for makefile.toml in parent directories (#65)
* Global configuration file to define default log level, task name (#64)
* Reformat changelog (#66)

### v0.9.5 (2018-01-22)

* Fixed critical loading issue due to #56

### v0.9.4 (2018-01-19)

* Support new skip_core_tasks config flag (#56)

### v0.9.3 (2018-01-05)

* Coverage test files user level pattern (#59 and #60)

### v0.9.2 (2017-12-29)

* Coverage will execute all test files without any need for specific naming conventions or configuration (#50 and #51)

### v0.9.1 (2017-12-28)

* Bug fixes for git info loading and coverage on workspace level (#49 and #52)

### v0.9.0 (2017-12-18)

* Disabled tasks should be skipped including dependencies (#47)

### v0.8.0 (2017-12-17)

* Support multiple rustfmt versions (#45)

### v0.7.11 (2017-12-14)

* Fixed rustfmt backward compatability break issue

### v0.7.10 (2017-12-06)

* Workspace support fixes for windows

### v0.7.5 (2017-12-03)

* Support conditional skipping of workspace members from flow (#42)

### v0.7.4 (2017-10-25)

* kcov support for excluding lines/blocks from coverage report

### v0.7.2 (2017-10-23)

* Added workspace level coverage task without the need to run full CI flow (workspace-coverage)

### v0.7.1 (2017-10-21)

* Added support for cross platform scripts using script runner @shell

### v0.7.0 (2017-10-20)

* Refactored how rust code based scripts are defined to enable future expansion of this feature.

### v0.6.5 (2017-10-12)

* ci-flow and build-flow will build/run benches and examples based on new environment variables: CARGO_MAKE_BUILD_EXAMPLES, CARGO_MAKE_BUILD_BENCH and CARGO_MAKE_RUN_BENCH

### v0.6.1 (2017-10-08)

* Tasks can now run rust code using rust_script attribute and tasks can no longer hold multiple different action definitions.

### v0.5.3 (2017-09-29)

* Added support to evaluate env var values from script output and define cwd on task level

### v0.5.2 (2017-09-24)

* Added various check and build tasks

### v0.5.0 (2017-09-21)

* Disable by default coverage flow on mac

### v0.4.1 (2017-09-18)

* Added watch-flow task

### v0.3.77 (2017-09-17)

* Automatically define install_crate and enable cargo install arguments via install_crate_args

### v0.3.76 (2017-09-16)

* Build failure will not cause panic but will exit

### v0.3.75 (2017-09-12)

* By default build and test should work with "--all-features" flag

### v0.3.66 (2017-08-23)

* Add support for workspace glob members

### v0.3.64 (2017-08-22)

* Cargo.toml parsing fix

### v0.3.63 (2017-08-21)

* Add support for Cargo.toml workspace.exclude

### v0.3.62 (2017-08-21)

* Add workspace member detection based on dependency paths

### v0.3.59 (2017-08-20)

* Support load_script platform overrides

### v0.3.58 (2017-08-19)

* Added load_script capability

### v0.3.56 (2017-08-18)

* Set environment variables during task invocation

### v0.3.53 (2017-08-09)

* Added new condition types: env, env_set and env_not_set

### v0.3.51 (2017-08-09)

* Added experimental cli arg to enable access unsupported experimental predefined tasks

### v0.3.49 (2017-08-08)

* Added condition attribute

### v0.3.46 (2017-08-06)

* Added bintray upload task

### v0.3.43 (2017-08-02)

* Added --env/-e cli args to set environment variables via command line

### v0.3.41 (2017-08-01)

* Added github-publish task

### v0.3.38 (2017-07-28)

* Added run_script which allows executing sub tasks

### v0.3.37 (2017-07-25)

* Added condition script capability for tasks

### v0.3.36 (2017-07-22)

* Added coverage-lcov task (not fully tested)

### v0.3.34 (2017-07-21)

* Added coverage-tarpaulin task

### v0.3.33 (2017-07-21)

* Added more environment variables for workspace support

### v0.3.32 (2017-07-20)

* Added --list-all-steps cli option

### v0.3.28 (2017-07-17)

* workspace level ci flow

### v0.3.27 (2017-07-16)

* cargo make ci-flow on travis now automatically runs code coverage and uploads to codecov

### v0.3.25 (2017-07-16)

* New --no-workspace cli arg

### v0.3.24 (2017-07-15)

* Workspace support

### v0.3.23 (2017-07-14)

* Added codecov task in default toml

### v0.3.20 (2017-07-14)

* Added coverage task in default toml

### v0.3.16 (2017-07-14)

* Added more environment variables based on target environment and rust compiler

### v0.3.15 (2017-07-13)

* Added common init and end tasks

### v0.3.13 (2017-07-10)

* cargo-make now defines rust version env vars

### v0.3.11 (2017-07-09)

* cargo-make now defines env vars based on project git repo information

### v0.3.10 (2017-07-06)

* cargo-make now defines env vars based on project Cargo.toml

### v0.3.6 (2017-07-05)

* Added --cwd cli arg to enable setting working directory

### v0.3.5 (2017-07-04)

* Added clippy task

### v0.3.4 (2017-07-03)

* Added --print-steps cli arg

### v0.3.1 (2017-07-02)

* Added CARGO_MAKE_TASK env var holding the main task name

### v0.3.0 (2017-07-02)

* Renamed few cli options

### v0.2.20 (2017-07-02)

* Added -v and --verbose cli arg

### v0.2.19 (2017-07-01)

* Added extend config level attribute

### v0.2.17 (2017-06-30)

* Added force task attribute

### v0.2.12 (2017-06-28)

* Published website

### v0.2.8 (2017-06-28)

* Platform specific task override

### v0.2.7 (2017-06-26)

* Platform specific alias

### v0.2.6 (2017-06-26)

* Enable task attributes override

### v0.2.3 (2017-06-25)

* Added disabled task attribute support

### v0.2.0 (2017-06-24)

* Internal fixes (renamed dependencies attribute)

### v0.1.2 (2017-06-24)

* Print build time, added internal docs, unit tests and coverage

### v0.1.1 (2017-06-24)

* Added support for env vars, task alias and crate installation

### v0.1.0 (2017-06-23)

* Initial release.
