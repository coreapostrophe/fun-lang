#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/expr_test.rs");
    t.pass("tests/error_test.rs");
}
