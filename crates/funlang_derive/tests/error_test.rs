use funlang_error::{ErrorCascade, ErrorSpan};

#[derive(funlang_derive::Error)]
enum ParserError {
    #[message = "something went wrong during addition"]
    AdditionException,
    #[message = "\"{}\" can't be parsed to type `{}`"]
    InvalidNumber(String, String),
}

fn main() {
    let _cascaded_error = ErrorCascade::new(ParserError::AdditionException)
        .set_span(ErrorSpan::new(0, 1, 1))
        .set_embedded_error(Box::new(ErrorCascade::new(ParserError::InvalidNumber(
            "abc".to_string(),
            std::any::type_name::<u32>().to_string(),
        ))));
    let _cascaded_span_error = ErrorCascade::new(ParserError::AdditionException)
        .set_span(ErrorSpan::new(0, 1, 1))
        .set_embedded_error(Box::new(
            ErrorCascade::new(ParserError::InvalidNumber(
                "abc".to_string(),
                std::any::type_name::<u32>().to_string(),
            ))
            .set_span(ErrorSpan::new(0, 2, 1)),
        ));

    // println!("cascaded error: {}", cascaded_error);
    // println!("cascaded error & span: {}", cascaded_span_error);
}
