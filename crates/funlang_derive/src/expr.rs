use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Attribute, Data, DeriveInput, Expr, Meta, Variant};

pub fn generate_expr(input: DeriveInput) -> TokenStream {
    let data = input.data;
    let generated_struct = handle_data(data);
    quote!(#generated_struct)
}

pub fn handle_data(data: Data) -> Option<TokenStream> {
    match data {
        Data::Enum(data_enum) => {
            let variants = data_enum.variants;
            let struct_definitions = variants.iter().map(|variant| build_struct(variant));
            Some(quote!(#(#struct_definitions)*))
        }
        Data::Struct(_) => None,
        Data::Union(_) => None,
    }
}

pub fn build_struct(variant: &Variant) -> TokenStream {
    let identifier = &variant.ident;
    let formatted_identifier: Expr = {
        let identifier_string = &identifier.to_string();
        let formatted_identifier_string = format!("{}Expr", identifier_string);
        syn::parse_str(&formatted_identifier_string).unwrap()
    };
    let fields = match variant.attrs.get(0) {
        Some(attribute) => build_fields(attribute),
        _ => None,
    };

    match fields {
        Some(fields) => quote_spanned!(identifier.span() =>
            #[derive(std::fmt::Debug, core::clone::Clone)]
            pub struct #formatted_identifier {
                #(#fields)*
            }
        ),
        None => quote!(),
    }
}

pub fn build_fields(attribute: &Attribute) -> Option<Vec<TokenStream>> {
    let meta = &attribute.meta;
    match meta {
        Meta::List(meta_list) => {
            let tokens_string = &meta_list.tokens.to_string();
            let token_list: Vec<TokenStream> = tokens_string
                .split(',')
                .map(|named_value| {
                    let trimmed_named_value = named_value.trim();
                    let splitted_named_value: Vec<String> = trimmed_named_value
                        .split(':')
                        .map(|s| s.trim().to_string())
                        .collect();
                    let identifier = splitted_named_value.get(0).unwrap();
                    let identifier: Expr = syn::parse_str(&identifier).unwrap();
                    let parsed_type = splitted_named_value.get(1).unwrap();
                    let parsed_type: Expr = syn::parse_str(&parsed_type).unwrap();
                    quote!(pub #identifier : #parsed_type,)
                })
                .collect();
            Some(token_list)
        }
        Meta::NameValue(_) => None,
        Meta::Path(_) => None,
    }
}
