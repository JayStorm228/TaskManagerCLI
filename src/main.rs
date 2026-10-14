//! A command-line task manager.
//!
//! The application allows users to create, list, inspect, complete,
//! and delete tasks. Tasks can also be assigned priorities.
//!
//! Commands are entered interactively through standard input.

#![warn(missing_docs)]

mod cli;
mod errors;
mod task;
mod taskmanager;
mod command;
use std::io;
mod command_line;

fn main() -> io::Result<()> {
    let mut interface = cli::Cli::new();
    loop {
        let mut cmdline = command_line::CmdLine::new();
        if let Err(e) = cmdline.input() {
            eprintln!("{e}");
            continue;
        }
        if let Err(e) = cmdline.parse() {
            eprintln!("{e}");
            continue;
        }
        let cmd = match command::Command::parse(cmdline) {
            Ok(cmd) => cmd,
            Err(e) => {
                eprintln!("{e}");
                continue;
            }
        };

        if let Err(e) = interface.execute(cmd) {
            eprintln!("{e}");
            continue;
        }
    }
}

#[cfg(test)]
mod tests {
    pub mod cli_tests;
    pub mod command_error;
    pub mod command_line_tests;
    pub mod command_parse;
    pub mod task_error;
    pub mod taskmanager_tests;
    pub mod test_priority;
}
