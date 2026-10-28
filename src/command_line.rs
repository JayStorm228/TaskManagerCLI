use std::collections::HashMap;
use std::io::{ self, Write };

use crate::errors::CmdLineError;
type Args = HashMap<String, String>;
type Tokens = Vec<String>;

/// Stores raw command line and parses it into arguments
///
/// `args` format:
///
/// ```text
/// {
///     "command": "add",
///     "--title": "Buy Milk",
///     "--description": "Go store and buy milk"
/// }
/// ```
///
/// The first token is always stored under the the `"command"` key
/// The remaining tokens must follow  one of these patterns:
/// `--name value` or `--name "Long value"`
#[derive(Debug, Default)]
pub struct CmdLine {
    args: Args,
    input: String,
}

impl CmdLine {
    /// Creates an empty command line
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns a reference to the argument-value map
    pub fn get_args(&self) -> &Args {
        &self.args
    }

    /// Returns a reference to the user's input string
    pub fn get_input(&self) -> &str {
        &self.input
    }

    /// Sets the input string for tests
    #[cfg(test)]
    pub(crate) fn set_input_for_test(&mut self, input: &str) {
        self.input = input.to_string();
    }

    /// Reads one line from standard input
    ///
    /// Displays the "> " prompt, flushes standard output,
    /// and then reads a line from standard input.
    ///
    /// # Errors
    ///
    /// Returns an io::Error if flushing standard output
    /// or reading from standard input fails.
    pub fn input(&mut self) -> io::Result<()> {
        print!("> ");
        io::stdout().flush()?;

        let mut command_line = String::new();
        io::stdin().read_line(&mut command_line)?;

        self.input = command_line.trim().to_string();
        Ok(())
    }

    /// Splits the input string and fills `args`.
    ///
    /// The first token is stored under `"command"` key.
    /// Every subsequent argument name must be followed by a value
    ///
    /// # Errors
    ///
    /// Returns
    /// - [`CmdLineError::EmptyCommandLine`], if the input is empty
    /// - [`CmdLineError::InvalidArgument`] if an argument name does not match `--name` pattern.
    /// - [`CmdLineError::MissingArgumentValue`] if an argument name has no matching value.
    /// - [`CmdLineError::DuplicateArgument`] if a duplicate argument is found.
    /// - [`CmdLineError::UnexpectedCharAfterQuote`] if an unexpected character appears immediately after a closing quote
    /// - [`CmdLineError::UnterminatedQuote`] if quote string is not properly terminated
    /// - [`CmdLineError::UnterminatedEscapeSequence`] if escape sequence is not properly terminated (from self.tokenize method)
    pub fn parse(&mut self) -> Result<(), CmdLineError> {
        let tokens = self.tokenize()?;

        let Some(command) = tokens.first() else {
            return Err(CmdLineError::EmptyCommandLine);
        };

        let mut parsed = HashMap::new();
        parsed.insert(String::from("command"), command.clone());

        let mut index = 1;

        while index < tokens.len() {
            let argument = &tokens[index];

            if !argument.starts_with("--") {
                return Err(CmdLineError::InvalidArgument(argument.clone()));
            }

            let Some(value) = tokens.get(index + 1) else {
                return Err(CmdLineError::MissingArgumentValue(argument.clone()));
            };

            if parsed.insert(argument.clone(), value.clone()).is_some() {
                return Err(CmdLineError::DuplicateArgument(argument.clone()));
            }

            index += 2;
        }

        self.args = parsed;
        Ok(())
    }
    /// Splits the stored input field into tokens
    /// used fof command parsing
    ///
    /// Supports escape sequences, quoted strings, and whitespace-separated tokens.
    ///
    /// # Errors
    /// Returns
    /// - [`CmdLineError::UnexpectedCharAfterQuote`] if an unexpected character appears immediately after a closing quote
    /// - [`CmdLineError::UnterminatedQuote`] if quote string is not properly terminated
    /// - [`CmdLineError::UnterminatedEscapeSequence`] if escape sequence is not properly terminated
    fn tokenize(&self) -> Result<Tokens, CmdLineError> {
        let mut tokens = Tokens::new();
        let mut current = String::new();
        let mut in_quotes = false;
        let mut escaped = false;
        let mut quote_just_closed = false;

        for ch in self.input.chars() {
            if quote_just_closed {
                if ch.is_whitespace() {
                    tokens.push(std::mem::take(&mut current));
                    quote_just_closed = false;
                } else {
                    return Err(CmdLineError::UnexpectedCharAfterQuote(ch));
                }
            } else if escaped {
                current.push(ch);
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                if in_quotes {
                    quote_just_closed = true;
                }

                in_quotes = !in_quotes;
            } else if in_quotes {
                current.push(ch);
            } else if ch.is_whitespace() {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            } else {
                current.push(ch);
            }
        }

        if in_quotes {
            return Err(CmdLineError::UnterminatedQuote(self.input.clone()));
        }
        if escaped {
            return Err(CmdLineError::UnterminatedEscapeSequence(self.input.clone()));
        }
        if quote_just_closed || !current.is_empty() {
            tokens.push(current);
        }

        Ok(tokens)
    }
}
