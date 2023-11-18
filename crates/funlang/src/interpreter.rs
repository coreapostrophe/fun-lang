use funlang_error::ErrorCascade;

use crate::{
    ast::{stmt::Stmt, traits::Executable},
    environment::Environment,
    errors::InterpreterError,
};

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&self, statements: Vec<Stmt>) -> Result<(), ErrorCascade<InterpreterError>> {
        for statement in statements {
            statement.execute(&self.environment)?
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

        let interpreter = Interpreter::new();
        assert!(interpreter.interpret(parser_result.unwrap()).is_ok());
    }
}
