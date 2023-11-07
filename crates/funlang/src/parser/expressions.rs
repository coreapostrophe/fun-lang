use crate::token::Token;

pub enum Expr {}

pub struct Binary {
    pub left: Expr,
    pub operand: Token,
    pub right: Expr,
}

impl Binary {
    pub fn new(left: Expr, operand: Token, right: Expr) -> Self {
        Self {
            left,
            operand,
            right,
        }
    }
}
