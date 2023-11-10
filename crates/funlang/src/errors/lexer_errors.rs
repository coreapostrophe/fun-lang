use std::{error::Error, fmt::Display};

use crate::token::Span;

#[allow(dead_code)]
#[derive(Debug)]
pub enum LexerError {
    MissingSource,
    UnexpectedCharacter,
    InvalidCharacterIndex(Span),
    UnterminatedString(Span),
    InvalidNumber(Span),
}

impl LexerError {
    fn format_error(error: &LexerError, span: Option<&Span>, message: &str) -> String {
        match span {
            Some(span) => format!("[line {}:{} {:?}] {}", span.line, span.col, error, message),
            None => format!("[{:?}] {}", error, message),
        }
    }
}

impl Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Self::MissingSource => Self::format_error(self, None, "lexer does not have a source"),
            Self::UnexpectedCharacter => {
                Self::format_error(self, None, "unexpected character")
            }
            Self::InvalidCharacterIndex(span) => Self::format_error(
                self,
                Some(span),
                "character being indexed is out of bounds",
            ),
            Self::UnterminatedString(span) => {
                Self::format_error(self, Some(span), "string was not closed")
            }
            Self::InvalidNumber(span) => {
                Self::format_error(self, Some(span), "invalid number")
            }
        };
        write!(f, "{}", error_message)
    }
}

impl Error for LexerError {}
