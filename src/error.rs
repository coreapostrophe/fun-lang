use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct Source {
    line_number: u32,
    line_offset: u32,
}

impl Source {
    pub fn new(line_number: u32, line_offset: u32) -> Self {
        Self {
            line_number,
            line_offset,
        }
    }
}

#[macro_export]
macro_rules! source {
    ($line_number:expr, $line_offset:expr) => {
        Source::new($line_number, $line_offset)
    };
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum CompilerError {
    UnexpectedCharacter(Source),
    CharacterNotFound(Source),
}

impl CompilerError {
    fn format_error(error: &CompilerError, source: &Source, message: &str) -> String {
        format!(
            "line {}:{} {:?} - {}",
            source.line_number, source.line_offset, error, message
        )
    }
}

impl Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let error_message = match self {
            Self::UnexpectedCharacter(source) => {
                Self::format_error(self, source, "unexpected character")
            }
            Self::CharacterNotFound(source) => {
                Self::format_error(self, source, "character is not found")
            }
        };
        write!(f, "{}", error_message)
    }
}

impl Error for CompilerError {}
