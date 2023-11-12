use std::{
    error::Error,
    fmt::{Debug, Display},
};

#[derive(Debug, Clone)]
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

pub trait ErrorType: Debug + Display {}

pub trait Spanned {
    fn span(&self) -> Option<&ErrorSpan>;
    fn set_span(&mut self, span: ErrorSpan);
}

pub trait CascadingError: Error + Spanned {
    fn cascade(&self, cascaded_span: Option<ErrorSpan>) -> String;
}

#[derive(Debug)]
pub struct ErrorCascade<T: ErrorType> {
    pub error_type: T,
    pub span: Option<ErrorSpan>,
    pub embedded_error: Option<Box<dyn CascadingError>>,
}

impl<T: ErrorType> ErrorCascade<T> {
    pub fn new(error_type: T) -> Self {
        Self {
            error_type,
            span: None,
            embedded_error: None,
        }
    }
    pub fn set_span(mut self, span: ErrorSpan) -> Self {
        self.span = Some(span);
        self
    }
    pub fn set_embedded_error<E: ErrorType + 'static>(
        mut self,
        embedded_error: Box<ErrorCascade<E>>,
    ) -> Self {
        self.embedded_error = Some(embedded_error);
        self
    }
}

impl<T: ErrorType> Display for ErrorCascade<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message: String = self.cascade(self.span.clone());
        write!(f, "{}", message)
    }
}

impl<T: ErrorType> CascadingError for ErrorCascade<T> {
    fn cascade(&self, cascaded_span: Option<ErrorSpan>) -> String {
        match &self.embedded_error {
            Some(error) => error.cascade(self.span.clone().or(cascaded_span)),
            None => match self.span.clone().or(cascaded_span) {
                Some(span) => format!(
                    "[line {}:{} - {:?}] {}",
                    span.line, span.col, self.error_type, self.error_type
                ),
                None => format!("[{:?}] {}", self.error_type, self.error_type),
            },
        }
    }
}

impl<T: ErrorType> Spanned for ErrorCascade<T> {
    fn span(&self) -> Option<&ErrorSpan> {
        self.span.as_ref()
    }
    fn set_span(&mut self, span: ErrorSpan) {
        self.span = Some(span);
    }
}

impl<T: ErrorType> Error for ErrorCascade<T> {}
