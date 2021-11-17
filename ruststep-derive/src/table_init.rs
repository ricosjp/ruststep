use inflector::Inflector;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::{abort_call_site, OptionExt};
use quote::quote;

use crate::ruststep_crate;

pub fn derive_table_init(ast: &syn::DeriveInput) -> TokenStream2 {
    let ident = &ast.ident;
    match &ast.data {
        syn::Data::Struct(st) => match st.fields {
            syn::Fields::Named(_) => entity_impl_table_init(ident, st),
            syn::Fields::Unnamed(_) => tuple_impl_table_init(ident, st),
            syn::Fields::Unit => panic!("Unit struct is not supported."),
        },
        _ => abort_call_site!("Only struct is supprted currently"),
    }
}

fn entity_impl_table_init(ident: &syn::Ident, st: &syn::DataStruct) -> TokenStream2 {
    let mut table_names = Vec::new();
    let mut entity_names = Vec::new();
    for field in &st.fields {
        let ident = field.ident.as_ref().expect_or_abort("unreachable!");
        let name = ident.to_string().to_screaming_snake_case();
        table_names.push(ident);
        entity_names.push(name);
    }
    assert_eq!(table_names.len(), entity_names.len());

    let ruststep = ruststep_crate();

    quote! {
        #[automatically_derived]
        impl #ruststep::tables::TableInit for #ident {
            fn append_data_section(
                &mut self,
                data_sec: &#ruststep::ast::DataSection
            ) -> #ruststep::error::Result<()> {
                use #ruststep::{error::Error, tables::insert_record, ast::EntityInstance};
                for entity in &data_sec.entities {
                    match entity {
                        EntityInstance::Simple { id, record } => match record.name.as_str() {
                            #(
                            #entity_names => insert_record(&mut self.#table_names, *id, record)?,
                            )*
                            _ => {
                                return Err(Error::UnknownEntityName {
                                    entity_name: record.name.clone(),
                                    schema: "".to_string(),
                                });
                            }
                        },
                        EntityInstance::Complex { .. } => {
                            unimplemented!("Complex entity is not supported")
                        }
                    }
                }
                Ok(())
            }
        }

        #[automatically_derived]
        impl ::std::str::FromStr for #ident {
            type Err = #ruststep::error::Error;
            fn from_str(input: &str) -> #ruststep::error::Result<Self> {
                use #ruststep::{tables::TableInit, ast::DataSection};
                let data_sec = DataSection::from_str(input)?;
                Ok(Self::from_data_section(&data_sec)?)
            }
        }
    }
}

fn tuple_impl_table_init(ident: &syn::Ident, st: &syn::DataStruct) -> TokenStream2 {
    let mut table_names = Vec::new();
    let mut entity_names = Vec::new();
    for field in &st.fields {
        let ty = &field.ty;
        let ident = quote! { #ty }.to_string().to_screaming_snake_case();
        table_names.push(ident.clone());
        entity_names.push(ident);
    }
    assert_eq!(table_names.len(), entity_names.len());

    let ruststep = ruststep_crate();

    quote! {
        #[automatically_derived]
        impl #ruststep::tables::TableInit for #ident {
            fn append_data_section(
                &mut self,
                data_sec: &#ruststep::ast::DataSection
            ) -> #ruststep::error::Result<()> {
                use #ruststep::{error::Error, tables::insert_record, ast::EntityInstance};
                for entity in &data_sec.entities {
                    match entity {
                        EntityInstance::Simple { id, record } => match record.name.as_str() {
                            #(
                            #entity_names => insert_record(&mut self.#table_names, *id, record)?,
                            )*
                            _ => {
                                return Err(Error::UnknownEntityName {
                                    entity_name: record.name.clone(),
                                    schema: "".to_string(),
                                });
                            }
                        },
                        EntityInstance::Complex { .. } => {
                            unimplemented!("Complex entity is not supported")
                        }
                    }
                }
                Ok(())
            }
        }

        #[automatically_derived]
        impl ::std::str::FromStr for #ident {
            type Err = #ruststep::error::Error;
            fn from_str(input: &str) -> #ruststep::error::Result<Self> {
                use #ruststep::{tables::TableInit, ast::DataSection};
                let data_sec = DataSection::from_str(input)?;
                Ok(Self::from_data_section(&data_sec)?)
            }
        }
    }
}
