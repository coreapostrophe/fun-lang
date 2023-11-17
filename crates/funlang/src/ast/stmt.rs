use crate::{ast::expr::Expr, error, errors::InterpreterError, literal::LiteralData, token::Token};
use funlang_derive::Ast;
use funlang_error::ErrorCascade;

use super::traits::{Evaluable, Executable};

#[derive(Ast, Debug, Clone)]
pub enum Stmt {
    #[production(expression:Expr)]
    Expression(Box<ExpressionStmt>),

    #[production(expression:Expr)]
    Print(Box<PrintStmt>),

    #[production(name:Token, initializer:Expr)]
    Variable(Box<VariableStmt>),
}

impl Evaluable<LiteralData> for Stmt {
    fn evaluate(&self) -> Result<LiteralData, ErrorCascade<InterpreterError>> {
        match self {
            Self::Expression(expression_statement) => {
                match expression_statement.expression.evaluate() {
                    Ok(evaluated_value) => Ok(evaluated_value),
                    Err(error) => Err(error!(InterpreterError::EvaluatationException)
                        .set_embedded_error(Box::new(error))),
                }
            }
            Self::Print(print_statement) => match print_statement.expression.evaluate() {
                Ok(evaluated_value) => {
                    println!("{:?}", evaluated_value);
                    Ok(evaluated_value)
                }
                Err(error) => Err(error!(InterpreterError::EvaluatationException)
                    .set_embedded_error(Box::new(error))),
            },
            Self::Variable(_variable_statement) => {
                todo!() 
            }
        }
    }
}

impl Executable for Stmt {
    fn execute(&self) -> Result<(), ErrorCascade<InterpreterError>> {
        match self.evaluate() {
            Ok(_) => Ok(()),
            Err(error) => {
                Err(error!(InterpreterError::ExecutionException)
                    .set_embedded_error(Box::new(error)))
            }
        }
    }
}
