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
                    Consider using \"none\" if task has no priority",
                    value
                )
            }
        }
    }
}
impl std::error::Error for TaskError {}

#[derive(Debug, PartialEq, Eq)]
pub enum CmdLineError {
    /// An opening quote has no matching closing quote
    UnterminatedQuote(String),

    /// A non-whitespace character immediately follows a closing quote
    UnexpectedCharAfterQuote(char),

    /// Command line contains no tokens
    EmptyCommandLine,

    /// An argument specified as `--name` has no value
    MissingArgumentValue(String),

    /// The same argument (`--name`) specified more than once
    DuplicateArgument(String),

    /// An unexpected token follows the command and doesn't start with `--`
    InvalidArgument(String),

    /// a trailing backslash has no character to escape
    UnterminatedEscapeSequence(String),
}

impl fmt::Display for CmdLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnterminatedQuote(input) => {
                write!(f, "Unterminated quote in command line:\n{input}")
            }
            Self::UnexpectedCharAfterQuote(ch) =>
                write!(f, "After a closing quote, whitespace was expected, but found: '{ch}'"),
            Self::EmptyCommandLine =>
                write!(
                    f,
                    "Empty command line. Consider calling `CmdLine::input()` before parsing."
                ),
            Self::MissingArgumentValue(argument) => {
                write!(f, "Missing value for argument: \'{argument}\'")
            }
            Self::DuplicateArgument(argument) => { write!(f, "Duplicate argument: '{argument}'") }
            Self::InvalidArgument(token) =>
                write!(f, "Invalid argument: \'{token}\'. Arguments must start with \'--\'."),
            Self::UnterminatedEscapeSequence(input) => {
                write!(f, "Unterminated escape sequence found in command line:\n{input}")
            }
        }
    }
}

impl std::error::Error for CmdLineError {}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandError {
    /// Entered command is not supported
    CommandNotFound(String),

    /// No required arguments is provided
    ArgumentNotFound(String),

    /// Provided argument is invalid type, or cannot be used by corresponding command
    InvalidArgument(String),
}
impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CommandNotFound(cmd) => {
                write!(f, "Invalid command: {}.\nTry typing \"help\" for available commands", cmd)
            }
            Self::ArgumentNotFound(arg) => {
                write!(f, "Could not find required argument: {}", arg)
            }
            Self::InvalidArgument(msg) => { write!(f, "{}", msg) }
        }
    }
}
impl std::error::Error for CommandError {}
