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
    /// Во входной строке открыта кавычка, но она не была закрыта.
    UnterminatedQuote(String),

    /// Сразу после закрывающей кавычки найден символ, отличный от пробельного.
    UnexpectedCharAfterQuote(char),

    /// Командная строка не содержит ни одного токена.
    EmptyCommandLine,

    /// Аргумент объявлен как `--name`, но для него не указано значение.
    MissingArgumentValue(String),

    /// Один и тот же аргумент `--name` встречается более одного раза.
    DuplicateArgument(String),

    /// После команды найден токен, который не начинается с `--`.
    InvalidArgument(String),
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
                write!(f, "Missing value for argument: '{argument}'")
            }
            Self::DuplicateArgument(argument) => { write!(f, "Duplicate argument: '{argument}'") }
            Self::InvalidArgument(token) =>
                write!(f, "Invalid argument: '{token}'. Arguments must start with '--'."),
        }
    }
}

impl std::error::Error for CmdLineError {}

#[derive(Debug, PartialEq, Eq)]
pub enum CommandError {
    CommandNotFound(String),
    ArgumentNotFound(String),
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
