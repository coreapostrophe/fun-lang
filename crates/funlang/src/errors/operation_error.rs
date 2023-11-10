use std::{error::Error, fmt::Display};

#[allow(dead_code)]
#[derive(Debug)]
pub enum OperationError {
    InvalidNumber,
    InvalidBooleanAddition,
    InvalidIdentifierAddition,
}

impl OperationError {
    fn format_error(error: &OperationError, message: &str) -> String {
        format!("[{:?}] {}", error, message)
    }
}

impl Display for OperationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Self::InvalidNumber => Self::format_error(self, "invalid number"),
            Self::InvalidBooleanAddition => Self::format_error(self, "attempted to add booleans"),
            Self::InvalidIdentifierAddition => {
                Self::format_error(self, "attempted to add identifiers")
            }
        };
        write!(f, "{}", error_message)
    }
}

impl Error for OperationError {}
