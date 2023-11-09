use funlang_derive::Expr;

use crate::token::{LiteralData, Token};

#[derive(Expr)]
pub enum Expr {
    #[production(left:Token, token:Token, right:Expr)]
    Binary(Box<BinaryExpr>),

    #[production(expression:Expr)]
    Grouping(Box<GroupingExpr>),

    #[production(literal:LiteralData)]
    Literal(Box<LiteralExpr>),

    #[production(operator:Token, right:Expr)]
    Unary(Box<UnaryExpr>),
}
