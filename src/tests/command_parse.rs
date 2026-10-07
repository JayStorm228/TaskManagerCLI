use crate::command::{Command, parse_id};
use crate::errors::CommandError;

// ===== Command::parse =====

#[test]
fn test_parse_add_command() {
    let input = "add Buy milk from store";
    let cmd = Command::parse(input).unwrap();
    assert_eq!(
        cmd,
        Command::Add {
            title: "Buy".to_string(),
            description: "milk from store".to_string(),
        }
    );
}

#[test]
fn test_parse_add_single_word_title() {
    let input = "add Test";
    let cmd = Command::parse(input).unwrap();
    assert_eq!(
        cmd,
        Command::Add {
            title: "Test".to_string(),
            description: "".to_string(),
        }
    );
}

#[test]
fn test_parse_add_no_description() {
    let input = "add Task";
    let cmd = Command::parse(input).unwrap();
    assert_eq!(
        cmd,
        Command::Add {
            title: "Task".to_string(),
            description: "".to_string(),
        }
    );
}

#[test]
fn test_parse_show_command() {
    let input = "show 42";
    let cmd = Command::parse(input).unwrap();
    assert_eq!(cmd, Command::Show { id: 42 });
}

#[test]
fn test_parse_delete_command() {
    let input = "delete 7";
    let cmd = Command::parse(input).unwrap();
    assert_eq!(cmd, Command::Delete { id: 7 });
}

#[test]
fn test_parse_complete_command() {
    let input = "complete 10";
    let cmd = Command::parse(input).unwrap();
    assert_eq!(cmd, Command::Complete { id: 10 });
}

#[test]
fn test_parse_list_command() {
    let input = "list";
    let cmd = Command::parse(input).unwrap();
    assert_eq!(cmd, Command::List);
}

#[test]
fn test_parse_help_command() {
    let input = "help";
    let cmd = Command::parse(input).unwrap();
    assert_eq!(cmd, Command::Help);
}

#[test]
fn test_parse_exit_command() {
    let input = "exit";
    let cmd = Command::parse(input).unwrap();
    assert_eq!(cmd, Command::Exit);
}

#[test]
fn test_parse_unknown_command() {
    let input = "foobar";
    let err = Command::parse(input).unwrap_err();
    assert_eq!(err, CommandError::CommandNotFound("foobar"));
}

#[test]
fn test_parse_show_invalid_id() {
    let input = "show abc";
    let err = Command::parse(input).unwrap_err();
    assert_eq!(err, CommandError::WrongArgument("abc"));
}

#[test]
fn test_parse_delete_invalid_id() {
    let input = "delete not-a-number";
    let err = Command::parse(input).unwrap_err();
    assert_eq!(err, CommandError::WrongArgument("not-a-number"));
}

#[test]
fn test_parse_complete_invalid_id() {
    let input = "complete xyz";
    let err = Command::parse(input).unwrap_err();
    assert_eq!(err, CommandError::WrongArgument("xyz"));
}

#[test]
fn test_parse_show_missing_id() {
    let input = "show";
    let err = Command::parse(input).unwrap_err();
    assert_eq!(err, CommandError::WrongArgument(""));
}

#[test]
fn test_parse_delete_missing_id() {
    let input = "delete";
    let err = Command::parse(input).unwrap_err();
    assert_eq!(err, CommandError::WrongArgument(""));
}

#[test]
fn test_parse_empty_input() {
    let input = "";
    let err = Command::parse(input).unwrap_err();
    assert_eq!(err, CommandError::CommandNotFound(""));
}

#[test]
fn test_parse_extra_whitespace() {
    let input = "  show   5  ";
    let cmd = Command::parse(input).unwrap();
    assert_eq!(cmd, Command::Show { id: 5 });
}

// ===== parse_id =====

#[test]
fn test_parse_id_valid() {
    let mut parts = "123 rest".split_whitespace();
    let id = parse_id(&mut parts).unwrap();
    assert_eq!(id, 123);
}

#[test]
fn test_parse_id_invalid() {
    let mut parts = "abc rest".split_whitespace();
    let err = parse_id(&mut parts).unwrap_err();
    assert_eq!(err, CommandError::WrongArgument("abc"));
}

#[test]
fn test_parse_id_empty() {
    let mut parts = "".split_whitespace();
    let err = parse_id(&mut parts).unwrap_err();
    assert_eq!(err, CommandError::WrongArgument(""));
}

#[test]
fn test_parse_id_zero() {
    let mut parts = "0 next".split_whitespace();
    let id = parse_id(&mut parts).unwrap();
    assert_eq!(id, 0);
}

#[test]
fn test_parse_id_max_u32() {
    let max = u32::MAX.to_string();
    let mut parts = max.split_whitespace();
    let id = parse_id(&mut parts).unwrap();
    assert_eq!(id, u32::MAX);
}
