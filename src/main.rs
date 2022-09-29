mod shells;

use std::collections::HashMap;
use std::process;

use shells::Bash;
use shells::Fish;
use shells::Shell;
use shells::Zsh;

use clap::Parser;

/// A Struct for command line arguments parsing
#[derive(Debug, Parser)]
#[clap(about, author, version)]
struct Cli {
    /// The shell to check commands frequency
    /// i.e bash, fish or zsh
    shell: String,

    /// Reverse the sorting
    #[clap(short, long)]
    reverse: bool,
}

fn main() {
    let args = Cli::parse();

    // Matching the supplied shell after argument parsing
    match &args.shell[..] {
        "zsh" => print_sorted(Zsh, args.reverse),
        "fish" => print_sorted(Fish, args.reverse),
        "bash" => print_sorted(Bash, args.reverse),
        _ => {
            eprintln!("Could not find the given shell");
            process::exit(1);
        }
    }
}

/// A Function thats prints out the commands and their corresponding frequencies,
/// reverses the sorting if the reverse argument is true
fn print_sorted<T: Shell>(shell: T, reverse: bool) {
    // A Hashmap to store command and their corresponding frequency
    let mut command_freq_pair = HashMap::new();

    // Getting the commands ever ran for the passed shell
    let commands = shell.get_commands_ran();

    // Performing a count of every command and recording it's frequency
    for command in commands {
        let value = command_freq_pair.entry(command).or_insert(0);
        *value += 1;
    }

    // Converting the command_freq_pair into a vec so that it can be sorted
    let mut command_freq_pair: Vec<(&String, &i32)> = command_freq_pair.iter().collect();
    command_freq_pair.sort_by(|a, b| b.1.cmp(a.1));

    // Printing the results, checking if reversed is supplied, so that the results can be reversed
    if reverse {
        for a in command_freq_pair.into_iter().rev() {
            println!("{} => {}", a.0, a.1);
        }
    } else {
        for a in command_freq_pair {
            println!("{} => {}", a.0, a.1);
        }
    }
}
