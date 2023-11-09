use funlang_derive::Expr;

#[derive(Expr)]
enum _Expr {
    #[production(String, String)]
    Grouped,
    #[production(Grouped)]
    Test,
}

fn main() {
}
