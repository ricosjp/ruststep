use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[proc_macro_derive(Deserialize)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    impl_deserialize(&syn::parse(input).unwrap()).into()
}

fn impl_deserialize(ast: &syn::DeriveInput) -> TokenStream2 {
    let ident = &ast.ident;
    dbg!(ident);
    match &ast.data {
        syn::Data::Struct(st) => impl_deserialize_for_struct(st),
        syn::Data::Enum(e) => impl_deserialize_for_enum(e),
        _ => unimplemented!("Union is not supported"),
    }
}

fn impl_deserialize_for_struct(st: &syn::DataStruct) -> TokenStream2 {
    dbg!(st);
    quote! {}
}

fn impl_deserialize_for_enum(e: &syn::DataEnum) -> TokenStream2 {
    dbg!(e);
    quote! {}
}
