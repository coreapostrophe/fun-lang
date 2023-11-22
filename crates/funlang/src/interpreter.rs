use funlang_error::ErrorCascade;

use crate::{
    ast::{stmt::Stmt, traits::Executable},
    environment::Environment,
    errors::InterpreterError,
};

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    pub fn interpret(
        &mut self,
        statements: Vec<Stmt>,
    ) -> Result<(), ErrorCascade<InterpreterError>> {
        for statement in statements {
            statement.execute(&mut self.environment)?
        }
        Ok(())
    }
}

#[cfg(test)]
mod interpreter_tests {
    use super::*;
    use crate::{lexer::Lexer, parser::Parser};

    #[test]
    fn interprets_expression_statements() {
        let mut lexer = Lexer::new();
        let lexer_result = lexer.tokenize("(1 + 1) / 6;");
        assert!(lexer_result.is_ok());

        let mut parser = Parser::new();
        let parser_result = parser.parse(lexer_result.unwrap());
        assert!(parser_result.is_ok());

        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret(parser_result.unwrap()).is_ok());
    }

    #[test]
    fn interprets_print_statements() {
        let mut lexer = Lexer::new();
        let lexer_result = lexer.tokenize("print (1 + 1) / 6;");
        assert!(lexer_result.is_ok());

        let mut parser = Parser::new();
        let parser_result = parser.parse(lexer_result.unwrap());
        assert!(parser_result.is_ok());

        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret(parser_result.unwrap()).is_ok());
    }

    #[test]
    fn interprets_variable_declarations() {
        let mut lexer = Lexer::new();
        let lexer_result = lexer.tokenize("let a = 6;");
        assert!(lexer_result.is_ok());

        let mut parser = Parser::new();
        let parser_result = parser.parse(lexer_result.unwrap());
        assert!(parser_result.is_ok());

        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret(parser_result.unwrap()).is_ok());
    }

    #[test]
    fn interprets_variable_assignments() {
        let mut lexer = Lexer::new();
        let lexer_result = lexer.tokenize("let a = 6; a = 12;");
        assert!(lexer_result.is_ok());

        let mut parser = Parser::new();
        let parser_result = parser.parse(lexer_result.unwrap());
        assert!(parser_result.is_ok());

        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret(parser_result.unwrap()).is_ok());
    }

    #[test]
    fn interprets_block_statements() {
        let mut lexer = Lexer::new();
        let lexer_result = lexer.tokenize(
            "
        let a = 6;
        a = 12; 
        {
            a = 18;
            print a;
        }
        print a;
        ",
        );
        assert!(lexer_result.is_ok());

        let mut parser = Parser::new();
        let parser_result = parser.parse(lexer_result.unwrap());
        assert!(parser_result.is_ok());

        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret(parser_result.unwrap()).is_ok());
    }

    #[test]
    fn interprets_if_statements() {
        let mut lexer = Lexer::new();
        let lexer_result = lexer.tokenize(
            "
        let a = 6;
        if a = 12 {
            print \"a is 12\";
        } else {
            print \"a is 6\";
        }
        ",
        );
        assert!(lexer_result.is_ok());

        let mut parser = Parser::new();
        let parser_result = parser.parse(lexer_result.unwrap());
        assert!(parser_result.is_ok());

        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret(parser_result.unwrap()).is_ok());
    }
}
