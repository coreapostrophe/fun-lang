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

#[derive(Error)]
pub enum OperationError {
    #[message = "invalid number"]
    InvalidNumber,
    #[message = "attempted to add booleans"]
    InvalidBooleanAddition,
    #[message = "attempted to add identifiers"]
    InvalidIdentifierAddition,
}

#[derive(Error)]
pub enum ParserError {
    #[message = "indexed token does not have a span"]
    MissingSpan,
    #[message = "parser does not have a token list input"]
    MissingTokens,
    #[message = "token being indexed is out of bounds"]
    InvalidTokenIndex,
    #[message = "invalid number"]
    InvalidNumber(ErrorMeta),
    #[message = "attempted to negate a boolean"]
    NegatedBoolean(ErrorMeta),
    #[message = "invalid data"]
    InvalidLiteralData(ErrorMeta),
    #[message = "grouping symbol was not closed"]
    UnterminatedGrouping(ErrorMeta),
    #[message = "unexpected expression"]
    UnexpectedExpression(ErrorMeta),
    #[message = "invalid unary operator"]
    InvalidUnaryOperator(ErrorMeta),
    #[message = "invalid binary operator"]
    InvalidBinaryOperator(ErrorMeta),
    #[message = "something went wrong during addition"]
    AdditionException(ErrorMeta),
    #[message = "something went wrong during subtraction"]
    SubtractionException(ErrorMeta),
    #[message = "something went wrong during division"]
    DivisionException(ErrorMeta),
    #[message = "something went wrong during multiplication"]
    MultiplicationException(ErrorMeta),
}
