use std::{error::Error, fmt::Display};

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
    fn format_error(error: &ParserError, message: &str) -> String {
        format!("[{:?}] {}", error, message)
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Self::InvalidLiteralData => Self::format_error(self, "attempted to parse invalid data"),
            Self::MissingTokens => {
                Self::format_error(self, "parser does not have a token list input")
            }
            Self::InvalidTokenIndex => {
                Self::format_error(self, "token being indexed is out of bounds")
            }
            Self::UnterminatedGrouping => {
                Self::format_error(self, "grouping symbol was not closed")
            }
            Self::UnexpectedExpression => {
                Self::format_error(self, "attempted to parse an unexpected expression")
            }
        };
        write!(f, "{}", error_message)
    }
}

impl Error for ParserError {}
