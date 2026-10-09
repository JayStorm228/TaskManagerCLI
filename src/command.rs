use crate::errors::CommandError;
use std::io::{ self, Write };

/// Represents a command that can be executed by the application.
///
/// Commands are created by parsing user input with [`Command::parse`].
/// Each variant represents an available operation, such as adding,
/// displaying, completing, or deleting a task.
#[derive(Debug, PartialEq)]
pub enum Command {
    /// Creates a new task.
    Add {
        /// The title of the task.
        title: String,

        /// The description of the task.
        description: String,

        /// The priority name supplied by the user.
        priority: String,
    },

    /// Displays detailed information about a task.
    Show {
        /// The identifier of the task to display.
        id: u32,
    },

    /// Deletes a task.
    Delete {
        /// The identifier of the task to delete.
        id: u32,
    },

    /// Marks a task as completed.
    Complete {
        /// The identifier of the task to complete.
        id: u32,
    },

    /// Displays all tasks.
    List,

    /// Displays the available commands and their arguments.
    Help,

    /// Terminates the application.
    Exit,
}

impl Command {
    /// Parses a command from a string.
    ///
    /// The input is split into whitespace-separated tokens. The first token
    /// determines the command, and the remaining tokens are interpreted
    /// according to that command's expected arguments.
    ///
    /// For the `add` command, the title is taken from the first argument,
    /// the priority from the second argument, and all remaining tokens are
    /// joined into the description. If the priority is omitted and only the title is, it defaults
    /// to `"None"`.
    ///
    /// # Arguments
    ///
    /// * `input` - The input string to parse.
    ///
    /// # Returns
    ///
    /// Returns the corresponding [`Command`] if the command name is recognized
    /// and its required arguments can be parsed.
    ///
    /// # Errors
    ///
    /// Returns [`CommandError::CommandNotFound`] if the command name is unknown,
    /// or [`CommandError::WrongArgument`] if a required numeric identifier
    /// cannot be parsed as a `u32`.
    pub fn parse<'a>(input: &'a str) -> Result<Self, CommandError<'a>> {
        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap_or("");

        match command {
            "add" => {
                let title = parts.next().unwrap_or("");
                let priority = parts.next().unwrap_or("None");
                let description = parts.collect::<Vec<_>>().join(" ");
                Ok(Self::Add {
                    title: title.to_string(),
                    description: description.to_string(),
                    priority: priority.to_string(),
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

/// Parses a task identifier from a sequence of whitespace-separated tokens.
///
/// The function reads the next token and attempts to parse it as a `u32`.
///
/// # Arguments
///
/// * `parts` - A mutable iterator over the remaining command tokens.
///
/// # Returns
///
/// Returns the parsed identifier on success.
///
/// # Errors
///
/// Returns [`CommandError::WrongArgument`] if no token is available
/// or the token cannot be parsed as a `u32`.
pub fn parse_id<'a>(parts: &mut std::str::SplitWhitespace<'a>) -> Result<u32, CommandError<'a>> {
    let id_raw = parts.next().unwrap_or("");
    match id_raw.trim().parse::<u32>() {
        Ok(value) => Ok(value),
        Err(_) => Err(CommandError::WrongArgument(id_raw)),
    }
}

/// Reads a line of user input from standard input.
///
/// Prints the `>` prompt, flushes standard output, and reads one line
/// from standard input.
///
/// # Returns
///
/// Returns the input line as a [`String`], including its trailing newline
/// if one was read.
///
/// # Errors
///
/// Returns an [`io::Error`] if flushing standard output or reading from
/// standard input fails.
pub fn parse_input() -> Result<String, io::Error> {
    print!("> ");
    io::stdout().flush()?;

    let mut command_line = String::new();
    io::stdin().read_line(&mut command_line)?;

    Ok(command_line)
}
