mod cli;
mod command;
mod errors;
mod task;
mod taskmanager;
use std::io;

use crate::command::Command;

fn main() -> io::Result<()> {
    let mut interface = cli::Cli::new();
    loop {
        let input = match command::parse_input() {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        let command = match Command::parse(&input) {
            Ok(value) => value,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        if let Err(e) = interface.execute(command) {
            eprintln!("{e}");
            continue;
        }
    }
}

#[cfg(test)]
mod tests {
    pub mod cli_tests;
    pub mod command_error;
    pub mod command_parse;
    pub mod task_error;
    pub mod taskmanager_tests;
    pub mod test_priority;
}
