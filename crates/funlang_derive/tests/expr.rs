#[derive(funlang_derive::Expr)]
enum _Expr {
    #[production = ()]
    Grouped,
    #[production(GroupedExpr)]
    Test,
}

fn main() {}
