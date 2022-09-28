
mod shells;

use std::collections::HashMap;
use std::process;

use shells::Shell;
use shells::Zsh;
use shells::Fish;
use shells::Bash;

use clap::Parser;

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

    match &args.shell[..] {
        "zsh" => print_sorted(Zsh, args.reverse),
        "fish" => print_sorted(Fish, args.reverse),
        "bash" => print_sorted(Bash, args.reverse),
        _ => {
            eprintln!("Could not find the given shell");
            process::exit(1);
        },
    }

}

fn print_sorted<T: Shell>(shell: T, reverse: bool) {
    let mut command_freq = HashMap::new();

    let commands = shell.get_commands_ran();

    for command in commands {
        let value = command_freq.entry(command).or_insert(0);
        *value += 1;
    }

    let mut fin: Vec<(&String, &i32)> = command_freq.iter().collect();
    fin.sort_by(|a, b| b.1.cmp(a.1));

    if reverse {
        for a in fin.into_iter().rev() {
            println!("{} => {}", a.0, a.1);
        }
    } else {
        for a in fin {
            println!("{} => {}", a.0, a.1);
        }
    }
}
