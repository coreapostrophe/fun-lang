use funlang_derive::Expr;
use funlang_error::ErrorMeta;

use crate::{
    errors::ParserError,
    literal::LiteralData,
    token::{Token, TokenType},
};

trait Evaluable<R> {
    fn evaluate(&self) -> Result<R, ParserError>;
}

#[derive(Expr, Debug, Clone)]
pub enum Expr {
    #[production(left:Expr, operator:Token, right:Expr)]
    Binary(Box<BinaryExpr>),

    #[production(expression:Expr)]
    Grouping(Box<GroupingExpr>),

    #[production(literal:LiteralData)]
    Literal(Box<LiteralExpr>),

    #[production(operator:Token, right:Expr)]
    Unary(Box<UnaryExpr>),
}

impl Evaluable<LiteralData> for Expr {
    fn evaluate(&self) -> Result<LiteralData, ParserError> {
        match self {
            Expr::Unary(unary_expr) => unary_expr.evaluate(),
            Expr::Binary(binary_expr) => binary_expr.evaluate(),
            Expr::Literal(literal_expr) => literal_expr.evaluate(),
            Expr::Grouping(grouping_expr) => grouping_expr.evaluate(),
        }
    }
}

impl Evaluable<LiteralData> for LiteralExpr {
    fn evaluate(&self) -> Result<LiteralData, ParserError> {
        Ok(self.literal.clone())
    }
}

impl Evaluable<LiteralData> for GroupingExpr {
    fn evaluate(&self) -> Result<LiteralData, ParserError> {
        self.expression.evaluate()
    }
}

impl Evaluable<LiteralData> for BinaryExpr {
    fn evaluate(&self) -> Result<LiteralData, ParserError> {
        let _left = self.left.evaluate()?;
        let _right = self.right.evaluate()?;
        let operator = &self.operator.token_type;

        match operator {
            TokenType::Plus => match _left + _right {
                Ok(literal_value) => Ok(literal_value),
                Err(embedded_error) => Err(ParserError::AdditionException(ErrorMeta {
                    span: None,
                    embedded_error: Some(Box::new(embedded_error)),
                })),
            },
            TokenType::Minus => {
                todo!()
            }
            TokenType::Star => {
                todo!()
            }
            TokenType::Slash => {
                todo!()
            }
            _ => {
                todo!()
            }
        }
    }
}

impl Evaluable<LiteralData> for UnaryExpr {
    fn evaluate(&self) -> Result<LiteralData, ParserError> {
        let right = self.right.evaluate()?;
        let span = self
            .operator
            .span
            .as_ref()
            .ok_or(ParserError::MissingSpan)?
            .clone();
        let operator = &self.operator.token_type;

        match operator {
            TokenType::Bang => match right {
                LiteralData::Null => Ok(LiteralData::Bool(true)),
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
                LiteralData::Null => Ok(LiteralData::Bool(true)),
                LiteralData::Number(number) => Ok(LiteralData::Number(-number)),
                LiteralData::Bool(_) => Err(ParserError::NegatedBoolean(ErrorMeta::new(
                    Some(span.clone().into()),
                    None,
                ))),
                LiteralData::String(string) => match string.parse::<f32>() {
                    Ok(parsed_number) => Ok(LiteralData::Number(parsed_number)),
                    Err(_) => Err(ParserError::InvalidNumber(ErrorMeta::new(
                        Some(span.clone().into()),
                        None,
                    ))),
                },
            },
            _ => Err(ParserError::InvalidUnaryOperator(ErrorMeta::new(
                Some(span.clone().into()),
                None,
            ))),
        }
    }
}
