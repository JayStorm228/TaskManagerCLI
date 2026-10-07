use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TaskError {
    AlreadyCompleted,
    ShortTitle(u32),
    IDNotFound(u32),
}

impl fmt::Display for TaskError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::AlreadyCompleted => {
                write!(formatter, "Task is already completed")
            }
            TaskError::IDNotFound(value) => {
                write!(formatter, "Cannot find task with this ID: {}", value)
            }
            TaskError::ShortTitle(value) => {
                write!(
                    formatter,
                    "Title is too short! Must be at least {} characters",
                    value
                )
            }
        }
    }
}
impl std::error::Error for TaskError {}
#[derive(Debug, PartialEq)]
pub enum CommandError<'a> {
    CommandNotFound(&'a str),
    WrongArgument(&'a str),
}
impl<'a> fmt::Display for CommandError<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommandError::CommandNotFound(command) => {
                write!(formatter, "Invalid command: {}", command)
            }
            CommandError::WrongArgument(arg) => {
                write!(formatter, "Invalid arguments: {}", arg)
            }
        }
    }
}
impl<'a> std::error::Error for CommandError<'a> {}
