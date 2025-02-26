extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Expr, Fields, Item, Lit};

// TODO: clean up this mess.

/// Helps read and write serialized structs. These are structs that have named fields with known types.
/// Note: This adds a hidden "None" at the end of the struct as well.
#[proc_macro_attribute]
pub fn serialized_struct(_metadata: TokenStream, input: TokenStream)
                  -> TokenStream {
    let mut input = syn::parse_macro_input!(input as syn::ItemStruct);

    for mut field in &mut input.fields {
        let our_custom = &field.attrs[0];
        let our_custom_name = our_custom.meta.require_name_value().unwrap();
        let our_custom_name = match &our_custom_name.value {
            Expr::Lit(_0) => {
                match &_0.lit {
                    Lit::Str(_0) => {
                        Some(_0.value())
                    }
                    _ => None
                }
            }
            _ => None
        }.unwrap();
        field.attrs.clear();
        let field_tokens = field.to_token_stream();
        let new_field_token_stream = quote! {
            #[br(parse_with = crate::structs::read_struct_field, args(#our_custom_name))]
            #field_tokens
        };
        let buffer = ::syn::parse::Parser::parse2(
            syn::Field::parse_named,
            new_field_token_stream,
        ).unwrap();
        *field = buffer;
    }

    // Add "None" field
    let none_field_stream = quote! {
        #[br(temp)]
        #[bw(ignore)]
        none_field: crate::structs::PrimaryAssetNameProperty
    };
    let buffer = ::syn::parse::Parser::parse2(
        syn::Field::parse_named,
        none_field_stream,
    ).unwrap();
    match &mut input.fields {
        Fields::Named(_0) => {
            _0.named.push(buffer)
        }
        _ => {}
    }

    let output = quote! {
        #[binrw]
        #input
    };

    output.into_token_stream().into()
}

/// Denotes a field with a known name.
#[proc_macro_attribute]
pub fn serialized_field(attr: TokenStream, input: TokenStream) -> TokenStream {
    let input2 = parse_macro_input!(input as Item);

    let output = quote! {
        #[binrw]
        #input2
    };

    output.into_token_stream().into()
}
