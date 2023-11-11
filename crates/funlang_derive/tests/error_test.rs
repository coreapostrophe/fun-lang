use funlang_error::ErrorMeta;

#[derive(funlang_derive::Error, Debug)]
enum ParserError {
    #[message = "source is missing"]
    _MissingSource,
    #[message = "number is invalid"]
    _InvalidNumber(ErrorMeta),
}

fn main() {}
