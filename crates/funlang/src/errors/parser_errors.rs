use std::{error::Error, fmt::Display};

use super::lexer_errors::Source;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParserError {
    MissingTokens,
    InvalidTokenIndex,
    InvalidLiteralData,
    UnterminatedGrouping,
    UnexpectedExpression,
}

impl ParserError {
    fn format_error(error: &ParserError, source: Option<&Source>, message: &str) -> String {
        match source {
            Some(source) => format!(
                "[line {}:{} {:?}] - {}",
                source.line_number, source.line_offset, error, message
            ),
            None => format!("{:?} - {}", error, message),
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Self::MissingTokens => {
                Self::format_error(self, None, "parser does not have a token list input")
            }
            Self::InvalidTokenIndex => {
                Self::format_error(self, None, "token being indexed is out of bounds")
            }
            Self::InvalidLiteralData => {
                Self::format_error(self, None, "attempted to parse invalid data")
            }
            Self::UnterminatedGrouping => {
                Self::format_error(self, None, "grouping symbol was not closed")
            }
            Self::UnexpectedExpression => {
                Self::format_error(self, None, "attempted to parse an unexpected expression")
            }
        };
        write!(f, "{}", error_message)
    }
}

impl Error for ParserError {}