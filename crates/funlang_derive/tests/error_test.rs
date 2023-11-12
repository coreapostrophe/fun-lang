use funlang_error::{ErrorCascade, ErrorSpan};

#[derive(funlang_derive::Error)]
enum ParserError {
    #[message = "source is missing"]
    MissingSource,
    #[message = "number is invalid"]
    InvalidNumber,
}

fn main() {
    let cascaded_error = ErrorCascade::new(ParserError::InvalidNumber)
        .set_span(ErrorSpan::new(0, 1, 1))
        .set_embedded_error(Box::new(ErrorCascade::new(ParserError::MissingSource)));
    let cascaded_span_error = ErrorCascade::new(ParserError::InvalidNumber)
        .set_span(ErrorSpan::new(0, 1, 1))
        .set_embedded_error(Box::new(
            ErrorCascade::new(ParserError::MissingSource).set_span(ErrorSpan::new(0, 2, 1)),
        ));

    println!("cascaded error: {}", cascaded_error);
    println!("cascaded error & span: {}", cascaded_span_error);
}
