#[derive(funlang_derive::Expr)]
enum _Expr {
    #[production(String = left, String = right)]
    Grouped,
}

fn main() {}
