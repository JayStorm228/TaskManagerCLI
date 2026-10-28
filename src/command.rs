use crate::{ command_line::CmdLine, errors::CommandError };

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
    /// Parses a command from the provided [`CmdLine`].
    ///
    /// Converts the command name and its arguments into a corresponding
    /// [`Command`] variant.
    ///
    /// Optional arguments are assigned default values when omitted:
    /// - `--description`: an empty string.
    /// - `--priority`: `"none"`.
    ///
    /// # Errors
    ///
    /// Returns:
    /// - [`CommandError::CommandNotFound`] if the command is unknown.
    /// - [`CommandError::ArgumentNotFound`] if a required argument is missing.
    /// - [`CommandError::InvalidArgument`] if a task ID cannot be parsed as a [`u32`].
    pub fn parse(cmdline: CmdLine) -> Result<Self, CommandError> {
        let args = cmdline.get_args();
        let cmd = match args.get("command") {
            Some(cmd) => cmd.clone(),
            None => "".to_string(),
        };

        match cmd.as_str() {
            "add" => {
                let title = match args.get("--title") {
                    Some(title) => title.clone(),
                    None => {
                        return Err(CommandError::ArgumentNotFound("--title".to_string()));
                    }
                };
                let description = match args.get("--description") {
                    Some(desc) => desc.clone(),
                    None => "".to_string(),
                };
                let priority = match args.get("--priority") {
                    Some(p) => { p.clone() }
                    None => "none".to_string(),
                };
                Ok(Command::Add { title, description, priority })
            }
            "show" => {
                let id = match args.get("--id") {
                    Some(id) => id.clone(),
                    None => {
                        return Err(CommandError::ArgumentNotFound("--id".to_string()));
                    }
                };
                let id = parse_id(id)?;
                Ok(Command::Show { id })
            }
            "delete" => {
                let id = match args.get("--id") {
                    Some(id) => id.clone(),
                    None => {
                        return Err(CommandError::ArgumentNotFound("--id".to_string()));
                    }
                };
                let id = parse_id(id)?;
                Ok(Command::Delete { id })
            }
            "complete" => {
                let id = match args.get("--id") {
                    Some(id) => id.clone(),
                    None => {
                        return Err(CommandError::ArgumentNotFound("--id".to_string()));
                    }
                };
                let id = parse_id(id)?;
                Ok(Command::Complete { id })
            }
            "list" => Ok(Self::List),
            "help" => Ok(Self::Help),
            "exit" => Ok(Self::Exit),
            _ => Err(CommandError::CommandNotFound(cmd.to_string())),
        }
    }
}
/// Parses a task identifier into an unsigned 32-bit integer.
///
/// Leading and trailing whitespace is ignored.
///
/// # Errors
///
/// Returns [`CommandError::InvalidArgument`] if `id` is not a valid [`u32`] value.
fn parse_id(id: String) -> Result<u32, CommandError> {
    let id = match id.trim().parse::<u32>() {
        Ok(val) => val,
        Err(_) => {
            return Err(
                CommandError::InvalidArgument(
                    format!("Could not parse given ID.\nID must be an unsigned int:{}", id)
                )
            );
        }
    };
    Ok(id)
}
