use std::fmt;
/// Represents errors that can occur while managing tasks.
///
/// These errors include attempts to complete an already completed task,
/// invalid task titles, missing task identifiers, and invalid priority names.
#[derive(Debug, PartialEq)]
pub enum TaskError {
    /// The task has already been completed.
    AlreadyCompleted,

    /// The task title is shorter than the minimum allowed length.
    ///
    /// The contained value specifies the minimum required number of characters.
    ShortTitle(u32),

    /// No task with the specified identifier exists.
    ///
    /// The contained value is the requested task identifier.
    IDNotFound(u32),

    /// The supplied priority string could not be parsed.
    ///
    /// The contained value is the original input string.
    ParseTaskPriorityError(String),
}

impl fmt::Display for TaskError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::AlreadyCompleted => { write!(formatter, "Task is already completed") }
            TaskError::IDNotFound(value) => {
                write!(formatter, "Cannot find task with this ID: {}", value)
            }
            TaskError::ShortTitle(value) => {
                write!(formatter, "Title is too short! Must be at least {} characters", value)
            }
            TaskError::ParseTaskPriorityError(value) => {
                write!(
                    formatter,
                    "Error parsing task priority: {}\n\
                    Consider uning \"none\" if task has no priority",
                    value
                )
            }
        }
    }
}
impl std::error::Error for TaskError {}

/// Represents errors encountered while parsing a command.
///
/// The error retains a reference to the input token that caused the failure,
/// rather than allocating a new string.
///
/// # Lifetime
///
/// The lifetime `'a` is tied to the input string from which the token
/// was obtained.
#[derive(Debug, PartialEq)]
pub enum CommandError<'a> {
    /// The supplied command name is not recognized.
    ///
    /// The contained string slice refers to the unrecognized command token.
    CommandNotFound(&'a str),

    /// A command argument is missing or has an invalid format.
    ///
    /// The contained string slice refers to the argument that could not
    /// be parsed.
    WrongArgument(&'a str),
}
impl<'a> fmt::Display for CommandError<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::CommandNotFound(command) => {
                write!(formatter, "Invalid command: {}", command)
            }
            CommandError::WrongArgument(arg) => { write!(formatter, "Invalid arguments: {}", arg) }
        }
    }
}
impl<'a> std::error::Error for CommandError<'a> {}
