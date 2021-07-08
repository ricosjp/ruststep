use proc_macro2::{Span, TokenStream as TokenStream2};
use proc_macro_crate::*;
use quote::{format_ident, quote};

use super::*;

fn holder_ident(ident: &syn::Ident) -> syn::Ident {
    format_ident!("{}Holder", ident)
}

fn holder_visitor_ident(ident: &syn::Ident) -> syn::Ident {
    format_ident!("{}HolderVisitor", ident)
}

/// This must be same between codegens
fn table_arg() -> syn::Ident {
    syn::Ident::new("table", Span::call_site())
}

fn ruststep_path() -> TokenStream2 {
    let path = crate_name("ruststep").unwrap();
    match path {
        FoundCrate::Itself => quote! { crate },
        FoundCrate::Name(name) => {
            let ident = syn::Ident::new(&name, Span::call_site());
            quote! { ::#ident }
        }
    }
}

/// Map `A` to `PlaceHolder<AHolder>`
fn type_to_place_holder(ty: &syn::Type) -> TokenStream2 {
    let ruststep = ruststep_path();
    if let syn::Type::Path(path) = ty {
        let ty = holder_ident(
            // FIXME This should accept path,
            // e.g. `::some_schema::A` to `::some_schema::AHolder`
            path.path
                .get_ident()
                .expect("Member of struct must be an ident"),
        );
        quote! { #ruststep::place_holder::PlaceHolder<#ty> }
    } else {
        panic!("Member of struct must be a Path")
    }
}

fn preprocess_attributes(
    st: &syn::DataStruct,
) -> (Vec<&syn::Ident>, Vec<TokenStream2>, Vec<TokenStream2>) {
    let table_arg = table_arg();

    let mut attrs = Vec::new();
    let mut types = Vec::new();
    let mut into_owned = Vec::new();

    for field in &st.fields {
        let ident = field
            .ident
            .as_ref()
            .expect("Tuple struct case is not supported");
        attrs.push(ident);

        if is_use_place_holder(&field.attrs) {
            types.push(type_to_place_holder(&field.ty));
            into_owned.push(quote! { #ident.into_owned(#table_arg)? })
        } else {
            let ty = &field.ty;
            types.push(quote! { #ty });
            into_owned.push(quote! { #ident })
        }
    }
    (attrs, types, into_owned)
}

pub fn def_holder(ident: &syn::Ident, st: &syn::DataStruct) -> TokenStream2 {
    let holder_ident = holder_ident(ident);
    let (attrs, attr_types, _) = preprocess_attributes(st);
    quote! {
        #[derive(Debug, Clone, PartialEq)]
        pub struct #holder_ident {
            #(#attrs: #attr_types),*
        }
    }
}

pub fn impl_holder(ident: &syn::Ident, table: &TableAttr, st: &syn::DataStruct) -> TokenStream2 {
    let name = ident.to_string();
    let holder_ident = holder_ident(ident);
    let visitor_ident = holder_visitor_ident(ident);
    let (attrs, _, into_owned) = preprocess_attributes(st);
    let attr_len = attrs.len();
    let TableAttr { table, .. } = table;
    let table_arg = table_arg();
    let ruststep = ruststep_path();

    quote! {
        impl #ruststep::tables::Holder for #holder_ident {
            type Table = #table;
            type Owned = #ident;
            type Visitor = #visitor_ident;
            fn into_owned(self, #table_arg: &Self::Table) -> #ruststep::error::Result<Self::Owned> {
                let #holder_ident { #(#attrs),* } = self;
                Ok(#ident { #(#attrs : #into_owned),* })
            }
            fn name() -> &'static str {
                #name
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

pub fn impl_deserialize(ident: &syn::Ident) -> TokenStream2 {
    let holder_ident = holder_ident(ident);
    let ruststep = ruststep_path();
    quote! {
        impl<'de> ::serde::de::Deserialize<'de> for #holder_ident {
            fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
            where
                D: ::serde::de::Deserializer<'de>,
            {
                use #ruststep::tables::Holder;
                deserializer.deserialize_tuple_struct(Self::name(), Self::attr_len(), Self::visitor_new())
            }
        }
    } // quote!
}

pub fn def_visitor(ident: &syn::Ident, st: &syn::DataStruct) -> TokenStream2 {
    let name = ident.to_string();
    let holder_ident = holder_ident(ident);
    let visitor_ident = holder_visitor_ident(ident);
    let (attrs, _, _) = preprocess_attributes(st);
    let ruststep = ruststep_path();
    quote! {
        pub struct #visitor_ident;

        impl<'de> ::serde::de::Visitor<'de> for #visitor_ident {
            type Value = #holder_ident;
            fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(formatter, #name)
            }

            fn visit_seq<A>(self, mut seq: A) -> ::std::result::Result<Self::Value, A::Error>
            where
                A: ::serde::de::SeqAccess<'de>,
            {
                use #ruststep::tables::Holder;
                if let Some(size) = seq.size_hint() {
                    if size != #holder_ident::attr_len() {
                        todo!("Create another error and send it")
                    }
                }
                #(
                    let #attrs = seq.next_element()?.unwrap();
                )*
                Ok(#holder_ident { #(#attrs),* })
            }

            // Entry point for Record or Parameter::Typed
            fn visit_map<A>(self, mut map: A) -> ::std::result::Result<Self::Value, A::Error>
            where
                A: ::serde::de::MapAccess<'de>,
            {
                use #ruststep::tables::Holder;
                let key: String = map
                    .next_key()?
                    .expect("Empty map cannot be accepted as ruststep Holder"); // this must be a bug, not runtime error
                if key != #holder_ident::name() {
                    todo!("Create Error type and send it")
                }
                let value = map.next_value()?; // send to Self::visit_seq
                Ok(value)
            }
        }
    } // quote
}

pub fn impl_entity_table(ident: &syn::Ident, table: &TableAttr) -> TokenStream2 {
    let TableAttr { table, .. } = table;
    let holder_ident = holder_ident(ident);
    let ruststep = ruststep_path();

    quote! {
        impl #ruststep::tables::EntityTable<#holder_ident> for #table {
            fn get_owned(&self, entity_id: u64) -> #ruststep::error::Result<#ident> {
                todo!()
            }
            fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = #ruststep::error::Result<#ident>> + 'table> {
                todo!()
            }
        }
    }
}
