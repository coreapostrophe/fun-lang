use funlang_derive::Expr;

use crate::token::Token;

#[derive(Expr)]
pub enum Expr {
    #[production(left:Token, token:Token, right:Expr)]
    Binary(Box<BinaryExpr>),

    #[production(expression:Expr)]
    Grouping(Box<GroupingExpr>),

    #[production(operator:Token, right:Expr)]
    Unary(Box<UnaryExpr>),
}
