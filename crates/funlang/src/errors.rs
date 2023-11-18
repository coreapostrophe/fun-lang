use funlang_derive::Error;

#[derive(Error)]
pub enum InterpreterError {
    #[message = "something went wrong while attempting to execute statement"]
    ExecutionException,
    #[message = "something went wrong while attempting to evaluate expression"]
    EvaluatationException,
    #[message = "`{}` string can not be parsed to type `number`"]
    InvalidParsedNumber(String),
    #[message = "`{}` is not a valid binary operator"]
    InvalidBinaryOperator(String),
    #[message = "`{}` is not a valid unary operator"]
    InvalidUnaryOperator(String),
    #[message = "Expression could not be identified"]
    UnexpectedExpression,
    #[message = "something went wrong during addition"]
    AdditionException,
    #[message = "something went wrong during subtraction"]
    SubtractionException,
    #[message = "something went wrong during division"]
    DivisionException,
    #[message = "something went wrong during multiplication"]
    MultiplicationException,
    #[message = "`(` grouping was not closed"]
    UnterminatedGrouping,
    #[message = "literal data could not be identified"]
    InvalidLiteralData,
    #[message = "indexed token does not have a span"]
    MissingSpan,
    #[message = "variable expression does not have an identifier"]
    MissingIdentifier,
    #[message = "variable with identifier `{}` does not exist"]
    InvalidIdentifier(String),
}

#[derive(Error)]
pub enum LexerError {
    #[message = "lexer does not have a source"]
    MissingSource,
    #[message = "unexpected character `{}`"]
    UnexpectedCharacter(String),
    #[message = "character being indexed is out of bounds"]
    InvalidCharacterIndex,
    #[message = "string literal was not closed"]
    UnterminatedString,
}

#[derive(Error)]
pub enum ParserError {
    #[message = "parser does not have a token list input"]
    MissingTokens,
    #[message = "token being indexed is out of bounds"]
    InvalidTokenIndex,
    #[message = "invalid number"]
    InvalidNumber,
    #[message = "attempted to negate a boolean"]
    NegatedBoolean,
    #[message = "literal data could not be identified"]
    InvalidLiteralData,
    #[message = "`(` grouping was not closed"]
    UnterminatedGrouping,
    #[message = "expressions should be terminated by `;`"]
    UnterminatedExpression,
    #[message = "unexpected expression"]
    UnexpectedExpression,
    #[message = "invalid unary operator"]
    InvalidUnaryOperator,
    #[message = "invalid binary operator"]
    InvalidBinaryOperator,
    #[message = "indexed token does not have a span"]
    MissingSpan,
    #[message = "expected an `=` symbol in variable declaration"]
    ExpectedEqual,
    #[message = "expected an identifier name"]
    ExpectedIdentifier,
    #[message = "expression before the `=` symbol should be a valid variable"]
    InvalidAssignmentTarget,
    #[message = "variable with identifier `{}` does not exist"]
    InvalidIdentifier(String),
}
