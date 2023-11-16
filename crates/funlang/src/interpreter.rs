use funlang_error::ErrorCascade;

use crate::{
    ast::{stmt::Stmt, traits::Executable},
    errors::InterpreterError,
};

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(statements: Vec<Stmt>) -> Result<(), ErrorCascade<InterpreterError>> {
        for statement in statements {
            statement.execute()?
        }
        Ok(())
    }
}

#[cfg(test)]
mod interpreter_tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn interprets_expressions() {
        let mut lexer = Lexer::new();
        let lexer_result = lexer.tokenize("print (1 + 1) / 6;");
        assert!(lexer_result.is_ok());

        let mut parser = Parser::new();
        let parser_result = parser.parse(lexer_result.unwrap());
        assert!(parser_result.is_ok());

        assert!(Interpreter::interpret(parser_result.unwrap()).is_ok());
    }
}
