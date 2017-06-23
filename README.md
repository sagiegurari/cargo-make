# cargo-make

[![crates.io](https://img.shields.io/crates/v/cargo-make.svg)](https://crates.io/crates/cargo-make) [![Build Status](https://travis-ci.org/sagiegurari/cargo-make.svg)](http://travis-ci.org/sagiegurari/cargo-make) [![Build status](https://ci.appveyor.com/api/projects/status/knyrs33tyjqgt06u?svg=true)](https://ci.appveyor.com/project/sagiegurari/simple-redis) [![codecov](https://codecov.io/gh/sagiegurari/cargo-make/branch/master/graph/badge.svg)](https://codecov.io/gh/sagiegurari/cargo-make)<br>
[![license](https://img.shields.io/crates/l/cargo-make.svg)](https://github.com/sagiegurari/cargo-make/blob/master/LICENSE) [![Libraries.io for GitHub](https://img.shields.io/librariesio/github/sagiegurari/cargo-make.svg)](https://libraries.io/cargo/cargo-make) [![Documentation](https://docs.rs/cargo-make/badge.svg)](https://docs.rs/crate/cargo-make/) [![downloads](https://img.shields.io/crates/d/cargo-make.svg)](https://crates.io/crates/cargo-make)<br>
[![cargo-make](https://img.shields.io/badge/built%20with-cargo--make-e49d41.svg)](https://github.com/sagiegurari/cargo-make)

> [Rust](https://www.rust-lang.org/) task runner and build tool.

**Currently in initial development**

* [Overview](#overview)
* [Installation](#installation)
* [Usage](#usage)
    * [Simple Example](#usage-simple)
    * [Default Tasks and Extending](#usage-default-tasks)
    * [Continues Integration](#usage-ci)
    * [Cli Options](#usage-cli)
* [Roadmap](#roadmap)
* [API Documentation](https://sagiegurari.github.io/cargo-make/)
* [Contributing](.github/CONTRIBUTING.md)
* [Release History](#history)
* [License](#license)

<a name="overview"></a>
## Overview
The cargo-make task runner enables to define and configure sets of tasks and run them as a flow.<br>
A task is a command or a script to execute.<br>
Tasks can have depedencies which are also tasks that will be executed before the task itself.<br>
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
[tasks.format]
install_script = ["cargo install rustfmt"]
command = "cargo"
args = ["fmt", "--", "--write-mode=overwrite"]

[tasks.clean]
command = "cargo"
args = ["clean"]

[tasks.build]
command = "cargo"
args = ["build"]
depedencies = ["clean"]

[tasks.test]
command = "cargo"
args = ["test"]
depedencies = ["clean"]

[tasks.my-flow]
depedencies = [
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
[cargo-make] info - Using Build File: ./examples/simple-example.toml
[cargo-make] info - Task: my-flow
[cargo-make] info - Running Task: format
[cargo-make] info - Execute Command: "cargo" "fmt" "--" "--write-mode=overwrite"
[cargo-make] info - Running Task: clean
[cargo-make] info - Execute Command: "cargo" "clean"
[cargo-make] info - Running Task: build
[cargo-make] info - Execute Command: "cargo" "build"
   Compiling vec_map v0.8.0
   Compiling serde v1.0.8
   Compiling unicode-width v0.1.4
   Compiling quote v0.3.15
   Compiling unicode-xid v0.0.4
   Compiling libc v0.2.24
   Compiling ansi_term v0.9.0
   Compiling bitflags v0.9.1
   Compiling unicode-segmentation v1.1.0
   Compiling strsim v0.6.0
   Compiling synom v0.11.3
   Compiling syn v0.11.11
   Compiling term_size v0.3.0
   Compiling atty v0.2.2
   Compiling textwrap v0.6.0
   Compiling clap v2.25.0
   Compiling serde_derive_internals v0.15.1
   Compiling toml v0.4.2
   Compiling serde_derive v1.0.8
   Compiling cargo-make v0.1.0 (file:///home/ubuntu/workspace/rust/cargo-make)
    Finished dev [unoptimized + debuginfo] target(s) in 253.16 secs
[cargo-make] info - Running Task: test
[cargo-make] info - Execute Command: "cargo" "test"
   Compiling cargo-make v0.1.0 (file:///home/ubuntu/workspace/rust/cargo-make)
    Finished dev [unoptimized + debuginfo] target(s) in 12.80 secs
     Running target/debug/deps/cargo_make-542f1253498e7764

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

[cargo-make] info - Running Task: my-flow
````

We now created a build script that can run on any platform.

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

There is no need to redefine existing properties of the task, only what needs to be added or overwritten.<br>
The default toml file comes with many steps and flows already built in, so it is worth to check it first.

<a name="usage-ci"></a>
### Continues Integration
cargo-make comes with a predefined flow for continues integration build executed by internal or online services such as travis-ci and appveyor.<br>
For travis-ci, simple change the script to invoke the cargo-make installation and invocation as follows:

````yaml
script:
  - cargo install --debug cargo-make
  - cargo make ci-flow
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

<a name="roadmap"></a>
## Roadmap
The cargo-make task runner is still in initial development and there are many things planned for the comming release.<br>
Here are a few of the top priorities:

* Support platform specific task overrides
* Environment variables definition via toml file
* Git related task definitions in default toml
* Crate publishing task definitions in default toml
* Setup a website with more in depth explanations and examples

## API Documentation
See full docs at: [API Docs](https://sagiegurari.github.io/cargo-make/)

## Contributing
See [contributing guide](.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

| Date        | Version | Description |
| ----------- | ------- | ----------- |
| 2017-06-24  | v0.1.0  | Initial release. |

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.