
use std::process;
use std::fs;
use std::env;

// Default paths for shells' history files
const ZSH_HISTORY_PATH: &str = "/home/USER/.zsh_history";
const BASH_HISTORY_PATH: &str = "/home/USER/.bash_history";
const FISH_HISTORY_PATH: &str = "/home/USER/.local/share/fish/fish_history";


// A trait to be implemented by shells
pub trait Shell {
    // A function that returns a vector of commands ever ran by a shell as Strings
    fn get_commands_ran(&self) -> Vec<String>;
}

pub struct Zsh;

impl Shell for Zsh {
    fn get_commands_ran(&self) -> Vec<String> {
        let history_file_path = ZSH_HISTORY_PATH.to_owned().replace("USER", &env::var("USER").unwrap_or_else(|err| {
            eprintln!("Could not determine the user: {}", err);
            process::exit(1);
        }));

        let file_contents = fs::read_to_string(history_file_path).unwrap_or_else( |err| {
            eprintln!("Could not load the zsh history file: {}", err);
            process::exit(1);
        });

        let mut commands = Vec::new();

        for line in file_contents.lines() {
            let command = if let Some(semicolon_pos) = line.find(';') {
                if let Some(space_pos) = line[semicolon_pos..].find(' ') {
                    &line[(semicolon_pos + 1)..(semicolon_pos + space_pos)]
                } else {
                    continue
                }
            } else {
                continue
            };
            commands.push(command.to_owned());
        }
        commands
    }
    
}

pub struct Bash;

impl Shell for Bash {
    fn get_commands_ran(&self) -> Vec<String> {
        let history_file_path = BASH_HISTORY_PATH.to_owned().replace("USER", &env::var("USER").unwrap_or_else(|err| {
            eprintln!("Could not determine the user: {}", err);
            process::exit(1);
        }));

        let file_contents = fs::read_to_string(history_file_path).unwrap_or_else( |err| {
            eprintln!("Could not load the bash history file: {}", err);
            process::exit(1);
        });

        let mut commands = Vec::new();

        for line in file_contents.lines() {
            let command = if let Some(space_pos) = line.find(' ') {
                &line[..space_pos]
            } else {
                continue
            };
            commands.push(command.to_owned());
        }
        commands
    }
}

pub struct Fish;

impl Shell for Fish {
    fn get_commands_ran(&self) -> Vec<String> {
        let history_file_path = FISH_HISTORY_PATH.to_owned().replace("USER", &env::var("USER").unwrap_or_else(|err| {
            eprintln!("Could not determine the user: {}", err);
            process::exit(1);
        }));
        
        let file_contents = fs::read_to_string(history_file_path).unwrap_or_else( |err| {
            eprintln!("Could not load the fish history file: {}", err);
            process::exit(1);
        });

        let mut commands = Vec::new();

        // The header in fish history line
        let header = "cmd: ";
        let header_len = header.len();

        for line in file_contents.lines() {
            let command = if let Some(header_pos) = line.find(header) {
                if let Some(space_pos) = line[(header_pos + header_len)..].find(' ') {
                    &line[(header_pos + header_len)..(space_pos + header_pos + header_len)]
                } else {
                    continue
                }
            } else {
                continue
            };
            commands.push(command.to_owned());
        }
        commands
    }
}