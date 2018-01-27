
<a name="overview"></a>
## Overview
The cargo-make task runner enables to define and configure sets of tasks and run them as a flow.<br>
A task is a command, script, rust code or other sub tasks to execute.<br>
Tasks can have dependencies which are also tasks that will be executed before the task itself.<br>
With a simple toml based configuration file, you can define a multi platform build script that can run build, test, generate documentation,run bench tests, run security validations and more, executed by running a single command.

<a name="installation"></a>
## Installation
In order to install, just run the following command

```sh
cargo install --force cargo-make
```

This will install cargo-make in your ~/.cargo/bin.<br>
Make sure to add ~/.cargo/bin directory to your PATH variable.

<a name="tutorial"></a>
## Tutorial
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

We will create a toml file as follows:

```toml
[tasks.format]
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
```

We would execute the flow with the following command:

```sh
cargo make --makefile simple-example.toml my-flow
```

The output would look something like this:

```console
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
```

We now created a build script that can run on any platform.

<a name="usage-task-dependencies-alias"></a>
### Tasks, Dependencies and Aliases
In many cases, certain tasks depend on other tasks.<br>
For example you would like to format the code before running build and run the build before running tests.<br>
Such flow can be defined as follows:

```toml
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
```

When you run:

```sh
cargo make --makefile ./my_build.toml test
```

It will try to run test, see that it has dependencies and those have other dependencies.<br>
Therefore it will create an execution plan for the tasks based on the tasks and their dependencies.<br>
In our case it will invoke format -> build -> test.<br>

The same task will never be executed twice so if we have for example:

```toml
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
```

In this example, A depends on B and C, and both B and C are dependent on D.<br>
Task D however will not be invoked twice.<br>
The output of the execution will look something like this:

```console
[cargo-make] info - Task: A
[cargo-make] info - Setting Up Env.
[cargo-make] info - Running Task: D
[cargo-make] info - Execute Command: "sh" "/tmp/cargo-make/CNuU47tIix.sh"
hello
[cargo-make] info - Running Task: B
[cargo-make] info - Running Task: C
[cargo-make] info - Running Task: A
```

As you can see, 'hello' was printed once by task D as it was only invoked once.<br>
But what if we want to run D twice?<br>
Simple answer would be to duplicate task D and have B depend on D and C depend on D2 which is a copy of D.<br>
But duplicating can lead to bugs and to huge makefiles, so we have aliases for that.<br>
An alias task has its own name and points to another task.<br>
All of the definitions of the alias task are ignored.<br>
So now, if we want to have D execute twice we can do the following:

```toml
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
```

Now C depends on D2 and D2 is an alias for D.<br>
Execution output of such make file would like as follows:

```console
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
```

Now you can see that 'hello' was printed twice.

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
script = [
    "echo hello"
]

[tasks.do_nothing]
```

If you run task **my_task** on windows or mac, it will invoke the **do_nothing** task.<br>
However, if executed on a linux platform, it will invoke the **run** task.

<a name="usage-task-command-script-task"></a>
### Commands, Scripts and Sub Tasks
The actual operation that a task invokes can be defined in 3 ways.<br>
The below explains each one:

* **run_task** - Invokes another task with the name defined in this attribute. Unlike dependencies which are invoked before the current task, the task defined in the **run_task** is invoked after the current task.
* **command** - The command attribute defines what executable to invoke. You can use the **args** attribute to define what attributes to provide as part of the command.
* **script** - Invokes the script. You can change the executable used to invoke the script using the **script_runner** attribute. If not defined, the default platform runner is used (cmd for windows, sh for others).

Only one of the definitions will be used.<br>
If multiple attributes are defined (for example both command and script), the task will fail during invocation.

The script attribute may hold non OS scripts, for example rust code to be compiled and executed.<br>
In order to use non OS script runners, you must define the special script_runner with the **@** prefix.<br>
The following runners are currently supported:

* **@rust** - Compiles and executes the defined rust code. See [example](#usage-task-command-script-task-examplerust)
* **@shell** - For windows platform, it will try to convert the shell commands to windows batch commands (only basic scripts are supported) and execute the script, for other platforms the script will be executed as is. See [example](#usage-task-command-script-task-exampleshell2batch)

Below are some basic examples of each action type.

<a name="usage-task-command-script-task-examplesubtask"></a>
#### Sub Task
In this example, if we execute the **flow** task, it will invoke the **echo** task defined in the **run_task** attribute.

```toml
[tasks.echo]
script = [
    "echo hello world"
]

[tasks.flow]
run_task = "echo"
```

<a name="usage-task-command-script-task-examplecommand"></a>
#### Command
For running commands, you can also define the command line arguments as below example invokes cargo command with the plugin name as a command line argument:

```toml
[tasks.build-with-verbose]
command = "cargo"
args = ["build", "--verbose", "--all-features"]
```

<a name="usage-task-command-script-task-examplescript"></a>
#### Script
Below simple script which prints hello world.

```toml
[tasks.hello-world]
script = [
    "echo start...",
    "echo \"Hello World From Script\"",
    "echo end..."
]
```

You can use multi line toml string to make the script more readable as follows:

```toml
[tasks.hello-world]
script = [
'''
echo start...
echo "Hello World From Script"
echo end...
'''
]
```

<a name="usage-task-command-script-task-examplerust"></a>
#### Rust Code
In this example, when the **rust** task is invoked, the **script** content will be compiled and executed.
You can see how dependencies are defined in Cargo.toml format inside the code.

```toml
[tasks.rust]
script_runner = "@rust"
script = [
'''
//! ```cargo
//! [dependencies]
//! time = "*"
//! ```
extern crate time;
fn main() {
    println!("{}", time::now().rfc822z());
}
'''
]
```

<a name="usage-task-command-script-task-exampleshell2batch"></a>
#### Cross Platform Shell
In this example, when the **shell** task is invoked, the **script** content will be automatically converted to windows batch commands (in case we are on windows platform) and invoked.

```toml
[tasks.shell]
script_runner = "@shell"
script = [
'''
rm ./myfile.txt
'''
]
```

<a name="usage-default-tasks"></a>
### Default Tasks and Extending
There is no real need to define the tasks that were shown in the previous examples.<br>
cargo-make comes with a built in toml file that will serve as a base for every execution.<br>
The **optional** external toml file that is provided while running cargo-make will only extend and add or overwrite
tasks that are defined in the [default Makefile.toml](https://github.com/sagiegurari/cargo-make/blob/master/src/Makefile.stable.toml).<br>
Lets take the build task definition which comes already in the default toml:

```toml
[tasks.build]
command = "cargo"
args = ["build"]
```

If for example, you would like to add verbose output to it, you would just need to change the args and add the --verbose as follows:

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
The default toml file comes with many steps and flows already built in, so it is worth to check it first.

You can also extend additional external files from your external file by using the extend attribute, for example:

```toml
extend = "my_common_makefile.toml"
```

The file path in the extend attribute is always relative to the current toml file you are in and not to the process working directory.

The extend attribute can be very useful when you have a workspace with a Makefile.toml that contains all of the common custom tasks and in each project you can have a simple Makefile.toml which just has
the extend attribute pointing to the workspace makefile.

<a name="usage-load-scripts"></a>
#### Load Scripts
In more complex scenarios, you may want multiple unrelated projects to share some common custom tasks, for example if you wish to notify some internal company server of the build status.<br>
Instead of redefining those tasks in each project you can create a single toml file with those definitions and have all projects extend that file.<br>
The extend however, only knows to find the extending files in the file system, so in order to pull some common toml from a remote server (using http or git clone and so on...), you can use the load scripts.

Load scripts are defined in the config section using the load_script attribute and are invoked **before** the extend attribute is evaluated.<br>
This allows you to first pull the toml file from the remote server and put it in a location defined by the extend attribute.

Here is an example of a load script which downloads the common toml from a remote server using HTTP:

```toml
[config]
load_script = ["wget -O /home/myuser/common.toml companyserver.com/common.toml"]
```

Here is an example of pulling the common toml file from some git repo:

```toml
[config]
load_script = ["git clone git@mygitserver:user/project.git /home/myuser/common"]
```

You can run any command or set of commands you want, therefore you can build a more complex flow of how and from where to fetch the common toml file and where to put it.<br>
If needed, you can override the load_script per platform using the **linux_load_script**, **windows_load_script** and **mac_load_script** attributes.

<a name="usage-ignoring-errors"></a>
### Ignoring Errors
In some cases you want to run optional tasks as part of a bigger flow, but do not want to break your entire build in case of any error in those optional tasks.<br>
For those tasks, you can add the force=true attribute.

```toml
[tasks.unstable_task]
force = true
```

<a name="usage-platform-override"></a>
### Platform Override
In case you want to override a task or specific attributes in a task for specific platforms, you can define an override task with the platform name (currently linux, windows and mac) under the specific task.<br>
For example:

```toml
[tasks.hello-world]
script = [
    "echo \"Hello World From Unknown\""
]

[tasks.hello-world.linux]
script = [
    "echo \"Hello World From Linux\""
]
```

If you run cargo make with task 'hello-world' on linux, it would redirect to hello-world.linux while on other platforms it will execute the original hello-world.<br>
In linux the output would be:

```console
[cargo-make] info - Task: hello-world
[cargo-make] info - Setting Up Env.
[cargo-make] info - Running Task: hello-world
[cargo-make] info - Execute Command: "sh" "/tmp/cargo-make/kOUJfw8Vfc.sh"
Hello World From Linux
[cargo-make] info - Build done in 0 seconds.
```

While on other platforms

```console
[cargo-make] info - Task: hello-world
[cargo-make] info - Setting Up Env.
[cargo-make] info - Running Task: hello-world
[cargo-make] info - Execute Command: "sh" "/tmp/cargo-make/2gYnulOJLP.sh"
Hello World From Unknown
[cargo-make] info - Build done in 0 seconds.
```

In the override task you can define any attribute that will override the attribute of the parent task, while undefined attributes will use the value from the parent task and will not be modified.<br>
In case you need to delete attributes from the parent (for example you have a command defined in the parent task but you want to have a script defined in the override task), then you will
have to clear the parent task in the override task using the clear attribute as follows:

```toml
[tasks.hello-world.linux]
clear = true
script = [
    "echo \"Hello World From Linux\""
]
```

This means, however, that you will have to redefine all attributes in the override task that you want to carry with you from the parent task.<br>
**Important - alias comes before checking override task so if parent task has an alias it will be redirected to that task instead of the override.**<br>
**To have an alias redirect per platform, use the linux_alias, windows_alias, mac_alias attributes.**<br>
**In addition, aliases can not be defined in platform override tasks, only in parent tasks.**

<a name="usage-env"></a>
### Environment Variables
cargo-make enables you to defined environment variables in several ways.

<a name="usage-env-config"></a>
#### Global Configuration
You can define env vars to be set as part of the execution of the flow in the global env block for your makefile, for example:

```yaml
[env]
RUST_BACKTRACE = "1"
EVALUATED_VAR = { script = ["echo SOME VALUE"] }
TEST1 = "value1"
TEST2 = "value2"
COMPOSITE = "${TEST1} ${TEST2}"
```

Environment variables can be defined as a simple key/value pair or key and the output (second line) of the provided script.
In addition, you can define environment variables values based on other environment variables using the ${} syntax.

All environment variables defined in the env block and in the [default Makefile.toml](https://github.com/sagiegurari/cargo-make/blob/master/src/Makefile.stable.toml) will be set before running the tasks.

<a name="usage-env-task"></a>
#### Task
Environment variables can be defined inside tasks using the env attribute, so when a task is invoked (after its dependencies), the environment variables will be set, for example:

```yaml
[tasks.test-flow]
env = { "SOME_ENV_VAR" = "value" }
run_task = "actual-task"

[tasks.actual-task]
condition = { env_set = [ "SOME_ENV_VAR" ] }
script = [
    "echo var: ${SOME_ENV_VAR}"
]
```

In task level, environment variables can also be defined as key/value pair or key/script as in the global env block.

<a name="usage-env-cli"></a>
#### Command Line
Environment variables can be defined in the command line using the --env/-e argument as follows:

```console
cargo make --env ENV1=VALUE1 --env ENV2=VALUE2 -e ENV3=VALUE3
```

<a name="usage-env-global"></a>
#### Global
In addition to manually setting environment variables, cargo-make will also automatically add few environment variables on its own which can be helpful when running task scripts, commands, conditions, etc:

* **CARGO_MAKE** - Set to "true" to help sub processes identify they are running from cargo make.
* **CARGO_MAKE_TASK** - Holds the name of the main task being executed.
* **CARGO_MAKE_WORKING_DIRECTORY** - The current working directory (can be defined by setting the --cwd cli option)
* **CARGO_MAKE_RUST_VERSION** - The rust version (for example 1.20.0)
* **CARGO_MAKE_RUST_CHANNEL** - Rust channel (stable, beta, nightly)
* **CARGO_MAKE_RUST_TARGET_ARCH** - x86, x86_64, arm, etc ... (see rust cfg feature)
* **CARGO_MAKE_RUST_TARGET_ENV** - gnu, msvc, etc ... (see rust cfg feature)
* **CARGO_MAKE_RUST_TARGET_OS** - windows, macos, ios, linux, android, etc ... (see rust cfg feature)
* **CARGO_MAKE_RUST_TARGET_POINTER_WIDTH** - 32, 64
* **CARGO_MAKE_RUST_TARGET_VENDOR** - apple, pc, unknown
* **CARGO_MAKE_CRATE_HAS_DEPENDENCIES** - Holds TRUE/FALSE based if there are dependencies defined in the Cargo.toml or not (defined as FALSE if no Cargo.toml is found)
* **CARGO_MAKE_CRATE_IS_WORKSPACE** - Holds TRUE/FALSE based if this is a workspace crate or not (defined even if no Cargo.toml is found)
* **CARGO_MAKE_CRATE_WORKSPACE_MEMBERS** - Holds list of member paths (defined as empty value if no Cargo.toml is found)

The following environment variables will be set by cargo-make if Cargo.toml file exists and the relevant value is defined:

* **CARGO_MAKE_CRATE_NAME** - Holds the crate name from the Cargo.toml file found in the cwd.
* **CARGO_MAKE_CRATE_FS_NAME** - Same as CARGO_MAKE_CRATE_NAME however some characters are replaced (for example '-' to '_').
* **CARGO_MAKE_CRATE_VERSION** - Holds the crate version from the Cargo.toml file found in the cwd.
* **CARGO_MAKE_CRATE_DESCRIPTION** - Holds the crate description from the Cargo.toml file found in the cwd.
* **CARGO_MAKE_CRATE_LICENSE** - Holds the crate license from the Cargo.toml file found in the cwd.
* **CARGO_MAKE_CRATE_DOCUMENTATION** - Holds the crate documentation link from the Cargo.toml file found in the cwd.
* **CARGO_MAKE_CRATE_HOMEPAGE** - Holds the crate homepage link from the Cargo.toml file found in the cwd.
* **CARGO_MAKE_CRATE_REPOSITORY** - Holds the crate repository link from the Cargo.toml file found in the cwd.

The following environment variables will be set by cargo-make if the project is part of a git repo:

* **CARGO_MAKE_GIT_BRANCH** - The current branch name.
* **CARGO_MAKE_GIT_USER_NAME** - The user name pulled from the git config user.name key.
* **CARGO_MAKE_GIT_USER_EMAIL** - The user email pulled from the git config user.email key.

<a name="usage-conditions"></a>
### Conditions
Conditions allow you to evaluate at runtime if to run a specific task or not.<br>
These conditions are evaluated before the task is running its installation and/or commands and if the condition is not fulfilled, the task will not be invoked.<br>
The task dependencies however are not affected by parent task condition outcome.

There are two types of conditions:

* Criteria
* Scripts

The task runner will evaluate any condition defined and a task definition may contain both types at the same time.

<a name="usage-conditions-structure"></a>
#### Criteria
The condition attribute may define multiple parameters to validate.<br>
All defined parameters must be valid for the condition as a whole to be true and enable the task to run.

Below is an example of a condition script that checks that we are running on windows or linux (but not mac) and that we are running on beta or nightly (but not stable):

```toml
[tasks.test-condition]
condition = { platforms = ["windows", "linux"], channels = ["beta", "nightly"] }
script = [
    "echo \"condition was met\""
]
```

The following condition types are available:

* **platforms** - List of platform names (windows, linux, mac)
* **channels** - List of rust channels (stable, beta, nightly)
* **env_set** - List of environment variables that must be defined
* **env_not_set** - List of environment variables that must not be defined
* **env** - Map of environment variables that must be defined and equal to the provided values

Few examples:

```toml
[tasks.test-condition]
condition = { platforms = ["windows", "linux"], channels = ["beta", "nightly"], env_set = [ "KCOV_VERSION" ], env_not_set = [ "CARGO_MAKE_SKIP_CODECOV" ], env = { "TRAVIS" = "true", "CARGO_MAKE_RUN_CODECOV" = "true" } }
```

<a name="usage-conditions-script"></a>
#### Scripts
These script are invoked before the task is running its installation and/or commands and if the exit code of the condition script is non zero, the task will not be invoked.

Below is an example of a condition script that always returns a non zero value, in which case the command is never executed:

```toml
[tasks.never]
condition_script = [
    "exit 1"
]
command = "cargo"
args = ["build"]
```

Condition scripts can be used to ensure that the task is only invoked if a specific condition is met, for example if a specific 3rd party is installed.

<a name="usage-conditions-and-subtasks"></a>
#### Combining Conditions and Sub Tasks
Conditions and run_task combined can enable you to define a conditional sub flow.<br>
For example, if you have a coverage flow that should only be invoked on linux in a travis build, and only if the CARGO_MAKE_RUN_CODECOV environment variable is defined as "true":

```toml
[tasks.ci-coverage-flow]
description = "Runs the coverage flow and uploads the results to codecov."
condition = { platforms = ["linux"], env = { "TRAVIS" = "true", "CARGO_MAKE_RUN_CODECOV" = "true" } }
run_task = "codecov-flow"

[tasks.codecov-flow]
description = "Runs the full coverage flow and uploads the results to codecov."
windows_alias = "empty"
dependencies = [
    "coverage-flow",
    "codecov"
]
```

The first task **ci-coverage-flow** defines the condition that checks we are on linux, running as part of a travis build and the CARGO_MAKE_RUN_CODECOV environment variable is set to "true".<br>
Only if all conditions are met, it will run the **codecov-flow** task.<br>
We can't define the condition directly on the **codecov-flow** task, as it will invoke the task dependencies before checking the condition.

<a name="usage-ci"></a>
### Continuous Integration
cargo-make comes with a predefined flow for continuous integration build executed by internal or online services such as travis-ci and appveyor.<br>
It is recommended to install cargo-make with the debug flag for faster installation.

<a name="usage-ci-travis"></a>
#### Travis
Add the following to .travis.yml file:

```yaml
script:
  - cargo install --debug cargo-make
  - cargo make ci-flow
```

If you want to run code coverage and upload it to codecov, also define the following environment variable:

```yaml
env:
  global:
    - CARGO_MAKE_RUN_CODECOV="true"
```

You can see full yaml file at: [.travis.yml](https://github.com/sagiegurari/cargo-make/blob/master/.travis.yml)

When working with workspaces, in order to run the ci-flow for each member and package all coverage data, use the following command:

```yaml
script:
  - cargo install --debug cargo-make
  - cargo make workspace-ci-flow --no-workspace
```

For faster cargo-make installation as part of the build, you can also pull the binary version of cargo-make directly and invoke it without running cargo install which should reduce your build time, as follows

```yml
script:
  - wget -O ~/.cargo/bin/cargo-make https://bintray.com/sagiegurari/cargo-make/download_file?file_path=cargo-make_v{{ site.version }}
  - chmod 777 ~/.cargo/bin/cargo-make
  - cargo-make make ci-flow
```

The specific version of cargo-make requested is defined in the suffix of the cargo-make file name in the form of: cargo-make_v[VERSION], for example

```sh
https://bintray.com/sagiegurari/cargo-make/download_file?file_path=cargo-make_v{{ site.version }}
```

In order to pull the latest prebuild cargo-make binary, use the following example:

```yml
env:
  global:
  - CARGO_MAKE_URL="https://bintray.com/sagiegurari/cargo-make/download_file?file_path=cargo-make_v"

before_install:
  - curl -SsL $CARGO_MAKE_URL$(cargo search cargo-make | grep cargo-make | cut -d\" -f2) > ~/.cargo/bin/cargo-make
  - chmod 777 ~/.cargo/bin/cargo-make
  - cargo-make make ci-flow
```

<a name="usage-ci-appveyor"></a>
#### AppVeyor
Add the following to appveyor.yml file:

```yaml
build: false

test_script:
  - cargo install --debug cargo-make
  - cargo make ci-flow
```

You can see full yaml file at: [appveyor.yml](https://github.com/sagiegurari/cargo-make/blob/master/appveyor.yml)

When working with workspaces, in order to run the ci-flow for each member and package all coverage data, use the following command:

```yaml
build: false

test_script:
  - cargo install --debug cargo-make
  - cargo make workspace-ci-flow --no-workspace
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
  - cargo make workspace-ci-flow --no-workspace
```

To upload your coverage information to codecov, you'll need to go to repo settings for your GitLab repo,
[and add a secret variable](https://docs.gitlab.com/ce/ci/variables/README.html#secret-variables) with your codecov token for that repository.

Then you can add the following in your `gitlab-ci.yml` to enable coverage support:

```yaml
variables:
  CARGO_MAKE_RUN_CODECOV: "true"
```

<a name="usage-predefined-flows"></a>
### Predefined Flows
The [default Makefile.toml](https://github.com/sagiegurari/cargo-make/blob/master/src/Makefile.stable.toml) file comes with many predefined tasks and flows.<br>
The following are some of the main flows that can be used without any need of an external Makefile.toml definition.

* **default** - Can be executed without adding the task name, simply run 'cargo make'. This task is an alias for dev-test-flow.
* **dev-test-flow** - Also the default flow so it can be invoked without writing any task name (simple run ```cargo make```).<br>This task runs formatting, cargo build and cargo test and will most likely be the set of tasks that you will run while developing and testing a rust project.
* **watch-flow** - Watches for any file change and if any change is detected, it will invoke the test flow.
* **ci-flow** - Should be used in CI builds (such as travis/appveyor) and it runs build and test with verbose level.
* **workspace-ci-flow** - Should be used in CI builds (such as travis/appveyor) for workspace projects.
* **publish-flow** - Cleans old target directory and publishes the project.
* **build-flow** - Runs full cycle of build, tests, security checks, dependencies up to date validations and documentation generation.<br>This flow can be used to make sure your project is fully tested and up to date.
* **coverage-flow** - Creates coverage report from all unit and integration tests (not supported on windows). By default cargo-make uses kcov for code coverage, however additional unsupported implementations are defined.
* **codecov-flow** - Runs the coverage-flow and uploads the coverage results to codecov (not supported on windows).

<a name="usage-predefined-flows-coverage"></a>
#### Coverage
cargo-make has built in support for multiple coverage tasks.<br>
Switching between them without modifying the flows is done by changing the main coverage task alias.

Currently the main coverage task is defined as follows:

```toml
[tasks.coverage]
alias = "coverage-kcov"
```

To switch to another provider simply change the alias to that specific task name, for example if we would like to use the already defined tarpaulin provider:

```toml
[tasks.coverage]
alias = "coverage-tarpaulin"
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

<a name="usage-predefined-flows-cargo"></a>
#### Cargo Commands and Plugins

* **clean** - Runs the cargo clean command.
* **build** - Runs the rust compiler.
* **build-verbose** - Runs the rust compiler with verbose output.
* **build-release** - Runs release build.* **test** - Runs all available tests.
* **test-verbose** - Runs all available tests with verbose output.
* **bench** - Runs all available bench files.
* **check** - Runs cargo check.
* **check-tests** - Runs cargo check for project tests.
* **check-examples** - Runs cargo check for project examples.
* **docs** - Generate rust documentation.
* **package** - Runs the cargo package command.
* **publish** - Runs the cargo publish command.
* **format** - Runs the cargo rustfmt plugin.
* **outdated** - Runs verify-outdated cargo plugin.
* **verify-project** - Runs verify-project cargo plugin.
* **audit** - Runs verify-audit cargo plugin.
* **clippy** - Runs clippy code linter.

<a name="usage-predefined-flows-git"></a>
#### Git Commands

* **git-status** - Runs git status command.
* **git-add** - Runs the cargo add command.
* **git-commit** - Runs git commit command.
* **git-commit-message** - Runs git commit command with the message defined in the COMMIT_MSG environment variable.
* **git-push** - Runs git push command.
* **git-pull** - Runs git pull command.
* **github-publish** - Creates a new github release.
* **github-publish-custom-name** - Creates a new github release.

<a name="usage-predefined-flows-flows"></a>
#### Flows/Other

* **empty** - Empty Task
* **init** - By default this task is invoked at the start of every cargo-make run.
* **end** - By default this task is invoked at the end of every cargo-make run.
* **default** - Default task points to the development testing flow
* **ci-flow** - CI task will run cargo build and cargo test with verbose output
* **workspace-ci-flow** - CI task will run CI flow for each member and merge coverage reports
* **build-flow** - Full sanity testing flow.
* **dev-test-flow** - Development testing flow will first format the code, and than run cargo build and test
* **dev-watch-flow** - Alias for test-flow
* **watch-flow** - Watches for any file change and if any change is detected, it will invoke the test flow.
* **copy-apidocs** - Copies the generated documentation to the docs/api directory.
* **clean-apidocs** - Delete API docs.
* **format-flow** - Runs the cargo rustfmt plugin as part of a flow.
* **publish-flow** - Publish flow - First clean the target directory of any old leftovers, package and publish
* **bench-flow** - Runs a bench flow.
* **check-flow** - Runs cargo check flow.
* **bench-ci-flow** - Runs/Compiles the benches if conditions are met.
* **examples-ci-flow** - Compiles the examples if conditions are met.
* **delete-lock** - Deletes the Cargo.lock file.
* **codecov** - Runs codecov script to upload coverage results to codecov.
* **coverage** - Runs coverage (by default using kcov).
* **coverage-flow** - Runs the full coverage flow.
* **coverage-kcov** - Installs (if missing) and runs coverage using kcov (not supported on windows)
* **coverage-tarpaulin** - Runs coverage using tarpaulin rust crate (linux only)
* **workspace-coverage** - Runs coverage task for all members and packages all of them (by default the codecov flow).
* **codecov-flow** - Runs the full coverage flow and uploads the results to codecov.
* **ci-coverage-flow** - Runs the coverage flow and uploads the results to codecov.
* **workspace-members-ci** - Runs the ci-flow for every workspace member.
* **build-publish-flow** - Runs full sanity, generates github release and publishes the crate.
* **upload-artifacts** - Uploads the binary artifact from the cargo package/publish output.
* **bintray-upload** - Uploads the binary artifact from the cargo package/publish output to bintray.

<a name="usage-predefined-flows-disable"></a>
#### Disabling Predefined Tasks/Flows

In order to prevent loading of internal core tasks and flows, simply add the following configuration property in your external Makefile.toml:

```toml
[config]
skip_core_tasks = true
```

<a name="usage-workspace-support"></a>
### Workspace Support
In case cargo-make detects that the current working directory is a workspace crate (crate with Cargo.toml which defines a workspace and its members), it will not invoke the requested tasks in that directory.<br>
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

And we ran ```cargo make mytask```, it will go to each workspace member directory and execute: ```cargo make mytask``` at that directory,
where mytask is the original task that was requested on the workspace level.<br>
The order of the members is defined by the member attribute in the workspace Cargo.toml.

We can use this capability to run same functionality on all workspace member crates, for example if we want to format all crates, we can run in the workspace directory: ```cargo make format```.

In case you wish to run the tasks on the workspace level and not on the members, use the ```--no-workspace``` cli flag when running cargo make, for example:

```sh
cargo make --no-workspace mytask
```

You can define a composite flow that runs both workspace level tasks and member level tasks using this flag.<br>
This is an example of a workspace level Makefile.toml which enables to run such a flow:

```toml
[tasks.composite]
dependencies = ["member_flow", "workspace_flow"]

[tasks.member_flow]
command = "cargo"
args = ["make", "member_task"]

[tasks.workspace_flow]
#run some workspace level command or flow
```

You can start this composite flow as follows:

```sh
cargo make --no-workspace composite
```

<a name="usage-workspace-support-skip-members"></a>
#### Skipping Specific Members

In most cases you will want to run a specific flow on all members, but in rare cases you will want to skip specific members.

By setting the **CARGO_MAKE_WORKSPACE_SKIP_MEMBERS** environment variable to hold the member names to skip (seperated by a ';' character), you can define if you want those members not to participate in the flow.

In the below example we will skip member3 and member4 (should be defined in the workspace level Makefile.toml):

```toml
[env]
CARGO_MAKE_WORKSPACE_SKIP_MEMBERS = "member3;member4"
```

However there are some cases you will want to skip specific members only if a specific condition is met.

For example, you want to build a member module only if we are running on a rust nightly compiler.

This is a simple example of a conditioned skip for member3 and memeber4 (should be defined in the workspace level Makefile.toml):

```toml
[tasks.workspace-task]
condition = { channels = ["beta", "stable"] }
env = { "CARGO_MAKE_MEMBER_TASK" = "member-task", "CARGO_MAKE_WORKSPACE_SKIP_MEMBERS" = "member3;member4" }
run_task = "do-on-members"
```

You will have to invoke this as a composite flow:

```sh
cargo make workspace-task --no-workspace
```

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

<a name="usage-cli"></a>
### Cli Options
These are the following options available while running cargo-make:

```console
USAGE:
    cargo make [FLAGS] [OPTIONS] [TASK]

FLAGS:
        --disable-check-for-updates    Disables the update check during startup
        --experimental                 Allows access unsupported experimental predefined tasks.
    -h, --help                         Prints help information
        --list-all-steps               Lists all known steps
        --no-workspace                 Disable workspace support (tasks are triggered on workspace and not on members)
        --print-steps                  Only prints the steps of the build in the order they will be invoked but without invoking them
    -v, --verbose                      Sets the log level to verbose (shorthand for --loglevel verbose)
    -V, --version                      Prints version information

OPTIONS:
        --cwd <DIRECTORY>         Will set the current working directory. The search for the makefile will be from this directory if defined.
    -e, --env <ENV>...            Set environment variables
    -l, --loglevel <LOG LEVEL>    The log level [default: info]  [values: verbose, info, error]
        --makefile <FILE>         The optional toml file containing the tasks definitions [default: Makefile.toml]
    -t, --task <TASK>             The task name to execute (can omit the flag if the task name is the last argument) [default: default]

ARGS:
    <TASK>
```

<a name="cargo-make-global-config"></a>
### Global Configuration
Some of the default CLI values and cargo-make behaviour can be configured via optional global configuration file located at: ~/.cargo-make/config.toml

The following example config.toml shows all possible configurations with their default values:

```toml
# The default log level if not defined by the --loglevel cli argument
log_level = "info"

# The default task name if no task was provided as part of the cargo-make invocation
default_task_name = "default"
```

<a name="descriptor-definition"></a>
## Makefile Definition

[Config Section](https://sagiegurari.github.io/cargo-make/api/cargo_make/types/struct.ConfigSection.html)

[Task](https://sagiegurari.github.io/cargo-make/api/cargo_make/types/struct.Task.html)

[Platform Override](https://sagiegurari.github.io/cargo-make/api/cargo_make/types/struct.PlatformOverrideTask.html)

[Condition](https://sagiegurari.github.io/cargo-make/api/cargo_make/types/struct.TaskCondition.html)

More info can be found in the [types](https://sagiegurari.github.io/cargo-make/api/cargo_make/types/index.html) section of the API documentation.

<a name="task-name-conventions"></a>
## Task Naming Conventions
This section explains the logic behind the default task names.<br>
While the default names logic can be used as a convention for any new task defined in some project Makefile.toml, it is not required.

The [default Makefile.toml](https://github.com/sagiegurari/cargo-make/blob/master/src/Makefile.stable.toml) file comes with three types of tasks:

* Single command or script task (for example ```cargo build```)
* Tasks that come before or after the single command tasks
* Tasks that define flows using dependencies

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

In the [default Makefile.toml](https://github.com/sagiegurari/cargo-make/blob/master/src/Makefile.stable.toml), all pre/post tasks are empty and are there as placeholders
for external Makefile.toml to override so custom functionality can be defined easily before/after running a specific task.

Flows are named with the flow suffix, for example: ci-flow

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
While already feature rich, cargo-make is still young and under development.<br>
You can view the future development items list in the [github project issues](https://github.com/sagiegurari/cargo-make/issues)

## Contributing
See [contributing guide](https://github.com/sagiegurari/cargo-make/blob/master/.github/CONTRIBUTING.md)

<a name="history"></a>
## Release History

See [Changelog](https://github.com/sagiegurari/cargo-make/blob/master/CHANGELOG.md)

<a name="license"></a>
## License
Developed by Sagie Gur-Ari and licensed under the Apache 2 open source license.
