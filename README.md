# cargo-make

[![crates.io](https://img.shields.io/crates/v/cargo-make.svg)](https://crates.io/crates/cargo-make) [![Build Status](https://travis-ci.org/sagiegurari/cargo-make.svg)](http://travis-ci.org/sagiegurari/cargo-make) [![Build status](https://ci.appveyor.com/api/projects/status/knyrs33tyjqgt06u?svg=true)](https://ci.appveyor.com/project/sagiegurari/cargo-make) [![codecov](https://codecov.io/gh/sagiegurari/cargo-make/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/cargo-make)<br>
[![license](https://img.shields.io/crates/l/cargo-make.svg)](https://github.com/sagiegurari/cargo-make/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/cargo-make.svg)](https://libraries.io/cargo/cargo-make) [![Documentation](https://docs.rs/cargo-make/badge.svg)](https://docs.rs/crate/cargo-make/) [![downloads](https://img.shields.io/crates/d/cargo-make.svg)](https://crates.io/crates/cargo-make)<br>
[![Built with cargo-make](https://img.shields.io/badge/built%20with-cargo--make-e49d41.svg)](https://sagiegurari.github.io/cargo-make)

> [Rust](https://www.rust-lang.org/) task runner and build tool.

* [Overview](#overview)
* [Installation](#installation)
* [Usage](#usage)
    * [Simple Example](#usage-simple)
    * [Tasks, Dependencies and Aliases](#usage-task-dependencies-alias)
    * [Default Tasks and Extending](#usage-default-tasks)
    * [Continues Integration](#usage-ci)
    * [Environment Variables](#usage-env)
    * [Cli Options](#usage-cli)
    * [Task Definition](#usage-task-def)
* [Badge](#badge)
* [Roadmap](#roadmap)
* [API Documentation](https://sagiegurari.github.io/cargo-make/api.html)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](#history)
* [License](#license)

<a name="overview"></a>
## Overview
The cargo-make task runner enables to define and configure sets of tasks and run them as a flow.<br>
A task is a command or a script to execute.<br>
Tasks can have dependencies which are also tasks that will be executed before the task itself.<br>
With a simple toml based configuration file, you can define a multi platform build script that can run build, test, documentation generation, bench tests execution, security validations and more by running a single command.

<a name="installation"></a>
## Installation
In order to install, just run the following command

```sh
cargo install cargo-make
```

This will install cargo-make in your ~/.cargo/bin.<br>
Make sure to add ~/.cargo/bin directory to your PATH variable.

<a name="usage"></a>
## Usage
When using cargo-make, all tasks are defined and configured via toml files.<br>
Below are simple instructions to get your started off quickly.

<a name="usage-simple"></a>
### Simple Example
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
### Tasks, Dependencies and Aliases
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

<a name="usage-default-tasks"></a>
### Default Tasks and Extending
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

<a name="usage-env"></a>
### Environment Variables
You can also define env vars to be set as part of the execution of the flow in the env block, for examle:

````yaml
[env]
RUST_BACKTRACE="1"
````

All env vars defined in the env block and in the [default toml](https://github.com/sagiegurari/cargo-make/blob/master/src/default.toml) will be defined before running the tasks.

<a name="usage-ci"></a>
### Continues Integration
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

<a name="usage-cli"></a>
### Cli Options
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

### Task Definition
The following is the task definition:

````rs
#[derive(Serialize, Deserialize, Debug, Clone)]
/// Holds a single task configuration such as command and dependencies list
pub struct Task {
    /// if true, the command/script of this task will not be invoked, depedencies however will be
    pub disabled: Option<bool>,
    /// if defined, task points to another task and all other properties are ignored
    pub alias: Option<String>,
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
    pub dependencies: Option<Vec<String>>
}
````

<a name="badge"></a>
## Badge
If you are using cargo-make in your project and want to display it in your project README or website, you can embed the "Built with cargo-make" badge.<br>
Here are few snapshots:

### Markdown


````md
[![Built with cargo-make](https://img.shields.io/badge/built%20with-cargo--make-e49d41.svg)](https://sagiegurari.github.io/cargo-make)
````

### HTML

````html
<a href="https://sagiegurari.github.io/cargo-make">
  <img src="https://img.shields.io/badge/built%20with-cargo--make-e49d41.svg" alt="Built with cargo-make">
</a>
````

<a name="roadmap"></a>
## Roadmap
The cargo-make task runner is still under heavy development and there are many things planned for the coming release.<br>
You can view the current list in the [project board](https://github.com/sagiegurari/cargo-make/projects)

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/cargo-make/api.html)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

| Date        | Version | Description |
| ----------- | ------- | ----------- |
| 2017-06-25  | v0.2.2  | Added disabled task attribute support |
| 2017-06-24  | v0.2.0  | Internal fixes (renamed dependencies attribute) |
| 2017-06-24  | v0.1.2  | Print build time, added internal docs, unit tests and coverage |
| 2017-06-24  | v0.1.1  | Added support for env vars, task alias and crate installation |
| 2017-06-23  | v0.1.0  | Initial release. |

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.