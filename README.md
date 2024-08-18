# cargo-make

[![crates.io](https://img.shields.io/crates/v/cargo-make.svg)](https://crates.io/crates/cargo-make)
[![CI](https://github.com/sagiegurari/cargo-make/workflows/CI/badge.svg?branch=master)](https://github.com/sagiegurari/cargo-make/actions)
[![codecov](https://codecov.io/gh/sagiegurari/cargo-make/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/cargo-make)
[![license](https://img.shields.io/crates/l/cargo-make.svg)](https://github.com/sagiegurari/cargo-make/blob/master/LICENSE)
[![Crates.io](https://img.shields.io/crates/d/cargo-make?label=cargo%20installs)](https://crates.io/crates/cargo-make)
[![GitHub All Releases](https://img.shields.io/github/downloads/sagiegurari/cargo-make/total?label=binary%20downloads)](https://github.com/sagiegurari/cargo-make/releases)
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

> [Rust](https://www.rust-lang.org/) task runner and build tool.

* [Overview](#overview)
* [Installation](#installation)
    * [Arch Linux](#installation-arch-linux)
    * [Binary Release](#installation-binary-release)
* [Usage](#usage)
    * [Simple Example](#usage-simple)
    * [Tasks, Dependencies, and Aliases](#usage-task-dependencies-alias)
    * [Commands, Scripts, and Sub Tasks](#usage-task-command-script-task)
        * [Sub Task](#usage-task-command-script-task-examplesubtask)
        * [Command](#usage-task-command-script-task-examplecommand)
        * [Script](#usage-task-command-script-task-examplescript)
        * [Duckscript](#usage-task-command-script-task-exampleduckscript)
        * [Rust Code](#usage-task-command-script-task-examplerust)
        * [Cross Platform Shell](#usage-task-command-script-task-exampleshell2batch)
        * [Other Programming Languages](#usage-task-command-script-task-examplegeneric)
        * [Shebang Support](#usage-task-command-script-task-exampleshebang)
    * [Default Tasks and Extending](#usage-default-tasks)
        * [Extending External Makefiles](#usage-workspace-extending-external-makefile)
        * [Automatically Extend Workspace Makefile](#usage-workspace-extend)
        * [Load Scripts](#usage-load-scripts)
        * [Predefined Makefiles](#usage-predefined-makefiles)
        * [The Default Task](#usage-default-task)
    * [Extending Tasks](#usage-extending-tasks)
        * [Task Override](#usage-task-override)
        * [Platform Override](#usage-platform-override)
        * [Extend Attribute](#usage-task-extend-attribute)
    * [Environment Variables](#usage-env)
        * [Declaration](#env-declaration)
        * [Global Configuration](#usage-env-config)
        * [Task](#usage-env-task)
        * [Command Line](#usage-env-cli)
        * [Env File](#usage-env-file)
        * [Env Setup Scripts](#usage-env-setup-scripts)
        * [Loading Order](#usage-env-vars-loading-order)
        * [Note about Ordering](#env-note-about-ordering)
        * [Global](#usage-env-global)
    * [Setting Up Working Directory](#usage-setting-up-working-directory)
    * [Ignoring Errors](#usage-ignoring-errors)
    * [Conditions](#usage-conditions)
        * [Criteria](#usage-conditions-structure)
        * [Scripts](#usage-conditions-script)
        * [And/Or/Group Or](#usage-conditions-and-or)
        * [Combining Conditions and Sub Tasks](#usage-conditions-and-subtasks)
        * [Running Tasks Only If Sources Changed](#usage-running-tasks-only-if-sources-changed)
    * [Installing Dependencies](#usage-installing-dependencies)
        * [Cargo Plugins](#usage-installing-cargo-plugins)
        * [Crates](#usage-installing-crates)
        * [Rustup Components](#usage-installing-rustup-components)
        * [Native Dependencies](#usage-installing-native-dependencies)
        * [Defining Version](#usage-installing-version)
        * [Global Lock Of Versions](#usage-installing-locked)
        * [Alternate Cargo Install Commands](#usage-installing-alternate-cargo-install-commands)
        * [Installation Priorities](#usage-installing-dependencies-priorities)
        * [Multiple Installations](#usage-installing-dependencies-multiple)
    * [Workspace Support](#usage-workspace-support)
        * [Disabling Workspace Support](#usage-workspace-disabling-workspace-support)
        * [Composite Flow](#usage-workspace-composite-flow)
        * [Profiles](#usage-workspace-profiles)
        * [Skipping/Including Specific Members](#usage-workspace-support-skip-include-members)
        * [Workspace Emulation](#usage-workspace-emulation)
    * [Toolchain](#usage-toolchain)
    * [Init and End tasks](#usage-init-end-tasks)
    * [Catching Errors](#usage-catching-errors)
    * [Cargo Alias Tasks](#usage-cargo-alias-tasks)
    * [Profiles](#usage-profiles)
        * [Environment Variables](#usage-profiles-env)
        * [Conditions](#usage-profiles-conditions)
        * [Built In Profiles](#usage-profiles-built-in)
    * [Private Tasks](#usage-private-tasks)
    * [Deprecated Tasks](#usage-deprecated-tasks)
    * [Watch](#usage-watch)
        * [Running Multiple Blocking Watches](#usage-watch-running-multiple-blocking-watches)
    * [Functions](#usage-functions)
        * [Split](#usage-functions-split)
        * [GetAt](#usage-functions-getat)
        * [Remove Empty](#usage-functions-remove-empty)
        * [Trim](#usage-functions-trim)
        * [Decode](#usage-functions-decode)
    * [Continuous Integration](#usage-ci)
        * [Github Actions](#usage-ci-github-actions)
        * [Travis](#usage-ci-travis)
        * [AppVeyor](#usage-ci-appveyor)
        * [GitLab](#usage-ci-gitlab)
        * [CircleCI](#usage-ci-circleci)
        * [Azure Pipelines](#usage-ci-azure-pipelines)
        * [drone.io](#usage-ci-drone-io)
        * [Cirrus CI](#usage-ci-cirrus)
    * [Predefined Flows](#usage-predefined-flows)
        * [Coverage](#usage-predefined-flows-coverage)
        * [Full List](https://github.com/sagiegurari/cargo-make/blob/master/docs/cargo_make_task_list.md)
        * [Disabling Predefined Tasks/Flows](#usage-predefined-flows-disable)
        * [Modifying Predefined Tasks/Flows](#usage-predefined-flows-modify)
    * [Minimal Version](#usage-min-version)
    * [Performance Tuning](#usage-performance-tuning)
    * [Command Groups (Subcommands)](#usage-command-groups)
    * [Diff Changes](#usage-diff-changes)
    * [Unstable Features](#usage-unstable-features)
    * [CLI Options](#usage-cli)
    * [Plugins](#usage-plugins)
        * [Defining Plugins](#usage-plugins-defining-plugins)
        * [Plugin SDK](#usage-plugins-plugin-sdk)
        * [Plugin Example - Docker Integration](#usage-plugins-plugin-example-dockerize)
        * [Plugin Example - Run workspace members in parallel](#usage-plugins-plugin-example-parallel-workspace-members)
        * [Plugin Example - load Env From Rust Script](#usage-plugins-plugin-example-rustenv)
        * [Plugin Example - Adding Simpler Windows Powershell Support](#usage-plugins-plugin-example-powershell)
    * [Shell Completion](#usage-shell-completion)
        * [Bash](#usage-shell-completion-bash)
        * [zsh](#usage-shell-completion-zsh)
        * [Fig / Amazon CodeWhisperer for command line](#usage-shell-completion-fig)
    * [Global Configuration](#cargo-make-global-config)
* [Makefile Definition](#descriptor-definition)
* [Task Naming Conventions](#task-name-conventions)
* [Articles](#articles)
    * [Introduction and Basics](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-1-of-5-introduction-and-basics-b19ced7e7057)
    * [Extending Tasks, Platform Overrides, and Aliases](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-2-of-5-extending-tasks-platform-overrides-1527386dcf87)
    * [Environment Variables, Conditions, Sub Tasks, and Mixing](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-3-of-5-environment-variables-conditions-3c740a837a01)
    * [Workspace Support, Init/End Tasks, and Makefiles](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-4-of-5-workspace-support-init-end-tasks-c3e738699421)
    * [Predefined Tasks, CI Support, and Conventions](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-5-final-predefined-tasks-ci-support-and-4594812e57da)
* [Badge](#badge)
* [Roadmap](#roadmap)
* [Editor Support](#editor-support)
    * [vim](#editor-support-vim)
    * [vs-code](#editor-support-vs-code)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](https://github.com/sagiegurari/cargo-make/blob/master/CHANGELOG.md)
* [License](#license)

<a name="overview"></a>
## Overview
The cargo-make task runner enables to define and configure sets of tasks and run them as a flow.<br>
A task is a command, script, rust code, or other sub tasks to execute.<br>
Tasks can have dependencies which are also tasks that will be executed before the task itself.<br>
With a simple toml based configuration file, you can define a multi platform build script that can run build, test, generate documentation, run bench tests, run security validations and more, executed by running a single command.

<a name="installation"></a>
## Installation
In order to install, just run the following command

```sh
cargo install --force cargo-make
```

This will install cargo-make in your `~/.cargo/bin`.<br>
Make sure to add `~/.cargo/bin` directory to your `PATH` variable.<br>
<br>
You will have two executables available: *`cargo-make`* and *`makers`*<br>

* **cargo-make** - This is a cargo plugin invoked using **cargo make ...**
* **makers** - A standalone executable which provides same features and cli arguments as cargo-make, but is invoked directly and not as a cargo plugin.

See [Cli Options](#usage-cli) section for full CLI instructions.

In order to install with minimal features (for example, no TLS support), run the following:

```sh
cargo install --no-default-features --force cargo-make
```

<a name="installation-arch-linux"></a>
### Arch Linux

```sh
sudo pacman -S cargo-make
```

<a name="installation-binary-release"></a>
### Binary Release
Binary releases are available in the [github releases page](https://github.com/sagiegurari/cargo-make/releases).<br>
The following binaries are available for each release:

* x86_64-unknown-linux-gnu
* x86_64-unknown-linux-musl
* x86_64-apple-darwin
* x86_64-pc-windows-msvc
* aarch64-apple-darwin

<a name="usage"></a>
## Usage
When using cargo-make, all tasks are defined and configured via toml files.<br>
Below are simple instructions to get you started off quickly.

<a name="usage-simple"></a>
### Simple Example
In order to run a set of tasks, you first must define them in a toml file.<br>
For example, if we would like to have a script which:

* Formats the code
* Cleans old target directory
* Runs build
* Runs tests

By default, cargo-make reads tasks from `Makefile.toml` if it exists.

We will create a `Makefile.toml` file as follows:

```toml
[tasks.format]
install_crate = "rustfmt"
command = "cargo"
args = ["fmt", "--", "--emit=files"]

[tasks.clean]
command = "cargo"
args = ["clean"]

[tasks.build]
command = "cargo"
args = ["build"]
dependencies = ["clean"]

[tasks.test]
command = "cargo"
args = ["test"]
dependencies = ["clean"]

[tasks.my-flow]
dependencies = [
    "format",
    "build",
    "test"
]
```

We would execute the flow with the following command:

```sh
cargo make my-flow
```

The output would look something like this:

```console
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: my-flow
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: format
[cargo-make] INFO - Execute Command: "cargo" "fmt" "--" "--emit=files"
[cargo-make] INFO - Running Task: clean
[cargo-make] INFO - Execute Command: "cargo" "clean"
[cargo-make] INFO - Running Task: build
[cargo-make] INFO - Execute Command: "cargo" "build"
   Compiling bitflags v0.9.1
   Compiling unicode-width v0.1.4
   Compiling quote v0.3.15
   Compiling unicode-segmentation v1.1.0
   Compiling strsim v0.6.0
   Compiling libc v0.2.24
   Compiling serde v1.0.8
   Compiling vec_map v0.8.0
   Compiling ansi_term v0.9.0
   Compiling unicode-xid v0.0.4
   Compiling synom v0.11.3
   Compiling rand v0.3.15
   Compiling term_size v0.3.0
   Compiling atty v0.2.2
   Compiling syn v0.11.11
   Compiling textwrap v0.6.0
   Compiling clap v2.25.0
   Compiling serde_derive_internals v0.15.1
   Compiling toml v0.4.2
   Compiling serde_derive v1.0.8
   Compiling cargo-make v0.1.2 (file:///home/ubuntu/workspace)
    Finished dev [unoptimized + debuginfo] target(s) in 79.75 secs
[cargo-make] INFO - Running Task: test
[cargo-make] INFO - Execute Command: "cargo" "test"
   Compiling cargo-make v0.1.2 (file:///home/ubuntu/workspace)
    Finished dev [unoptimized + debuginfo] target(s) in 5.1 secs
     Running target/debug/deps/cargo_make-d5f8d30d73043ede

running 10 tests
test log::tests::create_info ... ok
test log::tests::get_level_error ... ok
test log::tests::create_verbose ... ok
test log::tests::get_level_info ... ok
test log::tests::get_level_other ... ok
test log::tests::get_level_verbose ... ok
test installer::tests::is_crate_installed_false ... ok
test installer::tests::is_crate_installed_true ... ok
test command::tests::validate_exit_code_error ... ok
test log::tests::create_error ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

[cargo-make] INFO - Running Task: my-flow
[cargo-make] INFO - Build done in 72 seconds.
```

We now created a build script that can run on any platform.

The tasks can be stored in any toml file.  Invoke cargo-make with `--makefile other-filename.toml`
to start processing using `other-filename.toml`.

**cargo-make can be invoked as a cargo plugin via `cargo make` command, _or_ as a standalone executable via `makers` command.**<br>
<br>
**Important Note: if you are running this example in a cargo workspace, you will need to add the following to the top of the file:**<br>

```toml
[env]
CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE = true
```
**More on workspace support in the relevant sections in this document.**

<a name="usage-task-dependencies-alias"></a>
### Tasks, Dependencies, and Aliases
In many cases, certain tasks depend on other tasks.<br>
For example you would like to format the code before running build and run the build before running tests.<br>
Such flow can be defined as follows:

```toml
[tasks.format]
install_crate = "rustfmt"
command = "cargo"
args = ["fmt", "--", "--emit=files"]

[tasks.build]
command = "cargo"
args = ["build"]
dependencies = ["format"]

[tasks.test]
command = "cargo"
args = ["test"]
dependencies = ["build"]
```

When you run:

```sh
cargo make --makefile ./my_build.toml test
```

It will try to run test, see that it has dependencies and those have other dependencies.<br>
Therefore it will create an execution plan for the tasks based on the tasks and their dependencies.<br>
In our case it will invoke format -> build -> test.<br>

The same task will never be executed twice. So, if we have, for example:

```toml
[tasks.A]
dependencies = ["B", "C"]

[tasks.B]
dependencies = ["D"]

[tasks.C]
dependencies = ["D"]

[tasks.D]
script = "echo hello"
```

In this example, A depends on B and C, and both B and C are dependent on D.<br>
Task D, however, will not be invoked twice.<br>
The output of the execution will look something like this:

```console
[cargo-make] INFO - Task: A
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: D
[cargo-make] INFO - Execute Command: "sh" "/tmp/cargo-make/CNuU47tIix.sh"
hello
[cargo-make] INFO - Running Task: B
[cargo-make] INFO - Running Task: C
[cargo-make] INFO - Running Task: A
```

As you can see, 'hello' was printed once by task D as it was only invoked once.<br>
But what if we want to run D twice?<br>
Simple answer would be to duplicate task D, have B depend on D, and C depend on D2, which is a copy of D.<br>
But duplicating can lead to bugs and to huge makefiles, so we have aliases for that.<br>
An alias task has its own name and points to another task.<br>
All of the definitions of the alias task are ignored.<br>
So now, if we want to have D execute twice, we can do the following:

```toml
[tasks.A]
dependencies = ["B", "C"]

[tasks.B]
dependencies = ["D"]

[tasks.C]
dependencies = ["D2"]

[tasks.D]
script = "echo hello"

[tasks.D2]
alias="D"
```

Now C depends on D2, and D2 is an alias for D.<br>
Execution output of such make file would like as follows:

```console
[cargo-make] INFO - Task: A
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: D
[cargo-make] INFO - Execute Command: "sh" "/tmp/cargo-make/HP0UD7pgoX.sh"
hello
[cargo-make] INFO - Running Task: B
[cargo-make] INFO - Running Task: D2
[cargo-make] INFO - Execute Command: "sh" "/tmp/cargo-make/TuuZJkqCE2.sh"
hello
[cargo-make] INFO - Running Task: C
[cargo-make] INFO - Running Task: A
```

Now you can see that 'hello' was printed twice.

Tasks may also depend on tasks in other files.
To do this, specify the dependency with the object format, providing the path.
cargo-make will use this path as it would any other supplied on the command line:
If a filename is supplied, it searches that file.
Otherwise it search for the default `Makefile.toml` on that path.

```toml
[tasks.install]
command = "mv"
args = ["src/B/out", "src/C/static"]
dependencies = [
  { name = "compile", path = "src/B" },
  { name = "clean", path = "src/C/tasks.toml" },
]
```

*The run_task attribute will tell a task to invoke another task in a new execution plan. This will also result in dependencies being invoked multiple times.*

It is also possible to define platform specific aliases, for example:

```toml
[tasks.my_task]
linux_alias = "linux_my_task"
windows_alias = "windows_my_task"
mac_alias = "mac_my_task"

[tasks.linux_my_task]

[tasks.mac_my_task]

[tasks.windows_my_task]
```

If platform specific alias is found and matches current platform it will take precedence over the non platform alias definition.<br>
For example:

```toml
[tasks.my_task]
linux_alias = "run"
alias = "do_nothing"

[tasks.run]
script = "echo hello"

[tasks.do_nothing]
```

If you run task **my_task** on windows or mac, it will invoke the **do_nothing** task.<br>
However, if executed on a linux platform, it will invoke the **run** task.

*As a side note, cargo-make will attempt to invoke the task dependencies in the order that they were defined, unless they are defined also as sub dependencies.*

<a name="usage-task-command-script-task"></a>
### Commands, Scripts, and Sub Tasks
The actual operation that a task invokes can be defined in 3 ways.<br>
The below explains each one:

* **run_task** - Invokes another task with the name defined in this attribute. Unlike dependencies which are invoked before the current task, the task defined in the **run_task** is invoked after the current task.
* **command** - The command attribute defines what executable to invoke. You can use the **args** attribute to define what command line arguments to provide as part of the command.
* **script** - Invokes the script. You can change the executable used to invoke the script using the **script_runner** attribute. If not defined, the default platform runner is used (`cmd` for Windows, `sh` for others).

Only one of the definitions will be used.<br>
If multiple attributes are defined (for example both command and script), the task will fail during invocation.

The script attribute may hold non OS scripts, for example rust code to be compiled and executed.<br>
In order to use non OS script runners, you must define the special script_runner with the **@** prefix.<br>
The following runners are currently supported:

* **@duckscript** - Executes the defined duckscript code. See [example](#usage-task-command-script-task-exampleduckscript)
* **@rust** - Compiles and executes the defined rust code. See [example](#usage-task-command-script-task-examplerust)
* **@shell** - For Windows platforms, it will try to convert the shell commands to Windows batch commands (only basic scripts are supported) and execute the script; for other platforms, the script will be executed as-is. See [example](#usage-task-command-script-task-exampleshell2batch)

Below are some basic examples of each action type.

<a name="usage-task-command-script-task-examplesubtask"></a>
#### Sub Task
In this example, if we execute the **flow** task, it will invoke the **echo** task defined in the **run_task** attribute.

```toml
[tasks.echo]
script = "echo hello world"

[tasks.flow]
run_task = "echo"
```

A more complex example below demonstrates the ability to define multiple task names and optional conditions attached to each task.<br>
The **first** task for which the conditions are met (or if no conditions are defined at all), will be invoked.<br>
If no task conditions are met, no sub task will be invoked.<br>
More on conditions can be found the [conditions section](#usage-conditions)

```toml
[tasks.test1]
command = "echo"
args = ["running test1"]

[tasks.test2]
command = "echo"
args = ["running test2"]

[tasks.test3]
command = "echo"
args = ["running test3"]

[tasks.test-default]
command = "echo"
args = ["running test-default"]

[tasks.test-routing]
run_task = [
    { name = "test1", condition = { platforms = ["windows", "linux"], channels = ["beta", "stable"] } },
    { name = "test2", condition = { platforms = ["mac"], rust_version = { min = "1.20.0", max = "1.30.0" } } },
    { name = "test3", condition_script = [ "somecommand" ] },
    { name = "test-default" }
]
```

It is also possible to run the sub task as a forked sub process using the **fork** attribute.<br>
This prevents any environment changes done in the sub task to impact the rest of the flow in the parent process.<br>
Example of invoking the sub task in a forked sub process:

```toml
[tasks.echo]
command = "echo"
args = ["hello world"]

[tasks.fork-example]
run_task = { name = "echo", fork = true }
```

The **name** attribute can hold either a single task name or a list of tasks.<br>
In case of a list, the tasks would be invoked one after the other in sequence.<br>
For example, below **simple-multi** and **routing-multi** both demonstrate different ways to define multi task invocations via **run_task**:

```toml
[tasks.echo1]
command = "echo"
args = ["1"]

[tasks.echo2]
command = "echo"
args = ["2"]

[tasks.simple-multi]
run_task = { name = ["echo1", "echo2"] }

[tasks.routing-multi]
run_task = [
    { name = ["echo1", "echo2"] },
]
```

You can also setup a **cleanup** task to run after the sub task even if the sub task failed.<br>
This is only supported in combination with **fork=true** attribute.<br>
For example:<br>

```toml
[tasks.echo1]
command = "echo"
args = ["1"]

[tasks.echo2]
command = "echo"
args = ["2"]

[tasks.fail]
script =  "exit 1"

[tasks.cleanup]
command = "echo"
args = ["cleanup"]

[tasks.cleanup-example]
run_task = { name = ["echo1", "echo2", "fail"], fork = true, cleanup_task = "cleanup" }
```

In order to run multiple tasks in parallel, add **parallel = true** to the `run_task` object.<br>
For example:

```toml
[tasks.echo1]
command = "echo"
args = ["1"]

[tasks.echo2]
command = "echo"
args = ["2"]

[tasks.parallel-multi]
run_task = { name = ["echo1", "echo2"], parallel = true }
```

This allows to run independent tasks in parallel and speed up the overall performance of the flow.<br>
Be aware that parallel invocation of tasks will cause issues if the following feature are used:

* Setting the task's current working directory via **cwd** attribute will result in all parallel tasks being affected.
* Avoid using **`CARGO_MAKE_CURRENT_TASK_`** type environment variables as those may hold incorrect values.

In addition, in some scenarios, child processes may be left as zombie processes.<br>
It is possible to setup a manual cleanup task to resolve it.

<a name="usage-task-command-script-task-examplecommand"></a>
#### Command
When running commands, you can also define the command line arguments, as shown in the example below, to invoke the cargo command with the plugin name as a command line argument:

```toml
[tasks.build-with-verbose]
command = "cargo"
args = ["build", "--verbose", "--all-features"]
```

It is possible to provide environment variables as part of the command and arguments to be replaced in runtime with actual values, for example:

```toml
[env]
SIMPLE = "SIMPLE VALUE"
ECHO_CMD = "echo"

[tasks.expand]
command = "${ECHO_CMD}"
args = [
    "VALUE: ${SIMPLE}"
]
```

cargo-make CLI also supports additional arguments which will be available to all tasks.<br>
The following example prints additional arguments:

```toml
[tasks.varargs]
command = "echo"
args = [
    "args are:", "${@}"
]
```

*For native scripts, use that native script syntax.*<br>
*For shell you can use ```${0}``` and for windows: ```%*```*

Invoking cargo-make with additional arguments would result in the following:

```console
> cargo make varargs arg1 arg2 arg3

[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: varargs
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: varargs
[cargo-make] INFO - Execute Command: "echo" "args are:" "arg1" "arg2" "arg3"
args are: arg1 arg2 arg3
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

Invoking cargo-make without any additional arguments would result in the following:

```console
> cargo make varargs

[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: varargs
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: varargs
[cargo-make] INFO - Execute Command: "echo" "args are:"
args are:
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

This can also be used for templating, for example:

```toml
[tasks.varargs]
command = "echo"
args = [
    "args are:", "-o=${@}"
]
```

Would output:

```console
> cargo make varargs arg1 arg2 arg3

[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: varargs
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: varargs
[cargo-make] INFO - Execute Command: "echo" "args are:" "arg1" "arg2" "arg3"
args are: -o=arg1 -o=arg2 -o=arg3
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

Command line arguments can also contain [built-in functions](#usage-functions) (see below).

<a name="usage-task-command-script-task-examplescript"></a>
#### Script
Below is simple script which prints hello world.

```toml
[tasks.hello-world]
script = [
    "echo start...",
    "echo \"Hello World From Script\"",
    "echo end..."
]
```

You can use multi-line toml string to make the script more readable as follows:

```toml
[tasks.hello-world]
script = '''
echo start...
echo "Hello World From Script"
echo end...
'''
```

cargo-make CLI also supports additional arguments which will be available to all tasks.<br>
The following example prints additional arguments:

```toml
[tasks.cli-args]
script = "echo args are: ${@}"
```

Invoking cargo-make with additional arguments would result in the following:

```console
> cargo make cli-args arg1 arg2 arg3

[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: cli-args
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: cli-args
+ cd /projects/rust/cargo-make/examples
+ echo args are: arg1 arg2 arg3
args are: arg1 arg2 arg3
[cargo-make] INFO - Running Task: end
```

Invoking cargo-make without any additional arguments would result in the following:

```console
> cargo make cli-args

[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: Makefile.toml
[cargo-make] INFO - Task: cli-args
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: cli-args
+ cd /projects/rust/cargo-make/examples
+ echo args are:
args are:
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

It is also possible to point to an existing script (instead of holding the script text inside the makefile) by using the **file** property as follows:

```toml
[tasks.hello-world-from-script-file]
script = { file = "script.sh" }
```

Script file paths are always relative to the current working directory, unless specified by the **absolute_path** attribute. For example:

```toml
[tasks.hello-world-from-script-file-absolute-path]
script = { file = "${CARGO_MAKE_WORKING_DIRECTORY}/script.sh", absolute_path = true }
```

File paths support environment substitution.<br><br>
**Favor commands over scripts, as commands support more features such as [automatic dependencies installation](#usage-installing-dependencies), [argument functions](#usage-functions), and more...**

In order to share common script content among multiple tasks, you can use the script pre/main/post form as follows:

```toml
[tasks.base-script]
script.pre = "echo start"
script.main = "echo old"
script.post = "echo end"

[tasks.extended-script]
extend = "base-script"
script.main = "echo new"
```

Running extended-script task would print:

```console
start
new
end
```

<a name="usage-task-command-script-task-exampleduckscript"></a>
#### Duckscript
[Duckscript](https://sagiegurari.github.io/duckscript/) is incredibly simple shell like language which provides cross platform shell scripting capability.<br>
[Duckscript](https://sagiegurari.github.io/duckscript/) is embedded inside cargo-make so unlike other scripting solutions or commands, duckscript can change cargo-make environment variables from inside the script.<br>
In addition you can run cargo-make tasks from within duckscript script.<br>
This allows a really powerful two way integration with cargo-make.

```toml
[tasks.duckscript-example]
script_runner = "@duckscript"
script = '''
task_name = get_env CARGO_MAKE_CURRENT_TASK_NAME
echo The currently running cargo make task is: ${task_name}

# since all env vars are auto loaded as duckscript variables by cargo-make
# you can access them directly
echo The currently running cargo make task is: ${CARGO_MAKE_CURRENT_TASK_NAME}

cd .. # this changes cargo-make current working directory (cargo-make will revert to original directory after script execution)
pwd
set_env CARGO_MAKE_CURRENT_TASK_NAME tricking_cargo_make
'''
```

The next example shows how to invoke cargo-make tasks from duckscript:

```toml
[tasks.run-task-from-duckscript]
script_runner = "@duckscript"
script = '''
echo first invocation of echo1 task:
cm_run_task echo1
echo second invocation of echo1 task:
cm_run_task echo1

echo running task: echo2:
cm_run_task echo2
'''

[tasks.echo1]
command = "echo"
args = ["1"]

[tasks.echo2]
command = "echo"
args = ["2"]
```

Same as OS scripts, the @duckscript runner also supports the cargo-make CLI arguments access.<br>
In addition, all environment variables are preloaded as duckscript variables, and can be directly read from the script. (No need to invoke the **get_env** command!)

<a name="usage-task-command-script-task-examplerust"></a>
#### Rust Code
In this example, when the **rust** task is invoked, the **script** content will be compiled and executed.
You can see how dependencies are defined in `Cargo.toml` format inside the code.

```toml
[tasks.rust]
script_runner = "@rust"
script = '''
//! ```cargo
//! [dependencies]
//! envmnt = "*"
//! ```
fn main() {
    let value = envmnt::get_or("PATH", "NO PATH VAR DEFINED");
    println!("Path Value: {}", &value);
}
'''
```

Same as OS scripts, the @rust runner also supports the cargo-make CLI arguments access.<br>
There are several different rust script runners currently available:

* [rust-script](https://crates.io/crates/rust-script)
* [cargo-script](https://crates.io/crates/cargo-script)
* [cargo-play](https://crates.io/crates/cargo-play)

By default, rust-script is used, however this can be changed via environment variable **`CARGO_MAKE_RUST_SCRIPT_PROVIDER`** which should hold the crate name.<br>
This enables to define a different runner for each task by setting it in the **env** block of the specific tasks.<br>
For example:

```toml
[tasks.rust-script]
env = { "CARGO_MAKE_RUST_SCRIPT_PROVIDER" = "rust-script" }
script_runner = "@rust"
script = '''
fn main() {
    println!("test");
}
'''

[tasks.cargo-script]
env = { "CARGO_MAKE_RUST_SCRIPT_PROVIDER" = "cargo-script" }
script_runner = "@rust"
script = '''
fn main() {
    println!("test");
}
'''

[tasks.cargo-play]
env = { "CARGO_MAKE_RUST_SCRIPT_PROVIDER" = "cargo-play" }
script_runner = "@rust"
script = '''
fn main() {
    println!("test");
}
'''
```

Keep in mind that dependencies used by the rust script are defined differently for each runner.<br>
Please see the specific crate docs for learn more.

<a name="usage-task-command-script-task-exampleshell2batch"></a>
#### Cross Platform Shell
In this example, when the **shell** task is invoked, the **script** content will be automatically converted to Windows batch commands (when running on a Windows platform) and invoked.

```toml
[tasks.shell]
script_runner = "@shell"
script = '''
rm ./myfile.txt
'''
```

Same as OS scripts, the @shell runner also supports the cargo-make CLI arguments access.<br>
<br>
See [shell2batch](https://github.com/sagiegurari/shell2batch) project for complete set of features.

<a name="usage-task-command-script-task-examplegeneric"></a>
#### Other Programming Languages
cargo-make can also run scripts written in various scripting languages such as Python, Perl, Ruby, Javascript, and more...<br>
Any runner which takes the form of **command file** (for example **`python ./program.py`**) is supported.

Below are few examples:

```toml
[tasks.python]
script_runner = "python"
script_extension = "py"
script = '''
print("Hello, World!")
'''

[tasks.perl]
script_runner = "perl"
script_extension = "pl"
script = '''
print "Hello, World!\n";
'''

[tasks.javascript]
script_runner = "node"
script_extension = "js"
script = '''
console.log('Hello, World!');
'''

[tasks.php]
script_runner = "php"
script_extension = "php"
script = '''
<?php
echo "Hello, World!\n";
'''

[tasks.powershell]
script_runner = "powershell"
script_extension = "ps1"
script = '''
Write-Host "Hello, World!"
'''
```

In case you need to provider the script runner arguments before the script file, you can use the **script_runner_args** attribute.<br>
For example:

```toml
[tasks.php-with-args]
script_runner = "php"
script_runner_args = ["-f"]
script_extension = "php"
script = '''
<?php
echo "Hello, World!\n";
'''
```

*script_runner_args requires script_extension defined as well.*

<a name="usage-task-command-script-task-exampleshebang"></a>
#### Shebang Support
Instead of defining custom runners via **script_runner** attribute, it's possible to define it in the script shebang line.

In case of Windows, make sure not to use a runner which doesn't have the **#** character defined as comment (for example, `cmd.exe` does not!), which would lead to an error.

Example task using bash:

```toml
[tasks.shebang-sh]
script = '''
#!/usr/bin/env bash
echo hello
'''
```

Output:

```console
> cargo make --cwd ./examples --makefile ./shebang.toml shebang-sh
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: ./shebang.toml
[cargo-make] INFO - Task: shebang-sh
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: shebang-sh
[cargo-make] INFO - Execute Command: "/usr/bin/env" "bash" "/tmp/cargo-make/cJf6XEXrL9.sh"
hello
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

Example task using Python:

```toml
[tasks.shebang-python]
script = '''
#!/usr/bin/env python3
print("Hello, World!")
'''
```

Output:

```console
> cargo make --cwd ./examples --makefile ./shebang.toml shebang-python
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: ./shebang.toml
[cargo-make] INFO - Task: shebang-python
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: shebang-python
[cargo-make] INFO - Execute Command: "/usr/bin/env" "python3" "/tmp/cargo-make/Wy3QMJiQaS.sh"
Hello, World!
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

Another trick you can do with shebangs is to define one of the special runners like @duckscript as follows:

```toml
[tasks.duckscript-shebang-example]
script = '''
#!@duckscript
echo Running duckscript without runner attribute.
'''
```

However that language must support comments starting with the **#** character.

<a name="usage-default-tasks"></a>
### Default Tasks and Extending
There is no real need to define some of the basic **build**, **test**, ... tasks that were shown in the previous examples.<br>
cargo-make comes with a built-in toml file that will serve as a base for every execution.<br>
The **optional** external toml file that is provided while running cargo-make will only extend and add or overwrite
tasks that are defined in the [default makefiles](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/descriptor/makefiles/).<br>

Let's take the built-in **build** task, defined in the default toml:

```toml
[tasks.build]
description = "Runs the rust compiler."
category = "Build"
command = "cargo"
args = ["build", "--all-features"]
```

If for example, you would like to add verbose output to it and remove the **--all-features** flag, you would just need to change the args and add the --verbose as follows:

```toml
[tasks.build]
args = ["build", "--verbose"]
```

If you want to disable some existing task (will also disable its dependencies), you can do it as follows:

```toml
[tasks.build]
disabled = true
```

There is no need to redefine existing properties of the task, only what needs to be added or overwritten.<br>
The default toml file comes with many steps and flows already built-in, so it is worth it to check it out first.<br>

In case you do want to delete all of the original task attributes in your extended task, you can use the **clear** attribute as follows:

```toml
[tasks.sometask]
clear = true
command = "echo"
args = [
    "extended task"
]
```

You can also extend additional external files from your external makefile by using the **extend** attribute, for example:

```toml
extend = "my_common_makefile.toml"
```

The file path in the **extend** attribute is always relative to the current toml file you are in, not to the process working directory.

The extend attribute can be very useful when you have a workspace with a `Makefile.toml` that contains all of the common custom tasks and in each project you can have a simple `Makefile.toml` which just has
the extend attribute pointing to the workspace makefile.

<a name="usage-workspace-extending-external-makefile"></a>
#### Extending External Makefiles
In order for a makefile to extend additional external files from your external file by using the extend attribute, for example:

```toml
extend = "my_common_makefile.toml"
```

The file path in the extend attribute is always relative to the current toml file you are in and not to the process working directory.<br>
The makefile pointed to in the extend attribute must exist or the build will fail.

In order to define optional extending makefiles, you will need to pass the optional flag in addition to the path as follows:

```toml
extend = { path = "does_not_exist_makefile.toml", optional = true }
```

You can also define a list of makefiles to extend from.<br>
All will be loaded in the order you define.<br>
For example:

```toml
extend = [ { path = "must_have_makefile.toml" }, { path = "optional_makefile.toml", optional = true }, { path = "another_must_have_makefile.toml" } ]
```

<a name="usage-workspace-extend"></a>
#### Automatically Extend Workspace Makefile
When running cargo make for modules which are part of a workspace, you can automatically have the member crates makefile (even if doesn't exist) extend the workspace level makefile.

The workspace level makefile **env** section must contain the following environment variable (you can also set it via CLI).

```toml
[env]
CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE = true
```

This allows you to maintaining a single makefile for the entire workspace but having access to those custom tasks in every member crate.
<br>
**This is only relevant for workspace builds which are triggered in the workspace root.<br>
Flows that start directly in the member crate, must manually extend the workspace level makefile using the extend keyword.**

<a name="usage-load-scripts"></a>
#### Load Scripts
In more complex scenarios, you may want multiple unrelated projects to share some common custom tasks. For example, you may wish to notify some internal company server of the build status.<br>
Instead of redefining those tasks in each project, you can create a single toml file with those definitions and have all projects extend that file.<br>
However, this “extend” functionality only knows to find the such files in the local file system. So, in order to pull some common toml from a remote server, (using `http` or `git clone` and so on...), you can use the load scripts.

Load scripts are defined in the config section using the **load_script** attribute and are invoked **before** the extend attribute is evaluated.<br>
This allows you to first pull the toml file from the remote server and put it in a location defined by the extend attribute.

Here is an example of a load script which downloads the common toml from a remote server using HTTP:

```toml
[config]
load_script = "wget -O /home/myuser/common.toml companyserver.com/common.toml"
```

Here is an example of pulling the common toml file from some git repo:

```toml
[config]
load_script = "git clone git@mygitserver:user/project.git /home/myuser/common"
```

You can run any command or set of commands you want. Therefore, you can build a more complex flow of how and from where to fetch the common toml file, and where to put it.<br>
If needed, you can override the load_script per platform using the **linux_load_script**, **windows_load_script** and **mac_load_script** attributes.

<a name="usage-predefined-makefiles"></a>
#### Predefined Makefiles

While cargo-make comes with many built in tasks, defined in the [default makefiles](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/descriptor/makefiles/), they are not always relevant for every project.<br>
The [cargo-make-tasks](https://github.com/sagiegurari/cargo-make-tasks/) repository holds a collection of additional makefiles that can be loaded and provide replacement tasks for the built in cargo-make tasks.<br>
For example the cmake.toml provides cmake related tasks for projects using cmake.

See the [cargo-make-tasks](https://github.com/sagiegurari/cargo-make-tasks/) repository for more information and usage examples.

<a name="usage-default-task"></a>
#### The Default Task

When invoking the cargo make command without a task name, the default task is invoked.<br>
The default task is actually an alias to another task defined as follows:

```toml
[tasks.default]
alias = "dev-test-flow"
```

There are multiple ways to define the default task differently, for example:

* Alias to another task in your custom makefile

```toml
[tasks.default]
alias = "my-custom-task"
```

* Clear the alias and define the task actions

```toml
[tasks.default]
clear = true # clears the alias
command = "echo"
args = ["custom!!!"]
```

<a name="usage-extending-tasks"></a>
### Extending Tasks

There are multiple ways of extending tasks in the same or from extended makefiles.

* [Task Override](#usage-task-override)
* [Platform Override](#usage-platform-override)
* [Extend Attribute](#usage-task-extend-attribute)

<a name="usage-task-override"></a>
#### Task Override
cargo-make comes with many predefined tasks and flows that can be used without redefining them in your project.<br>
However in some cases, you would like to change them a bit to fit your needs without rewriting the entire task.<br>
Let's take, for example, the **build** task which is predefined internally inside cargo-make as follows:

```toml
[tasks.build]
description = "Runs the rust compiler."
category = "Build"
command = "cargo"
args = ["build", "--all-features"]
```

If you do not want to use the **--all-features** mode, you can just change the args of the task in your external `Makefile.toml` as follows:

```toml
[tasks.build]
args = ["build"]
```

When cargo-make starts up, it will load the external `Makefile.toml` and the internal makefile definitions and will merge them.<br>
Since the external file overrides the internal definitions, only the args attribute for the **build** task which was redefined,
will override the args attribute which was defined internally, and the actual result would be:

```toml
[tasks.build]
description = "Runs the rust compiler."
category = "Build"
command = "cargo"
args = ["build"]
```

The same process can be used to override tasks from other makefiles loaded using the extend keyword from [Extending External Makefiles](#usage-workspace-extending-external-makefile) section.

<a name="usage-platform-override"></a>
#### Platform Override
If you want to override a task (or specific attributes in a task) for specific platforms, you can define an override task with the platform name (currently Linux, Windows, and macOS) under the specific task.<br>
For example:

```toml
[tasks.hello-world]
script = '''
echo "Hello World From Unknown"
'''

[tasks.hello-world.linux]
script = '''
echo "Hello World From Linux"
'''
```

If you run cargo make with task 'hello-world' on Linux, it would redirect to hello-world.linux while on other platforms it will execute the original hello-world.<br>
In Linux the output would be:

```console
[cargo-make] INFO - Task: hello-world
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: hello-world
[cargo-make] INFO - Execute Command: "sh" "/tmp/cargo-make/kOUJfw8Vfc.sh"
Hello World From Linux
[cargo-make] INFO - Build done in 0 seconds.
```

While on other platforms it would output:

```console
[cargo-make] INFO - Task: hello-world
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: hello-world
[cargo-make] INFO - Execute Command: "sh" "/tmp/cargo-make/2gYnulOJLP.sh"
Hello World From Unknown
[cargo-make] INFO - Build done in 0 seconds.
```

In the override task, you can define any attribute that will override the attribute of the parent task, while undefined attributes will use the value from the parent task and will not be modified.<br>
If you need to delete attributes from the parent (for example, you have a command defined in the parent task, but you want to have a script defined in the override task), then you will
have to clear the parent task in the override task using the clear attribute as follows:

```toml
[tasks.hello-world.linux]
clear = true
script = '''
echo "Hello World From Linux"
'''
```

This means, however, that you will have to redefine all attributes in the override task that you want to carry with you from the parent task.<br>
**Important: alias comes before checking override task, so if the parent task has an alias, it will be redirected to that task instead of the override.**<br>
**To have an alias redirect per-platform, use the linux_alias, windows_alias, mac_alias attributes.**<br>
**In addition, aliases cannot be defined in platform override tasks, only in parent tasks.**

<a name="usage-task-extend-attribute"></a>
#### Extend Attribute
Until now, the override capability enabled to override the task with the same name from different makefile or in different platforms.<br>
However, the **extend** keyword is also available on the task level and enables you to override any task by name.<br>
Let's look at the following example:

```toml
[tasks.1]
category = "1"
description = "1"
command = "echo"
args = ["1"]

[tasks.2]
extend = "1"
category = "2"
args = ["2"]

[tasks.3]
extend = "2"
args = ["3"]
```

When task **3** is loaded, it loads task **2** which loads task **1**.<br>
The final task **3** definition would be:

```toml
[tasks.3]
extend = "2"
category = "2"
description = "1"
command = "echo"
args = ["3"]
```

We run task **3** the output would be:

```console
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: task_extend.toml
[cargo-make] INFO - Task: 3
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: 3
[cargo-make] INFO - Execute Command: "echo" "3"
3
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```


<a name="usage-env"></a>
### Environment Variables
`cargo-make` enabled the definition of environmental variables in several ways, which can later be accessed throughout task execution.

Because environmental variables play a significant role in `cargo-make`, it provides multiple declarative ways to provide them at different levels of granularity.

* [Declaration](#env-declaration)
* [Global Configuration](#usage-env-config)
* [Task](#usage-env-task)
* [Command Line](#usage-env-cli)
* [Env File](#usage-env-file)
* [Env Setup Scripts](#usage-env-setup-scripts)
* [Loading Order](#usage-env-vars-loading-order)
* [Note about Ordering](#env-note-about-ordering)
* [Global](#usage-env-global)

<a name="env-declaration"></a>
#### Declaration

There are multiple ways to declare environmental variables, all of which are suited for specific suitcases.

##### Simple

The most ordinary one is the definition of a simple `KEY=Value` pair, which is reminiscent of tools like [dotenv](https://www.npmjs.com/package/dotenv) and [bash scripts](https://www.gnu.org/software/bash/). Values can use other variables as values, which are interpolated at runtime, using the `${variable}` syntax.

```toml
STRING = "value"
RUST_BACKTRACE = 1
BOOL_VALUE = true
COMPOSITE = "${BOOL_VALUE} ${RUST_BACKTRACE}"
```

##### List

`cargo-make` also supports lists, which are joined using `;` at runtime.

```toml
LIST_VALUE = [ "VALUE1", "VALUE2", "VALUE3" ]
```

##### Script

`cargo-make` supports the use of simple scripts. The output of the said script will then determine the value of the environmental variable.

The script's object has two additional arguments: `multiline` and `depends_on`. If `multiple` is set to `true`, the supplied script will be evaluated as a script with multiple lines. `depends_on` is a list of environmental variables this script depends on, which is taken into account during reordering if unset `cargo-make` will try to guess the variables used during reordering.

> **Note:** This uses the default OS command runner (`cmd` on Windows, `sh` on UNIX systems), other runners like `duckscript`, `rust`, etc. are **not** supported.

```toml
EVALUATED_VAR = { script = ["echo SOME VALUE"] }
```

##### Decode Map

`cargo-make` supports the use of mappings where a `source` is matched against a dictionary of possible `mapping`s, where each key of the `mapping` is compared against the evaluated `source` value. Should the key and `source` be the same, the corresponding value to the key will be the value of the environmental variable. If no key is matched, the `default_value` is used if provided. Otherwise, it will default to an empty string instead.

```toml
LIBRARY_EXTENSION = { source = "${CARGO_MAKE_RUST_TARGET_OS}", default_value = "unknown", mapping = {"linux" = "so", "macos" = "dylib", "windows" = "dll", "openbsd" = "so" } }
```

##### Path

`cargo-make` supports the use of glob syntax to find all files and directories in a given directory. The list of files will be joined using `;` during execution.

```toml
PATH_GLOB = { glob = "./src/**/mod.rs", include_files = true, include_dirs = false, ignore_type = "git" }
```

##### Conditional

`cargo-make` supports conditional variables, which are set to the `value` specified if the `condition` evaluates to true. To learn more about conditions, refer to [this chapter](#usage-conditions)

##### Unset

Variables can be unset.

```toml
VARIABLE = {unset = true}
```

<a name="usage-env-config"></a>
#### Global Configuration

Environmental variables can be set globally using the top level `[env]` key, with the ability to provide multiple profiles, which can be selected using `--profile <name>` when executing `cargo make`.

Environment variables set in the global `[env]` block [and default `Makefile.toml`](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/descriptor/makefiles/stable.toml) will be set before running any tasks.

##### Example

```toml
[env]
RUST_BACKTRACE = 1
EVALUATED_VAR = { script = ["echo SOME VALUE"] }
TEST1 = "value1"
TEST2 = "value2"
BOOL_VALUE = true
DEV = false
PROD = false
COMPOSITE = "${TEST1} ${TEST2}"
MULTI_LINE_SCRIPT = { script = ["echo 1\necho 2"], multi_line = true }
CONDITIONAL_SCRIPT = { script = ["echo conditional_script"], condition = { env_not_set = ["CONDITIONAL_SCRIPT"] } }
LIBRARY_EXTENSION = { source = "${CARGO_MAKE_RUST_TARGET_OS}", default_value = "unknown", mapping = {"linux" = "so", "macos" = "dylib", "windows" = "dll", "openbsd" = "so" } }
TO_UNSET = { unset = true }
PREFER_EXISTING = { value = "new", condition = { env_not_set = ["PREFER_EXISTING"] } }
OVERWRITE_EXISTING = { value = "new", condition = { env_set = ["OVERWRITE_EXISTING"] } }
ENV_FROM_LIST = ["ARG1", "${SIMPLE}", "simple value: ${SIMPLE} script value: ${SCRIPT}"]
PATH_GLOB = { glob = "./src/**/mod.rs", include_files = true, include_dirs = false, ignore_type = "git" }

# profile based environment override
[env.development]
DEV = true

[env.production]
PROD = true
```

<a name="usage-env-task"></a>
#### Task

Environmental variables can be set in a task's scope, and will be merged with the global environment when that task gets executed. This means that the evaluation of environmental variables takes place after all dependencies have run, but _before_ the task itself runs.

> **Note:** Reordering of task variables with global variables will **not** take place. Tasks simply overwrite previously declared variables.

> **Note:** Variables are **not** cleaned up after execution, meaning that tasks following the executed task will inherit the variables set by the previous task.

`cargo-make` supports the same capabilities outlined for global configuration on a individual task level.

```toml
[tasks.test-flow]
env = { "SOME_ENV_VAR" = "value" }
run_task = "actual-task"

[tasks.actual-task]
condition = { env_set = [ "SOME_ENV_VAR" ] }
script = '''
echo var: ${SOME_ENV_VAR}
'''
```

<a name="usage-env-cli"></a>
#### Command Line
Environment variables can be defined in the command line using the `--env` / `-e` argument as follows:

```console
cargo make --env ENV1=VALUE1 --env ENV2=VALUE2 -e ENV3=VALUE3
```

<a name="usage-env-file"></a>
#### Env File
It is also possible to provide an env file path as part of the CLI args as follows:

```console
cargo make --env-file=./env/production.env
```

This allows using the same `Makefile.toml`, but with a different set of environmental variables loaded from the env file.

The env file is a simple `key=value`, which is similar to [dotenv](https://www.npmjs.com/package/dotenv), but only supports variable interpolation using the `${}` syntax.

```properties
#just a comment...
ENV1_TEST=TEST1
ENV2_TEST=TEST2
ENV3_TEST=VALUE OF ENV2 IS: ${ENV2_TEST}
```

Paths to environment files can also be defined globally in the `env_files` key of the `Makefile.toml`, which will be loaded in the order they are defined. All relative paths are relative to the directory containing the `Makefile.toml` they were defined in.

> **Note:** `env_files` can also be used on a task level. Be aware that relative paths will instead be relative to the **current working directory**

```toml
env_files = [
    "./env1.env",
    "./env2.env"
]
```

To only load environmental variables whenever a variable hasn't been defined yet, use the `defaults_only` property.

```toml
env_files = [
    { path = "./load_only_undefined.env", defaults_only = true },
    { path = "./load_all.env" }
]
```

Use the `profile` property to only load environmental variables whenever a specific profile is active.

> To learn more about profiles, check the [profiles section](#usage-profiles).

```toml
env_files = [
    { path = "./profile.env", profile = "development" },
    { path = "./env.env" }
]
```

<a name="usage-env-setup-scripts"></a>
#### Env Setup Scripts

Environment setup scripts are invoked after environment files and the env block. They are defined globally by the **env_scripts** attribute. These scripts can run anything needed before starting up the flow.

In the case of `duckscript` scripts invoked by the embedded runtime, it is possible to modify the `cargo-make` runtime environment variables directly.

For Example:

```toml
env_scripts = [
'''
#!@duckscript
echo first env script...

composite_env_value = get_env COMPOSITE
echo COMPOSITE = ${composite_env_value}

set_env COMPOSITE_2 ${composite_env_value}
''',
'''
#!@duckscript
echo second env script...

composite_env_value = get_env COMPOSITE_2
echo COMPOSITE_2 = ${composite_env_value}
'''
]

[env]
SIMPLE = "SIMPLE VALUE"
SCRIPT = { script = ["echo SCRIPT VALUE"] }
COMPOSITE = "simple value: ${SIMPLE} script value: ${SCRIPT}"
```

In this example, since the **env** block is invoked before the env scripts, the `duckscript`s have access to the `COMPOSITE` environment variable.<br>
These scripts use that value to create a new environment variable **`COMPOSITE_2`**, and in the second script, we print it.

<a name="usage-env-vars-loading-order"></a>
#### Loading Order

`cargo-make` will load the environment variables in the following order

* Load environment file provided on the command line
* Setup internal environment variables (see [Global](#usage-env-global) section). **Does not per-task variables.**
* Load global environment files defined in the **env_files** attribute.
* Load global environment variables provided on the command line.
* Load global environment variables defined in the **env** block and relevant sub env blocks based on profile/additional profiles.
* Load global environment variables defined in the **env.\[current profile\]** block.
* Load global environment setup scripts defined in the **env_scripts** attribute.
* **Per Task**
  * Load environment files defined in the **env_files** attribute (relative paths are treated differently than global env_files).
  * Setup **per task** internal environment variables (see [Global](#usage-env-global) section).
  * Load environment variables defined in the **env** block (same behavior as global env block).

During each step, variables can be reordered to ensure all dependencies are specified. The environmental variables will be interpolated before every task run.

<a name="env-note-about-ordering"></a>
#### Note about Ordering

The ordering of environmental variables in `cargo-make` is not necessarily the same between definition and evaluation. `cargo-make` instead looks at the values and reorders variables depending on the variables they mention.

This behavior has many benefits, like the ability to reference other variables freely or redefine them, in different scopes.

```toml
[env]
VAR1="${VAR2}"
VAR2=2
```

A naive implementation would now result in `VAR1=""`, `VAR2=2`, this behavior can be very unexpected, especially when extending existing declarations of environment variables. `cargo-make` is different and uses an approach that is similar to tools like [`terraform`](https://www.terraform.io), it will recognize that `VAR1` depends on `VAR2`, which will output `VAR1=2`, `VAR2=2`.

```toml
[env]
VAR1="${VAR2}"

[env.prod]
VAR2=2

[env.devel]
VAR2=3
```

This is an extended example, which would not work using the naive implementation, because the different profiles are merged with the environment (basically appending them). This is not the case with `cargo-make`, which will recognize dependencies and correctly resolve all values.

###### Naive Implementation

```
--release=test
    VAR1=""
--release=prod
    VAR1=""
    VAR2=2
--release=devel
    VAR1=""
    VAR2=3
```

###### `cargo-make` Implementation

```
--release=test
    VAR1=""
--release=prod
    VAR1="2"
    VAR2=2
--release=devel
    VAR1="3"
    VAR2=3
```

<a name="usage-env-global"></a>
#### Global
In addition to manually setting environment variables, cargo-make will also automatically add a few environmental variables, which can be helpful when running task scripts, commands, conditions, and more.

* **`CARGO_MAKE`** - Set to "true" to help sub-processes identify they are running from `cargo` make.
* **`CARGO_MAKE_TASK`** - Holds the name of the main task being executed.
* **`CARGO_MAKE_TASK_ARGS`** - A list of arguments provided to cargo-make after the task name, separated with a ';' character.
* **`CARGO_MAKE_CURRENT_TASK_NAME`** - Holds the currently executed task name.
* **`CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE`** - Holds the full path to the makefile, which **initially** defined the currently executed task (not available for internal core tasks).
* **`CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE_DIRECTORY`** - Holds the full path to the directory containing the makefile **initially** defined the currently executed task (not available for internal core tasks).
* **`CARGO_MAKE_COMMAND`** - The command used to invoke cargo-make (for example: *cargo make* and *makers*)
* **`CARGO_MAKE_WORKING_DIRECTORY`** - The current working directory (can be defined by setting the `--cwd` CLI option)
* **`CARGO_MAKE_WORKSPACE_WORKING_DIRECTORY`** - The original working directory of the workspace. Enables workspace members access to the workspace level `CARGO_MAKE_WORKING_DIRECTORY`.
* **`CARGO_MAKE_PROFILE`** - The current profile name in lower case (should not be manually modified by global/task env blocks)
* **`CARGO_MAKE_ADDITIONAL_PROFILES`** - The additional profile names in lower case, separated with a `;` character (should not be manually modified by global/task env blocks)
* **`CARGO_MAKE_PROJECT_NAME`** - For standalone crates, this will be the same as `CARGO_MAKE_CRATE_NAME`, and for workspace, it will default to the working directory basename.
* **`CARGO_MAKE_PROJECT_VERSION`** For standalone crates, this will be the same as `CARGO_MAKE_CRATE_VERSION`, and for workspaces, it will be the main crate version (main crate defined by the optional **main_project_member** attribute in the config section).
* **`CARGO_MAKE_CARGO_HOME`** - The path to `CARGO_HOME` as described in the [cargo documentation](https://doc.rust-lang.org/cargo/guide/cargo-home.html)
* **`CARGO_MAKE_CARGO_PROFILE`** - The [cargo profile](https://doc.rust-lang.org/cargo/reference/manifest.html#the-profile-sections) name mapped from the **`CARGO_MAKE_PROFILE`** (unmapped value will default to `CARGO_MAKE_PROFILE` value)
* **`CARGO_MAKE_RUST_VERSION`** - The rust version (for example 1.20.0)
* **`CARGO_MAKE_RUST_CHANNEL`** - Rust channel (stable, beta, nightly)
* **`CARGO_MAKE_RUST_TARGET_ARCH`** - x86, x86_64, arm, etc ... (see rust cfg feature)
* **`CARGO_MAKE_RUST_TARGET_ENV`** - gnu, msvc, etc ... (see rust cfg feature)
* **`CARGO_MAKE_RUST_TARGET_OS`** - Windows, macOS, iOS, Linux, Android, etc. ... (see rust cfg feature)
* **`CARGO_MAKE_RUST_TARGET_POINTER_WIDTH`** - 32, 64
* **`CARGO_MAKE_RUST_TARGET_VENDOR`** - apple, pc, unknown
* **`CARGO_MAKE_RUST_TARGET_TRIPLE`** - x86_64-unknown-linux-gnu, x86_64-apple-darwin, x86_64-pc-windows-msvc, etc ...
* **`CARGO_MAKE_CRATE_TARGET_DIRECTORY`** - Gets target directory where cargo stores the output of a build, respects `${CARGO_TARGET_DIR}`, `.cargo/config.toml`'s and `${CARGO_HOME}/config.toml`, but not `--target-dir` command-line flag.
* **`CARGO_MAKE_CRATE_CUSTOM_TRIPLE_TARGET_DIRECTORY`** - Like `CARGO_MAKE_CRATE_TARGET_DIRECTORY` but respects `build.target` in `.cargo/config.toml`.
* **`CARGO_MAKE_CRATE_HAS_DEPENDENCIES`** - Holds `true`/`false` based if there are dependencies defined in the `Cargo.toml` or not (defined as *false* if no `Cargo.toml` is found)
* **`CARGO_MAKE_CRATE_IS_WORKSPACE`** - Holds `true`/`false` based if this is a workspace crate or not (defined even if no `Cargo.toml` is found)
* **`CARGO_MAKE_CRATE_WORKSPACE_MEMBERS`** - Holds a list of member paths (defined as empty value if no `Cargo.toml` is found)
* **`CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER`** - Holds the name of the current workspace member being built (only if flow started as a workspace level flow)
* **`CARGO_MAKE_CRATE_LOCK_FILE_EXISTS`** - Holds `true`/`false` if a `Cargo.lock` file exists in the current working directory (in workspace projects, each member has a different working directory).
* **`CARGO_MAKE_CRATE_TARGET_TRIPLE`** - Gets target triple that will be build with by default, respects `.cargo/config.toml` and `${CARGO_HOME}/config.toml`.
* **`CARGO_MAKE_WORKSPACE_PACKAGE_NAME`** - Holds the root package name of the workspace from the `Cargo.toml` file in the current working directory.
* **`CARGO_MAKE_WORKSPACE_PACKAGE_VERSION`** - Holds the root package version of the workspace from the `Cargo.toml` file in the current working directory.
* **`CARGO_MAKE_WORKSPACE_PACKAGE_DESCRIPTION`** - Holds the root package description of the workspace from the `Cargo.toml` file in the current working directory.
* **`CARGO_MAKE_WORKSPACE_PACKAGE_LICENSE`** - Holds the root package license of the workspace from the `Cargo.toml` file in the current working directory.
* **`CARGO_MAKE_WORKSPACE_PACKAGE_DOCUMENTATION`** - Holds the root package documentation link of the workspace from the `Cargo.toml` file in the current working directory.
* **`CARGO_MAKE_WORKSPACE_PACKAGE_HOMEPAGE`** - Holds the root package homepage link of the workspace from the `Cargo.toml` file in the current working directory.
* **`CARGO_MAKE_WORKSPACE_PACKAGE_REPOSITORY`** - Holds the root package repository link of the workspace from the `Cargo.toml` file in the current working directory.
* **`CARGO_MAKE_CI`** - Holds `true`/`false` if the task runs in a continuous integration system (such as Travis CI).
* **`CARGO_MAKE_PR`** - Holds `true`/`false` if the task runs in a continuous integration system (such as Travis CI) as part of a pull request build (unknown is set as false).
* **`CARGO_MAKE_CI_BRANCH_NAME`** - Holds the continuous integration branch name (if available).
* **`CARGO_MAKE_CI_VENDOR`** - Holds the continuous integration vendor name (if available).
* **`CARGO_MAKE_DUCKSCRIPT_VERSION`** - The embedded `duckscript` runtime version.
* **`CARGO_MAKE_DUCKSCRIPT_SDK_VERSION`** - The embedded `duckscript` SDK version.

The following environment variables will be set by cargo-make if `Cargo.toml` file exists and the relevant value is defined:

* **`CARGO_MAKE_CRATE_NAME`** - Holds the crate name from the `Cargo.toml` file in the current working directory.
* **`CARGO_MAKE_CRATE_FS_NAME`** - Same as `CARGO_MAKE_CRATE_NAME` however some characters are replaced (for example '-' to '_').
* **`CARGO_MAKE_CRATE_VERSION`** - Holds the crate version from the `Cargo.toml` file found in the current working directory.
* **`CARGO_MAKE_CRATE_DESCRIPTION`** - Holds the crate description from the `Cargo.toml` file in the current working directory.
* **`CARGO_MAKE_CRATE_LICENSE`** - Holds the crate license from the `Cargo.toml` file in the current working directory.
* **`CARGO_MAKE_CRATE_DOCUMENTATION`** - Holds the crate documentation link from the `Cargo.toml` file in the current working directory.
* **`CARGO_MAKE_CRATE_HOMEPAGE`** - Holds the crate homepage link from the `Cargo.toml` file in the current working directory.
* **`CARGO_MAKE_CRATE_REPOSITORY`** - Holds the crate repository link from the `Cargo.toml` file in the current working directory.

The following environment variables will be set by cargo-make if the project is part of a git repo:

* **`CARGO_MAKE_GIT_BRANCH`** - The current branch name.
* **`CARGO_MAKE_GIT_USER_NAME`** - The user name pulled from the git config user.name key.
* **`CARGO_MAKE_GIT_USER_EMAIL`** - The user email, which was taken from the git config `user.email` key.
* **`CARGO_MAKE_GIT_HEAD_LAST_COMMIT_HASH`** - The last HEAD commit hash.
* **`CARGO_MAKE_GIT_HEAD_LAST_COMMIT_HASH_PREFIX`** - The last HEAD commit hash prefix.

<a name="usage-setting-up-working-directory"></a>
### Setting Up Working Directory
To modify the current working directory for a specific task (not entire run), use the **cwd** attribute.<br>
For example:

```toml
[tasks.move-dir]
cwd = "./mysubdir/"
```

<a name="usage-ignoring-errors"></a>
### Ignoring Errors
In some cases you want to run optional tasks as part of a bigger flow, but do not want to break your entire build in case of any error in those optional tasks.<br>
For those tasks, you can add the **ignore_errors=true** attribute.

```toml
[tasks.unstable_task]
ignore_errors = true
```

<a name="usage-conditions"></a>
### Conditions
Conditions allow you to evaluate at runtime if to run a specific task or not.<br>
These conditions are evaluated before the task is running its installation and/or commands and if the condition is not fulfilled, the task will not be invoked.<br>
The task dependencies however are not affected by parent task condition outcome.

There are two types of conditions:

* [Criteria](#usage-conditions-structure)
* [Scripts](#usage-conditions-script)

The task runner will evaluate any condition defined and a task definition may contain both types at the same time.

<a name="usage-conditions-structure"></a>
#### Criteria
The condition attribute may define multiple parameters to validate.<br>
All defined parameters must be valid for the condition as a whole to be true and enable the task to run.

Below is an example of a condition definition that checks that we are running on Windows or Linux (but not macOS) and that we are running on beta or nightly (but not stable):

```toml
[tasks.test-condition]
condition = { platforms = ["windows", "linux"], channels = ["beta", "nightly"] }
script = '''
echo "condition was met"
'''
```

The following condition types are available:

* **profile** - See [profiles](#usage-profiles) for more info
* **os** - List of OS names (Windows, macOS, iOS, Linux, Android, etc... as defined by cfg!(target_os))
* **platforms** - List of platform names (windows, linux, mac)
* **channels** - List of rust channels (stable, beta, nightly)
* **env_set** - List of environment variables that must be defined
* **env_not_set** - List of environment variables that must not be defined
* **env_true** - List of environment variables that must be defined and must not be set to any of the following (case insensitive): false, no, 0 or empty
* **env_false** - List of environment variables that must be defined and set to any of the following (case insensitive): false, no, 0 or empty
* **env** - Map of environment variables that must be defined and equal to the provided values
* **env_contains** - Map of environment variables that must be defined and contain (case insensitive) the provided values
* **rust_version** - Optional definition of min, max, and/or specific rust version
* **files_exist** - List of absolute path files to check they exist. Environment substitution is supported so you can define relative paths such as **`${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml`**
* **files_not_exist** - List of absolute path files to check they do not exist. Environment substitution is supported so you can define relative paths such as **`${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml`**
* **files_modified** - Lists input and output globs. If any input file is newer than all output files, the condition is met. Environment substitution is supported so you can define relative paths such as **`${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml`**

Few examples:

```toml
[tasks.test-condition]
condition = {
    profiles = ["development", "production"],
    platforms = ["windows", "linux"],
    channels = ["beta", "nightly"],
    env_set = [ "CARGO_MAKE_KCOV_VERSION" ],
    env_not_set = [ "CARGO_MAKE_SKIP_CODECOV" ],
    env = { "CARGO_MAKE_CI" = true, "CARGO_MAKE_RUN_CODECOV" = true },
    rust_version = { min = "1.20.0", max = "1.30.0" },
    files_exist = ["${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml"],
    files_not_exist = ["${CARGO_MAKE_WORKING_DIRECTORY}/Cargo2.toml"],
    files_modified = { input = ["${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml", "./src/**/*.rs"], output = ["./target/**/myapp*"] }
}
```

To setup a custom failure message, use the **fail_message** inside the condition object, for example:

```toml
[tasks.test-condition-with-message]
condition = { platforms = ["windows"], fail_message = "Condition Failed." }
command = "echo"
args = ["condition was met"]
```

Fail messages are only printed if log level is verbose or reduce output flag is set to false in the config as follows:

```toml
[config]
reduce_output = false
```

<a name="usage-conditions-script"></a>
#### Scripts
These script are invoked before the task is running its installation and/or commands and if the exit code of the condition script is non zero, the task will not be invoked.

Below is an example of a condition script that always returns a non zero value, in which case the command is never executed:

```toml
[tasks.never]
condition_script = """
exit 1
"""
command = "cargo"
args = ["build"]
```

Condition scripts can be used to ensure that the task is only invoked if a specific condition is met, for example if a specific 3rd party is installed.

To setup a custom failure message, use the **fail_message** inside the condition object, for example:

```toml
[tasks.test-condition-script-with-message]
condition = { fail_message = "Condition Script Failed." }
condition_script = [
    "exit 1"
]
command = "echo"
args = ["condition was met"]
```

<a name="usage-conditions-and-or"></a>
#### And/Or/Group Or

By default all conditions groups and all conditions inside each group are evaluated and an 'AND' is used to validate everything is as requested.<br>
However, there are other condition types available:

* Or - All groups and all conditions inside each group are searched for a single condition that is met
* GroupOr - All conditions in each group are searched for a single condition that is met but all condition groups must pass.

Simply add the condition_type with any of these values inside the condition object.<br>
For example:

```
[tasks.test-or-condition]
condition = { condition_type = "Or", env_true = [
  "TRUE_ENV",
  "FALSE_ENV",
], env_false = [
  "TRUE_ENV",
  "FALSE_ENV",
] }
script = '''
echo "condition was met"
'''
```

<a name="usage-conditions-and-subtasks"></a>
#### Combining Conditions and Sub Tasks

Conditions and run_task combined can enable you to define a conditional sub flow.<br>
For example, if you have a coverage flow that should only be invoked on linux in a CI build, and only if the `CARGO_MAKE_RUN_CODECOV` environment variable is defined as "true":

```toml
[tasks.ci-coverage-flow]
description = "Runs the coverage flow and uploads the results to codecov."
condition = { platforms = ["linux"], env = { "CARGO_MAKE_CI" = true, "CARGO_MAKE_RUN_CODECOV" = true } }
run_task = "codecov-flow"

[tasks.codecov-flow]
description = "Runs the full coverage flow and uploads the results to codecov."
windows_alias = "empty"
dependencies = [
    "coverage-flow",
    "codecov"
]
```

The first task **ci-coverage-flow** defines the condition that checks we are on linux, running as part of a CI build and the `CARGO_MAKE_RUN_CODECOV` environment variable is set to "true".<br>
Only if all conditions are met, it will run the **codecov-flow** task.<br>
We can't define the condition directly on the **codecov-flow** task, as it will invoke the task dependencies before checking the condition.

<a name="usage-running-tasks-only-if-sources-changed"></a>
#### Running Tasks Only If Sources Changed

The **files_modified** condition enables tasks to be skipped based on file modifications timestamp.<br>
The condition will cause the task to be skipped if no input file was found to be newer then any of the files in the output.<br>
The input and output are defined as arrays of **globs** (not regex) of files to check.<br>
In the below example, if the target binaries are newer then the Cargo.toml or any of the rust sources in the src directory, it will not run cargo build command.

```toml
[tasks.compile-if-modified]
condition = { files_modified = { input = ["${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml", "./src/**/*.rs"], output = ["./target/**/myapp*"] } }
command = "cargo"
args = ["build"]
```

<a name="usage-installing-dependencies"></a>
### Installing Dependencies

Some tasks will require third party crates, rustup components, or other native tools.<br>
cargo-make provides multiple ways to setup those dependencies before running the task.

* [Cargo Plugins](#usage-installing-cargo-plugins)
* [Crates](#usage-installing-crates)
* [Rustup Components](#usage-installing-rustup-components)
* [Native Dependencies](#usage-installing-native-dependencies)
* [Defining Version](#usage-installing-version)
* [Global Lock Of Versions](#usage-installing-locked)
* [Installation Priorities](#usage-installing-dependencies-priorities)
* [Multiple Installations](#usage-installing-dependencies-multiple)

<a name="usage-installing-cargo-plugins"></a>
#### Cargo Plugins

When a task invokes a cargo plugin using the **command** attribute, for example:

```toml
[tasks.audit]
command = "cargo"
args = ["audit"]
```

cargo-make will first check the command is available.<br>
Only if the command is not available, it will attempt to install it by running **cargo install cargo-<first arg>**<br>
In case the cargo plugin has a different name, you can specify it manually via **install_crate** attribute.<br>
You can specify additional installation arguments using the **install_crate_args** attribute (for example: version).

To disable the automatic crate installation, you can set the **install_crate** attribute as false, for example:

```toml
[tasks.test]
command = "cargo"
args = ["test"]
install_crate = false
```

<a name="usage-installing-crates"></a>
#### Crates

cargo-make can verify third party crates are installed if the relevant installation info is provided.<br>
First it will check the crate is installed, and only if not available it will attempt to install it.<br>
Installation of third party crates is first done via rustup if the component name is provided.<br>
If rustup failed or component name is not provided, it will resort to using cargo install command.<br>
For example:

```toml
[tasks.rustfmt]
install_crate = { crate_name = "rustfmt-nightly", rustup_component_name = "rustfmt-preview", binary = "rustfmt", test_arg = "--help" }
command = "rustfmt"
```

In this example, cargo will first test that the command **rustfmt --help** works well and only if fails, it will first attempt
to install via rustup the component **rustfmt-preview** and if failed, it will try to run cargo install for the crate name **rustfmt-nightly**.

If passing multiple arguments is necessary, `test_arg` may contain an array of arguments. For example:

```toml
[tasks.doc-upload]
install_crate = { crate_name = "cargo-travis", binary = "cargo", test_arg = ["doc-upload", "--help"] }
command = "cargo"
args = ["doc-upload"]
```

In this example, cargo-make will test the presence of cargo-travis by running the command `cargo doc-upload --help`, and
install the crate only if this command fails.

<a name="usage-installing-rustup-components"></a>
#### Rustup Components

Rustup components that are not deployed as crates or components which are pure sources (no executable binary), can also be installed via cargo-make.<br>
The following example show how to install a rustup component with binaries:

```toml
[tasks.install-rls]
install_crate = { rustup_component_name = "rls-preview", binary = "rls", test_arg = "--help" }
```

In this example, cargo-make will first check if **rls** binary is available and only if failed to execute it, it will
install the **rls** component using rustup.<br>
<br>
Some rustup components are pure sources and therefore in those cases, cargo-make cannot verify that they are already installed, and
will attempt to install them every time.<br>
Example:

```toml
[tasks.install-rust-src]
install_crate = { rustup_component_name = "rust-src" }
```

<a name="usage-installing-native-dependencies"></a>
#### Native Dependencies

Native dependencies can also be installed, however it is up to the Makefile author to write the script which checks the dependency exists and if
not, to install it correctly.<br>
This is done by setting up an installation script in the **install_script** attribute of the task.<br>
It is possible to use platform overrides to specify different installation scripts for Linux/macOS/Windows platforms.<br>
For example:

```toml
[tasks.coverage-kcov]
windows_alias = "empty"
install_script = '''
KCOV_INSTALLATION_DIRECTORY=""
KCOV_BINARY_DIRECTORY=""
if [ -n "CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY" ]; then
    mkdir -p ${CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY}
    cd ${CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY}
    KCOV_INSTALLATION_DIRECTORY="$(pwd)/"
    cd -
    echo "Kcov Installation Directory: ${KCOV_INSTALLATION_DIRECTORY}"
    KCOV_BINARY_DIRECTORY="${KCOV_INSTALLATION_DIRECTORY}/build/src/"
    echo "Kcov Binary Directory: ${KCOV_BINARY_DIRECTORY}"
fi

# get help info to fetch all supported command line arguments
KCOV_HELP_INFO=`${KCOV_BINARY_DIRECTORY}kcov --help` || true

# check needed arguments are supported, else install
if [[ $KCOV_HELP_INFO != *"--include-pattern"* ]] || [[ $KCOV_HELP_INFO != *"--exclude-line"* ]] || [[ $KCOV_HELP_INFO != *"--exclude-region"* ]]; then
    # check we are on a supported platform
    if [ "$(grep -Ei 'debian|buntu|mint' /etc/*release)" ]; then
        echo "Installing/Upgrading kcov..."
        sudo apt-get update || true
        sudo apt-get install -y libcurl4-openssl-dev libelf-dev libdw-dev cmake gcc binutils-dev

        mkdir -p ${CARGO_MAKE_KCOV_DOWNLOAD_DIRECTORY}
        cd ${CARGO_MAKE_KCOV_DOWNLOAD_DIRECTORY}
        KCOV_DOWNLOAD_DIRECTORY=$(pwd)

        wget https://github.com/SimonKagstrom/kcov/archive/v${CARGO_MAKE_KCOV_VERSION}.zip
        unzip v${CARGO_MAKE_KCOV_VERSION}.zip
        cd kcov-${CARGO_MAKE_KCOV_VERSION}
        mkdir -p build
        cd ./build
        cmake ..
        make

        # if custom installation directory, leave kcov as local
        if [ -n "CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY" ]; then
            cd ${KCOV_DOWNLOAD_DIRECTORY}/kcov-${CARGO_MAKE_KCOV_VERSION}
            mv ./* ${KCOV_INSTALLATION_DIRECTORY}
        else
            sudo make install
            cd ../..
            rm -rf kcov-${CARGO_MAKE_KCOV_VERSION}
        fi
    fi
fi
'''
```

This task checks if kcov is installed; if not, it will install it and any other dependency it requires.

<a name="usage-installing-version"></a>
#### Defining Version

It is possible to define minimal version of depended crates, for example:

```toml
[tasks.simple-example]
install_crate = { min_version = "0.0.1" }
command = "cargo"
args = ["make", "--version"]

[tasks.complex-example]
install_crate = { crate_name = "cargo-make", binary = "cargo", test_arg = ["make", "--version"], min_version = "0.0.1" }
command = "cargo"
args = ["make", "--version"]
```

This ensures we are using a crate version that supports the feature we require for the build.<br>
Currently there are few limitations when defining **min_version**:

* Specifying **toolchain** in the task or **rustup_component_name** in the install_crate structure, will make cargo-make ignore the min version value.
* In case cargo-make is unable to detect the currently installed version due to any error, cargo-make will assume the version is valid and printout a warning.

If you want to ensure a specific version is used, you can define the **version** attribute instead, for example:

```toml
[tasks.complex-example]
install_crate = { crate_name = "cargo-make", binary = "cargo", test_arg = ["make", "--version"], version = "0.0.1" }
command = "cargo"
args = ["make", "--version"]
```

<a name="usage-installing-locked"></a>
#### Global Lock Of Versions

In case [min_version](#usage-installing-version) is defined,
you can have the **--locked** flag automatically added to the crate installation command
by defining the **`CARGO_MAKE_CRATE_INSTALLATION_LOCKED`=true** environment variable.
If version is defined instead of min_version, this will automatically be set as true.

<a name="usage-installing-alternate-cargo-install-commands"></a>
#### Alternate Cargo Install Commands

You can specify a different cargo install command in order to make the crate installation to use some custom cargo installer plugin.
For example, if you want to use instead of **install** a plugin such as **local-install** simply add the **install_command** attribute with the relevant value.<br>
For example:

```toml
[tasks.alt-command-example1]
install_crate = { install_command = "custom-install" }
command = "cargo"
args = ["somecrate"]

[tasks.alt-command-example2]
install_crate = { crate_name = "somecrate", install_command = "custom-install" }
```

By default, the **--force** flag is added. In order to remove it, add the force=false to the install_crate definition as follows:

```toml
[tasks.alt-command-example2]
install_crate = { crate_name = "somecrate", install_command = "custom-install", force = false }
```

<a name="usage-installing-dependencies-priorities"></a>
### Installation Priorities

Only one type of installation will be invoked per task.<br>
The following defines the installation types sorted by priority for which cargo-make uses to decide which installation flow to invoke:

* **install_crate** - Enables to install crates and rustup components.
* **install_script** - Custom script which can be used to install or run anything that is needed by the task command.
* **automatic cargo plugin** - In case the command is **cargo**, cargo-make will check which cargo plugin to automatically install (if needed).

In case multiple installation types are defined (for example both install_crate and install_script), only one installation type will be invoked based on the above priority list.

<a name="usage-installing-dependencies-multiple"></a>
### Multiple Installations

In some cases, tasks require multiple items installed in order to run properly.<br>
For example, you might need rustup component **rls** and **rust-src** and cargo plugin **cargo-xbuild** at the same task.<br>
In order to achieve this, you can split the task to invocation task and installation task and set the installation task as a dependency.<br>
The following example defines a flow of two similar tasks that have the same dependencies: cargo-xbuild crate, rls rustup binary component and rust-src rustup sources only component.<br>
You can have both rustup dependencies as an installation only tasks which are set as dependencies for the xbuild tasks.<br>
Since dependencies are only invoked once, it will also ensure that those rustup components are not installed twice.

```toml
[tasks.install-rls]
# install rls-preview only if needed
install_crate = { rustup_component_name = "rls-preview", binary = "rls", test_arg = "--help" }

[tasks.install-rust-src]
# always install rust-src via rustup component add
install_crate = { rustup_component_name = "rust-src" }

[tasks.xbuild1]
# run cargo xbuild, if xbuild is not installed, it will be automatically installed for you
command = "cargo"
args = [ "xbuild", "some arg" ]
dependencies = [ "install-rls", "install-rust-src" ]

[tasks.xbuild2]
# run cargo xbuild, if xbuild is not installed, it will be automatically installed for you
command = "cargo"
args = [ "xbuild", "another arg" ]
dependencies = [ "install-rls", "install-rust-src" ]

[tasks.myflow]
dependencies = [ "xbuild1", "xbuild2" ]
```

<a name="usage-workspace-support"></a>
### Workspace Support
In case cargo-make detects that the current working directory is a workspace root (A directory with `Cargo.toml` which defines a workspace and its members), it will not invoke the requested tasks in that directory.<br>
Instead, it will generate a task definition in runtime which will go to each member directory and invoke the requested task on that member.<br>
For example if we have the following directory structure:

```console
workspace
├── Cargo.toml
├── member1
│   └── Cargo.toml
└── member2
    └── Cargo.toml
```

And we ran **cargo make mytask**, it will go to each workspace member directory and execute: **cargo make mytask** at that directory,
where mytask is the original task that was requested on the workspace level.<br>
The order of the members is defined by the member attribute in the workspace `Cargo.toml`.

This flow is called a **workspace** flow, as it identifies the workspace and handles the request for each workspace member, while the root directory which defines the workspace structure is ignored.

We can use this capability to run same functionality on all workspace member crates, for example if we want to format all crates, we can run in the workspace directory: **cargo make format**.<br>

Member crate makefiles can also automatically extend the workspace directory makefile.<br>
See more info at the [relevant section.](#usage-workspace-extend)

<a name="usage-workspace-disabling-workspace-support"></a>
#### Disabling Workspace Support
In case you wish to run the tasks on the workspace root directory and not on the members (for example generating a workspace level README file), use the **`--no-workspace`** CLI flag when running cargo make.<br>
For example:

```sh
cargo make --no-workspace mytask
```

This makes cargo-make ignore that this directory is a workspace root, and just runs a simple flow as if this was a simple directory with a makefile.

Another way to call a task on the workspace level (rather than for each member) is to define that task in the workspace `Makefile.toml` with **workspace** set to false as follows:

```toml
[tasks.ignore-members]
workspace = false
```

Setting **workspace=false** for the task requested on the cargo-make command line is equivalent to calling it with the **--no-workspace** flag.<br>
This flag is only checked for the task on the cargo-make command line and is completely ignored for all other tasks which are executed as part of the flow.<br>
By default, the workspace flag for all tasks is set to true, but that can be configured differently in the config section as follows:

```toml
[config]
default_to_workspace = false
```

In which case, workspace level support is **always** disabled unless a task defines **workspace=true**.

<a name="usage-workspace-composite-flow"></a>
#### Composite Flow

You can define a composite flow that runs tasks on both the workspace root directory and member directories.<br>
This is an example of a workspace level `Makefile.toml` which enables to run such a flow:

```toml
[tasks.composite]
dependencies = ["member_flow", "workspace_flow"]

[tasks.member_flow]
# by forking, cargo make starts and by default detects it is a workspace and runs the member_task for each member
run_task = { name = "member_task", fork = true }

[tasks.workspace_flow]
#run some workspace level command or flow
```

You can start this composite flow as follows:

```sh
cargo make --no-workspace composite
```

<a name="usage-workspace-profiles"></a>
#### Profiles

You can prevent profiles from being passed down to workspace members by setting **`CARGO_MAKE_USE_WORKSPACE_PROFILE`** to false:

```toml
[env]
CARGO_MAKE_USE_WORKSPACE_PROFILE = false
```

See more on profiles in the [profile section](#usage-profiles).

<a name="usage-workspace-support-skip-include-members"></a>
#### Skipping/Including Specific Members

In most cases you will want to run a specific flow on all members, but in rare cases you will want to skip specific members.

By setting the **`CARGO_MAKE_WORKSPACE_SKIP_MEMBERS`** environment variable to hold the member names to skip (as an array), you can define if you want those members not to participate in the flow.

In the below example we will skip member3 and member4 (should be defined in the workspace level `Makefile.toml`):

```toml
[env]
CARGO_MAKE_WORKSPACE_SKIP_MEMBERS = ["member3", "member4"]
```

You can also define glob paths, for example:

```toml
[env]
CARGO_MAKE_WORKSPACE_SKIP_MEMBERS = "tools/*"
```

However there are some cases you will want to skip specific members only if a specific condition is met.<br>
For example, you want to build a member module only if we are running on a rust nightly compiler.<br>
This is a simple example of a conditioned skip for member3 and member4 (should be defined in the workspace level `Makefile.toml`):

```toml
[tasks.workspace-task]
condition = { channels = ["beta", "stable"] }
env = { "CARGO_MAKE_WORKSPACE_SKIP_MEMBERS" = ["member3", "member4"] }
run_task = { name = "member-task", fork = true }
```

You will have to invoke this as a composite flow:

```sh
cargo make workspace-task --no-workspace
```

In addition you can also state the opposite, meaning which members to include via **`CARGO_MAKE_WORKSPACE_INCLUDE_MEMBERS`** environment variable.<br>
It follows the same rules as the **`CARGO_MAKE_WORKSPACE_SKIP_MEMBERS`** environment variable.<br>
If you define both, the included members will be a subset of the non excluded members, meaning both filters will apply.

<a name="usage-workspace-emulation"></a>
#### Workspace Emulation
Workspace emulation enables you to create a workspace like structure for your project without actually defining a rust workspace.<br>
This means you can have a project directory without a `Cargo.toml` and have many child crates.<br>
This enables to run cargo make on all **member** crates while on the root project folder without having the need of an actual cargo workspace which has some side effects (such as shared target folder and dependencies).

In order to setup the workspace emulation, you will need to define the following in your workspace level `Makefile.toml`:

```toml
[env]
# this tells cargo-make that this directory acts as a workspace root
CARGO_MAKE_WORKSPACE_EMULATION = true

# a list of crate members. since we do not have a Cargo.toml, we will need to specify this in here.
CARGO_MAKE_CRATE_WORKSPACE_MEMBERS = [
    "member1",
    "member2"
]
```

<a name="usage-toolchain"></a>
### Toolchain
cargo-make supports setting the toolchain to be used when invoking commands and installing rust dependencies by setting
the **toolchain** attribute as part of the task definition.<br>
The following example shows how to print both stable and nightly rustc versions currently installed:

```toml
[tasks.rustc-version-stable]
toolchain = "stable"
command = "rustc"
args = [ "--version" ]

[tasks.rustc-version-nightly]
toolchain = "nightly"
command = "rustc"
args = [ "--version" ]

[tasks.rustc-version-flow]
dependencies = [
    "rustc-version-stable",
    "rustc-version-nightly"
]
```

An example output of the above **rustc-version-flow** is:

```console
[cargo-make] INFO - Task: rustc-version-flow
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: rustc-version-stable
[cargo-make] INFO - Execute Command: "rustup" "run" "stable" "rustc" "--version"
rustc 1.30.1 (1433507eb 2018-11-07)
[cargo-make] INFO - Running Task: rustc-version-nightly
[cargo-make] INFO - Execute Command: "rustup" "run" "nightly" "rustc" "--version"
rustc 1.32.0-nightly (451987d86 2018-11-01)
[cargo-make] INFO - Running Task: rustc-version-flow
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 2 seconds.
```

When defined with scripts (as opposed to commands), the **`CARGO`** environment variable will be defined for the requested toolchain.<br>
The following example shows how to print both stable and nightly CARGO binary paths:

```toml
[tasks.echo-cargo-stable]
toolchain = "stable"
script = '''
echo ${CARGO}
'''

[tasks.echo-cargo-nightly]
toolchain = "nightly"
script = '''
echo ${CARGO}
'''

[tasks.echo-cargo-all]
dependencies = ["echo-cargo-stable", "echo-cargo-nightly"]
```

An example output of the above **echo-cargo-all** is:

```console
[cargo-make] INFO - Task: echo-cargo-all
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: legacy-migration
[cargo-make] INFO - Running Task: echo-cargo-stable
/home/someuser/.rustup/toolchains/stable-armv7-unknown-linux-gnueabihf/bin/cargo
[cargo-make] INFO - Running Task: echo-cargo-nightly
/home/someuser/.rustup/toolchains/nightly-armv7-unknown-linux-gnueabihf/bin/cargo
[cargo-make] INFO - Build Done in 4.44 seconds.
```

It's also possible to assert a minimum required version of rustc with a channel. This can help
to document required compiler features and to remind developers to upgrade their installation.

```toml
[tasks.requires-stable-edition-2021]
toolchain = { channel = "stable", min_version = "1.56" }
command = "rustc"
args = ["--version"]
```

The task will fail when the toolchain is either not installed or the existing version is smaller
than the specified **min_version**.

<a name="usage-init-end-tasks"></a>
### Init and End tasks
Every task or flow that is executed by the cargo-make has additional 2 tasks.<br>
An init task that gets invoked at the start of all flows and end task that is invoked at the end of all flows.<br>
The names of the init and end tasks are defined in the config section in the toml file, the below shows the default settings:

```toml
[config]
init_task = "init"
end_task = "end"

[tasks.init]

[tasks.end]
```

By default the init and end tasks are empty and can be modified by external toml files or you can simply change the names of the init and end tasks in the external toml files to point to different tasks.<br>
These tasks allow common actions to be invoked no matter what flow you are running.

Important to mention that init and end tasks invocation is different than other tasks.

* Aliases and dependencies are ignored
* If the same task is defined in the executed flow, those tasks will be invoked multiple times

Therefore it is not recommended to use the init/end tasks also inside your flows.

<a name="usage-catching-errors"></a>
### Catching Errors
By default any error in any task that does not have **ignore_errors=true** set to it, will cause the entire flow to fail.<br>
However, there are scenarios in which you would like to run some sort of cleanups before the failed flow finishes.<br>
cargo make enables you to define an **on error** task which will only be invoked in case the flow failed.<br>
In order to define this special task you must add the **on_error_task** attribute in the **config** section in your Makefile and point it to your task, for example:

```toml
[config]
on_error_task = "catch"

[tasks.catch]
script = '''
echo "Doing cleanups in catch"
'''
```

<a name="usage-cargo-alias-tasks"></a>
### Cargo Alias Tasks

[Cargo alias commands](https://doc.rust-lang.org/cargo/reference/config.html#alias) can be automatically loaded as cargo-make tasks.<br>
To automatically loading them, the following must be defined in the `Makefile.toml` config section:

```toml
[config]
load_cargo_aliases = true
```

Each alias defined in the config.toml will be loaded as a task with the same name as the alias.<Br>
In case a task with that name already exists, it will be ignored.<br>
The task definition will simply call cargo and the alias value, therefore no automatic cargo plugin installation will be invoked.

<a name="usage-profiles"></a>
### Profiles

Profiles are a useful tool used to define custom behaviour.<br>
In order to set the execution profile, use the **`--profile`** or **`-p`** CLI argument and provide the profile name.<br>
Profile names are automatically converted to underscores and are trimmed.<br>
If no profile name is provided, the profile will be defaulted to **development**.

Example Setting Profile:

```sh
cargo make --profile production mytask
```

Profiles provide multiple capabilities:

* [Environment variables](#usage-profiles-env) overrides
* [Conditions by profiles](#usage-profiles-conditions), for example:
```toml
condition = { profiles = ["development", "production"] }
```
* [New environment variable](#usage-env-global) **`CARGO_MAKE_PROFILE`** which holds the profile name and can be used by conditions, scripts and commands.

It is possible to activate multiple profiles simultaneously using **additional_profiles**, but these have limited support.<br>
The default profile (if not provided via command line) is `"development"`.<br>
However, this can be overridden by setting the `CARGO_MAKE_DEFAULT_PROFILE` environment variable.

```toml
[config]
additional_profiles = ["second_profile", "another_profile"]
```

Additional profiles can be used to define additional environment blocks and they will be defined in a new environment variable **`CARGO_MAKE_ADDITIONAL_PROFILES`**

<a name="usage-profiles-env"></a>
#### Environment Variables

Profiles enable you to define a new subset of environment variables that will only be set in runtime if the current profile matches the env profile.

```toml
[env]
RUST_BACKTRACE = "1"
EVALUATED_VAR = { script = ["echo SOME VALUE"] }
TEST1 = "value1"
TEST2 = "value2"
COMPOSITE = "${TEST1} ${TEST2}"

# profile based environment override
[env.development]
DEV = true

[env.production]
PROD = true
```

For example, given the following makefile with 2 profile-based env maps:

```toml
[env]
COMMON = "COMMON"
PROFILE_NAME = "${CARGO_MAKE_PROFILE}"

[env.development]
IS_DEV = true
IS_PROD = false

[env.production]
IS_DEV = false
IS_PROD = true

[tasks.echo]
script = [
'''
echo COMMON: ${COMMON}
echo PROFILE_NAME: ${PROFILE_NAME}
echo IS_DEV: ${IS_DEV}
echo IS_PROD: ${IS_PROD}
'''
]
```

We run the **echo** task with **production** profile as follows:

```sh
cargo make --cwd ./examples --makefile profile.toml --profile production echo
```

Output:

```console
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: profile.toml
[cargo-make] INFO - Task: echo
[cargo-make] INFO - Profile: production
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: echo
+ cd /media/devhdd/projects/rust/cargo-make/examples
+ echo COMMON: COMMON
COMMON: COMMON
+ echo PROFILE_NAME: production
PROFILE_NAME: production
+ echo IS_DEV: FALSE
IS_DEV: FALSE
+ echo IS_PROD: TRUE
IS_PROD: TRUE
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

Env files also can be filtered based on profile using the **profile** attribute as follows:

```toml
env_files = [
    { path = "./development.env", profile = "development" },
    { path = "./production.env", profile = "production" },
    { path = "./env.env" }
]
```

Additional profiles defined in the config section will also result in additional env blocks/files to be loaded, for example:

```toml
env_files = [
    { path = "./second.env", profile = "second_profile" },
    { path = "./another.env", profile = "another_profile" }
]

[config]
additional_profiles = ["second_profile", "another_profile"]

[env.second_profile]
IS_SECOND_AVAILABLE = true

[env.another_profile]
IS_OTHER_AVAILABLE = true
```

This could be quite handy in having environment variable blocks which will enable/disable specific tasks.

<a name="usage-profiles-conditions"></a>
#### Conditions

[Conditions](#usage-conditions) enable you to trigger/skip tasks.<br>
Conditions have built in support for profiles, so you can trigger/skip tasks based on the profile name.

Example:

```toml
[tasks.echo-development]
condition = { profiles = [ "development" ] }
command = "echo"
args = [ "running in development profile" ]

[tasks.echo-production]
condition = { profiles = [ "production" ] }
command = "echo"
args = [ "running in production profile" ]
```

<a name="usage-profiles-built-in"></a>
#### Built In Profiles

cargo-make comes with few built in profiles to quickly enable additional conditional tasks.

* **ci-coverage-tasks** - Will enable all code coverage tasks and setup rust compilation to remove dead code.
* **none-thread-safe-tests** - Sets up rust test runner to a single thread
* **multi-phase-tests** - Enable to split the tests to multiple phases (thread safe, multi threaded, custom)
* **ci-static-code-analysis-tasks** - Will enable all static code analysis tasks such as format checking and clippy as part of the CI flow (see special note about backward compatibility below).
* **ci-all-build-tasks** - Will enable all extra compilation tasks (i.e. bench and example code) as part of the CI flow (see special note about backward compatibility below).
* **all-default-tasks** - Will enable extra tasks invoked while running the default task (such as toml formatting).

*Some of these profiles may change in the future to enable more tasks which may break your build and by definition will never be backward compatible.*<br>
*Use them with care.*

<a name="usage-private-tasks"></a>
### Private Tasks

Private tasks are tasks that should only be invoked by other tasks and not directly from the CLI.

In order to define a task as private, add the **private** attribute with value true as follows:

```toml
[tasks.internal-task]
private = true
```

<a name="usage-deprecated-tasks"></a>
### Deprecated Tasks

It is possible to mark tasks as deprecated in order to warn users that they should no longer use this task and switch to a newer/different task instead.<br>
Once invoked, a warning message will be displayed with the deprecation information.<br>
You can define a task deprecated by setting the **deprecated** to true or by providing a relevant message.<br>
For example:

```toml
[tasks.legacy]
deprecated = "Please use task OTHER instead"

[tasks.legacy-extended]
extend = "legacy"
deprecated = false

[tasks.legacy2]
deprecated = true
```

When invoking **legacy** task for example, the output is:

```console
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: deprecated.toml
[cargo-make] INFO - Task: legacy
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Running Task: legacy
[cargo-make] WARN - Task: legacy is deprecated - Please use task OTHER instead
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Build Done in 0 seconds.
```

When listing tasks, deprecated tasks will contain this information as well:

```console
No Category
----------
default - Empty Task
empty - Empty Task
legacy - No Description. (deprecated - Please use task OTHER instead)
legacy-extended - No Description.
legacy2 - No Description. (deprecated)
```

<a name="usage-watch"></a>
### Watch
Watching for changes in your project and firing a task via cargo-make is very easy.<br>
Simply add the **watch** attribute for the task and set it to true and once the task is triggered, it will run every time a file changes in the project.<br>
The process needs to be killed in order to stop the watch.

Example:

```toml
[tasks.watch-example]
command = "echo"
args = [ "Triggered by watch" ]
watch = true
```

Below is a sample output of invoking the task:

```console
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: ./examples/watch.toml
[cargo-make] INFO - Task: watch-example
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: watch-example
[cargo-make] INFO - Running Task: watch-example-watch
[cargo-make] INFO - Execute Command: "cargo" "watch" "-q" "-x" "make --disable-check-for-updates --no-on-error --loglevel=info --makefile=/projects/rust/cargo-make/examples/watch.toml watch-example"
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: /projects/rust/cargo-make/examples/watch.toml
[cargo-make] INFO - Task: watch-example
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: watch-example
[cargo-make] INFO - Execute Command: "echo" "Triggered by watch"
Triggered by watch
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
^C
```

You can also fine tune the watch setup (which is based on **cargo-watch**) by providing an object to the **watch** attribute as follows:

```toml
[tasks.watch-args-example]
command = "echo"
args = [ "Triggered by watch" ]
watch = { postpone = true, no_git_ignore = true, ignore_pattern = "examples/files/*", watch = ["./docs/"] }
```

See [the documentation](https://sagiegurari.github.io/cargo-make/api/cli/types/struct.WatchOptions.html) for a description of all the options available.

<a name="usage-watch-running-multiple-blocking-watches"></a>
#### Running Multiple Blocking Watches

In scenarios that you are required to run multiple blocking watches (for example running compilation + http server) you will need to run all such watches as parallel forked sub tasks.<br>
In order to implement that, you will need to use both fork=true and parallel=true attributes.<br>
For example:

```toml
[tasks.multiple-watches]
run_task = { name = ["build", "http-server", "something-else"], fork = true, parallel = true }
```

<a name="usage-functions"></a>
### Functions

cargo-make comes with built in functions which help extend capabilities missing with environment variables.<br>
Functions are not supported everywhere in the makefile and are currently only supported in command arguments array structure.<br>
In order to define a function call, the following format is used **@@FUNCTION_NAME(ARG1,ARG2,ARG3,...)**<br>
For example:

```toml
[tasks.split-example]
command = "echo"
args = ["@@split(ENV_VAR,|)"]
```

Currently Supported Functions:

* [Split](#usage-functions-split)
* [GetAt](#usage-functions-getat)
* [Remove Empty](#usage-functions-remove-empty)
* [Trim](#usage-functions-trim)
* [Decode](#usage-functions-decode)

<a name="usage-functions-split"></a>
#### Split

The split function accepts two arguments:

* environment variable name
* split by character

And returns an array of sub strings.<br>
This enables to split an environment variable to multiple command arguments, for example:

```toml
[env]
MULTIPLE_VALUES="1 2 3 4"

[tasks.split]
command = "echo"
args = ["@@split(MULTIPLE_VALUES, )"]

[tasks.no-split]
command = "echo"
args = ["${MULTIPLE_VALUES}"]
```

```console
> cargo make --cwd ./examples --makefile functions.toml split
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: split
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: split
[cargo-make] INFO - Execute Command: "echo" "1" "2" "3" "4"
1 2 3 4
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.

> cargo make --cwd ./examples --makefile functions.toml no-split
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: no-split
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: no-split
[cargo-make] INFO - Execute Command: "echo" "1 2 3 4"
1 2 3 4
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

The split function also supports optional third *mode* attribute.<br>
If mode: *remove-empty* the output will not include empty values.

<a name="usage-functions-getat"></a>
#### GetAt

The getat function accepts three arguments:

* environment variable name
* split by character
* index of the item to return

And returns an array with a single value based on the given index.<br>
This enables to split an environment variable and extract only the needed param, for example:

```toml
[env]
MULTIPLE_VALUES="1 2 3 4"

[tasks.getat]
command = "echo"
args = ["@@getat(MULTIPLE_VALUES,|,3)"]
```

```console
> cargo make --cwd ./examples --makefile functions.toml getat
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: getat
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: getat
[cargo-make] INFO - Execute Command: "echo" "4"
4
[cargo-make] INFO - Build Done in 0 seconds.
```

<a name="usage-functions-remove-empty"></a>
#### Remove Empty

The remove empty function accepts a single argument:

* environment variable name

It will completely remove that command line argument in case the environment variable is not defined or is empty or it returns the actual environment variable value.

```toml
[tasks.remove-empty]
command = "echo"
args = ["1", "@@remove-empty(DOES_NOT_EXIST)", "2"]
```

```console
> cargo make --cwd ./examples --makefile functions.toml remove-empty
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: remove-empty
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: remove-empty
[cargo-make] INFO - Execute Command: "echo" "1" "2"
1 2
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

<a name="usage-functions-trim"></a>
#### Trim

The trim function accepts the following arguments:

* environment variable name
* optionally a trim type: start/end (if not provided, it will trim both start and end)

It will completely remove that command line argument in case the environment variable is not defined or after it is trimmed, it is empty or it returns the actual environment variable value.

```toml
[env]
TRIM_VALUE="   123    "

[tasks.trim]
command = "echo"
args = ["@@trim(TRIM_VALUE)"]
```

```console
> cargo make --cwd ./examples --makefile functions.toml remove-empty
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: trim
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: trim
[cargo-make] INFO - Execute Command: "echo" "123"
123
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

Below are examples when using the start/end attributes:

```toml
[env]
TRIM_VALUE="   123    "

[tasks.trim-start]
command = "echo"
args = ["@@trim(TRIM_VALUE,start)"]

[tasks.trim-end]
command = "echo"
args = ["@@trim(TRIM_VALUE,end)"]
```

```console
> cargo make --cwd ./examples --makefile functions.toml trim-start
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: trim-start
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: trim-start
[cargo-make] INFO - Execute Command: "echo" "123    "
123
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.

> cargo make --cwd ./examples --makefile functions.toml trim-end
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: trim-end
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: trim-end
[cargo-make] INFO - Execute Command: "echo" "   123"
   123
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

<a name="usage-functions-decode"></a>
#### Decode

The decode function accepts the following arguments:

* environment variable name
* optional a list of mapping values (source/target)
* optional default value

It will completely remove that command line argument in case the output it is empty.

For example:

```toml
[tasks.decode]
command = "echo"
args = ["Env:", "${CARGO_MAKE_PROFILE}", "Decoded:", "@@decode(CARGO_MAKE_PROFILE,development,dev,ci,test)"]
```

We check the `CARGO_MAKE_PROFILE` environment variable value and look for it in the mappings.<br>
If the value is **development** it will be mapped to **dev** while **ci** is mapped to **test**.<br>
In case no mapping is found, the original value is returned.<br>
Sample run for a mapping that was found:

```console
cargo make --cwd ./examples --makefile functions.toml -e DECODE_ENV_VAR=development decode
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: decode
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Running Task: decode
[cargo-make] INFO - Execute Command: "echo" "Env:" "development" "Decoded:" "dev"
Env: development Decoded: dev
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Build Done in 0 seconds.
```

Another sample run for a mapping that was not found:

```console
cargo make --cwd ./examples --makefile functions.toml -e DECODE_ENV_VAR=unmapped decode
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: decode
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Running Task: decode
[cargo-make] INFO - Execute Command: "echo" "Env:" "unmapped" "Decoded:" "unmapped"
Env: unmapped Decoded: unmapped
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Build Done in 0 seconds.
```

Another example:

```toml
[tasks.decode-with-default]
command = "echo"
args = ["Env:", "${DECODE_ENV_VAR}", "Decoded:", "@@decode(DECODE_ENV_VAR,development,dev,ci,test,unknown)"]
```

Same as previous example, but the difference here is that if not mapping is found, the default value (last argument) is returned.<br>
Sample run:

```console
cargo make --cwd ./examples --makefile functions.toml -e DECODE_ENV_VAR=unmapped decode-with-default
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: decode-with-default
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Running Task: decode-with-default
[cargo-make] INFO - Execute Command: "echo" "Env:" "unmapped" "Decoded:" "unknown"
Env: unmapped Decoded: unknown
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Build Done in 0 seconds.
```

Mapped values can hold environment expressions, for example:

```toml
[tasks.decode-with-eval]
command = "echo"
args = ["Env:", "${DECODE_ENV_VAR}", "Decoded:", "@@decode(DECODE_ENV_VAR,test,The current profile is: ${CARGO_MAKE_PROFILE})"]
```

Sample run:

```console
cargo make --cwd ./examples --makefile functions.toml -e DECODE_ENV_VAR=test decode-with-eval
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: functions.toml
[cargo-make] INFO - Task: decode-with-eval
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Running Task: decode-with-eval
[cargo-make] INFO - Execute Command: "echo" "Env:" "test" "Decoded:" "The current profile is: development"
Env: test Decoded: The current profile is: development
[cargo-make] INFO - Running Task: empty
[cargo-make] INFO - Build Done in 0 seconds.
```

<a name="usage-ci"></a>
### Continuous Integration
cargo-make comes with a predefined flow for continuous integration build executed by internal or online services such as travis-ci and appveyor.<br>
It is recommended to install cargo-make with the debug flag for faster installation.

<a name="usage-ci-github-actions"></a>
#### Github Actions
Add the following to your workflow yml file:

```yaml
- name: Install cargo-make
  uses: actions-rs/cargo@v1
  with:
    command: install
    args: --debug cargo-make
- name: Run CI
  uses: actions-rs/cargo@v1
  with:
    command: make
    args: ci-flow
```

This will use the latest cargo-make with all latest features.

You can see full yaml file at: [ci.yml](https://github.com/sagiegurari/ci_info/blob/master/.github/workflows/ci.yml)

If you want to run code coverage and upload it to codecov, also define the following environment variable:

```toml
CARGO_MAKE_RUN_CODECOV=true
```

When working with workspaces, in order to run the ci-flow for each member and package all coverage data, use the following command:

```yaml
- name: Install cargo-make
  uses: actions-rs/cargo@v1
  with:
    command: install
    args: --debug cargo-make
- name: Run CI
  uses: actions-rs/cargo@v1
  with:
    command: make
    args: --no-workspace workspace-ci-flow
```

To speed up cargo-make installation during the build, you can use the [rust-cargo-make](https://github.com/marketplace/actions/rust-cargo-make) github action to download the prebuilt binary.

<a name="usage-ci-travis"></a>
#### Travis
Add the following to `.travis.yml` file:

```yaml
script:
  - cargo install --debug cargo-make
  - cargo make ci-flow
```

This will use the latest cargo-make with all latest features.
<br>
When caching `cargo`:

```yaml
cache: cargo
script:
  - which cargo-make || cargo install cargo-make
  - cargo make ci-flow
```

*NOTE: While using cache, in order to update cargo-make, you will need to manually clear the travis cache*

If you want to run code coverage and upload it to codecov, also define the following environment variable:

```yaml
env:
  global:
    - CARGO_MAKE_RUN_CODECOV="true"
```

*NOTE: If you are using kcov coverage, you can cache the kcov installation by setting the `CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY` environment variable to a location which is cached by travis.*

When working with workspaces, in order to run the ci-flow for each member and package all coverage data, use the following command:

```yaml
script:
  - cargo install --debug cargo-make
  - cargo make --no-workspace workspace-ci-flow
```

<a name="usage-ci-appveyor"></a>
#### AppVeyor
Add the following to the `appveyor.yml` file:

```yaml
build: false

test_script:
  - cargo install --debug cargo-make
  - cargo make ci-flow
```

When working with workspaces, in order to run the ci-flow for each member and package all coverage data, use the following command:

```yaml
build: false

test_script:
  - cargo install --debug cargo-make
  - cargo make --no-workspace workspace-ci-flow
```

<a name="usage-ci-gitlab"></a>
#### GitLab CI
Add the following to your `gitlab-ci.yml` file:

```yaml
test:cargo:
  script:
  - cargo install --debug cargo-make
  - cargo make ci-flow
```

When working with workspaces, in order to run the ci-flow for each member and package all coverage data, use the following command:

```yaml
build: false

test:cargo:
  script:
  - cargo install --debug cargo-make
  - cargo make --no-workspace workspace-ci-flow
```

To upload your coverage information to codecov, you'll need to go to repo settings for your GitLab repo,
[and add a secret variable](https://docs.gitlab.com/ce/ci/variables/README.html#secret-variables) with your codecov token for that repository.

Then you can add the following in your `gitlab-ci.yml` to enable coverage support:

```yaml
variables:
  CARGO_MAKE_RUN_CODECOV: "true"
```

<a name="usage-ci-circleci"></a>
#### CircleCI
Add the following to your `.circleci/config.yml` file:

```yaml
- run:
    name: install cargo-make
    command: cargo install --debug cargo-make
- run:
    name: ci flow
    command: cargo make ci-flow
```

This will use the latest cargo-make with all latest features.
<br>
When caching `cargo`:

```yaml
  - restore_cache:
      key: project-cache
  # ....
  - run:
      name: install cargo-make
      command: which cargo-make || cargo install cargo-make
  - run:
      name: ci flow
      command: cargo make ci-flow
  # ....
  - save_cache:
      key: project-cache
      paths:
        - "~/.cargo"
```

*NOTE: While using cache, in order to update cargo-make, you will need to manually clear the CircleCI cache*

*NOTE: If you are using kcov coverage, you can cache the kcov installation by setting the `CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY` environment variable to a location which is cached by CircleCI.*

When working with workspaces, in order to run the ci-flow for each member and package all coverage data, use the following command:

```yaml
- run:
    name: install cargo-make
    command: cargo install --debug cargo-make
- run:
    name: ci flow
    command: cargo make --no-workspace workspace-ci-flow
```

<a name="usage-ci-azure-pipelines"></a>
#### Azure Pipelines
Add the following to your `azure-pipelines.yml` file:

```yaml
- script: cargo install --debug cargo-make
  displayName: install cargo-make
- script: cargo make ci-flow
  displayName: ci flow
```

When working with workspaces, in order to run the ci-flow for each member and package all coverage data, use the following setup:

```yaml
- script: cargo install --debug cargo-make
  displayName: install cargo-make
- script: cargo make --no-workspace workspace-ci-flow
  displayName: ci flow
```

<a name="usage-ci-drone-io"></a>
#### drone.io
This is a minimal `.drone.yml` example for running the ci-flow task with the docker runner:

```yaml
pipeline:
  ci-flow:
    image: rust:1.38-slim
    commands:
    - cargo install --debug cargo-make
    - cargo make ci-flow
```

<a name="usage-ci-cirrus"></a>
#### Cirrus CI
This is a minimal `.cirrus.yml` example for running the ci-flow task:

```yaml
container:
  image: rust:latest

task:
  name: ci-flow
  install_script: cargo install --debug cargo-make
  flow_script: cargo make ci-flow
```

<a name="usage-predefined-flows"></a>
### Predefined Flows
The [default makefiles](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/descriptor/makefiles/) file comes with many predefined tasks and flows.<br>
The following are some of the main flows that can be used without any need of an external `Makefile.toml` definition.

* **default** - Can be executed without adding the task name, simply run `cargo make`. This task is an alias for dev-test-flow.
* **dev-test-flow** - Also the default flow so it can be invoked without writing any task name (simply run **cargo make**).<br>This task runs formatting, cargo build and cargo test and will most likely be the set of tasks that you will run while developing and testing a rust project.
* **watch-flow** - Watches for any file change and if any change is detected, it will invoke the test flow.
* **ci-flow** - Should be used in CI builds (such as travis/appveyor) and it runs build and test with verbose level.
* **workspace-ci-flow** - Should be used in CI builds (such as travis/appveyor) for workspace projects.
* **publish-flow** - Cleans old target directory and publishes the project.
* **build-flow** - Runs full cycle of build, tests, security checks, dependencies up to date validations, and documentation generation.<br>This flow can be used to make sure your project is fully tested and up to date.
* **coverage-flow** - Creates coverage report from all unit and integration tests (not supported on windows). By default cargo-make uses kcov for code coverage; however, additional unsupported implementations are defined.
* **codecov-flow** - Runs the coverage-flow and uploads the coverage results to codecov (not supported on windows).

<a name="usage-predefined-flows-coverage"></a>
#### Coverage
cargo-make has built in support for multiple coverage tasks.<br>
Switching between them without modifying the flows is done by setting the coverage provider name in the **`CARGO_MAKE_COVERAGE_PROVIDER`** environment variable as follows:

```toml
[env]
# can be defined as kcov, tarpaulin, ...
CARGO_MAKE_COVERAGE_PROVIDER = "kcov"
```

In case you have a custom coverage task, it can be plugged into the coverage flow by changing the main coverage task alias, for example:

```toml
[tasks.coverage]
alias = "coverage-some-custom-provider"
```

You can run:

```sh
cargo make --list-all-steps | grep "coverage-"
```

To view all currently supported providers. Example output:

```console
ci-coverage-flow: No Description.
coverage-tarpaulin: Runs coverage using tarpaulin rust crate (linux only)
coverage-flow: Runs the full coverage flow.
coverage-kcov: Installs (if missing) and runs coverage using kcov (not supported on windows)
```

All built in coverage providers are supported by their authors and not by cargo-make.

Based on the above explanation, to generate a coverage report for a simple project, run the following command:

```sh
cargo make coverage
```

In order to run coverage in a workspace project and package all member coverage reports in the workspace level, run the following command:

```sh
cargo make --no-workspace workspace-coverage
```

If you are using **kcov**, you may declare the following environment variables in your Makefile.toml to customize the coverage task:

Specify lines or regions of code to ignore:

```toml
[env]
CARGO_MAKE_KCOV_EXCLUDE_LINE = "unreachable,kcov-ignore"             # your choice of pattern(s)
CARGO_MAKE_KCOV_EXCLUDE_REGION = "kcov-ignore-start:kcov-ignore-end" # your choice of markers
```

By default, the binaries executed to collect coverage are filtered by a regular expression. You may override the following in case it does not match the binaries generated on your system:

```toml
[env]
# for example: cargo make filter regex would be cargo_make-[a-z0-9]*$
CARGO_MAKE_TEST_COVERAGE_BINARY_FILTER = "${CARGO_MAKE_CRATE_FS_NAME}-[a-z0-9]*$"
```

*For grcov support, checkout the following repo for instructions:*<br>
*https://github.com/kazuk/cargo-make-coverage-grcov*

<a name="usage-predefined-flows-full"></a>
#### Full List

See [full list of all predefined tasks](https://github.com/sagiegurari/cargo-make/blob/master/docs/cargo_make_task_list.md) (generated via **`cargo make --list-all-steps`**)

<a name="usage-predefined-flows-disable"></a>
#### Disabling Predefined Tasks/Flows

In order to prevent loading of internal core tasks and flows, simply add the following configuration property in your external Makefile.toml:

```toml
[config]
skip_core_tasks = true
```

*Few empty tasks would be loaded even with skipping core tasks to ensure cargo-make has a default task defined.*

<a name="usage-predefined-flows-modify"></a>
#### Modifying Predefined Tasks/Flows

It is possible to modify the internal core tasks.<br>
All modifications are defines in the **config.modify_core_tasks** section.

```toml
[config.modify_core_tasks]
# if true, all core tasks are set to private (default false)
private = true

# if set to some value, all core tasks are modified to: <namespace>::<name> for example default::build
namespace = "default"
```

<a name="usage-min-version"></a>
### Minimal Version
In case you are using cargo-make features that are only available from a specific version, you can ensure the build will fail if it is invoked by an older cargo-make version.<br>
In order to specify the minimal version, use the **min_version** in the config section as follows:

```toml
[config]
min_version = "0.37.15"
```

<a name="usage-performance-tuning"></a>
### Performance Tuning
Some features of cargo-make can be disabled which can improve the startup time.<br>
Below is a list of all current features:

```toml
[config]
# Skip loading of all core tasks which saves up a bit on toml parsing and task creation
skip_core_tasks = true
# Skips loading Git related environment variables
skip_git_env_info = true
# Skips loading rust related environment variables
skip_rust_env_info = true
# Skips loading the current crate related environment variables
skip_crate_env_info = true
```

When running in a rust workspace, you can disable some of the features in the member makefiles.<br>
For example, if the members are in the same git repo as the entire project, you can add **skip_git_env_info** in the members
makefiles and they will still have the environment variables setup from the parent process.

For tasks that can be skipped in case no input file has been modified, see the [Running Tasks Only If Sources Changed](#usage-running-tasks-only-if-sources-changed) section.

<a name="usage-command-groups"></a>
### Command Groups (Subcommands)

You can expose the tasks as *groups* by creating a top level task which will call other internal tasks.

There are two possible approaches for this.

#### Local tasks

Using private tasks (private is not mandatory) are more suited for simple cases and redirect one command to another.

This approach has some limitations:
- `--list-all-steps` will not list private tasks
- the tasks required a specific naming pattern

For example, if you want to have server start/stop and client start/stop commands and execute them as follows:

```sh
cargo make server start
cargo make server stop
cargo make client start
cargo make client stop
```

You can define two top level tasks (server and client) that will invoke the internal ones.<br>
Example implementation:

```toml
[tasks.server]
private = false
extend = "subcommand"
env = { "SUBCOMMAND_PREFIX" = "server" }

[tasks.client]
private = false
extend = "subcommand"
env = { "SUBCOMMAND_PREFIX" = "client" }

[tasks.subcommand]
private = true
script = '''
#!@duckscript

cm_run_task ${SUBCOMMAND_PREFIX}_${1}
'''

[tasks.server_start]
private = true
command = "echo"
args = ["starting server..."]

[tasks.server_stop]
private = true
command = "echo"
args = ["stopping server..."]

[tasks.client_start]
private = true
command = "echo"
args = ["starting client..."]

[tasks.client_stop]
private = true
command = "echo"
args = ["stopping client..."]
```

#### External subcommand file

Another approach is to use a different configuration file for the subcommands.

This allows using `--list-all-steps` for the subcommand and also set `[config]` options in the subcommand file.

For projects with subprojects in folders, a `Makefile.toml` can be created for each subproject and be reached as subcommands from the main folder.

Example implementation of a `foo` sobcommand with a configuration file in the `foo/` folder.

```toml
[tasks.foo]
description = "Foo subcommands"
category = "Subcommands"
cwd = "foo/"
command = "makers"
args = ["${@}"]
```

Example implementation of a `foo` sobcommand with a `Makefile.foo.toml` configuration file in the same folder.

```toml
[tasks.foo]
description = "Foo subcommands"
category = "Subcommands"
command = "makers"
args = ["--makefile", "Makefile.foo.toml", "${@}"]
```

<a name="usage-diff-changes"></a>
### Diff Changes
Using the **`--diff-steps`** CLI command flag, you can diff your correct overrides compared to the prebuilt internal makefile flow.

Example Usage:

```console
cargo make --diff-steps --makefile ./examples/override_core.toml post-build
[cargo-make] INFO - cargo make 0.37.15
[cargo-make] INFO - Build File: ./examples/override_core.toml
[cargo-make] INFO - Task: post-build
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Printing diff...
[cargo-make] INFO - Execute Command: "git" "diff" "--no-index" "/tmp/cargo-make/Lz7lFgjj0x.toml" "/tmp/cargo-make/uBpOa9THwD.toml"
diff --git a/tmp/cargo-make/Lz7lFgjj0x.toml b/tmp/cargo-make/uBpOa9THwD.toml
index 5152290..ba0ef1d 100644
--- a/tmp/cargo-make/Lz7lFgjj0x.toml
+++ b/tmp/cargo-make/uBpOa9THwD.toml
@@ -42,7 +42,9 @@
         name: "post-build",
         config: Task {
             clear: None,
-            description: None,
+            description: Some(
+                "Override description"
+            ),
             category: Some(
                 "Build"
             ),
[cargo-make] INFO - Done
```

*Git is required to be available as it is used to diff the structures and output it to the console using standard git coloring scheme.*

<a name="usage-unstable-features"></a>
### Unstable Features
Some cargo-make capabilities, while working well, are not yet set as default behaviour.<br>
Therefore they are gated using the **unstable_features** attribute.<br>
In order to enable such a feature, you need to define its name.<br>
For example, for the xxx feature would be defined as follows:<br>

```toml
[config]
unstable_features = ["CTRL_C_HANDLING"]
```

Below is a list of currently existing unstable features

* **CTRL_C_HANDLING** - Adds <kbd>ctrl-c</kbd> handler, which will stop any currently running command invoked by the current task and exit cargo-make

<a name="usage-cli"></a>
### CLI Options
These are the following options available while running cargo-make:

```console
USAGE:
    [makers | cargo make | cargo-make make] [OPTIONS] [--] [<TASK_CMD>...]

ARGS:
    <TASK_CMD>    The task to execute, potentially including arguments which can be accessed in the task itself.

OPTIONS:
    --help, -h                           Print help information
    --version, -V                        Print version information
    --makefile <FILE>                    The optional toml file containing the tasks definitions
    --task, -t <TASK>                    The task name to execute (can omit the flag if the task name is the last argument) [default: default]
    --profile, -p <PROFILE>              The profile name (will be converted to lower case) [default: development]
    --cwd <DIRECTORY>                    Will set the current working directory. The search for the makefile will be from this directory if defined.
    --no-workspace                       Disable workspace support (tasks are triggered on workspace and not on members)
    --no-on-error                        Disable on error flow even if defined in config sections
    --allow-private                      Allow invocation of private tasks
    --skip-init-end-tasks                If set, init and end tasks are skipped
    --skip-tasks <SKIP_TASK_PATTERNS>    Skip all tasks that match the provided regex (example: pre.*|post.*)
    --env-file <FILE>                    Set environment variables from provided file
    --env, -e <ENV>                      Set environment variables
    --loglevel, -l <LOG LEVEL>           The log level (verbose, info, error, off) [default: info]
    --verbose, -v                        Sets the log level to verbose (shorthand for --loglevel verbose)
    --quiet                              Sets the log level to error (shorthand for --loglevel error)
    --silent                             Sets the log level to off (shorthand for --loglevel off)
    --no-color                           Disables colorful output
    --time-summary                       Print task level time summary at end of flow
    --experimental                       Allows access unsupported experimental predefined tasks.
    --disable-check-for-updates          Disables the update check during startup
    --output-format <OUTPUT FORMAT>      The print/list steps format (some operations do not support all formats) (default, short-description, markdown, markdown-single-page, markdown-sub-section, autocomplete)
    --output-file <OUTPUT_FILE>          The list steps output file name
    --hide-uninteresting                 Hide any minor tasks such as pre/post hooks.
    --print-steps                        Only prints the steps of the build in the order they will be invoked but without invoking them
    --list-all-steps                     Lists all known steps
    --list-category-steps <CATEGORY>     List steps for a given category
    --diff-steps                         Runs diff between custom flow and prebuilt flow (requires git)
```

<a name="usage-plugins"></a>
### Plugins

Plugins enable users to take full control of the task execution.<br>
cargo-make would still create the execution plan based on the tasks and their dependencies, but would leave the individual task execution to the plugin code.<br>
<br>
Plugins are basically a single duckscript code block with access to the task and flow meta data and can invoke cargo-make specific commands or general duckscript commands.<br>
For example, if a task defined a command and arguments to execute and the plugin simply needs to invoke them, you can implement a simple plugin as follows:

```sh
args_string = array_join ${task.args} " " # simple example which doesn't support args that contain spaces in them
exec --fail-on-error ${task.command} %{args_string}
```

Once a plugin is defined for a task, the task execution control moves to the plugin itself.<br>
All scripts, commands, conditions, env, etc... are ignored and should be handled by the plugin code itself.<br>
All specific task environment variables will not be defined globally (all CARGO_MAKE_CURRENT_TASK_ variables) and instead available in the task env block in the json string.

<a name="usage-plugins-defining-plugins"></a>
### Defining Plugins

Plugins are defined under the `plugin.impl` prefix, for example:

```toml
[plugins.impl.command-runner]
script = '''
echo task: ${task.name}

args_string = array_join ${task.args} " " # simple example which doesn't support args that contain spaces in them
exec --fail-on-error ${task.command} %{args_string}
'''
```

You can defining as many plugins as needed.<br>
It is also possible to provide them aliases to map new names to existing plugins.<br>
For example:

```toml
[plugins.aliases]
original = "new"
this = "that"
```

For a task to pass the execution control to the plugin, simply put the plugin name in the **plugin** attribute.<br>
For example:

```toml
[tasks.my-task]
plugin = "my-plugin"
# other attributes as needed...
```

**You can create reusable plugins and load them using the [load scripts](#usage-load-scripts) built in capability.**

<a name="usage-plugins-plugin-sdk"></a>
### Plugin SDK

The plugin SDK contains the following:

* [Common Duckscript SDK](https://github.com/sagiegurari/duckscript/blob/master/docs/sdk.md)
* Metadata variables
    * flow.task.name - Holds the flow task (not current task) which triggered this task
    * flow.cli.args - Array holding all the task arguments provided to cargo-make on the command line
    * plugin.impl.name - The current plugin name (after aliases modifications)
    * task.as_json - The entire task config as json string (can use json_parse to convert to duckscript variables).
    * task.has_condition - true if the task has any condition definition (including empty one)
    * task.has_env  - true if the task has any env definition (including empty one)
    * task.has_install_instructions - true of the task has installation definition.
    * task.has_command - true if the task has a command definition
    * task.has_script - true if the task has a script definition
    * task.has_run_task - true if the task has a run_task definition
    * task.has_dependencies - true if the task has dependencies
    * task.has_toolchain_specifier - true if the task has toolchain definition
    * task.name - The task name
    * task.description - The description
    * task.category - The category
    * task.disabled - true/false based on the disabled attributes
    * task.private - true/false based on the private attributes
    * task.deprecated - true/false based on the deprecated attributes
    * task.workspace - true/false based on the workspace attributes
    * task.plugin.name - The plugin name defined in the task (before aliases)
    * task.watch - true/false based on the watch attributes
    * task.ignore_errors - true/false based on the ignore_errors attributes
    * task.cwd - The task's current working directory value
    * task.command - The command
    * task.args - Array of all the command arguments
    * task.script_runner - The script runner value
    * task.script_runner_args = Array of all the script runner arguments
    * task.script_extension - The script file extension value
* cargo-make task script specific commands
    * ```cm_run_task [--async] takename``` - Runs a task and dependencies. Supports async execution (via --async flag). Must get the task name to invoke.
* cargo-make plugin specific commands
    * ```cm_plugin_run_task``` - Runs the current task that invoked the plugin (not including dependencies), including condition handling, env, cwd and all the logic that cargo-make has.
    * ```cm_plugin_run_custom_task``` - Accepts a task json string and runs the task definition (not including dependencies), including condition handling, env, cwd and all the logic that cargo-make has.
    * ```cm_plugin_check_task_condition``` - Returns true/false if the current task conditions are met
    * ```cm_plugin_force_plugin_set``` - All tasks that are going to be invoked in the future will call the current plugin regardless of their config
    * ```cm_plugin_force_plugin_clear``` - Undos the cm_plugin_force_plugin_set change and tasks will behave as before

<a name="usage-plugins-plugin-example-dockerize"></a>
### Plugin Example - Docker Integration

Below is a simple example which runs a task (and the rest of the flow from that point) in a docker container.

```toml
[plugins.impl.dockerize]
script = '''
plugin_force_set = get_env PLUGIN_FORCE_SET
plugin_force_set = eq "${plugin_force_set}" 1

if not ${plugin_force_set}
    cm_plugin_force_plugin_set
    set_env PLUGIN_FORCE_SET 1

    dockerfile = set ""
    fn add_docker
        dockerfile = set "${dockerfile}${1}\n"
    end

    taskjson = json_parse ${task.as_json}
    makefile = basename ${taskjson.env.CARGO_MAKE_CURRENT_TASK_INITIAL_MAKEFILE}

    add_docker "FROM debian:stable"
    add_docker "RUN mkdir /workdir/"
    add_docker "RUN mkdir /workdir/project/"
    add_docker "RUN apt-get update"
    add_docker "RUN apt-get install -y curl build-essential libssl-dev pkg-config"
    add_docker "ENV PATH=\"$PATH:$HOME/.cargo/bin\""
    add_docker "RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"
    add_docker "RUN $HOME/.cargo/bin/cargo install cargo-make"
    add_docker "RUN $HOME/.cargo/bin/cargo make --version"
    add_docker "RUN echo \"cd ./workdir/project/ && ls -lsa && $HOME/.cargo/bin/cargo make --makefile ${makefile} --profile ${CARGO_MAKE_PROFILE} ${CARGO_MAKE_TASK}\" > ./run.sh"
    add_docker "RUN chmod 777 ./run.sh"
    add_docker "ADD . /workdir/project/"
    add_docker "CMD [\"sh\", \"./run.sh\"]"

    writefile ./Dockerfile ${dockerfile}
    exec --fail-on-error docker build --tag cmimg:build ./

    exec --fail-on-error docker run cmimg:build
end
'''

[tasks.default]
alias = "docker_flow"

[tasks.docker_flow]
dependencies = ["part1", "part2", "part3"]

[tasks.base-task]
command = "echo"
args = ["task", "${CARGO_MAKE_CURRENT_TASK_NAME}"]

[tasks.part1]
plugin = "dockerize"
extend = "base-task"

[tasks.part2]
extend = "base-task"

[tasks.part3]
extend = "base-task"
```

Running:

```sh
cargo make docker_flow
```

Will result in creation of a new docker container that will run parts 1-3 inside it.<br>
**The example works. However, it does not support several features like passing CLI args, etc....**

<a name="usage-plugins-plugin-example-parallel-workspace-members"></a>
### Plugin Example - Run workspace members in parallel

The following example shows how to define a task on workspace level makefile to enable to invoke it on each member in parallel.

```toml
[plugins.impl.parallel-members]
script = '''
plugin_used = get_env PLUGIN_USED
plugin_used = eq "${plugin_used}" 1

if not ${plugin_used}
    set_env PLUGIN_USED 1
    members = split ${CARGO_MAKE_CRATE_WORKSPACE_MEMBERS} ,

    workspace_dir = pwd
    for member in ${members}
        cd ./${member}
        spawn cargo make --disable-check-for-updates --allow-private --no-on-error ${flow.task.name} %{args}
        cd ${workspace_dir}
    end

    release ${members}
else
    task_definition = json_parse --collection ${task.as_json}
    map_remove ${task_definition} workspace
    task_json = json_encode --collection ${task_definition}
    cm_plugin_run_custom_task ${task_json}
end
'''

[tasks.sometask]
# to make this task serial and not parallel, remove following 2 lines
plugin = "parallel-members"
workspace = false
```

<a name="usage-plugins-plugin-example-rustenv"></a>
### Plugin Example - Load Env From Rust Script

The following example shows how to enable rust scripts invoked from cargo-make to update the main cargo-make process env.<br>
It assumes the task has a script line and that the script is rust. It will execute it (ignoring any rust script provider config for sake of simplicity) and load each output line as an env key/value pair.

```toml
[plugins.impl.rust-env]
script = '''
# make sure the task has a script
assert ${task.has_script}

taskjson = json_parse ${task.as_json}
script = set ${taskjson.script}
writefile ./target/_tempplugin/main.rs ${script}

out = exec --fail-on-error rust-script ./target/_tempplugin/main.rs

output = trim ${out.stdout}
lines = split ${output} \n
for line in ${lines}
    parts = split ${line} =
    key = array_get ${parts} 0
    value = array_get ${parts} 1
    set_env ${key} ${value}
end
'''

[tasks.default]
alias = "test"

[tasks.test]
dependencies = ["dorust"]
command = "echo"
args = ["${ENV_FROM_RUST1}", "${ENV_FROM_RUST2}"]

[tasks.dorust]
private = true
plugin = "rust-env"
script = '''
fn main() {
    println!("ENV_FROM_RUST1=hello");
    println!("ENV_FROM_RUST2=world");
}
'''
```

<a name="usage-plugins-plugin-example-powershell"></a>
### Plugin Example - Adding Simpler Windows Powershell Support

In the below example, we add the a simple powershell command support.<br>
This plugin will take an existing task, set its command to powershell and prepend the **`-C`** argument.<br>
This example also shows how to create new tasks in runtime and invoke them.

```toml
[plugins.impl.powershell]
script = '''
# Adds simpler powershell integration

# make sure we are on windows
windows = is_windows
assert ${windows}

# make sure the task has args
args_empty = array_is_empty ${task.args}
assert_false ${args_empty}

task_definition = json_parse --collection ${task.as_json}

# prepend powershell args to task args
powershell_args = array -C
all_args = array_concat ${powershell_args} ${task.args}
args = map_get ${task_definition} args
release ${args}
map_put ${task_definition} args ${all_args}

# set powershell command
map_put ${task_definition} command pwsh.exe

powershell_task_json = json_encode --collection ${task_definition}

echo Custom Task:\n${powershell_task_json}
cm_plugin_run_custom_task ${powershell_task_json}
'''

[tasks.default]
alias = "test"

[tasks.test]
plugin = "powershell"
args = ["echo hello from windows powershell"]
```

<a name="usage-shell-completion"></a>
### Shell Completion

cargo-make comes with shell auto completion support, however in order to provide the exact task names that are
available in the current directory, it will run the `--list-all-steps` command which might take a bit to finish.

<a name="usage-shell-completion-bash"></a>
#### Bash
Source the `makers-completion.bash` file (found in `extra/shell` folder) at the start of your shell session.
It will enable auto completion for the **makers** executable.

<a name="usage-shell-completion-zsh"></a>
#### zsh
zsh supports bash auto completion. Therefore, the existing bash autocomplete can be used by running the following script:

```
autoload -U +X compinit && compinit
autoload -U +X bashcompinit && bashcompinit

# make sure to update the path based on your file system location
source ./extra/shell/makers-completion.bash
```

It will enable auto completion for the **makers** executable.

<a name="usage-shell-completion-fig"></a>
#### Fig / Amazon CodeWhisperer for command line

Fig supports cargo-make as of [this PR](https://github.com/withfig/autocomplete/pull/2180), no special configuration is needed, just download the latest version of [Fig](https://fig.io/) or [Amazon CodeWhisperer for command line](https://aws.amazon.com/blogs/devops/introducing-amazon-codewhisperer-for-command-line/).

Double check if `cargo-make` is globally installed by running:

```bash
cargo --list
```

If you can see `make` on the list, Fig should work and load the completion automatically from `./Makefile.toml` or any directory you specify with `--makefile <path>`

<a name="cargo-make-global-config"></a>
### Global Configuration
Some of the default CLI values and cargo-make behaviour can be configured via optional global configuration file `config.toml` located in the cargo-make directory.

The cargo-make directory location can be defined via `CARGO_MAKE_HOME` environment variable value.<br>
If `CARGO_MAKE_HOME` has not been defined, the cargo-make default location is:

| OS      | Location                          |
| ------- | --------------------------------- |
| Linux   | $XDG_CONFIG_HOME or $HOME/.config |
| Windows | RoamingAppData                    |
| Mac     | $HOME/Library/Preferences         |

If for any reason, the above paths are not valid for the given platform, it will default to `$HOME/.cargo-make`.

The following example `config.toml` shows all possible options with their default values:

```toml
# The default log level if not defined by the `--loglevel` CLI argument
log_level = "info"

# The default configuration whether output coloring is disabled
disable_color = false

# The default task name if no task was provided as part of the cargo-make invocation
default_task_name = "default"

# cargo-make checks for updates during invocation.
# This configuration defines the minimum amount of time which must pass before cargo-make invocations will try to check for updates.
# If the minimum amount of time did not pass, cargo-make will not check for updates (same as --disable-check-for-updates)
# Valid values are: always, daily, weekly, monthly
# If any other value is provided, it will be treated as weekly.
update_check_minimum_interval = "weekly"

# If set to true and cwd was not provided in the command line arguments and the current cwd is not the project root (Cargo.toml not present),
# cargo make will attempt to find the project root by searching the parent directories, until a directory with a Cargo.toml is found.
# cargo make will set the cwd to that directory and will use any Makefile.toml found at that location.
search_project_root = false
```

<a name="descriptor-definition"></a>
## Makefile Definition

[Config Section](https://sagiegurari.github.io/cargo-make/api/cli/types/struct.ConfigSection.html)

[Task](https://sagiegurari.github.io/cargo-make/api/cli/types/struct.Task.html)

[Platform Override](https://sagiegurari.github.io/cargo-make/api/cli/types/struct.PlatformOverrideTask.html)

[Condition](https://sagiegurari.github.io/cargo-make/api/cli/types/struct.TaskCondition.html)

More info can be found in the [types](https://sagiegurari.github.io/cargo-make/api/cli/types/index.html) section of the API documentation.

<a name="task-name-conventions"></a>
## Task Naming Conventions
This section explains the logic behind the default task names.<br>
While the default names logic can be used as a convention for any new task defined in some project Makefile.toml, it is not required.

The [default makefiles](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/descriptor/makefiles/) file comes with several types of tasks:

* Single command or script task (for example **cargo build**)
* Tasks that come before or after the single command tasks (hooks)
* Tasks that define flows using dependencies
* Tasks which only install some dependency

Single command tasks are named based on their command (in most cases), for example the task that runs cargo build is named build.

```toml
[tasks.build]
command = "cargo"
args = ["build"]
```

This allows to easily understand what this task does.

Tasks that are invoked before/after those tasks are named the same way as the original task but with the pre/post prefix.<br>
For example for task build the default toml also defines pre-build and post-build tasks.

```toml
[tasks.pre-build]

[tasks.post-build]
```

In the [default makefiles](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/descriptor/makefiles/), all pre/post tasks are empty and are there as placeholders
for external `Makefile.toml` to override so custom functionality can be defined easily before/after running a specific task.

Flows are named with the flow suffix.  For example, **ci-flow**:

```toml
[tasks.ci-flow]
# CI task will run cargo build and cargo test with verbose output
dependencies = [
    "pre-build",
    "build-verbose",
    "post-build",
    "pre-test",
    "test-verbose",
    "post-test"
]
```

This prevents flow task names to conflict with single command task names and quickly allow users to understand that this task is a flow definition.

Tasks which only install some dependency but do not invoke any command start with the **install-** prefix, for example:

```toml
[tasks.install-rust-src]
install_crate = { rustup_component_name = "rust-src" }
```

<a name="error-codes"></a>
## Error Codes

<a name="e001"></a>
### E001: Environment Variables Cycle Detected

A cycle between different environmental variables has been detected;
This can happen during the merging of environments (at every loading step).
Due to reordering and to make sure that no circular references exist,
this error is emitted.

You can fix this issue, by looking at your env config, and seeing if at any point a circular reference could have occurred.
The error message mentions the environment variables that are likely candidates for the cause of the cycle.

Your best bet is to try to break the cycle, by creating a new environmental variable or use a static value multiple times.
Cycles are usually caused by rapidly changing configs, forgotten and unused env variables or design problems,
even without cycle detection or no reordering this would likely cause hidden issues during
execution, as `cargo-make` would need to otherwise set instances to an empty value instead.
This way you are able to investigate and fix it yourself before it becomes an unexpected,
hidden and hard to debug issue.

> **Note:** Scripts are known to sometimes cause false-positives.
> In that case use the `depends_on` property, to explicitly tell `cargo-make`, which
> environmental variables should be considered a dependency instead of trying to guess from the script.


<a name="articles"></a>
## Articles
Below is a list of articles which explain most of the cargo-make features.

* [Introduction and Basics](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-1-of-5-introduction-and-basics-b19ced7e7057)
* [Extending Tasks, Platform Overrides and Aliases](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-2-of-5-extending-tasks-platform-overrides-1527386dcf87)
* [Environment Variables, Conditions, Sub Tasks and Mixing](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-3-of-5-environment-variables-conditions-3c740a837a01)
* [Workspace Support, Init/End Tasks and Makefiles](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-4-of-5-workspace-support-init-end-tasks-c3e738699421)
* [Predefined Tasks, CI Support and Conventions](https://medium.com/@sagiegurari/automating-your-rust-workflows-with-cargo-make-part-5-final-predefined-tasks-ci-support-and-4594812e57da)

The articles are missing some of the new features which have been added after they were published, such as:

* [Rust Task](#usage-task-command-script-task-examplerust)
* [Cross Platform Shell](#usage-task-command-script-task-exampleshell2batch)
* [Full List of Predefined Flows](#usage-predefined-flows)
* [Global Configuration](#cargo-make-global-config)
* [Catching Errors](#usage-catching-errors)
* [Env File](#usage-env-file)
* [Private Tasks](#usage-private-tasks)
* [Other Programming Languages](#usage-task-command-script-task-examplegeneric)
* [Rust Version Conditions](#usage-conditions-structure)
* [Toolchain](#usage-toolchain)
* [Watch](#usage-watch)
* [Profiles](#usage-profiles)
* [Functions](#usage-functions)
* [Minimal Version](#usage-min-version)
* [Deprecated Tasks](#usage-deprecated-tasks)

And more...

<a name="badge"></a>
## Badge
If you are using cargo-make in your project and want to display it in your project README or website, you can embed the "Built with cargo-make" badge.

[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)

Here are few snapshots:

### Markdown


```md
[![Built with cargo-make](https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg)](https://sagiegurari.github.io/cargo-make)
```

### HTML

```html
<a href="https://sagiegurari.github.io/cargo-make">
  <img src="https://sagiegurari.github.io/cargo-make/assets/badges/cargo-make.svg" alt="Built with cargo-make">
</a>
```

<a name="roadmap"></a>
## Roadmap
While already feature rich, cargo-make is still under heavy development.<br>
You can view the future development items list in the [github project issues](https://github.com/sagiegurari/cargo-make/issues)

<a name="editor-support"></a>
## Editor Support

<a name="editor-support-vim"></a>
### Vim

* [vim-cargo-make](https://github.com/nastevens/vim-cargo-make)
* [vim-duckscript](https://github.com/nastevens/vim-duckscript)

<a name="editor-support-vs-code"></a>
### VSCode

For debugging purposes there are some example .vscode files located within the [docs/vscode-example](./docs/vscode-example/) directory

You may also need:

  * A local install of LLVM (For the LLDB Debugger) installed and reachable on the path
  * VSCode Extension - CodeLLDB
  * VSCode Extension - "rust-analyser" (not the "rust" one)
  * VSCode Extension - "Task Explorer"
  * VSCode Extension - "crates"

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](https://github.com/sagiegurari/cargo-make/blob/master/CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
