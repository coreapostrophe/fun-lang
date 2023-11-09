use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Source {
    pub line_number: u32,
    pub line_offset: u32,
}

impl Source {
    pub fn new(line_number: u32, line_offset: u32) -> Self {
        Self {
            line_number,
            line_offset,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum LexerError {
    MissingSource,
    UnexpectedCharacter,
    InvalidCharacterIndex(Source),
    UnterminatedString(Source),
    InvalidNumber(Source),
}

impl LexerError {
    fn format_error(error: &LexerError, source: Option<&Source>, message: &str) -> String {
        match source {
            Some(source) => format!(
                "[line {}:{} {:?}] - {}",
                source.line_number, source.line_offset, error, message
            ),
            None => format!("{:?} - {}", error, message),
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Self::MissingSource => Self::format_error(self, None, "lexer does not have a source"),
            Self::UnexpectedCharacter => {
                Self::format_error(self, None, "attempted to tokenize an unexpected character")
            }
            Self::InvalidCharacterIndex(source) => Self::format_error(
                self,
                Some(source),
                "character being indexed is out of bounds",
            ),
            Self::UnterminatedString(source) => {
                Self::format_error(self, Some(source), "string was not closed")
            }
            Self::InvalidNumber(source) => {
                Self::format_error(self, Some(source), "attempted to parse an invalid number")
            }
        };
        write!(f, "{}", error_message)
    }
}

impl Error for LexerError {}
