use std::fmt::{Debug, Display};

use funlang_error::ErrorCascade;

use crate::{
    ast::{
        expr::{Expr, LiteralExpr},
        stmt::FunctionStmt,
        traits::Executable,
    },
    environment::Environment,
    error,
    errors::InterpreterError,
    literal::LiteralData,
    token::Token,
};

pub trait Callable {
    fn call(
        &self,
        environment: &mut Environment,
        arguments: Vec<LiteralData>,
    ) -> Result<LiteralData, ErrorCascade<InterpreterError>>;
}

#[derive(Debug, Clone)]
pub struct Function {
    arity: u32,
    declaration: Box<FunctionStmt>,
}

impl Function {
    pub fn new(arity: u32, declaration: Box<FunctionStmt>) -> Self {
        Self { arity, declaration }
    }

    pub fn arity(&self) -> u32 {
        self.arity
    }

    pub fn declaration(&self) -> &FunctionStmt {
        &self.declaration
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        format!("{:?}", self) == format!("{:?}", other)
    }
    fn ne(&self, other: &Self) -> bool {
        format!("{:?}", self) != format!("{:?}", other)
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<fn {}>", self.declaration.name.lexeme.as_ref().unwrap())
    }
}

impl Callable for Function {
    fn call(
        &self,
        environment: &mut Environment,
        arguments: Vec<LiteralData>,
    ) -> Result<LiteralData, ErrorCascade<InterpreterError>> {
        let mut environment = environment.create_scope();

        let param_iter: Vec<(usize, &Token)> = self.declaration.params.iter().enumerate().collect();
        for (index, param) in param_iter {
            environment.define(
                &param
                    .lexeme
                    .clone()
                    .ok_or(error!(InterpreterError::MissingIdentifier))?,
                Expr::Literal(Box::new(LiteralExpr {
                    literal: arguments
                        .get(index)
                        .ok_or(error!(InterpreterError::InvalidArguments(
                            self.arity(),
                            self.declaration.params.len() as u32
                        )))?
                        .clone(),
                })),
            );
        }

        self.declaration.body.execute(&mut environment)?;
        Ok(LiteralData::None)
    }
}
