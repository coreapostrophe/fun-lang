use funlang_error::ErrorMeta;

#[derive(funlang_derive::Error)]
enum ParserError {
    #[message = "source is missing"]
    MissingSource,
    #[message = "number is invalid"]
    InvalidNumber(ErrorMeta),
}

fn main() {
    let _error = ParserError::InvalidNumber(ErrorMeta {
        span: None,
        embedded_error: Some(Box::new(ParserError::MissingSource)),
    });
}
