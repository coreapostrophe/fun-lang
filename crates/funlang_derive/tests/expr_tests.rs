use funlang_derive::Expr;

#[derive(Expr)]
enum _Example {
    #[production(Expr, Object, Expr)]
    Hello
}