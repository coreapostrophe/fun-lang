use funlang_derive::Ast;
use funlang_error::ErrorCascade;

use crate::{
    error,
    errors::InterpreterError,
    literal::LiteralData,
    parse_string_to_num,
    token::{Token, TokenType}, environment::Environment,
};

use super::traits::Evaluable;

#[derive(Ast, Debug, Clone)]
pub enum Expr {
    #[production(name: Token, value: Expr)]
    Assign(Box<AssignExpr>),

    #[production(left: Expr, operator: Token, right: Expr)]
    Binary(Box<BinaryExpr>),

    #[production(expression: Expr)]
    Grouping(Box<GroupingExpr>),

    #[production(literal: LiteralData)]
    Literal(Box<LiteralExpr>),

    #[production(operator: Token, right: Expr)]
    Unary(Box<UnaryExpr>),

    #[production(name: Token)]
    Variable(Box<VariableExpr>),

    #[production(left: Expr, operator: Token, right: Expr)]
    Logical(Box<LogicalExpr>),
}

impl Evaluable<LiteralData> for Expr {
    fn evaluate(
        &self,
        environment: &mut Environment,
    ) -> Result<LiteralData, ErrorCascade<InterpreterError>> {
        match self {
            Expr::Unary(unary_expr) => unary_expr.evaluate(environment),
            Expr::Binary(binary_expr) => binary_expr.evaluate(environment),
            Expr::Literal(literal_expr) => literal_expr.evaluate(environment),
            Expr::Grouping(grouping_expr) => grouping_expr.evaluate(environment),
            Expr::Variable(variable_expr) => variable_expr.evaluate(environment),
            Expr::Assign(assignment_expr) => assignment_expr.evaluate(environment),
            Expr::Logical(logical_expr) => logical_expr.evaluate(environment),
        }
    }
}

impl Evaluable<LiteralData> for LogicalExpr {
    fn evaluate(
        &self,
        environment: &mut Environment,
    ) -> Result<LiteralData, ErrorCascade<InterpreterError>> {
        match &self.operator.token_type {
            TokenType::Or => Ok(LiteralData::Bool(
                self.left.evaluate(environment)?.is_truthy()?
                    || self.right.evaluate(environment)?.is_truthy()?,
            )),
            TokenType::And => Ok(LiteralData::Bool(
                self.left.evaluate(environment)?.is_truthy()?
                    && self.right.evaluate(environment)?.is_truthy()?,
            )),
            _ => Ok(LiteralData::Bool(false)),
        }
    }
}

impl Evaluable<LiteralData> for AssignExpr {
    fn evaluate(
        &self,
        _environment: &mut Environment,
    ) -> Result<LiteralData, ErrorCascade<InterpreterError>> {
        // let name = self
        //     .name
        //     .clone()
        //     .lexeme
        //     .ok_or(error!(InterpreterError::MissingIdentifier))?;
        // let variable = environment
        //     .mut_variable(&name)
        //     .ok_or(error!(InterpreterError::InvalidIdentifier(name)))?;
        // *variable = self.value.clone();

        // Ok(LiteralData::None)
        todo!()
    }
}

impl Evaluable<LiteralData> for VariableExpr {
    fn evaluate(
        &self,
        _environment: &mut Environment,
    ) -> Result<LiteralData, ErrorCascade<InterpreterError>> {
        // let identifier = self
        //     .name
        //     .lexeme
        //     .clone()
        //     .ok_or(error!(InterpreterError::MissingIdentifier))?;
        // let expression = environment
        //     .variable(&identifier)
        //     .ok_or(error!(InterpreterError::InvalidIdentifier(identifier)))?
        //     .clone();
        // Ok(expression.evaluate(environment)?)
        todo!()
    }
}

impl Evaluable<LiteralData> for LiteralExpr {
    fn evaluate(
        &self,
        _environment: &mut Environment,
    ) -> Result<LiteralData, ErrorCascade<InterpreterError>> {
        Ok(self.literal.clone())
    }
}

impl Evaluable<LiteralData> for GroupingExpr {
    fn evaluate(
        &self,
        environment: &mut Environment,
    ) -> Result<LiteralData, ErrorCascade<InterpreterError>> {
        self.expression.evaluate(environment)
    }
}

impl Evaluable<LiteralData> for BinaryExpr {
    fn evaluate(
        &self,
        environment: &mut Environment,
    ) -> Result<LiteralData, ErrorCascade<InterpreterError>> {
        let left = self.left.evaluate(environment)?;
        let right = self.right.evaluate(environment)?;
        let operator = &self.operator.token_type;
        let span = self
            .operator
            .span
            .as_ref()
            .ok_or(error!(InterpreterError::MissingSpan))?
            .clone();

        match operator {
            TokenType::Plus => match left + right {
                Ok(literal_value) => Ok(literal_value),
                Err(embedded_error) => Err(error!(InterpreterError::AdditionException)
                    .set_embedded_error(Box::new(embedded_error))),
            },
            TokenType::Minus => match left - right {
                Ok(literal_value) => Ok(literal_value),
                Err(embedded_error) => Err(error!(InterpreterError::SubtractionException)
                    .set_embedded_error(Box::new(embedded_error))),
            },
            TokenType::Star => match left * right {
                Ok(literal_value) => Ok(literal_value),
                Err(embedded_error) => Err(error!(InterpreterError::MultiplicationException)
                    .set_embedded_error(Box::new(embedded_error))),
            },
            TokenType::Slash => match left / right {
                Ok(literal_value) => Ok(literal_value),
                Err(embedded_error) => Err(error!(InterpreterError::DivisionException)
                    .set_embedded_error(Box::new(embedded_error))),
            },
            TokenType::Greater => Ok(LiteralData::Bool(left > right)),
            TokenType::GreaterEqual => Ok(LiteralData::Bool(left >= right)),
            TokenType::Less => Ok(LiteralData::Bool(left < right)),
            TokenType::LessEqual => Ok(LiteralData::Bool(left <= right)),
            TokenType::BangEqual => Ok(LiteralData::Bool(left != right)),
            TokenType::EqualEqual => Ok(LiteralData::Bool(left == right)),
            token_type => Err(error!(InterpreterError::InvalidBinaryOperator(
                token_type.to_string()
            ))
            .set_span(span.into())),
        }
    }
}

impl Evaluable<LiteralData> for UnaryExpr {
    fn evaluate(
        &self,
        environment: &mut Environment,
    ) -> Result<LiteralData, ErrorCascade<InterpreterError>> {
        let right = self.right.evaluate(environment)?;
        let span = self
            .operator
            .span
            .as_ref()
            .ok_or(error!(InterpreterError::MissingSpan))?
            .clone();
        let operator = &self.operator.token_type;

        match operator {
            TokenType::Bang => match right {
                LiteralData::None => Ok(LiteralData::Bool(true)),
                LiteralData::Bool(bool) => Ok(LiteralData::Bool(!bool)),
                LiteralData::Number(number) => {
                    if number != 0.0 {
                        Ok(LiteralData::Bool(true))
                    } else {
                        Ok(LiteralData::Bool(false))
                    }
                }
                LiteralData::String(string) => {
                    if string != "".to_string() {
                        Ok(LiteralData::Bool(true))
                    } else {
                        Ok(LiteralData::Bool(false))
                    }
                }
            },
            TokenType::Minus => match right {
                LiteralData::None => Ok(LiteralData::Bool(true)),
                LiteralData::Number(number_value) => Ok(LiteralData::Number(-number_value)),
                LiteralData::Bool(boolean_value) => {
                    let boolean_value = if boolean_value { 1.0 } else { 0.0 };
                    Ok(LiteralData::Number(-boolean_value))
                }
                LiteralData::String(string_value) => {
                    let parsed_number = parse_string_to_num!(
                        string_value,
                        error!(InterpreterError::InvalidParsedNumber(string_value))
                    )?;
                    Ok(LiteralData::Number(parsed_number))
                }
            },
            token_type => Err(error!(InterpreterError::InvalidUnaryOperator(
                token_type.to_string()
            ))
            .set_span(span.into())),
        }
    }
}
