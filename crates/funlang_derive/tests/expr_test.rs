#[derive(funlang_derive::Ast, Debug)]
enum Expr {
    #[production(left:String, right:String)]
    Grouped(Box<GroupedExpr>),
}

fn main() {
    let expr = Expr::Grouped(Box::new(GroupedExpr {
        left: "mock_left".to_string(),
        right: "mock_right".to_string(),
    }));

    println!("{:?}", expr);
}
