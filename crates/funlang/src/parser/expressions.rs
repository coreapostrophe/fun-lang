use funlang_derive::Expr;

use crate::token::Token;

#[derive(Expr)]
pub enum Expr {
    #[production(Expr, Token, Expr)]
    Binary(Box<BinaryExpr>),
    #[production(Expr)]
    Grouping(Box<GroupingExpr>),
    #[production(Token, Expr)]
    Unary(Box<UnaryExpr>),
}