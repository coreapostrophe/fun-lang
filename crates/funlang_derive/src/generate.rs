use proc_macro2::TokenStream;
use quote::{quote, quote_spanned};
use syn::{Attribute, Data, DeriveInput, Expr, Meta, Variant};

pub fn generate_struct(input: DeriveInput) -> TokenStream {
    let data = &input.data;
    let generated_struct = parse_data(data);
    quote!(#generated_struct)
}

pub fn parse_data(data: &Data) -> Option<TokenStream> {
    match data {
        Data::Enum(data_enum) => {
            let variants = &data_enum.variants;
            let struct_definitions = variants.iter().map(|variant| parse_variant(variant));
            Some(quote!(#(#struct_definitions)*))
        }
        Data::Struct(_) => None,
        Data::Union(_) => None,
    }
}

pub fn parse_variant(variant: &Variant) -> TokenStream {
    let identifier = &variant.ident;
    let formatted_identifier: Expr = {
        let identifier_string = &identifier.to_string();
        let formatted_identifier_string = format!("{}Expr", identifier_string);
        syn::parse_str(&formatted_identifier_string).unwrap()
    };
    let fields = match variant.attrs.get(0) {
        Some(attribute) => parse_fields(attribute),
        _ => None,
    };

    match fields {
        Some(fields) => quote_spanned!(identifier.span() =>
            pub struct #formatted_identifier {
                #(#fields)*
            }
        ),
        None => quote!(),
    }
}

pub fn parse_fields(attribute: &Attribute) -> Option<Vec<TokenStream>> {
    let meta = &attribute.meta;
    match meta {
        Meta::List(meta_list) => {
            let tokens_string = &meta_list.tokens.to_string();
            let token_list: Vec<TokenStream> = tokens_string
                .split(',')
                .enumerate()
                .map(|(index, s)| {
                    let trimmed_str = s.trim();
                    let snaked_str = capital_to_snake(trimmed_str);
                    let parsed_snaked_str: Expr =
                        syn::parse_str(&format!("{}{}", snaked_str, index)).unwrap();
                    let parsed_trimmed_str: Expr = syn::parse_str(&trimmed_str).unwrap();
                    quote!(pub #parsed_snaked_str : #parsed_trimmed_str,)
                })
                .collect();
            Some(token_list)
        }
        Meta::NameValue(_) => None,
        Meta::Path(_) => None,
    }
}

fn capital_to_snake(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}
