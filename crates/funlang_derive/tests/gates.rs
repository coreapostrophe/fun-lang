#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/expr.rs");
    t.pass("tests/error.rs");
}
