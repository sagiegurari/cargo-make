# Task List

## Build

* **build** - Runs the rust compiler.
* **build-flow** - Full sanity testing flow.
* **build-release** - Runs release build.
* **build-verbose** - Runs the rust compiler with verbose output. (deprecated - Use build task with CARGO_MAKE_CARGO_VERBOSE_FLAGS set to --verbose instead.)
* **end-build-flow** - No Description.
* **init-build-flow** - No Description.
* **post-build** - No Description.
* **pre-build** - No Description.
* **workspace-build-flow** - Full sanity testing flow.

## CI

* **audit** - Runs audit cargo plugin.
* **bench-ci-flow** - Runs/Compiles the benches if conditions are met.
* **ci-coverage-flow** - Runs the coverage flow and uploads the results to codecov.
* **ci-flow** - CI task will run cargo build and cargo test with verbose output
* **examples-ci-flow** - Compiles the examples if conditions are met.
* **outdated** - Runs cargo-outdated cargo plugin.
* **outdated-ci-flow** - Runs outdated cargo conditioned CI flow.
* **outdated-flow** - Runs outdated cargo flow.
* **post-audit** - No Description.
* **post-ci-flow** - No Description.
* **post-outdated** - No Description.
* **post-unused-dependencies** - No Description.
* **post-verify-project** - No Description.
* **post-workspace-ci-flow** - No Description.
* **pre-audit** - No Description.
* **pre-ci-flow** - No Description.
* **pre-outdated** - No Description.
* **pre-unused-dependencies** - No Description.
* **pre-verify-project** - No Description.
* **pre-workspace-ci-flow** - No Description.
* **setup-musl** - Sets up a musl build environment
* **setup-release-build-env** - Sets up a musl build environment
* **unused-dependencies** - Checks for unused dependencies.
* **unused-dependencies-flow** - Checks for unused dependencies.
* **verify-project** - Runs verify-project cargo plugin.
* **workspace-ci-flow** - CI task will run CI flow for each member and merge coverage reports
* **workspace-members-ci** - Runs the ci-flow for every workspace member.
* **zip-release-ci-flow** - Compiles the binary in release mode and zips it up

## Cleanup

* **clean** - Runs the cargo clean command.
* **delete-lock** - Deletes the Cargo.lock file.
* **post-clean** - No Description.
* **pre-clean** - No Description.

## Development

* **default** - Development testing flow will first format the code, and than run cargo build and test
* **dev-test-flow** - Development testing flow will first format the code, and than run cargo build and test
* **format** - Runs the cargo rustfmt plugin.
* **format-flow** - Runs the cargo rustfmt plugin as part of a flow.
* **post-format** - No Description.
* **pre-format** - No Description.
* **upgrade-dependencies** - Rebuilds the crate with most updated dependencies.
* **watch-flow** - Watches for any file change and if any change is detected, it will invoke the default flow.

## Documentation

* **clean-apidocs** - Delete API docs.
* **copy-apidocs** - Copies the generated documentation to the docs/api directory.
* **docs** - Generate rust documentation.
* **docs-flow** - Generate rust documentation.
* **post-docs** - No Description.
* **post-workspace-docs** - No Description.
* **pre-docs** - No Description.
* **pre-workspace-docs** - No Description.
* **workspace-docs** - Generate workspace level rust documentation.
* **workspace-docs-flow** - Generate workspace level rust documentation.

## Git

* **git-add** - Runs the cargo add command.
* **git-commit** - Runs git commit command.
* **git-commit-message** - Runs git commit command with the message defined in the COMMIT_MSG environment variable.
* **git-delete-merged-branches** - Deletes any merged git branches
* **git-pull** - Runs git pull command.
* **git-push** - Runs git push command.
* **git-status** - Runs git status command.
* **post-git-add** - No Description.
* **post-git-commit** - No Description.
* **post-git-push** - No Description.
* **post-git-status** - No Description.
* **pre-git-add** - No Description.
* **pre-git-commit** - No Description.
* **pre-git-push** - No Description.
* **pre-git-status** - No Description.

## Hooks

* **end** - By default this task is invoked at the end of every cargo-make run.
* **init** - By default this task is invoked at the start of every cargo-make run.

## No Category

* **build-release-for-target** - Makes a release build for a given target
* **github-hub-find** - No Description.
* **wait** - Waits based on the CARGO_MAKE_WAIT_MILLISECONDS environment variable value

## Publish

* **bintray-upload** - Uploads the binary artifact from the cargo package/publish output to bintray.
* **build-publish-flow** - Runs full sanity, generates github release and publishes the crate.
* **github-publish** - Creates a new github release.
* **github-publish-curl** - Creates a new github release using curl.
* **github-publish-custom-name** - Creates a new github release.
* **github-publish-hub** - Creates a new github release using hub.
* **github-publish-hublish** - Creates a new github release using cargo-hublish.
* **package** - Runs the cargo package command.
* **post-package** - No Description.
* **post-publish** - No Description.
* **pre-package** - No Description.
* **pre-publish** - No Description.
* **pre-publish-clean-flow** - Clears old artifactes before publishing
* **pre-publish-conditioned-clean-flow** - Clears old artifactes before publishing
* **pre-publish-delete-lock** - Deletes lock file before publishing
* **publish** - Runs the cargo publish command.
* **publish-flow** - Publish flow - First clean the target directory of any old leftovers, package and publish
* **upload-artifacts** - Uploads the binary artifact from the cargo package/publish output to bintray.
* **workspace-publish-flow** - Publish flow - First clean the target directory of any old leftovers, package and publish
* **zip-release-binary-for-target** - Zips up the release binary, README, and license(s)

## Test

* **bench** - Runs all available bench files.
* **bench-compile** - Compiles all available bench files.
* **bench-conditioned-compile** - Compiles all available bench files if conditions are met.
* **bench-conditioned-flow** - Runs the bench flow if conditions are met.
* **bench-flow** - Runs a bench flow.
* **check** - Runs cargo check.
* **check-examples** - Runs cargo check for project examples.
* **check-flow** - Runs cargo check flow.
* **check-format** - Runs cargo fmt to check appropriate code format.
* **check-tests** - Runs cargo check for project tests.
* **clippy** - Runs clippy code linter.
* **codecov** - Runs codecov script to upload coverage results to codecov.
* **codecov-flow** - Runs the full coverage flow and uploads the results to codecov.
* **conditioned-check-format** - Runs cargo fmt --check if conditions are met.
* **conditioned-clippy** - Runs clippy code linter if conditions are met.
* **coverage** - Runs coverage (by default using kcov).
* **coverage-flow** - Runs the full coverage flow.
* **coverage-kcov** - Installs (if missing) and runs coverage using kcov (not supported on windows)
* **coverage-tarpaulin** - Runs coverage using tarpaulin rust crate (linux only)
* **dev-watch-flow** - Runs pre/post hooks and cargo test.
* **examples-compile** - Runs cargo build for project examples.
* **examples-conditioned-compile** - Runs cargo build for project examples if conditions are met.
* **install-clippy** - Installs the clippy code linter.
* **install-clippy-any** - Installs the latest clippy code linter via cargo install via rustup or directly from github.
* **install-clippy-rustup** - Installs the clippy code linter via rustup.
* **post-bench** - No Description.
* **post-check** - No Description.
* **post-coverage** - No Description.
* **post-test** - No Description.
* **pre-bench** - No Description.
* **pre-check** - No Description.
* **pre-coverage** - No Description.
* **pre-test** - No Description.
* **test** - Runs all available tests.
* **test-custom** - Runs custom test command.
* **test-flow** - Runs pre/post hooks and cargo test.
* **test-multi-phases-flow** - Runs single/multi and custom test tasks.
* **test-single-threaded** - Runs all ignored tests with a single test thread.
* **test-thread-safe** - Runs all available tests without limiting test threads.
* **test-verbose** - Runs all available tests with verbose output. (deprecated - Use test task with CARGO_MAKE_CARGO_VERBOSE_FLAGS set to --verbose instead.)
* **test-with-args** - Runs cargo test with command line arguments.
* **workspace-coverage** - Runs coverage task for all members and packages all of them (by default the codecov flow).
* **workspace-coverage-pack** - Runs codecov script to upload coverage results to codecov.
* **workspace-members-coverage** - Runs the ci-flow for every workspace member.

## Tools

* **build-file-increment** - Increments (or creates) the build number in the build file, defined in CARGO_MAKE_BUILD_NUMBER_FILE environment variable.
* **build-file-increment-flow** - Increments (or creates) the build number in the build file, defined in CARGO_MAKE_BUILD_NUMBER_FILE environment variable.
* **diff-files** - Run diff on two provided files.
* **do-on-members** - Runs the requested task for every workspace member.
* **empty** - Empty Task
* **git-diff-files** - Run diff on two provided files.
* **install-rls** - No Description.
* **install-rust-src** - No Description.
* **post-build-file-increment** - No Description.
* **pre-build-file-increment** - No Description.
* **print-cargo-env** - No Description.
* **print-cargo-make-env** - No Description.
* **print-ci-env** - No Description.
* **print-crate-env** - No Description.
* **print-duckscript-env** - No Description.
* **print-env-flow** - No Description.
* **print-git-env** - No Description.
* **print-project-env** - No Description.
* **print-rust-env** - No Description.
* **setup-sudo-env** - Sets the sudo enable/disable environment variables.

