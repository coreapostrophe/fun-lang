use std::{error::Error, fmt::Display};

use crate::token::Span;

#[allow(dead_code)]
#[derive(Debug)]
pub enum ParserError {
    MissingSpan,
    MissingTokens,
    InvalidTokenIndex,
    InvalidNumber(Span),
    NegatedBoolean(Span),
    InvalidLiteralData(Span),
    UnterminatedGrouping(Span),
    UnexpectedExpression(Span),
    InvalidUnaryOperator(Span),
    InvalidBinaryOperator(Span),
}

impl ParserError {
    fn format_error(error: &ParserError, span: Option<&Span>, message: &str) -> String {
        match span {
            Some(span) => format!("[line {}:{} {:?}] {}", span.line, span.col, error, message),
            None => format!("[{:?}] {}", error, message),
        }
    }
}

impl Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Self::InvalidNumber(span) => Self::format_error(self, Some(span), "invalid number"),
            Self::InvalidLiteralData(span) => Self::format_error(self, Some(span), "invalid data"),
            Self::InvalidBinaryOperator(span) => {
                Self::format_error(self, Some(span), "invalid binary operator")
            }
            Self::MissingSpan => {
                Self::format_error(self, None, "indexed token does not have a span")
            }
            Self::MissingTokens => {
                Self::format_error(self, None, "parser does not have a token list input")
            }
            Self::InvalidTokenIndex => {
                Self::format_error(self, None, "token being indexed is out of bounds")
            }
            Self::UnterminatedGrouping(span) => {
                Self::format_error(self, Some(span), "grouping symbol was not closed")
            }
            Self::UnexpectedExpression(span) => {
                Self::format_error(self, Some(span), "unexpected expression")
            }
            Self::NegatedBoolean(span) => {
                Self::format_error(self, Some(span), "attempted to negate a boolean")
            }
            Self::InvalidUnaryOperator(span) => {
                Self::format_error(self, Some(span), "invalid unary operator")
            }
        };
        write!(f, "{}", error_message)
    }
}

impl Error for ParserError {}
