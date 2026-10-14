use crate::errors::CommandError;

#[test]
fn command_not_found_display() {
    let error = CommandError::CommandNotFound("unknown".to_string());
    assert_eq!(
        error.to_string(),
        "Invalid command: unknown.\nTry typing \"help\" for available commands"
    );
}

#[test]
fn required_argument_not_found_display() {
    let error = CommandError::ArgumentNotFound("--title".to_string());
    assert_eq!(error.to_string(), "Could not find required argument: --title");
}

#[test]
fn invalid_argument_display() {
    let message = "ID must be a non-negative integer";
    let error = CommandError::InvalidArgument(message.to_string());
    assert_eq!(error.to_string(), message);
}
