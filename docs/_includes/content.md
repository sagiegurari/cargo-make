
<a name="overview"></a>
## Overview
The cargo-make task runner enables to define and configure sets of tasks and run them as a flow.<br>
A task is a command, script, rust code or other sub tasks to execute.<br>
Tasks can have dependencies which are also tasks that will be executed before the task itself.<br>
With a simple toml based configuration file, you can define a multi platform build script that can run build, test, generate documentation, run bench tests, run security validations and more, executed by running a single command.

<a name="installation"></a>
## Installation
In order to install, just run the following command

```sh
cargo install --force cargo-make
```

This will install cargo-make in your ~/.cargo/bin.<br>
Make sure to add ~/.cargo/bin directory to your PATH variable.<br>
<br>
You will have two executables available: *cargo-make* and *makers*<br>

* **cargo-make** - This is a cargo plugin invoked using ```cargo make ...```
* **makers** - A standalone executable which provides same features and cli arguments as cargo-make but is invoked directly and not as a cargo plugin.

<a name="installation-binary-release"></a>
### Binary Release
Binary releases are available in the [github releases page](https://github.com/sagiegurari/cargo-make/releases).<br>
The following binaries are available for each release:

* x86_64-unknown-linux-musl
* x86_64-apple-darwin
* x86_64-pc-windows-msvc

Linux builds for arm are available on [bintray](https://bintray.com/sagiegurari/cargo-make/linux)

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
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: simple-example.toml
[cargo-make] INFO - Task: my-flow
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: format
[cargo-make] INFO - Execute Command: "cargo" "fmt" "--" "--write-mode=overwrite"
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

**cargo-make can be invoked as a cargo plugin via 'cargo make' command or as a standalone executable via 'makers' command.**

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

*As a side note, cargo-make will attempt to invoke the task dependencies in the order that they were defined unless they are defined also as sub dependencies.*

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

<a name="usage-task-command-script-task-examplecommand"></a>
#### Command
For running commands, you can also define the command line arguments as below example invokes cargo command with the plugin name as a command line argument:

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

cargo-make cli also supports additional arguments which will be available to all tasks.<br>
Following example task, will print those additional arguments:

```toml
[tasks.varargs]
command = "echo"
args = [
    "args are:", "${@}"
]
```

Invoking cargo-make with additional arguments would result in the following:

```console
> cargo make varargs arg1 arg2 arg3

[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: Makefile.toml
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

[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: Makefile.toml
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

[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: Makefile.toml
[cargo-make] INFO - Task: varargs
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: varargs
[cargo-make] INFO - Execute Command: "echo" "args are:" "arg1" "arg2" "arg3"
args are: -o=arg1 -o=arg2 -o=arg3
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
```

Command line arguments can also contain built in [functions which are explained later on in this document.](#usage-functions)

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

cargo-make cli also supports additional arguments which will be available to all tasks.<br>
Following example task, will print those additional arguments:

```toml
[tasks.cli-args]
script = [
    "echo args are: ${@}"
]
```

Invoking cargo-make with additional arguments would result in the following:

```console
> cargo make cli-args arg1 arg2 arg3

[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: Makefile.toml
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

[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: Makefile.toml
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

It is also possible to point to an existing script instead of holding the script text inside the makefile by using the **file** property as follows:

```toml
[tasks.hello-world-from-script-file]
script = { file = "script.sh" }
```

Script file paths are always relative to the current working directory unless specified by the **absolute_path** attribute, for example:

```toml
[tasks.hello-world-from-script-file-absolute-path]
script = { file = "${CARGO_MAKE_WORKING_DIRECTORY}/script.sh", absolute_path = true }
```

File paths support environment substitution.<br><br>
**Favor commands over scripts, as commands support more featues such as [automatic dependencies installation](#usage-installing-dependencies), [argument functions](#usage-functions), and more...**

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

Same as OS scripts, the @rust runner also supports the cargo-make CLI arguments access.<br>
There are several different rust script runners currently available:

* [cargo-script](https://crates.io/crates/cargo-script)
* [cargo-play](https://crates.io/crates/cargo-play)

By default, cargo-script is used, however this can be changed via environment variable **CARGO_MAKE_RUST_SCRIPT_PROVIDER** which should hold the crate name.<br>
This enables to define a different runner for each task by setting it in the **env** block of the specific tasks.<br>
For example:

```toml
[tasks.cargo-script]
env = { "CARGO_MAKE_RUST_SCRIPT_PROVIDER" = "cargo-script" }
script_runner = "@rust"
script = [
'''
fn main() {
    println!("test");
}
'''
]

[tasks.cargo-play]
env = { "CARGO_MAKE_RUST_SCRIPT_PROVIDER" = "cargo-play" }
script_runner = "@rust"
script = [
'''
fn main() {
    println!("test");
}
'''
]
```

Keep in mind that dependencies used by the rust script are defined differently for each runner.<br>
Please see the specific crate docs for learn more.

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

Same as OS scripts, the @shell runner also supports the cargo-make CLI arguments access.<br>
<br>
See [shell2batch](https://github.com/sagiegurari/shell2batch) project for complete set of features.

<a name="usage-task-command-script-task-examplegeneric"></a>
#### Other Programming Languages
cargo-make can also run scripts written in various scripting languages such as python, perl, ruby, javascript and more...<br>
Any runner which takes the form of ```command file``` (for example ```python ./program.py```) is supported.

Below are few examples:

```toml
[tasks.python]
script_runner = "python"
script_extension = "py"
script = [
'''
print("Hello, World!")
'''
]

[tasks.perl]
script_runner = "perl"
script_extension = "pl"
script = [
'''
print "Hello, World!\n";
'''
]

[tasks.javascript]
script_runner = "node"
script_extension = "js"
script = [
'''
console.log('Hello, World!');
'''
]

[tasks.powershell]
script_runner = "powershell"
script_extension = "ps1"
script = [
'''
Write-Host "Hello, World!"
'''
]
```

<a name="usage-task-command-script-task-exampleshebang"></a>
#### Shebang Support
Instead of defining custom runners via **script_runner** attribute, it's possible to define it in the script shebang line.

In case of windows, make sure not to use a runner which doesn't have the ```#``` character defined as comment (for example cmd.exe does not), which would lead to an error.

Example task using bash:

```toml
[tasks.shebang-sh]
script = [
'''
#!/usr/bin/env bash
echo hello
'''
]
```

Output:

```console
> cargo make --cwd ./examples --makefile ./shebang.toml shebang-sh
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: ./shebang.toml
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

Example task using python:

```toml
[tasks.shebang-python]
script = [
'''
#!/usr/bin/env python3
print("Hello, World!")
'''
]
```

Output:

```console
> cargo make --cwd ./examples --makefile ./shebang.toml shebang-python
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: ./shebang.toml
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

<a name="usage-default-tasks"></a>
### Default Tasks and Extending
There is no real need to define some of the basic build, test, ... tasks that were shown in the previous examples.<br>
cargo-make comes with a built in toml file that will serve as a base for every execution.<br>
The **optional** external toml file that is provided while running cargo-make will only extend and add or overwrite
tasks that are defined in the [default Makefile.toml](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/Makefile.stable.toml).<br>
Lets take the build task definition which comes already in the default toml:

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
The default toml file comes with many steps and flows already built in, so it is worth to check it first.<br>

In case you do want to delete all of the original task attributes in your extended task, you can use the clear attribute as follows:

```toml
[tasks.sometask]
clear = true
command = "echo"
args = [
    "extended task"
]
```

You can also extend additional external files from your external makefile by using the extend attribute, for example:

```toml
extend = "my_common_makefile.toml"
```

The file path in the extend attribute is always relative to the current toml file you are in and not to the process working directory.

The extend attribute can be very useful when you have a workspace with a Makefile.toml that contains all of the common custom tasks and in each project you can have a simple Makefile.toml which just has
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

The workspace level makefile **env** section must contain the following environment variable (can also be set via cli)

```toml
[env]
CARGO_MAKE_EXTEND_WORKSPACE_MAKEFILE = "true"
```

This allows you to maintina a single makefile for the entire workspace but having access to those custom tasks in every member crate.

<a name="usage-load-scripts"></a>
#### Load Scripts
In more complex scenarios, you may want multiple unrelated projects to share some common custom tasks, for example if you wish to notify some internal company server of the build status.<br>
Instead of redefining those tasks in each project you can create a single toml file with those definitions and have all projects extend that file.<br>
The extend however, only knows to find the extending files in the localt file system, so in order to pull some common toml from a remote server (using http or git clone and so on...), you can use the load scripts.

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
Let’s take for example the **build** task which is predefined internally inside cargo-make as follows:

```toml
[tasks.build]
description = "Runs the rust compiler."
category = "Build"
command = "cargo"
args = ["build", "--all-features"]
```

If for example you do not want to use the **--all-features** mode, you can just change the args of the task in your external Makefile.toml as follows:

```toml
[tasks.build]
args = ["build"]
```

When cargo-make starts up, it will load the external Makefile.toml and the internal makefile definitions and will merge them.<br>
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
[cargo-make] INFO - Task: hello-world
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: hello-world
[cargo-make] INFO - Execute Command: "sh" "/tmp/cargo-make/kOUJfw8Vfc.sh"
Hello World From Linux
[cargo-make] INFO - Build done in 0 seconds.
```

While on other platforms

```console
[cargo-make] INFO - Task: hello-world
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: hello-world
[cargo-make] INFO - Execute Command: "sh" "/tmp/cargo-make/2gYnulOJLP.sh"
Hello World From Unknown
[cargo-make] INFO - Build done in 0 seconds.
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
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: task_extend.toml
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
cargo-make enables you to defined environment variables in several ways.<br>
Environment variables can later be used in commands, scripts, conditions, functions and more, so it is important to have a powerful way to define them for your build.

* [Global Configuration](#usage-env-config)
* [Task](#usage-env-task)
* [Command Line](#usage-env-cli)
* [Env File](#usage-env-file)
* [Global](#usage-env-global)

<a name="usage-env-config"></a>
#### Global Configuration
You can define env vars to be set as part of the execution of the flow in the global env block for your makefile, for example:

```toml
[env]
RUST_BACKTRACE = "1"
EVALUATED_VAR = { script = ["echo SOME VALUE"] }
TEST1 = "value1"
TEST2 = "value2"
BOOL_VALUE = true
DEV = false
PROD = false
COMPOSITE = "${TEST1} ${TEST2}"
MULTI_LINE_SCRIPT = { script = ["echo 1\necho 2"], multi_line = true }
LIBRARY_EXTENSION = { source = "${CARGO_MAKE_RUST_TARGET_OS}", default_value = "unknown", mapping = {"linux" = "so", "macos" = "dylib", "windows" = "dll", "openbsd" = "so" } }
TO_UNSET = { unset = true }

# profile based environment override
[env.development]
DEV = true

[env.production]
PROD = true
```

Environment variables can be defined as:

* Simple key/value pair, where the value can be either string or boolean
  * ```RUST_BACKTRACE = "1"```
  * ```BOOL_VALUE = true```
* Key and output of a script - ```EVALUATED_VAR = { script = ["echo SOME VALUE"] }```
* Key and a decode map (if **default_value** not provided, it will default to the source value) - ```LIBRARY_EXTENSION = { source = "${CARGO_MAKE_RUST_TARGET_OS}", default_value = "unknown", mapping = {"linux" = "so", "macos" = "dylib", "windows" = "dll", "openbsd" = "so" } }```
* Key and a value expression built from strings and other env variables using the ${} syntax - ```COMPOSITE = "${TEST1} and ${TEST2}"```

All environment variables defined in the env block and in the [default Makefile.toml](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/Makefile.stable.toml) will be set before running the tasks.<br>
To unset an environment variable, use the ```MY_VAR = { unset = true }``` syntax.<br>
See more on profile based environment setup in the [profile environment section](#usage-profiles-env)

<a name="usage-env-task"></a>
#### Task
Environment variables can be defined inside tasks using the env attribute, so when a task is invoked (after its dependencies), the environment variables will be set, for example:

```toml
[tasks.test-flow]
env = { "SOME_ENV_VAR" = "value" }
run_task = "actual-task"

[tasks.actual-task]
condition = { env_set = [ "SOME_ENV_VAR" ] }
script = [
    "echo var: ${SOME_ENV_VAR}"
]
```

In task level, environment variables capabilities are the same as in the [global level](#usage-env-config).

<a name="usage-env-cli"></a>
#### Command Line
Environment variables can be defined in the command line using the --env/-e argument as follows:

```console
cargo make --env ENV1=VALUE1 --env ENV2=VALUE2 -e ENV3=VALUE3
```

<a name="usage-env-file"></a>
#### Env File
It is also possible to provide an env file path as part of the cli args as follows:

```console
cargo make --env-file=./env/production.env
```

This allows to use the same Makefile.toml but with different environment variables loaded from different env files.

The env file, is a simple key=value file.<br>
In addition, you can define environment variables values based on other environment variables using the ${} syntax.<br>
For example:

```properties
#just a comment...
ENV1_TEST=TEST1
ENV2_TEST=TEST2
ENV3_TEST=VALUE OF ENV2 IS: ${ENV2_TEST}
```

<a name="usage-env-global"></a>
#### Global
In addition to manually setting environment variables, cargo-make will also automatically add few environment variables on its own which can be helpful when running task scripts, commands, conditions, etc:

* **CARGO_MAKE** - Set to "true" to help sub processes identify they are running from cargo make.
* **CARGO_MAKE_TASK** - Holds the name of the main task being executed.
* **CARGO_MAKE_TASK_ARGS** - A list of arguments provided to cargo-make after the task name, seperated with a ';' character.
* **CARGO_MAKE_COMMAND** - The command used to invoke cargo-make (for example: *cargo make* and *makers*)
* **CARGO_MAKE_WORKING_DIRECTORY** - The current working directory (can be defined by setting the --cwd cli option)
* **CARGO_MAKE_PROFILE** - The current profile name in lower case (should not be manually modified by global/task env blocks)
* **CARGO_MAKE_ADDITIONAL_PROFILES** - The additional profile names in lower case, seperated with a ';' character (should not be manually modified by global/task env blocks)
* **CARGO_MAKE_CARGO_PROFILE** - The [cargo profile](https://doc.rust-lang.org/cargo/reference/manifest.html#the-profile-sections) name mapped from the **CARGO_MAKE_PROFILE** (unmapped value will default to CARGO_MAKE_PROFILE value)
* **CARGO_MAKE_RUST_VERSION** - The rust version (for example 1.20.0)
* **CARGO_MAKE_RUST_CHANNEL** - Rust channel (stable, beta, nightly)
* **CARGO_MAKE_RUST_TARGET_ARCH** - x86, x86_64, arm, etc ... (see rust cfg feature)
* **CARGO_MAKE_RUST_TARGET_ENV** - gnu, msvc, etc ... (see rust cfg feature)
* **CARGO_MAKE_RUST_TARGET_OS** - windows, macos, ios, linux, android, etc ... (see rust cfg feature)
* **CARGO_MAKE_RUST_TARGET_POINTER_WIDTH** - 32, 64
* **CARGO_MAKE_RUST_TARGET_VENDOR** - apple, pc, unknown
* **CARGO_MAKE_CRATE_HAS_DEPENDENCIES** - Holds true/false based if there are dependencies defined in the Cargo.toml or not (defined as *false* if no Cargo.toml is found)
* **CARGO_MAKE_CRATE_IS_WORKSPACE** - Holds true/false based if this is a workspace crate or not (defined even if no Cargo.toml is found)
* **CARGO_MAKE_CRATE_WORKSPACE_MEMBERS** - Holds list of member paths (defined as empty value if no Cargo.toml is found)
* **CARGO_MAKE_CRATE_CURRENT_WORKSPACE_MEMBER** - Holds the name of the current workspace member being built (only if flow started as a workspace level flow)
* **CARGO_MAKE_CRATE_LOCK_FILE_EXISTS** - Holds true/false based if a Cargo.lock file exists in current working directory (in workspace projects, each member has a different working directory).
* **CARGO_MAKE_CI** - Holds true/false based if the task is running in a continuous integration system (such as Travis CI).
* **CARGO_MAKE_PR** - Holds true/false based if the task is running in a continuous integration system (such as Travis CI) as part of a pull request build (unknown is set as false).

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

<a name="usage-ignoring-errors"></a>
### Ignoring Errors
In some cases you want to run optional tasks as part of a bigger flow, but do not want to break your entire build in case of any error in those optional tasks.<br>
For those tasks, you can add the ignore_errors=true attribute.

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

Below is an example of a condition definition that checks that we are running on windows or linux (but not mac) and that we are running on beta or nightly (but not stable):

```toml
[tasks.test-condition]
condition = { platforms = ["windows", "linux"], channels = ["beta", "nightly"] }
script = [
    "echo \"condition was met\""
]
```

The following condition types are available:

* **profile** - See [profiles](#usage-profiles) for more info
* **platforms** - List of platform names (windows, linux, mac)
* **channels** - List of rust channels (stable, beta, nightly)
* **env_set** - List of environment variables that must be defined
* **env_not_set** - List of environment variables that must not be defined
* **env_true** - List of environment variables that must be defined and must not be set to any of the following (case insensitive): false, no, 0 or empty
* **env_false** - List of environment variables that must be defined and set to any of the following (case insensitive): false, no, 0 or empty
* **env** - Map of environment variables that must be defined and equal to the provided values
* **rust_version** - Optional definition of min, max and/or specific rust version
* **files_exist** - List of absolute path files to check they exist. Environment substitution is supported so you can define relative paths such as **${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml**
* **files_not_exist** - List of absolute path files to check they do not exist. Environment substitution is supported so you can define relative paths such as **${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml**

Few examples:

```toml
[tasks.test-condition]
condition = { profiles = ["development", "production"], platforms = ["windows", "linux"], channels = ["beta", "nightly"], env_set = [ "CARGO_MAKE_KCOV_VERSION" ], env_not_set = [ "CARGO_MAKE_SKIP_CODECOV" ], env = { "CARGO_MAKE_CI" = "true", "CARGO_MAKE_RUN_CODECOV" = "true" }, rust_version = { min = "1.20.0", max = "1.30.0" } files_exist = ["${CARGO_MAKE_WORKING_DIRECTORY}/Cargo.toml"] files_not_exist = ["${CARGO_MAKE_WORKING_DIRECTORY}/Cargo2.toml"] }
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
For example, if you have a coverage flow that should only be invoked on linux in a CI build, and only if the CARGO_MAKE_RUN_CODECOV environment variable is defined as "true":

```toml
[tasks.ci-coverage-flow]
description = "Runs the coverage flow and uploads the results to codecov."
condition = { platforms = ["linux"], env = { "CARGO_MAKE_CI" = "true", "CARGO_MAKE_RUN_CODECOV" = "true" } }
run_task = "codecov-flow"

[tasks.codecov-flow]
description = "Runs the full coverage flow and uploads the results to codecov."
windows_alias = "empty"
dependencies = [
    "coverage-flow",
    "codecov"
]
```

The first task **ci-coverage-flow** defines the condition that checks we are on linux, running as part of a CI build and the CARGO_MAKE_RUN_CODECOV environment variable is set to "true".<br>
Only if all conditions are met, it will run the **codecov-flow** task.<br>
We can't define the condition directly on the **codecov-flow** task, as it will invoke the task dependencies before checking the condition.

<a name="usage-installing-dependencies"></a>
### Installing Dependencies

Some tasks will require third party crates, rustup components or other native tools.<br>
cargo-make provides multiple ways to setup those dependencies before running the task.

* [Cargo Plugins](#usage-installing-cargo-plugins)
* [Crates](#usage-installing-crates)
* [Rustup Components](#usage-installing-rustup-components)
* [Defining Minimal Version](#usage-installing-min-version)
* [Native Dependencies](#usage-installing-native-dependencies)
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
Only if the command is not available, it will attempt to install it by running ```cargo install cargo-<first arg>```<br>
In case the cargo plugin has a different name, you can specify it manually via **install_crate** attribute.<br>
You can specify additional installation arguments using the **install_crate_args** attribute (for example: version).

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

In this example, cargo will first test that the command ```rustfmt --help``` works well and only if fails, it will first attempt
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

<a name="usage-installing-min-version"></a>
#### Defining Minimal Version

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
Currently there are few limitations when defining min_version:

* Specifing **toolchain** in the task or **rustup_component_name** in the install_crate structure, will make cargo-make ignore the min version value.
* In case cargo-make is unable to detect the currently installed version due to any error, cargo-make will assume the version is valid and printout a warning.

<a name="usage-installing-native-dependencies"></a>
#### Native Dependencies

Native dependencies can also be installed, however it is up to the Makefile author to write the script which checks the dependency exists and if
not, to install it correctly.<br>
This is done by setting up an installation script in the **install_script** attribute of the task.<br>
It is possible to use platform overrides to specify different installation scripts for linux/mac/windows platforms.<br>
For example:

```toml
[tasks.coverage-kcov]
windows_alias = "empty"
install_script = [
'''
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
]
```

This task, checks if kcov is installed and if not, will install it and any other dependency it requires.

<a name="usage-installing-dependencies-priorities"></a>
### Installation Priorities

Only one type of installation will be invoked per task.<br>
The following defines the installation types sorted by priority for which cargo-make uses to decide which installation flow to invoke:

* **install_crate** - Enables to install crates and rustup components.
* **install_script** - Custom script which can be used to install or run anything that is needed by the task command.
* **automatic cargo plugin** - In case the command is **cargo**, cargo-make will check which cargo plugin to automatically install (if needed).

In case multiple installation types are defined (for example both install_crate and install_script) only one installation type will be invoked based on the above priority list.

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

Another way to call a task on the workspace level and not for each member, is to define that task in the workspace Makefile.toml with **workspace** set to false as follows:

```toml
[tasks.ignore-members]
workspace = false
```

Setting **workspace=false** for the task requested on the cargo-make command line is equivalent to calling it with the **--no-workspace** flag.<br>
This flag is only checked for the task on the cargo-make command line and is completely ignored for all other tasks which are executed as part of the flow.<br>
By default the workspace flag for all tasks is set to true, but that can be configured differently in the config section as follows:

```toml
[config]
default_to_workspace = false
```

In which case, workspace level support is **always** disabled unless a task defines **workspace=true**.

<a name="usage-workspace-support-skip-members"></a>
#### Skipping Specific Members

In most cases you will want to run a specific flow on all members, but in rare cases you will want to skip specific members.

By setting the **CARGO_MAKE_WORKSPACE_SKIP_MEMBERS** environment variable to hold the member names to skip (seperated by a ';' character), you can define if you want those members not to participate in the flow.

In the below example we will skip member3 and member4 (should be defined in the workspace level Makefile.toml):

```toml
[env]
CARGO_MAKE_WORKSPACE_SKIP_MEMBERS = "member3;member4"
```

You can also define glob paths, for example:

```toml
[env]
CARGO_MAKE_WORKSPACE_SKIP_MEMBERS = "tools/*"
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

<a name="usage-toochain"></a>
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
By default any error in any task that does not have ```ignore_errors=true``` set to it, will cause the entire flow to fail.<br>
However, there are scenarios in which you would like to run some sort of cleanups before the failed flow finishes.<br>
cargo make enables you to define an **on error** task which will only be invoked in case the flow failed.<br>
In order to define this special task you must add the **on_error_task** attribute in the the **config** section in your Makefile and point it to your task, for example:

```toml
[config]
on_error_task = "catch"

[tasks.catch]
script = [
    "echo \"Doing cleanups in catch\""
]
```

<a name="usage-profiles"></a>
### Profiles

Profiles are a useful tool used to define custom behaviour.<br>
In order to set the execution profile, use the **--profile** or **-p** cli argument and provide the profile name.<br>
Profile names are automatically converted to underscores and are trimmed.<br>
If no profile name is provided, the profile will be defaulted to **development**.

Example Setting Profile:

```sh
cargo make --profile production mytask
```

Profiles provide multiple capabilities:

* [Environment variables](#usage-profiles-env) overrides
* [Conditions by profiles](#usage-profiles-conditions), for example: ```condition = { profiles = ["development", "production"] }```
* [New environment variable](#usage-env-global) **CARGO_MAKE_PROFILE** which holds the profile name and can be used by conditions, scripts and commands.

Additional profiles can be set in the config section but have limited support.

```toml
[config]
additional_profiles = ["second_profile", "another_profile"]
```

Additional profiles can be used to define additional environment blocks and they will be defined in a new environment variable **CARGO_MAKE_ADDITIONAL_PROFILES**

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
DEV = "TRUE"

[env.production]
PROD = "TRUE"
```

Example:

We have the following makefile with 2 profile based env maps

```toml
[env]
COMMON = "COMMON"
PROFILE_NAME = "${CARGO_MAKE_PROFILE}"

[env.development]
IS_DEV = "TRUE"
IS_PROD = "FALSE"

[env.production]
IS_DEV = "FALSE"
IS_PROD = "TRUE"

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
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: profile.toml
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

Additional profiles defined in the config section will also result in additional env blocks to be loaded, for example:

```toml
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
* **ci-static-code-analysis-tasks** - Will enable all static code analysis tasks such as format checking and clippy as part of the CI flow (see special note about backward compatibility below).
* **ci-all-build-tasks** - Will enable all extra compilation tasks (i.e. bench and example code) as part of the CI flow (see special note about backward compatibility below).

*Some of these profiles may change in the future to enable more tasks which may break your build and by definition will never be backward compatible.*<br>
*Use them with care.*

<a name="usage-private-tasks"></a>
### Private Tasks

Private tasks are tasks that should only be invoked by other tasks and not directly from the cli.

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
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: deprecated.toml
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
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: ./examples/watch.toml
[cargo-make] INFO - Task: watch-example
[cargo-make] INFO - Setting Up Env.
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: watch-example
[cargo-make] INFO - Running Task: watch-example-watch
[cargo-make] INFO - Execute Command: "cargo" "watch" "-q" "-x" "make --disable-check-for-updates --no-on-error --loglevel=info --makefile=/projects/rust/cargo-make/examples/watch.toml watch-example"
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: /projects/rust/cargo-make/examples/watch.toml
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
watch = { postpone = true, no_git_ignore = true, ignore_pattern = "examples/files/*" }
```

<a name="usage-functions"></a>
### Functions

cargo-make comes with built in functions which help extend capabilities missing with environment variables.<br>
Functions are not supported everywhere in the makefile and are currently only supported in command arguments array structure.<br>
In order to define a function call, the following format is used ```@@FUNCTION_NAME(ARG1,ARG2,ARG3,...)```<br>
For example:

```toml
[tasks.split-example]
command = "echo"
args = ["@@split(ENV_VAR,|)"]
```

Currently Supported Functions:

* [Split](#usage-functions-split)
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
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: functions.toml
[cargo-make] INFO - Task: split
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: split
[cargo-make] INFO - Execute Command: "echo" "1" "2" "3" "4"
1 2 3 4
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.

> cargo make --cwd ./examples --makefile functions.toml no-split
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: functions.toml
[cargo-make] INFO - Task: no-split
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: no-split
[cargo-make] INFO - Execute Command: "echo" "1 2 3 4"
1 2 3 4
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.
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
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: functions.toml
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
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: functions.toml
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
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: functions.toml
[cargo-make] INFO - Task: trim-start
[cargo-make] INFO - Profile: development
[cargo-make] INFO - Running Task: init
[cargo-make] INFO - Running Task: trim-start
[cargo-make] INFO - Execute Command: "echo" "123    "
123
[cargo-make] INFO - Running Task: end
[cargo-make] INFO - Build Done  in 0 seconds.

> cargo make --cwd ./examples --makefile functions.toml trim-end
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: functions.toml
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

We check the CARGO_MAKE_PROFILE environment variable value and look for it in the mappings.<br>
If the value is **development** it will be mapped to **dev** while **ci** is mapped to **test**.<br>
In case no mapping is found, the original value is returned.<br>
Sample run for a mapping that was found:

```console
cargo make --cwd ./examples --makefile functions.toml -e DECODE_ENV_VAR=development decode
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: functions.toml
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
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: functions.toml
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
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: functions.toml
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
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: functions.toml
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

<a name="usage-ci-travis"></a>
#### Travis
Add the following to .travis.yml file:

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

You can see full yaml file at: [.travis.yml](https://github.com/sagiegurari/rust_info/blob/master/.travis.yml)

*NOTE: If you are using kcov coverage, you can cache the kcov installation by setting the CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY environment variable to a location which is cached by travis.*

When working with workspaces, in order to run the ci-flow for each member and package all coverage data, use the following command:

```yaml
script:
  - cargo install --debug cargo-make
  - cargo make --no-workspace workspace-ci-flow
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

*NOTE: If you are using kcov coverage, you can cache the kcov installation by setting the CARGO_MAKE_KCOV_INSTALLATION_DIRECTORY environment variable to a location which is cached by CircleCI.*

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

<a name="usage-predefined-flows"></a>
### Predefined Flows
The [default Makefile.toml](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/Makefile.stable.toml) file comes with many predefined tasks and flows.<br>
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
Switching between them without modifying the flows is done by setting the coverage provider name in the **CARGO_MAKE_COVERAGE_PROVIDER** environment variable as follows:

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

<a name="usage-predefined-flows-full"></a>
#### Full List

Full list of all predefined tasks (generated via ```cargo make --list-all-steps```)

##### Build

* **build** - Runs the rust compiler. 
* **build-flow** - Full sanity testing flow. 
* **build-release** - Runs release build. 
* **build-verbose** - Runs the rust compiler with verbose output. 
* **end-build-flow** - No Description. 
* **init-build-flow** - No Description. 
* **post-build** - No Description. 
* **pre-build** - No Description. 

##### CI

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
* **unused-dependencies** - Checks for unused dependencies. 
* **unused-dependencies-flow** - Checks for unused dependencies. 
* **verify-project** - Runs verify-project cargo plugin. 
* **workspace-ci-flow** - CI task will run CI flow for each member and merge coverage reports 
* **workspace-members-ci** - Runs the ci-flow for every workspace member. 

##### Cleanup

* **clean** - Runs the cargo clean command. 
* **delete-lock** - Deletes the Cargo.lock file. 
* **post-clean** - No Description. 
* **pre-clean** - No Description. 

##### Development

* **default** - Development testing flow will first format the code, and than run cargo build and test 
* **dev-test-flow** - Development testing flow will first format the code, and than run cargo build and test 
* **format** - Runs the cargo rustfmt plugin. 
* **format-flow** - Runs the cargo rustfmt plugin as part of a flow. 
* **post-format** - No Description. 
* **pre-format** - No Description. 
* **upgrade-dependencies** - Rebuilds the crate with most updated dependencies. 
* **watch-flow** - Watches for any file change and if any change is detected, it will invoke the default flow. 

##### Documentation

* **clean-apidocs** - Delete API docs. 
* **copy-apidocs** - Copies the generated documentation to the docs/api directory. 
* **docs** - Generate rust documentation. 
* **post-docs** - No Description. 
* **pre-docs** - No Description. 

##### Git

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

##### Hooks

* **end** - By default this task is invoked at the end of every cargo-make run. 
* **init** - By default this task is invoked at the start of every cargo-make run. 

##### Publish

* **bintray-upload** - Uploads the binary artifact from the cargo package/publish output to bintray. 
* **build-publish-flow** - Runs full sanity, generates github release and publishes the crate. 
* **github-publish** - Creates a new github release. 
* **github-publish-curl** - Creates a new github release using curl. 
* **github-publish-custom-name** - Creates a new github release. 
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

##### Test

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
* **test-flow** - Runs pre/post hooks and cargo test. 
* **test-verbose** - Runs all available tests with verbose output. 
* **test-with-args** - Runs cargo test with command line arguments. 
* **workspace-coverage** - Runs coverage task for all members and packages all of them (by default the codecov flow). 
* **workspace-coverage-pack** - Runs codecov script to upload coverage results to codecov. 
* **workspace-members-coverage** - Runs the ci-flow for every workspace member. 

##### Tools

* **diff-files** - Run diff on two provided files. 
* **do-on-members** - Runs the requested task for every workspace member. 
* **empty** - Empty Task 
* **git-diff-files** - Run diff on two provided files. 
* **install-rls** - No Description. 
* **install-rust-src** - No Description. 

<a name="usage-predefined-flows-disable"></a>
#### Disabling Predefined Tasks/Flows

In order to prevent loading of internal core tasks and flows, simply add the following configuration property in your external Makefile.toml:

```toml
[config]
skip_core_tasks = true
```

<a name="usage-predefined-flows-modify"></a>
#### Modifing Predefined Tasks/Flows

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
min_version = "{{ site.version }}"
```

<a name="usage-diff-changes"></a>
### Diff Changes
Using the **--diff-steps** cli command flag, you can diff your correct overrides compared to the prebuilt internal makefile flow.

Example Usage:

```console
cargo make --diff-steps --makefile ./examples/override_core.toml post-build
[cargo-make] INFO - cargo make {{ site.version }}
[cargo-make] INFO - Using Build File: ./examples/override_core.toml
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
+                "Overide description"
+            ),
             category: Some(
                 "Build"
             ),
[cargo-make] INFO - Done
```

*Git is required to be available as it is used to diff the structures and output it to the console using standard git coloring scheme.*

<a name="usage-cli"></a>
### Cli Options
These are the following options available while running cargo-make:

```console
USAGE:
    cargo make [FLAGS] [OPTIONS] [--] [ARGS]
    OR
    makers [FLAGS] [OPTIONS] [--] [ARGS]

FLAGS:
        --allow-private                Allow invocation of private tasks
        --diff-steps                   Runs diff between custom flow and prebuilt flow (requires git)
        --disable-check-for-updates    Disables the update check during startup
        --experimental                 Allows access unsupported experimental predefined tasks.
    -h, --help                         Prints help information
        --list-all-steps               Lists all known steps
        --no-color                     Disables colorful output
        --no-on-error                  Disable on error flow even if defined in config sections
        --no-workspace                 Disable workspace support (tasks are triggered on workspace and not on members)
        --print-steps                  Only prints the steps of the build in the order they will be invoked but without invoking them
        --skip-init-end-tasks          If set, init and end tasks are skipped
    -v, --verbose                      Sets the log level to verbose (shorthand for --loglevel verbose)
    -V, --version                      Prints version information

OPTIONS:
        --cwd <DIRECTORY>                  Will set the current working directory. The search for the makefile will be
                                           from this directory if defined.
    -e, --env <ENV>...                     Set environment variables
        --env-file <FILE>                  Set environment variables from provided file
    -l, --loglevel <LOG LEVEL>             The log level [default: info]  [possible values: verbose, info, error]
        --makefile <FILE>                  The optional toml file containing the tasks definitions [default: Makefile.toml]
        --output-format <OUTPUT FORMAT>    The print/list steps format (some operations do not support all formats) [default: default]  [possible values: default, short-description, markdown]
    -p, --profile <PROFILE>                The profile name (will be converted to lower case) [default: development]
    -t, --task <TASK>                      The task name to execute (can omit the flag if the task name is the last
                                           argument) [default: default]

ARGS:
    <TASK>            The task name to execute
    <TASK_ARGS>...    Task arguments which can be accessed in the task itself.
```

<a name="cargo-make-global-config"></a>
### Global Configuration
Some of the default CLI values and cargo-make behaviour can be configured via optional global configuration file config.toml located in the cargo-make directory.

The cargo-make directory location can be defined via CARGO_MAKE_HOME environment variable value.<br>
If CARGO_MAKE_HOME has not been defined, the cargo-make default location is:

| OS      | Location                          |
| ------- | --------------------------------- |
| Linux   | $XDG_CONFIG_HOME or $HOME/.config |
| Windows | RoamingAppData                    |
| Mac     | $HOME/Library/Preferences         |

If for any reason, the above paths are not valid for the given platform, it will default to $HOME/.cargo-make

The following example config.toml shows all possible options with their default values:

```toml
# The default log level if not defined by the --loglevel cli argument
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

The [default Makefile.toml](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/Makefile.stable.toml) file comes with several types of tasks:

* Single command or script task (for example ```cargo build```)
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

In the [default Makefile.toml](https://github.com/sagiegurari/cargo-make/blob/master/src/lib/Makefile.stable.toml), all pre/post tasks are empty and are there as placeholders
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

Tasks which only install some dependency but do not invoke any command start with the **install-** prefix, for example:

```toml
[tasks.install-rust-src]
install_crate = { rustup_component_name = "rust-src" }
```

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
* [Toolchain](#usage-toochain)
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
