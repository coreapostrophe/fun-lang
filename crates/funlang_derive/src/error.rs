use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse_quote, parse_str, Data, DeriveInput, Expr, Fields, GenericParam, Generics, Meta, Variant,
};

pub fn generate_error(input: DeriveInput) -> TokenStream {
    let identifier = input.ident;
    let generics = add_trait_bounds(input.generics);
    let display_arms = build_display_arms(&input.data);
    let debug_arms = build_debug_arms(&input.data);

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote!(
        impl #impl_generics std::fmt::Debug for #identifier #type_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let error_message: String = match self {
                    #(#debug_arms)*
                };
                core::write!(f, "{}", error_message)
            }
        }

        impl #impl_generics std::fmt::Display for #identifier #type_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let error_message: String = match self {
                    #(#display_arms)*
                };
                core::write!(f, "{}", error_message)
            }
        }

        impl #impl_generics funlang_error::ErrorType for #identifier #type_generics #where_clause {}
    )
}

fn build_debug_arms(data: &Data) -> Vec<TokenStream> {
    let mut match_arms = Vec::<TokenStream>::new();
    match data {
        Data::Enum(enum_data) => {
            let variants = &enum_data.variants;
            for variant in variants.iter() {
                let variant_identifier = &variant.ident;
                let stringified_identifier = variant_identifier.to_string();
                let match_arm = match &variant.fields {
                    Fields::Unnamed(..) => {
                        quote!(Self::#variant_identifier(..)  => #stringified_identifier.to_string(),)
                    }
                    _ => quote!(Self::#variant_identifier  => #stringified_identifier.to_string(),),
                };
                match_arms.push(match_arm);
            }
        }
        Data::Struct(..) => (),
        Data::Union(..) => (),
    }
    match_arms
}

fn build_display_arms(data: &Data) -> Vec<TokenStream> {
    let mut match_arms = Vec::<TokenStream>::new();
    match data {
        Data::Enum(enum_data) => {
            let variants = &enum_data.variants;
            for variant in variants.iter() {
                match_arms.push(build_display_arm(variant));
            }
        }
        Data::Struct(..) => (),
        Data::Union(..) => (),
    }
    match_arms
}

fn build_display_arm(variant: &Variant) -> TokenStream {
    let identifier = &variant.ident;
    let error_message = match variant.attrs.get(0) {
        Some(attribute) => match &attribute.meta {
            Meta::NameValue(meta_name_value) => {
                let expr = &meta_name_value.value;
                quote!(#expr)
            }
            _ => quote!(""),
        },
        _ => quote!(""),
    };
    let format_args = build_format_args(&variant.fields);
    match format_args {
        Some(format_args) => {
            quote!(Self::#identifier (#(#format_args)*) => format!("{}", format_args!(#error_message, #(#format_args)*)),)
        }
        None => quote!(Self::#identifier => #error_message.to_string(),),
    }
}

fn build_format_args(fields: &Fields) -> Option<Vec<TokenStream>> {
    match fields {
        Fields::Unnamed(fields) => Some(
            fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, _field)| {
                    let field_name: Expr = parse_str(&format!("field{}", index)).unwrap();
                    quote!(#field_name,)
                })
                .collect(),
        ),
        Fields::Named(..) => None,
        Fields::Unit => None,
    }
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(std::fmt::Debug));
        }
    }
    generics
}
