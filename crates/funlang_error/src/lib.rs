use std::error::Error;

#[derive(Debug)]
pub struct ErrorSpan {
    pub line: usize,
    pub col: usize,
    pub len: usize,
}

impl ErrorSpan {
    pub fn new(line: usize, col: usize, len: usize) -> Self {
        Self { line, col, len }
    }
}

#[derive(Debug)]
pub struct ErrorMeta {
    pub span: Option<ErrorSpan>,
    pub embedded_error: Option<Box<dyn Error>>,
}

impl ErrorMeta {
    pub fn new(span: Option<ErrorSpan>, embedded_error: Option<Box<dyn Error>>) -> Self {
        Self {
            span,
            embedded_error,
        }
    }
}
