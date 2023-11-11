use std::error::Error;

pub struct Span {
    pub line: usize,
    pub col: usize,
    pub len: usize,
}

impl Span {
    pub fn new(line: usize, col: usize, len: usize) -> Self {
        Self { line, col, len }
    }
}

pub struct ErrorMeta {
    pub span: Span,
    pub error: Option<Box<dyn Error>>,
}
