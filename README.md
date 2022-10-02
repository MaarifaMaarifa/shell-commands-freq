# shell-commands-freq

A simple program written in Rust that prints out the list of commands and their respective frequencies based on how many times entered in the specified shell.

### shells supported;

- bash

- zsh

- fish

## usage

### **Getting commands and their frequencies**

```sh
# Replace "shell" with the name of your shell
shell-commands-freq "shell"
```

### **Getting a reversed sort**

```sh
# Replace "shell" with the name of your shell
shell-commands-freq -r "shell" 
```

## Installation

#### using cargo

shell-commands-freq is available as a crate in crate.io, and can be installed as follows.

```sh
cargo install shell-commands-freq
```

#### building from source



```shell
git clone https://github.com/MaarifaMaarifa/shell-commands-freq
cd shell-commands-freq
cargo install --force --path .
```
