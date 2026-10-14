use std::collections::HashMap;
use std::io::{ self, Write };

use crate::errors::CmdLineError;
type Args = HashMap<String, String>;
/// Разбор и хранение командной строки.
///
/// Структура хранит исходную строку и разобранные аргументы.
///
/// Формат `args`:
///
/// ```text
/// {
///     "command": "copy",
///     "--source": "file.txt",
///     "--destination": "backup.txt"
/// }
/// ```
///
/// Первый токен всегда сохраняется под ключом `"command"`.
/// Остальные токены должны иметь вид `--имя значение`.
#[derive(Debug, Default)]
pub struct CmdLine {
    args: Args,
    input: String,
}

impl CmdLine {
    /// Создаёт пустую командную строку.
    pub fn new() -> Self {
        Self::default()
    }

    /// Возвращает ссылку на разобранные аргументы.
    pub fn get_args(&self) -> &Args {
        &self.args
    }

    /// Возвращает исходную введённую командную строку.
    pub fn get_input(&self) -> &str {
        &self.input
    }

    /// Устанавливает исходную строку исключительно для модульных тестов.
    #[cfg(test)]
    pub(crate) fn set_input_for_test(&mut self, input: &str) {
        self.input = input.to_string();
    }

    /// Читает одну строку из стандартного ввода.
    ///
    /// Перед чтением выводит приглашение `> ` и сбрасывает буфер стандартного вывода.
    ///
    /// # Ошибки
    ///
    /// Возвращает [`io::Error`], если не удалось сбросить буфер вывода
    /// или прочитать строку из стандартного ввода.
    pub fn input(&mut self) -> io::Result<()> {
        print!("> ");
        io::stdout().flush()?;

        let mut command_line = String::new();
        io::stdin().read_line(&mut command_line)?;

        self.input = command_line;
        Ok(())
    }

    /// Разбирает текущую строку и заполняет `args`.
    ///
    /// Первый токен становится значением ключа `"command"`.
    /// Каждый следующий токен должен быть именем аргумента вида `--name`,
    /// за которым следует его значение.
    ///
    /// # Ошибки
    ///
    /// Возвращает [`CmdLineError`], если строка пуста, содержит незакрытую кавычку,
    /// некорректный аргумент, аргумент без значения или повторяющийся аргумент.
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

    /// Разбивает исходную строку на токены.
    ///
    /// Поддерживает разделение по пробелам, двойные кавычки
    /// и экранирование символов через `\`.
    fn tokenize(&self) -> Result<Vec<String>, CmdLineError> {
        let mut tokens = Vec::new();
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

        if quote_just_closed || !current.is_empty() {
            tokens.push(current);
        }

        Ok(tokens)
    }
}
