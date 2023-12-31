use crate::{
    ast::expr::Expr, environment::Environment, error, errors::InterpreterError,
    functions::Function, literal::LiteralData, token::Token,
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

    #[production(statements: Vec<Stmt>)]
    Block(Box<BlockStmt>),

    #[production(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>)]
    If(Box<IfStmt>),

    #[production(condition: Expr, body: Stmt)]
    While(Box<WhileStmt>),

    #[production(name: Token, params: Vec<Token>, body: Stmt)]
    Function(Box<FunctionStmt>),
}

impl Executable<LiteralData> for Stmt {
    fn execute(
        &self,
        environment: &mut Environment,
    ) -> Result<LiteralData, ErrorCascade<InterpreterError>> {
        match self {
            Self::Expression(expression_statement) => {
                let evaluated_value = expression_statement.expression.evaluate(environment)?;
                Ok(evaluated_value)
            }
            Self::Print(print_statement) => {
                match print_statement.expression.evaluate(environment) {
                    Ok(evaluated_value) => {
                        println!("{}", evaluated_value);
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
            Self::Block(block_statement) => {
                let mut environment = environment.create_scope();

                for statement in &block_statement.statements {
                    statement.execute(&mut environment)?;
                }
                Ok(LiteralData::None)
            }
            Self::If(if_statement) => {
                if if_statement.condition.evaluate(environment)?.is_truthy()? {
                    if_statement.then_branch.execute(environment)?;
                } else {
                    if let Some(else_branch) = &if_statement.else_branch {
                        else_branch.execute(environment)?;
                    }
                }
                Ok(LiteralData::None)
            }
            Self::While(while_statement) => {
                while while_statement
                    .condition
                    .evaluate(environment)?
                    .is_truthy()?
                {
                    while_statement.body.execute(environment)?;
                }
                Ok(LiteralData::None)
            }
            Self::Function(function_statement) => {
                let name = function_statement
                    .name
                    .lexeme
                    .clone()
                    .ok_or(error!(InterpreterError::MissingIdentifier))?;
                let arity = function_statement.params.len() as u32;
                let function_value = Function::new(arity, function_statement.clone());

                environment.define(
                    &name,
                    Expr::Literal(Box::new(LiteralExpr {
                        literal: LiteralData::Function(function_value),
                    })),
                );

                Ok(LiteralData::None)
            }
        }
    }
}
