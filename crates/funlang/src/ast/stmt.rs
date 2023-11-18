use crate::{
    ast::expr::Expr, environment::Environment, error, errors::InterpreterError,
    literal::LiteralData, token::Token,
};
use funlang_derive::Ast;
use funlang_error::ErrorCascade;

use super::{
    expr::LiteralExpr,
    traits::{Evaluable, Executable},
};

#[derive(Ast, Debug, Clone)]
pub enum Stmt {
    #[production(expression: Expr)]
    Expression(Box<ExpressionStmt>),

    #[production(expression: Expr)]
    Print(Box<PrintStmt>),

    #[production(name: Token, initializer: Option<Expr>)]
    Variable(Box<VariableStmt>),
}

impl Evaluable<LiteralData> for Stmt {
    fn evaluate(
        &self,
        environment: &mut Environment,
    ) -> Result<LiteralData, ErrorCascade<InterpreterError>> {
        match self {
            Self::Expression(expression_statement) => {
                match expression_statement.expression.evaluate(environment) {
                    Ok(evaluated_value) => Ok(evaluated_value),
                    Err(error) => Err(error!(InterpreterError::EvaluatationException)
                        .set_embedded_error(Box::new(error))),
                }
            }
            Self::Print(print_statement) => {
                match print_statement.expression.evaluate(environment) {
                    Ok(evaluated_value) => {
                        println!("{:?}", evaluated_value);
                        Ok(evaluated_value)
                    }
                    Err(error) => Err(error!(InterpreterError::EvaluatationException)
                        .set_embedded_error(Box::new(error))),
                }
            }
            Self::Variable(variable_statement) => {
                if let Some(name) = variable_statement.name.lexeme.as_ref() {
                    match variable_statement.as_ref().initializer {
                        Some(ref initializer) => {
                            environment.define(&name, initializer.clone());
                        }
                        None => environment.define(
                            &name,
                            Expr::Literal(Box::new(LiteralExpr {
                                literal: LiteralData::None,
                            })),
                        ),
                    }
                }
                Ok(LiteralData::None)
            }
        }
    }
}

impl Executable for Stmt {
    fn execute(&self, environment: &mut Environment) -> Result<(), ErrorCascade<InterpreterError>> {
        match self.evaluate(environment) {
            Ok(_) => Ok(()),
            Err(error) => {
                Err(error!(InterpreterError::ExecutionException)
                    .set_embedded_error(Box::new(error)))
            }
        }
    }
}
