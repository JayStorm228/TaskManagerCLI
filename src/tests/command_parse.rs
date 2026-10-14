use crate::command::Command;
use crate::command_line::CmdLine;
use crate::errors::CommandError;

fn parse_command(input: &str) -> Result<Command, CommandError> {
    let mut line = CmdLine::new();
    line.set_input_for_test(input);
    line.parse().expect("The command-line syntax must be valid for these tests");
    Command::parse(line)
}

fn parsed(input: &str) -> Command {
    match parse_command(input) {
        Ok(command) => command,
        Err(error) => panic!("Unexpected command error: {error}"),
    }
}

fn parse_error(input: &str) -> CommandError {
    match parse_command(input) {
        Ok(command) => panic!("Expected an error, got: {command:?}"),
        Err(error) => error,
    }
}

#[test]
fn parses_add_with_all_arguments() {
    assert_eq!(
        parsed(r#"add --title "Buy milk" --priority high --description "Two liters""#),
        Command::Add {
            title: "Buy milk".to_string(),
            description: "Two liters".to_string(),
            priority: "high".to_string(),
        }
    );
}

#[test]
fn parses_add_independently_of_argument_order() {
    assert_eq!(
        parsed(r#"add --description "Two liters" --priority low --title "Buy milk""#),
        Command::Add {
            title: "Buy milk".to_string(),
            description: "Two liters".to_string(),
            priority: "low".to_string(),
        }
    );
}

#[test]
fn add_defaults_to_empty_description_and_none_priority() {
    assert_eq!(parsed("add --title Report"), Command::Add {
        title: "Report".to_string(),
        description: String::new(),
        priority: "none".to_string(),
    });
}

#[test]
fn add_preserves_priority_for_taskmanager_validation() {
    assert_eq!(parsed("add --title Report --priority urgent"), Command::Add {
        title: "Report".to_string(),
        description: String::new(),
        priority: "urgent".to_string(),
    });
}

#[test]
fn add_requires_title() {
    match parse_error("add --priority high") {
        CommandError::ArgumentNotFound(arg) => assert_eq!(arg, "--title"),
        other => panic!("Unexpected error: {other}"),
    }
}

#[test]
fn add_keeps_empty_explicit_description() {
    assert_eq!(parsed(r#"add --title Report --description """#), Command::Add {
        title: "Report".to_string(),
        description: String::new(),
        priority: "none".to_string(),
    });
}

#[test]
fn add_supports_escaped_quotes_in_title() {
    assert_eq!(parsed(r#"add --title "Read \"Rust Book\"""#), Command::Add {
        title: "Read \"Rust Book\"".to_string(),
        description: String::new(),
        priority: "none".to_string(),
    });
}

#[test]
fn parses_show_delete_and_complete() {
    assert_eq!(parsed("show --id 42"), Command::Show { id: 42 });
    assert_eq!(parsed("delete --id 7"), Command::Delete { id: 7 });
    assert_eq!(parsed("complete --id 10"), Command::Complete { id: 10 });
}

#[test]
fn parses_zero_and_max_u32_ids() {
    assert_eq!(parsed("show --id 0"), Command::Show { id: 0 });
    assert_eq!(parsed(&format!("delete --id {}", u32::MAX)), Command::Delete { id: u32::MAX });
}

#[test]
fn parses_id_with_surrounding_whitespace() {
    assert_eq!(parsed(r#"show --id " 42 ""#), Command::Show { id: 42 });
}

#[test]
fn id_commands_require_id_argument() {
    for input in ["show", "delete", "complete"] {
        match parse_error(input) {
            CommandError::ArgumentNotFound(arg) => assert_eq!(arg, "--id"),
            other => panic!("Unexpected error for {input}: {other}"),
        }
    }
}

#[test]
fn id_commands_reject_non_numeric_id() {
    for input in ["show --id abc", "delete --id xyz", "complete --id nope"] {
        match parse_error(input) {
            CommandError::InvalidArgument(message) => {
                assert!(message.contains("Could not parse given ID"));
                assert!(message.contains("ID must be an unsigned int"));
            }
            other => panic!("Unexpected error for {input}: {other}"),
        }
    }
}

#[test]
fn rejects_negative_id() {
    match parse_error("show --id -1") {
        CommandError::InvalidArgument(message) => assert!(message.contains("-1")),
        other => panic!("Unexpected error: {other}"),
    }
}

#[test]
fn rejects_id_larger_than_u32_max() {
    let input = format!("show --id {}", u64::from(u32::MAX) + 1);
    match parse_error(&input) {
        CommandError::InvalidArgument(message) => {
            assert!(message.contains("Could not parse given ID"));
        }
        other => panic!("Unexpected error: {other}"),
    }
}

#[test]
fn parses_commands_without_arguments() {
    assert_eq!(parsed("list"), Command::List);
    assert_eq!(parsed("help"), Command::Help);
    assert_eq!(parsed("exit"), Command::Exit);
}

#[test]
fn rejects_unknown_command() {
    match parse_error("foobar") {
        CommandError::CommandNotFound(command) => assert_eq!(command, "foobar"),
        other => panic!("Unexpected error: {other}"),
    }
}

#[test]
fn command_names_are_case_sensitive() {
    match parse_error("ADD --title Report") {
        CommandError::CommandNotFound(command) => assert_eq!(command, "ADD"),
        other => panic!("Unexpected error: {other}"),
    }
}

#[test]
fn tolerates_whitespace_around_command() {
    assert_eq!(parsed("  show   --id   5  "), Command::Show { id: 5 });
}
