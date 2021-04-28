//! ruststep-derive

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;

#[proc_macro_derive(Holder)]
pub fn holder(input: TokenStream) -> TokenStream {
    let item: syn::Item = syn::parse(input).expect("#[derive(Holder)] must be used for item");
    match item {
        syn::Item::Struct(item) => struct_holder(item),
        syn::Item::Enum(item) => enum_holder(item),
        _ => panic!("Must be struct or enum"),
    }
    .into()
}

fn struct_holder(input: syn::ItemStruct) -> TokenStream2 {
    match input.fields {
        syn::Fields::Named(fields) => {
            for field in fields.named {
                dbg!(field);
            }
        }
        syn::Fields::Unnamed(fields) => {
            for field in fields.unnamed {
                dbg!(field);
            }
        }
        _ => panic!(),
    }
    todo!()
}

fn enum_holder(_input: syn::ItemEnum) -> TokenStream2 {
    unimplemented!("#[derive(Holder)] for enum is not implemented yet")
}
