use crate::command_line::CmdLine;
use crate::errors::CmdLineError;

fn parse_line(input: &str) -> Result<CmdLine, CmdLineError> {
    let mut line = CmdLine::new();
    line.set_input_for_test(input);
    line.parse()?;
    Ok(line)
}

#[test]
fn new_line_is_empty() {
    let line = CmdLine::new();
    assert_eq!(line.get_input(), "");
    assert!(line.get_args().is_empty());
}

#[test]
fn parses_command_without_arguments() {
    let line = parse_line("list").unwrap();
    assert_eq!(line.get_args().len(), 1);
    assert_eq!(line.get_args().get("command").unwrap(), "list");
}

#[test]
fn parses_named_arguments_in_any_order() {
    let line = parse_line(
        r#"add --priority high --description "Buy two liters" --title "Buy milk""#
    ).unwrap();
    let args = line.get_args();

    assert_eq!(args.len(), 4);
    assert_eq!(args.get("command").unwrap(), "add");
    assert_eq!(args.get("--title").unwrap(), "Buy milk");
    assert_eq!(args.get("--priority").unwrap(), "high");
    assert_eq!(args.get("--description").unwrap(), "Buy two liters");
}

#[test]
fn ignores_extra_whitespace_between_tokens() {
    let line = parse_line("  show\t  --id  \t42  \n").unwrap();
    assert_eq!(line.get_args().get("command").unwrap(), "show");
    assert_eq!(line.get_args().get("--id").unwrap(), "42");
}

#[test]
fn preserves_whitespace_inside_quotes() {
    let line = parse_line("add --title \"  Buy   milk  \"").unwrap();
    assert_eq!(line.get_args().get("--title").unwrap(), "  Buy   milk  ");
}

#[test]
fn preserves_unicode_inside_quoted_value() {
    let line = parse_line(r#"add --title "Купить молоко 🥛""#).unwrap();
    assert_eq!(line.get_args().get("--title").unwrap(), "Купить молоко 🥛");
}

#[test]
fn parses_escaped_quotes_inside_quoted_value() {
    let line = parse_line(r#"add --title "Say \"hello\"""#).unwrap();
    assert_eq!(line.get_args().get("--title").unwrap(), "Say \"hello\"");
}

#[test]
fn parses_escaped_backslash_inside_quoted_value() {
    let line = parse_line(r#"add --title "C:\\Temp""#).unwrap();
    assert_eq!(line.get_args().get("--title").unwrap(), r"C:\Temp");
}

#[test]
fn accepts_explicit_empty_quoted_value() {
    let line = parse_line(r#"add --title """#).unwrap();
    assert_eq!(line.get_args().get("--title").unwrap(), "");
}

#[test]
fn preserves_original_input() {
    let input = "  add --title \"An example\"  ";
    let line = parse_line(input).unwrap();
    assert_eq!(line.get_input(), input);
}

#[test]
fn rejects_empty_input() {
    assert_eq!(parse_line("").unwrap_err(), CmdLineError::EmptyCommandLine);
    assert_eq!(parse_line(" \t\n ").unwrap_err(), CmdLineError::EmptyCommandLine);
}

#[test]
fn rejects_unterminated_quote() {
    let input = r#"add --title "Unfinished"#;
    assert_eq!(parse_line(input).unwrap_err(), CmdLineError::UnterminatedQuote(input.to_string()));
}

#[test]
fn rejects_character_immediately_after_closing_quote() {
    assert_eq!(
        parse_line(r#"add --title "hello"world"#).unwrap_err(),
        CmdLineError::UnexpectedCharAfterQuote('w')
    );
}

#[test]
fn rejects_positional_argument() {
    assert_eq!(
        parse_line("add BuyMilk").unwrap_err(),
        CmdLineError::InvalidArgument("BuyMilk".to_string())
    );
}

#[test]
fn rejects_short_option() {
    assert_eq!(
        parse_line("show -i 2").unwrap_err(),
        CmdLineError::InvalidArgument("-i".to_string())
    );
}

#[test]
fn rejects_option_without_value() {
    assert_eq!(
        parse_line("show --id").unwrap_err(),
        CmdLineError::MissingArgumentValue("--id".to_string())
    );
}

#[test]
fn rejects_duplicate_option() {
    assert_eq!(
        parse_line("show --id 1 --id 2").unwrap_err(),
        CmdLineError::DuplicateArgument("--id".to_string())
    );
}

#[test]
fn supports_reusing_line_for_new_input() {
    let mut line = CmdLine::new();
    line.set_input_for_test("show --id 5");
    line.parse().unwrap();
    assert_eq!(line.get_args().get("--id").unwrap(), "5");

    line.set_input_for_test("help");
    line.parse().unwrap();
    assert_eq!(line.get_args().len(), 1);
    assert_eq!(line.get_args().get("command").unwrap(), "help");
    assert!(!line.get_args().contains_key("--id"));
}

#[test]
fn failed_parse_does_not_partially_overwrite_previous_args() {
    let mut line = CmdLine::new();
    line.set_input_for_test("show --id 5");
    line.parse().unwrap();

    line.set_input_for_test("show --id 6 --id 7");
    assert_eq!(line.parse(), Err(CmdLineError::DuplicateArgument("--id".to_string())));
    assert_eq!(line.get_args().get("--id").unwrap(), "5");
}

#[test]
fn accepts_value_starting_with_double_dash() {
    // Текущая реализация трактует следующий токен как значение,
    // даже если он начинается с '--'. Фиксируем это поведение.
    let line = parse_line("add --title --example").unwrap();
    assert_eq!(line.get_args().get("--title").unwrap(), "--example");
}

#[test]
fn command_line_error_implements_standard_error() {
    let error = CmdLineError::EmptyCommandLine;
    assert!(std::error::Error::source(&error).is_none());
}
