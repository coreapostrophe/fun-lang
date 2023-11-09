#[derive(funlang_derive::Expr)]
enum _Expr {
    #[production(left:String, right:String)]
    Grouped,
}

fn main() {}
