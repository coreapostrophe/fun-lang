#[derive(funlang_derive::Ast, Debug)]
enum Expr {
    #[production(left: String, right: core::option::Option<String>)]
    Grouped(Box<GroupedExpr>),
}

fn main() {
    let _expr = Expr::Grouped(Box::new(GroupedExpr {
        left: "mock_left".to_string(),
        right: Some("mock_right".to_string()),
    }));

    // println!("{:?}", expr);
}
