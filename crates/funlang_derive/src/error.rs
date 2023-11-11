use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Meta, Variant};

pub fn generate_error(input: DeriveInput) -> TokenStream {
    let identifier = input.ident;
    let generics = add_trait_bounds(input.generics);
    let display_arms = build_display_arms(&input.data);
    let debug_arms = build_debug_arms(&input.data);

    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();

    quote!(
        impl #impl_generics #identifier #type_generics #where_clause {
            fn format_error(&self, meta: Option<&funlang_error::ErrorMeta>, message: &str) -> String {
                match &meta {
                    Some(meta) => match &meta.span {
                        Some(span) => match &meta.error {
                            Some(error) => error.to_string(),
                            None => std::format!("[line {}:{} - {:?}] {}", span.line, span.col, &self, message)
                        },
                        None => match &meta.error {
                            Some(error) => std::format!("[{:?}] {}", &error, message),
                            None => std::format!("[{:?}] {}", &self, message)
                        },
                    }
                    None => std::format!("[{:?}] {}", &self, message)
                }
            }
        }

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

        impl #impl_generics std::error::Error for #identifier #type_generics #where_clause {}
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
                let discriminant_initiator =
                    discriminant_switch(&variant.fields, quote!((_)), quote!());
                let match_arm = quote!(
                    Self::#variant_identifier #discriminant_initiator  => #stringified_identifier.to_string(),
                );
                match_arms.push(match_arm);
            }
        }
        Data::Struct(_) => (),
        Data::Union(_) => (),
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
        Data::Struct(_) => (),
        Data::Union(_) => (),
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
            Meta::List(_) => quote!(""),
            Meta::Path(_) => quote!(""),
        },
        _ => quote!(),
    };
    let discriminant_initiator = discriminant_switch(&variant.fields, quote!((meta)), quote!());
    let discriminant_argument =
        discriminant_switch(&variant.fields, quote!(Some(&meta),), quote!(None,));
    quote!(Self::#identifier #discriminant_initiator => self.format_error(#discriminant_argument #error_message),)
}

fn discriminant_switch(
    fields: &Fields,
    true_value: TokenStream,
    false_value: TokenStream,
) -> TokenStream {
    match fields {
        Fields::Unnamed(_) => true_value,
        Fields::Named(_) => false_value,
        Fields::Unit => false_value,
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
