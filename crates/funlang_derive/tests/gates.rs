#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/ast_test.rs");
    t.pass("tests/error_test.rs");
}
