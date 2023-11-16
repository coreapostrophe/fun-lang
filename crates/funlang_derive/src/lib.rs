use syn::{parse_macro_input, DeriveInput};

mod error;
mod ast;

#[proc_macro_derive(Ast, attributes(production))]
pub fn derive_ast(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let expanded = ast::generate_ast(parsed_input);
    proc_macro::TokenStream::from(expanded)
}

#[proc_macro_derive(Error, attributes(message))]
pub fn derive_error(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let parsed_input = parse_macro_input!(input as DeriveInput);
    let expanded = error::generate_error(parsed_input);
    proc_macro::TokenStream::from(expanded)
}
