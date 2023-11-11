use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput};

pub fn generate_error(input: DeriveInput) -> TokenStream {
    let data = &input.data;
    let generated_struct = handle_data(data);
    quote!(#generated_struct)
}

fn handle_data(_data: &Data) -> Option<TokenStream> {
    Some(quote!())
}
