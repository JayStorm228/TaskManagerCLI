use crate::errors::CommandError;

#[test]
fn test_command_not_found_display() {
    let err: CommandError = CommandError::CommandNotFound("add");
    assert_eq!(err.to_string(), "Invalid command: add");
}

#[test]
fn test_wrong_argument_display() {
    let err: CommandError = CommandError::WrongArgument("invalid-id");
    assert_eq!(err.to_string(), "Invalid arguments: invalid-id");
}

#[test]
fn test_command_error_debug() {
    let err1: CommandError = CommandError::CommandNotFound("done");
    let err2: CommandError = CommandError::WrongArgument("bad");

    assert_eq!(format!("{:?}", err1), "CommandNotFound(\"done\")");
    assert_eq!(format!("{:?}", err2), "WrongArgument(\"bad\")");
}

#[test]
fn test_command_error_partial_eq() {
    let err1: CommandError = CommandError::CommandNotFound("list");
    let err2: CommandError = CommandError::CommandNotFound("list");
    let err3: CommandError = CommandError::CommandNotFound("remove");

    assert_eq!(err1, err2);
    assert_ne!(err1, err3);

    let err4: CommandError = CommandError::WrongArgument("x");
    let err5: CommandError = CommandError::WrongArgument("x");
    let err6: CommandError = CommandError::WrongArgument("y");

    assert_eq!(err4, err5);
    assert_ne!(err4, err6);

    assert_ne!(
        CommandError::CommandNotFound("cmd"),
        CommandError::WrongArgument("arg")
    );
}

#[test]
fn test_command_error_is_error() {
    let err: CommandError = CommandError::CommandNotFound("test");
    assert!(std::error::Error::source(&err).is_none());
}
