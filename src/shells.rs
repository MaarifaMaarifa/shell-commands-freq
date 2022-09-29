//! # The Shells Module
//! 
//! The Module containing the common shells(i.e Bash, Fish, Zsh) and their various implementations

use std::env;
use std::fs;
use std::process;

/* Default paths for shells' history files
 The paths have the term USER for the user, this term will be  
 replaced according the user running the program */
const ZSH_HISTORY_PATH: &str = "/home/USER/.zsh_history";
const BASH_HISTORY_PATH: &str = "/home/USER/.bash_history";
const FISH_HISTORY_PATH: &str = "/home/USER/.local/share/fish/fish_history";

/// A trait to be implemented by shells
pub trait Shell {
    /// A function that returns a vector of commands ever ran by a shell as Strings.
    /// This makes it easier for every shell to implement the way it parses it's history file,
    /// as different shells record their history in their own ways.
    fn get_commands_ran(&self) -> Vec<String>;
}

/// A Function that returns the contents of history file of a shell through the provided default path
fn get_shell_history_file_contents(default_path: &str) -> String {
    /* getting the actual path of the shell history file
     Since their paths include the user name, the trick is done by checking
    the user through environment variable and replacing the word USER with
    the actual username*/
    let history_file_path = default_path.to_owned().replace(
        "USER",
        &env::var("USER").unwrap_or_else(|err| {
            eprintln!("Could not determine the user: {}", err);
            process::exit(1);
        }),
    );

    // Reading the history file and returning its contents
    fs::read_to_string(history_file_path).unwrap_or_else(|err| {
        eprintln!("Could not load the zsh history file: {}", err);
        process::exit(1);
    })
}

/// A struct for the Zsh shell
pub struct Zsh;

impl Shell for Zsh {
    fn get_commands_ran(&self) -> Vec<String> {
        let file_contents = get_shell_history_file_contents(ZSH_HISTORY_PATH);

        let mut commands = Vec::new();

        for line in file_contents.lines() {
            if let Some(semicolon_pos) = line.find(';') {
                if let Some(space_pos) = line[semicolon_pos..].find(' ') {
                    commands
                        .push((&line[(semicolon_pos + 1)..(semicolon_pos + space_pos)]).to_owned())
                }
            };
        }
        commands
    }
}

/// A struct for the Bash shell
pub struct Bash;

impl Shell for Bash {
    fn get_commands_ran(&self) -> Vec<String> {
        let file_contents = get_shell_history_file_contents(BASH_HISTORY_PATH);

        let mut commands = Vec::new();

        for line in file_contents.lines() {
            if let Some(space_pos) = line.find(' ') {
                commands.push((&line[..space_pos]).to_owned())
            };
        }
        commands
    }
}

/// A struct for the Fish shell
pub struct Fish;

impl Shell for Fish {
    fn get_commands_ran(&self) -> Vec<String> {
        let file_contents = get_shell_history_file_contents(FISH_HISTORY_PATH);

        let mut commands = Vec::new();

        // The header in fish history line
        let header = "cmd: ";
        let header_len = header.len();

        for line in file_contents.lines() {
            if let Some(header_pos) = line.find(header) {
                if let Some(space_pos) = line[(header_pos + header_len)..].find(' ') {
                    commands.push(
                        (&line[(header_pos + header_len)..(space_pos + header_pos + header_len)])
                            .to_owned(),
                    )
                }
            };
        }
        commands
    }
}
