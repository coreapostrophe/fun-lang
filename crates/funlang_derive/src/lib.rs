use syn::{parse_macro_input, DeriveInput};

mod expr;

#[proc_macro_derive(Expr, attributes(production))]
pub fn derive_expression(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let expanded = expr::generate_expr(parsed_input);
    proc_macro::TokenStream::from(expanded)
}
