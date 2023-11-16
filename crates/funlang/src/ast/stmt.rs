use crate::ast::expr::Expr;
use funlang_derive::Ast;

#[derive(Ast, Debug, Clone)]
pub enum Stmt {
    #[production(expression:Expr)]
    Expression(Box<ExpressionStmt>),

    #[production(expression:Expr)]
    Print(Box<PrintStmt>),
}
