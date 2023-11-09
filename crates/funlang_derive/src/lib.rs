use syn::{parse_macro_input, DeriveInput};

mod generate;

#[proc_macro_derive(Expr, attributes(production))]
pub fn derive_expression(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let expanded = generate::generate_struct(parsed_input);
    proc_macro::TokenStream::from(expanded)
}
