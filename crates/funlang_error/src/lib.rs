use std::error::Error;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ErrorMeta {
    pub span: Option<Span>,
    pub error: Option<Box<dyn Error>>,
}
