use crate::errors::CommandError;
use std::io::{self, Write};
#[derive(Debug, PartialEq)]
pub enum Command {
    Add { title: String, description: String },
    Show { id: u32 },
    Delete { id: u32 },
    Complete { id: u32 },
    List,
    Help,
    Exit,
}

impl Command {
    pub fn parse<'a>(input: &'a str) -> Result<Self, CommandError<'a>> {
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");

        match command {
            "add" => {
                let title = parts.next().unwrap_or("");
                let description = parts.collect::<Vec<_>>().join(" ");
                Ok(Self::Add {
                    title: title.to_string(),
                    description: description.to_string(),
                })
            }
            "show" => {
                let id = parse_id(&mut parts)?;
                Ok(Self::Show { id })
            }
            "delete" => {
                let id = parse_id(&mut parts)?;
                Ok(Self::Delete { id })
            }
            "complete" => {
                let id = parse_id(&mut parts)?;
                Ok(Self::Complete { id })
            }
            "list" => Ok(Self::List),
            "help" => Ok(Self::Help),
            "exit" => Ok(Self::Exit),
            _ => Err(CommandError::CommandNotFound(command)),
        }
    }
}

pub fn parse_id<'a>(parts: &mut std::str::SplitWhitespace<'a>) -> Result<u32, CommandError<'a>> {
    let id_raw = parts.next().unwrap_or("");
    match id_raw.trim().parse::<u32>() {
        Ok(value) => Ok(value),
        Err(_) => Err(CommandError::WrongArgument(id_raw)),
    }
}

pub fn parse_input() -> Result<String, io::Error> {
    print!("> ");
    io::stdout().flush()?;

    let mut command_line = String::new();
    io::stdin().read_line(&mut command_line)?;

    Ok(command_line)
}
