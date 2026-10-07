mod cli;
mod errors;
mod task;
mod taskmanager;
use std::io::{self, Write};

use crate::cli::Cli;
fn parse_id(parts: &mut std::str::SplitWhitespace) -> Option<u32> {
    match parts.next() {
        Some(value) => match value.trim().parse::<u32>() {
            Ok(id) => Some(id),
            Err(_) => {
                println!("Invalid ID!");
                None
            }
        },
        None => {
            println!("Empty ID!");
            None
        }
    }
}
fn main() -> io::Result<()> {
    let mut interface = cli::Cli {
        task_mgr: taskmanager::TaskManager::new(),
    };
    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut command_line = String::new();
        io::stdin().read_line(&mut command_line)?;
        let command_line = command_line.trim();

        let mut parts = command_line.split_whitespace();
        let command = parts.next().unwrap_or("");
        match command {
            "add" => {
                let title = match parts.next() {
                    Some(title) => {
                        if title.len() < 3 {
                            println!("Title must be at least 3 characters long!");
                            continue;
                        } else {
                            String::from(title)
                        }
                    }
                    None => {
                        println!("Title cannot be empty!");
                        continue;
                    }
                };
                let description = parts.collect::<Vec<_>>().join(" ");

                if let Err(e) = interface.new_task(title, description) {
                    eprintln!("{e}");
                }
            }
            "list" => {
                interface.list_tasks();
            }
            "show" => {
                if let Some(id) = parse_id(&mut parts) {
                    if let Err(e) = interface.show_task(id) {
                        eprintln!("{e}")
                    };
                }
            }
            "delete" => {
                if let Some(id) = parse_id(&mut parts) {
                    if let Err(e) = interface.delete_task(id) {
                        eprintln!("{e}")
                    }
                }
            }
            "help" => {
                Cli::help();
            }
            "complete" => {
                if let Some(id) = parse_id(&mut parts) {
                    if let Err(e) = interface.complete_task(id) {
                        eprintln!("{e}")
                    }
                }
            }
            "exit" => break Ok(()),
            _ => {
                println!("Wrong command!")
            }
        }
    }
}
