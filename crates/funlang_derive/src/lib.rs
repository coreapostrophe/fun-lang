use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Expr, attributes(production))]
pub fn derive_expression(input: TokenStream) -> TokenStream {

    let _input = parse_macro_input!(input as DeriveInput);

    let _expanded = quote!{

    };
    
    TokenStream::new()
}