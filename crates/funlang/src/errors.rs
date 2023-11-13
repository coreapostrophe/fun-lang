use funlang_derive::Error;

#[derive(Error)]
pub enum LexerError {
    #[message = "lexer does not have a source"]
    MissingSource,
    #[message = "unexpected character"]
    UnexpectedCharacter,
    #[message = "character being indexed is out of bounds"]
    InvalidCharacterIndex,
    #[message = "string was not closed"]
    UnterminatedString,
    #[message = "invalid number"]
    InvalidNumber,
}

#[derive(Error)]
pub enum OperationError {
    #[message = "invalid number"]
    InvalidNumber,
    #[message = "boolean operations are not allowed"]
    InvalidBooleanOperation,
    #[message = "attempted to subtract strings"]
    InvalidStringOperation,
    #[message = "attempted to perform null operations"]
    InvalidNullOperation,
    #[message = "string can't be parsed to a valid number"]
    InvalidParsedNumber,
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
    InvalidNumber,
    #[message = "attempted to negate a boolean"]
    NegatedBoolean,
    #[message = "invalid data"]
    InvalidLiteralData,
    #[message = "grouping symbol was not closed"]
    UnterminatedGrouping,
    #[message = "unexpected expression"]
    UnexpectedExpression,
    #[message = "invalid unary operator"]
    InvalidUnaryOperator,
    #[message = "invalid binary operator"]
    InvalidBinaryOperator,
    #[message = "something went wrong during addition"]
    AdditionException,
    #[message = "something went wrong during subtraction"]
    SubtractionException,
    #[message = "something went wrong during division"]
    DivisionException,
    #[message = "something went wrong during multiplication"]
    MultiplicationException,
}
