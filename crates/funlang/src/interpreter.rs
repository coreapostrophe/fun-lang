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
            statement.execute(&mut self.environment)?;
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
        if a == 12 or a == 6 {
            print \"a is 12 or 6\";
        } else {
            print \"a is not 12 or 6\";
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

    #[test]
    fn interprets_while_statements() {
        let mut lexer = Lexer::new();
        let lexer_result = lexer.tokenize(
            "
            let a = 0; 
            while a != 10 
            { 
                a = a + 1; 
                print a; 
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

    #[test]
    fn interprets_for_statements() {
        let mut lexer = Lexer::new();
        let lexer_result = lexer.tokenize("for let a = 0; a < 10; a = a + 1 { print a; }");
        assert!(lexer_result.is_ok());

        let mut parser = Parser::new();
        let parser_result = parser.parse(lexer_result.unwrap());
        assert!(parser_result.is_ok());
        assert!(parser_result.is_ok());

        let mut interpreter = Interpreter::new();
        assert!(interpreter.interpret(parser_result.unwrap()).is_ok());
    }
}
