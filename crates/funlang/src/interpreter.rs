use funlang_error::ErrorCascade;

use crate::{
    ast::{expr::Expr, traits::Evaluable},
    error,
    errors::InterpreterError,
    literal::LiteralData,
};

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(expression: Expr) -> Result<LiteralData, ErrorCascade<InterpreterError>> {
        match expression.evaluate() {
            Ok(evaluated_value) => Ok(evaluated_value),
            Err(error) => {
                Err(error!(InterpreterError::EvalutationException)
                    .set_embedded_error(Box::new(error)))
            }
        }
    }
}

#[cfg(test)]
mod interpreter_tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn interprets_expressions() {
        let mut lexer = Lexer::new();
        let lexer_result = lexer.tokenize("(1 + 1) / 6");
        assert!(lexer_result.is_ok());

        let mut parser = Parser::new();
        let parser_result = parser.parse(lexer_result.unwrap());
        assert!(parser_result.is_ok());

        let evaluated_value = Interpreter::interpret(parser_result.unwrap());
        assert!(evaluated_value.is_ok());

        assert_eq!(evaluated_value.unwrap(), LiteralData::Number(0.33333334));
    }
}
