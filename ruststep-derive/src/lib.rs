//! proc-macro for ruststep
//!
//! ```
//! pub struct Table; // moc table struct
//!
//! #[derive(Debug, Clone, PartialEq)]
//! pub struct A {
//!     pub x: f64,
//!     pub y: f64,
//! }
//!
//! #[derive(Debug, Clone, PartialEq, ruststep_derive::Holder)]
//! #[holder(table = Table)]
//! pub struct AHolder {
//!     x: f64,
//!     y: f64,
//! }
//! ```

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};

#[proc_macro_derive(Holder, attributes(holder))]
pub fn derive_holder(input: TokenStream) -> TokenStream {
    impl_holder(&syn::parse(input).unwrap()).into()
}

#[derive(Debug)]
struct TableAttr {
    table: syn::Ident,
    eq: syn::Token![=],
    name: syn::Ident,
}

impl syn::parse::Parse for TableAttr {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
        Ok(TableAttr {
            table: input.parse()?,
            eq: input.parse()?,
            name: input.parse()?,
        })
    }
}

fn get_table_ident(ast: &syn::DeriveInput) -> syn::Ident {
    for attr in &ast.attrs {
        if attr.path.is_ident("holder") {
            let table: TableAttr = attr.parse_args().unwrap();
            return table.name;
        }
    }
    unreachable!()
}

fn impl_holder(ast: &syn::DeriveInput) -> TokenStream2 {
    let table = get_table_ident(ast);
    let ident = &ast.ident;
    match &ast.data {
        syn::Data::Struct(st) => impl_holder_for_struct(ident, &table, st),
        _ => unimplemented!("Only struct is supprted currently"),
    }
}

fn impl_holder_for_struct(
    ident: &syn::Ident,
    table: &syn::Ident,
    st: &syn::DataStruct,
) -> TokenStream2 {
    let visitor_ident = format_ident!("{}Visitor", ident);
    let visitor_tt = visitor_for_struct(ident, st);
    let attr_len = st.fields.len();
    quote! {
        #visitor_tt

        impl<'de> ::serde::de::Deserialize<'de> for #ident {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                use ::ruststep::tables::Holder;
                deserializer.deserialize_tuple_struct(Self::name(), Self::attr_len(), Self::visitor_new())
            }
        }

        impl ::ruststep::tables::Holder for #ident {
            type Table = #table;
            type Owned = A;
            type Visitor = #visitor_ident;
            fn into_owned(self, _tables: &Self::Table) -> ::ruststep::error::Result<A> {
                let #ident { x, y } = self;
                Ok(A { x, y })
            }
            fn name() -> &'static str {
                "A"
            }
            fn attr_len() -> usize {
                #attr_len
            }
            fn visitor_new() -> Self::Visitor {
                #visitor_ident {}
            }
        }
    } // quote!
}

fn visitor_for_struct(ident: &syn::Ident, st: &syn::DataStruct) -> TokenStream2 {
    let name = ident.to_string();
    let visitor_ident = format_ident!("{}Visitor", ident);

    let attrs: Vec<_> = st.fields.iter().map(|field| &field.ident).collect();
    for field in &st.fields {
        dbg!(&field.ident);
    }

    quote! {
        pub struct #visitor_ident;

        impl<'de> ::serde::de::Visitor<'de> for #visitor_ident {
            type Value = #ident;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, #name)
            }

            fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
            where
                A: ::serde::de::SeqAccess<'de>,
            {
                use ::ruststep::tables::Holder;
                if let Some(size) = seq.size_hint() {
                    if size != #ident::attr_len() {
                        todo!("Create another error and send it")
                    }
                }
                #(
                    let #attrs = seq.next_element()?.unwrap();
                )*
                Ok(#ident { #(#attrs),* })
            }

            // Entry point for Record or Parameter::Typed
            fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
            where
                A: ::serde::de::MapAccess<'de>,
            {
                use ::ruststep::tables::Holder;
                let key: String = map
                    .next_key()?
                    .expect("Empty map cannot be accepted as ruststep Holder"); // this must be a bug, not runtime error
                if key != #ident::name() {
                    todo!("Create Error type and send it")
                }
                let value = map.next_value()?; // send to Self::visit_seq
                Ok(value)
            }
        }
    } // quote
}
