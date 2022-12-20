# VTInstall

This is a tool for creating install scripts for your custom linux distro.


## Command Line Arguments

### `--resume` (not yet implemented)
resumes a previously started install. 

### `--restart` (not yet implemented)
restarts the install at the beginning (not recommended). (Overrides `--resume`)

### `--dry-run`
performs a dry run of the install. Nothing will be installed or changed.

### `--compile`
compiles the install script to a shell script. (Overrides `--resume` and `--restart`)

### `--import-path <path> [<path> ... ]` (not yet implemented)
Adds `<path>` to the paths to be searched when importing. The paths will be searched in the order they are given.

### `--verify` (not yet implemented)
Only verifies the install script and all its dependencies. Nothing will be installed or changed. (Overrides `--resume`, `--restart`, `--compile`)

## Install scripts

### Comments
Comments are marked with a `#`


### `IMPORT <filepath>` (not yet implemented)
imports a command, to be usable after the instruction. It is searched for commands in the following order:
1. local folder
2. local subfolder `cmd`
3. paths provided py `--import-path` in the order they are given
4. global commands folder

Multiple definitions of commands with the same name in the same folder are not allowed.

### `ARG <arg-name>[=<default-value>]` (not yet implemented)
Creates an argument. All arguments without a default value need to be passed when running the script. The argument is only valid after the instruction. It can be used with `$<arg-name>`.


### `RUN <cmd> <args>`

Runs shell command with provided arguments.

### `DEPENDS [--arg <name>=<value>] <path>` (not yet implemented)

Executes the install script at the provided path. Each dependency is only run once in the installation. Arguments can be passed with `--arg`


### `COPY <source> <target>` (not yet implemented)

Copies folder or file from the source path to target path. If necessary, parent folders at the target are automatically created.


## Commands (not yet implemented)
Commands are structured as a JSON file. They can be located in the global commands folder, the same folder as the install script that uses it, in a subfolder of that folder called `cmd` or in any path that is provided with `--import-path`.
For the search order see the `IMPORT` instruction.

### Example Instruction file:
```json
{
    "name": "test",
    "execute": ["test.sh arg1 arg2 $@"],
    "dry-run": "test_dry.sh"
}
```
There are the following tags:


### `name`
The name of the command


### `execute`
Either a single shell command or an array of shell commands. When an array is defined they will be executed after another. Each step is only executed, if all of the previous steps are executed successfully. Similar to bash to pass one argument to a script `$1`, `$2`, ... can be used. To pass all arguments `$@` and `$*` are used.

### `dry-run` (optional)

Similar to the execute, this defines the shell commands defines the steps when a dry-run is performed. In the dry-run, it is not allowed to add, remove or modify any files or install any program.

### `before-all` (optional)

shell command to execute before the first time this command is used. This is useful for example for updated the repository before the first installation.




# Building

## 1. Install Cargo
install cargo on your system.
## 2. Cd into vtinstall
```console
cd vtinstall
```

## 3. Build
Run the following command:
```console
cargo build --release
```
The binary will be located at the following path: `target/release/vtinstall`