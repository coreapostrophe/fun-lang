use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Meta, Variant};

pub fn generate_error(input: DeriveInput) -> TokenStream {
    let identifier = input.ident;
    let generics = add_trait_bounds(input.generics);
    let match_arms = build_match_arms(&input.data);

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

        impl #impl_generics std::fmt::Display for #identifier #type_generics #where_clause {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let error_message: String = match self {
                    #(#match_arms)*
                };
                core::write!(f, "{}", error_message)
            }
        }

        impl #impl_generics std::error::Error for #identifier #type_generics #where_clause {}
    )
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in &mut generics.params {
        if let GenericParam::Type(ref mut type_param) = *param {
            type_param.bounds.push(parse_quote!(std::fmt::Debug));
        }
    }
    generics
}

fn build_match_arms(data: &Data) -> Vec<TokenStream> {
    let mut match_arms = Vec::<TokenStream>::new();
    match data {
        Data::Enum(enum_data) => {
            let variants = &enum_data.variants;
            for variant in variants.iter() {
                match_arms.push(build_arm(variant));
            }
        }
        Data::Struct(_) => (),
        Data::Union(_) => (),
    }
    match_arms
}

fn build_arm(variant: &Variant) -> TokenStream {
    let identifier = &variant.ident;
    let error_message = match variant.attrs.get(0) {
        Some(attribute) => match &attribute.meta {
            Meta::NameValue(meta_name_value) => {
                let expr = &meta_name_value.value;
                quote!(#expr)
            }
            Meta::List(_) => quote!(),
            Meta::Path(_) => quote!(),
        },
        _ => quote!(),
    };
    let has_discriminant = match variant.fields {
        Fields::Unnamed(_) => true,
        Fields::Named(_) => false,
        Fields::Unit => false,
    };
    let determinant_initiator = if has_discriminant {
        quote!((meta))
    } else {
        quote!()
    };
    let determinant_argument = if has_discriminant {
        quote!(Some(&meta),)
    } else {
        quote!(None,)
    };

    quote!(
        Self::#identifier #determinant_initiator => self.format_error(#determinant_argument #error_message),
    )
}
