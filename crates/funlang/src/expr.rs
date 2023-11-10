use funlang_derive::Expr;

use crate::{
    errors::parser_errors::ParserError,
    token::{LiteralData, Token, TokenType},
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
        unimplemented!()
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

impl Evaluable<LiteralData> for UnaryExpr {
    fn evaluate(&self) -> Result<LiteralData, ParserError> {
        let right = self.right.evaluate()?;
        let span = self
            .operator
            .span
            .as_ref()
            .ok_or(ParserError::MissingSpan)?
            .clone();
        let token_type = &self.operator.token_type;

        match token_type {
            TokenType::Bang => Err(ParserError::NegatedBoolean(span.clone())),
            TokenType::Minus => match right {
                LiteralData::Null => Ok(LiteralData::Bool(true)),
                LiteralData::Number(number) => Ok(LiteralData::Number(-number)),
                LiteralData::Bool(_) => Err(ParserError::NegatedBoolean(span.clone())),
                LiteralData::Identifier(_) => Err(ParserError::NegatedIdentifier(span.clone())),
                LiteralData::String(string) => match string.parse::<f32>() {
                    Ok(parsed_number) => Ok(LiteralData::Number(parsed_number)),
                    Err(_) => Err(ParserError::InvalidNumber(span.clone())),
                },
            },
            _ => Err(ParserError::UnexpectedUnaryOperator(span.clone())),
        }
    }
}
