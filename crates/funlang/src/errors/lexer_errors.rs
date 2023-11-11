use funlang_derive::Error;
use funlang_error::ErrorMeta;

#[derive(Error)]
pub enum LexerError {
    #[message = "lexer does not have a source"]
    MissingSource,
    #[message = "unexpected character"]
    UnexpectedCharacter,
    #[message = "character being indexed is out of bounds"]
    InvalidCharacterIndex(ErrorMeta),
    #[message = "string was not closed"]
    UnterminatedString(ErrorMeta),
    #[message = "invalid number"]
    InvalidNumber(ErrorMeta),
}
