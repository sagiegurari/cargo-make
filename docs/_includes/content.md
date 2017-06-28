<a name="overview"></a>
# Overview
The cargo-make task runner enables to define and configure sets of tasks and run them as a flow.<br>
A task is a command or a script to execute.<br>
Tasks can have dependencies which are also tasks that will be executed before the task itself.<br>
With a simple toml based configuration file, you can define a multi platform build script that can run build, test, documentation generation, bench tests execution, security validations and more by running a single command.

<a name="installation"></a>
# Installation
In order to install, just run the following command

```sh
cargo install cargo-make
```

This will install cargo-make in your ~/.cargo/bin.<br>
Make sure to add ~/.cargo/bin directory to your PATH variable.

<a name="usage"></a>
# Usage
When using cargo-make, all tasks are defined and configured via toml files.<br>
Below are simple instructions to get your started off quickly.

<a name="usage-simple"></a>
## Simple Example
In order to run a set of tasks, you first must define them in a toml file.<br>
For example, if we would like to have a script which:

* Formats the code
* Cleans old target directory
* Runs build
* Runs tests

We will create a toml file as follows:

````toml
install_crate = "rustfmt"
command = "cargo"
args = ["fmt", "--", "--write-mode=overwrite"]

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
````

We would execute the flow with the following command:

````sh
cargo make -b simple-example.toml -t my-flow
````

The output would look something like this:

````console
[cargo-make] info - Using Build File: simple-example.toml
[cargo-make] info - Task: my-flow
[cargo-make] info - Setting Up Env.
[cargo-make] info - Running Task: format
[cargo-make] info - Execute Command: "cargo" "fmt" "--" "--write-mode=overwrite"
[cargo-make] info - Running Task: clean
[cargo-make] info - Execute Command: "cargo" "clean"
[cargo-make] info - Running Task: build
[cargo-make] info - Execute Command: "cargo" "build"
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
[cargo-make] info - Running Task: test
[cargo-make] info - Execute Command: "cargo" "test"
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

[cargo-make] info - Running Task: my-flow
[cargo-make] info - Build done in 72 seconds.
````

We now created a build script that can run on any platform.

<a name="usage-task-dependencies-alias"></a>
## Tasks, Dependencies and Aliases
In many cases, certain tasks depend on other tasks.<br>
For example you would like to format the code before running build and run the build before running tests.<br>
Such flow can be defined as follows:

````toml
[tasks.format]
install_crate = "rustfmt"
command = "cargo"
args = ["fmt", "--", "--write-mode=overwrite"]

[tasks.build]
command = "cargo"
args = ["build"]
dependencies = ["format"]

[tasks.test]
command = "cargo"
args = ["test"]
dependencies = ["build"]
````

When you run:

````sh
cargo make -b ./my_build.toml -t test
````

It will try to run test, see that it has dependencies and those have other dependencies.<br>
Therefore it will create an execution plan for the tasks based on the tasks and their dependencies.<br>
In our case it will invoke format -> build -> test.<br>

The same task will never be executed twice so if we have for example:

````toml
[tasks.A]
dependencies = ["B", "C"]

[tasks.B]
dependencies = ["D"]

[tasks.C]
dependencies = ["D"]

[tasks.D]
script = [
    "echo hello"
]
````

In this example, A depends on B and C, and both B and C are dependended on D.<br>
Task D however will not be invoked twice.<br>
The output of the execution will look something like this:

````console
[cargo-make] info - Task: A
[cargo-make] info - Setting Up Env.
[cargo-make] info - Running Task: D
[cargo-make] info - Execute Command: "sh" "/tmp/cargo-make/CNuU47tIix.sh"
hello
[cargo-make] info - Running Task: B
[cargo-make] info - Running Task: C
[cargo-make] info - Running Task: A
````

As you can see, 'hello' was printed once by task D as it was only invoked once.<br>
But what if we want to run D twice?<br>
Simple answer would be to duplicate task D and have B depend on D and C depend on D2 which is a copy of D.<br>
But duplicating can lead to bugs and to huge makefiles, so we have alias for that.<br>
An alias task has its own name and points to another task.<br>
All of the definitions of the alias task are ignored.<br>
So now, if we want to have D execute twice we can do the following:

````toml
[tasks.A]
dependencies = ["B", "C"]

[tasks.B]
dependencies = ["D"]

[tasks.C]
dependencies = ["D2"]

[tasks.D]
script = [
    "echo hello"
]

[tasks.D2]
alias="D"
````

Now C depends on D2 and D2 is an alias for D.<br>
Execution output of such make file would like as follows:

````console
[cargo-make] info - Task: A
[cargo-make] info - Setting Up Env.
[cargo-make] info - Running Task: D
[cargo-make] info - Execute Command: "sh" "/tmp/cargo-make/HP0UD7pgoX.sh"
hello
[cargo-make] info - Running Task: B
[cargo-make] info - Running Task: D2
[cargo-make] info - Execute Command: "sh" "/tmp/cargo-make/TuuZJkqCE2.sh"
hello
[cargo-make] info - Running Task: C
[cargo-make] info - Running Task: A
````

Now you can see that 'hello' was printed twice.

It is also possible to define platform specific aliases, for example:

````toml
[tasks.my_task]
linux_alias = "linux_my_task"
windows_alias = "windows_my_task"
mac_alias = "mac_my_task"

[tasks.linux_my_task]

[tasks.mac_my_task]

[tasks.windows_my_task]
````

If platform specific alias is found and matches current platform it will take precedence over the non platform alias definition.<br>
For example:

````toml
[tasks.my_task]
linux_alias = "run"
alias = "do_nothing"

[tasks.run]
script = [
    "echo hello"
]

[tasks.do_nothing]
````

If you run task **my_task** on windows or mac, it will invoke the **do_nothing** task.<br>
However, if executed on a linux platform, it will invoke the **run** task.

<a name="usage-default-tasks"></a>
## Default Tasks and Extending
There is no real need to define the tasks that were shown in the previous example.<br>
cargo-make comes with a built in toml file that will serve as a base for every execution.<br>
The **optional** external toml file that is provided while running cargo-make will only extend and add or overwrite
tasks that are defined in the [default toml](https://github.com/sagiegurari/cargo-make/blob/master/src/default.toml).<br>
Lets take the build task definition which comes alrady in the default toml:

````toml
[tasks.build]
command = "cargo"
args = ["build"]
````

If for example, you would like to add verbose output to it, you would just need to change the args and add the --verbose as follows:

````toml
[tasks.build]
args = ["build", "--verbose"]
````

If you want to disable some existing task (will not disable its dependencies), you can do it as follows:

````toml
[tasks.build]
disabled = true
````

There is no need to redefine existing properties of the task, only what needs to be added or overwritten.<br>
The default toml file comes with many steps and flows already built in, so it is worth to check it first.

<a name="usage-platform-override"></a>
## Platform Override
In case you want to override a task or specific attributes in a task for specific platforms, you can define an override task with the platform name (currently linux, windows and mac) under the specific task.<br>
For example:

````toml
[tasks.hello-world]
script = [
    "echo \"Hello World From Unknown\""
]

[tasks.hello-world.linux]
script = [
    "echo \"Hello World From Linux\""
]
````

If you run cargo make with task 'hello-world' on linux, it would redirect to hello-world.linux while on other platforms it will execute the original hello-world.<br>
In linux the output would be:

````console
[cargo-make] info - Task: hello-world
[cargo-make] info - Setting Up Env.
[cargo-make] info - Running Task: hello-world
[cargo-make] info - Execute Command: "sh" "/tmp/cargo-make/kOUJfw8Vfc.sh"
Hello World From Linux
[cargo-make] info - Build done in 0 seconds.
````

While on other platforms

````console
[cargo-make] info - Task: hello-world
[cargo-make] info - Setting Up Env.
[cargo-make] info - Running Task: hello-world
[cargo-make] info - Execute Command: "sh" "/tmp/cargo-make/2gYnulOJLP.sh"
Hello World From Unknown
[cargo-make] info - Build done in 0 seconds.
````

In the override task you can define any attribute that will override the attribute of the parent task, while undefined attributes will use the value from the parent task and will not be modified.<br>
In case you need to delete attributes from the parent (for example script is only invoked if command is not defined and you have command defined in the parent task and script in the override task), then you will
have to clear the parent task in the override task using the clear attribute as follows:

````toml
[tasks.hello-world.linux]
clear = true
script = [
    "echo \"Hello World From Linux\""
]
````

This means, however, that you will have to redefine all attributes in the override task that you want to carry with you from the parent task.<br>
**Important - alias comes before checking override task so if parent task has an alias it will be redirected to that task instead of the override.**<br>
**To override per platform, use the linux_alias, windows_alias, mac_alias attributes.<br>**
**In addition, aliases can not be defined in platform override tasks, only in parent tasks.**

<a name="usage-env"></a>
## Environment Variables
You can also define env vars to be set as part of the execution of the flow in the env block, for examle:

````yaml
[env]
RUST_BACKTRACE="1"
````

All env vars defined in the env block and in the [default toml](https://github.com/sagiegurari/cargo-make/blob/master/src/default.toml) will be defined before running the tasks.

<a name="usage-ci"></a>
## Continues Integration
cargo-make comes with a predefined flow for continues integration build executed by internal or online services such as travis-ci and appveyor.<br>
For travis-ci, simple change the script to invoke the cargo-make installation and invocation as follows:

````yaml
script:
  - cargo install --debug cargo-make
  - cargo make --task ci-flow
````

For appveyor:

````yaml
build: false

test_script:
  - cargo install --debug cargo-make
  - cargo make --task ci-flow
````

For online CI services, it is better to install with the debug flag to enable a much faster installation.

<a name="usage-predefined-flows"></a>
## Predefined Flows
The [default toml](https://github.com/sagiegurari/cargo-make/blob/master/src/default.toml) file comes with many predefined tasks and flows.<br>
The following are some of the main flows that can be used without any need of an external Makefile.toml definition.

* **dev-test-flow** - Also the default flow so it can be invoked without writing any task name (simple run ````cargo make````).<br>This task runs formatting, cargo build and cargo test and will most likely be the set of tasks that you will run while developing and testing a rust project.
* **ci-flow** - Should be used in CI builds (such as travis/appveyor) and it runs build and test with verbose level.
* **publish-flow** - Cleans old target directory and publishes the project.
* **build-flow** - Runs full cycle of build, tests, security checks, dependencies up to date validations and documentation generation.<br>This flow can be used to make sure your project is fully tested and up to date.

<a name="usage-cli"></a>
## Cli Options
These are the following options available while running cargo-make:

````console
USAGE:
    cargo-make make [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -b, --buildFile <FILE>        Build toml file containing the build descriptor (default: Makefile.toml if exists)
    -l, --loglevel <LOG LEVEL>    The log level (default: info) [values: verbose, info, error]
    -t, --task <TASK NAME>        The task name to execute (default: default)
````

<a name="descriptor-definition"></a>
# Makefile Definition
````rust
/// Holds the entire configuration such as task definitions and env vars
pub struct Config {
    /// The env vars to setup before running the tasks
    pub env: HashMap<String, String>,
    /// All task definitions
    pub tasks: HashMap<String, Task>
}

/// Holds a single task configuration such as command and dependencies list
pub struct Task {
    /// if true, the command/script of this task will not be invoked, dependencies however will be
    pub disabled: Option<bool>,
    /// if defined, task points to another task and all other properties are ignored
    pub alias: Option<String>,
    /// acts like alias if runtime OS is Linux (takes precedence over alias)
    pub linux_alias: Option<String>,
    /// acts like alias if runtime OS is Windows (takes precedence over alias)
    pub windows_alias: Option<String>,
    /// acts like alias if runtime OS is Mac (takes precedence over alias)
    pub mac_alias: Option<String>,
    /// if defined, the provided crate will be installed (if needed) before running the task
    pub install_crate: Option<String>,
    /// if defined, the provided script will be executed before running the task
    pub install_script: Option<Vec<String>>,
    /// The command to execute
    pub command: Option<String>,
    /// The command args
    pub args: Option<Vec<String>>,
    /// If command is not defined, and script is defined, the provided script will be executed
    pub script: Option<Vec<String>>,
    /// A list of tasks to execute before this task
    pub dependencies: Option<Vec<String>>,
    /// override task if runtime OS is Linux (takes precedence over alias)
    pub linux: Option<PlatformOverrideTask>,
    /// override task if runtime OS is Windows (takes precedence over alias)
    pub windows: Option<PlatformOverrideTask>,
    /// override task if runtime OS is Mac (takes precedence over alias)
    pub mac: Option<PlatformOverrideTask>
}

/// Holds a single task configuration for a specific platform as an override of another task
pub struct PlatformOverrideTask {
    /// if true, it should ignore all data in base task
    clear: Option<bool>,
    /// if true, the command/script of this task will not be invoked, dependencies however will be
    disabled: Option<bool>,
    /// if defined, the provided crate will be installed (if needed) before running the task
    install_crate: Option<String>,
    /// if defined, the provided script will be executed before running the task
    install_script: Option<Vec<String>>,
    /// The command to execute
    command: Option<String>,
    /// The command args
    args: Option<Vec<String>>,
    /// If command is not defined, and script is defined, the provided script will be executed
    script: Option<Vec<String>>,
    /// A list of tasks to execute before this task
    dependencies: Option<Vec<String>>
}
````

<a name="task-name-conventions"></a>
# Task Naming conventions
This section explains the logic behind the default task names.<br>
While the default names logic can be used as a convention for any new task defined in some project Makefile.toml, it is not required.

The [default toml](https://github.com/sagiegurari/cargo-make/blob/master/src/default.toml) file comes with three types of tasks:

* Single command or script task (for example ````cargo build````)
* Tasks that come before or after the single command tasks
* Tasks that define flows using dependencies

Single command tasks are named based on their commmand (in most cases), for example the task that runs cargo build is named build.

````toml
[tasks.build]
command = "cargo"
args = ["build"]
````

This allows to easily understand what this task does.

Tasks that are invoked before/after those tasks are named the same way as the original task but with the pre/post prefix.<br>
For example for task build the default toml also defines pre-build and post-build tasks.

````toml
[tasks.pre-build]

[tasks.post-build]
````

In the [default toml](https://github.com/sagiegurari/cargo-make/blob/master/src/default.toml), all pre/post tasks are empty and are there as placeholders
for external Makefile.toml to override so custom functionality can be defined easily before/after running a specfific task.

Flows are named with the flow suffix, for example: ci-flow

````toml
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
````

This prevents flow task names to conflict with single command task names and quickly allow users to understand that this task is a flow definition.

<a name="badge"></a>
# Badge
If you are using cargo-make in your project and want to display it in your project README or website, you can embed the "Built with cargo-make" badge.

[![Built with cargo-make](https://img.shields.io/badge/built%20with-cargo--make-e49d41.svg)](https://sagiegurari.github.io/cargo-make)

Here are few snapshots:

## Markdown


````md
[![Built with cargo-make](https://img.shields.io/badge/built%20with-cargo--make-e49d41.svg)](https://sagiegurari.github.io/cargo-make)
````

## HTML

````html
<a href="https://sagiegurari.github.io/cargo-make">
  <img src="https://img.shields.io/badge/built%20with-cargo--make-e49d41.svg" alt="Built with cargo-make">
</a>
````

<a name="roadmap"></a>
# Roadmap
The cargo-make task runner is still under heavy development.<br>
You can view the future development items list in the [project board](https://github.com/sagiegurari/cargo-make/projects)

# Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="license"></a>
# License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.