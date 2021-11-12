use inflector::Inflector;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::{abort_call_site, OptionExt};
use quote::quote;

use crate::ruststep_crate;

pub fn derive_table_init(ast: &syn::DeriveInput) -> TokenStream2 {
    let ident = &ast.ident;
    match &ast.data {
        syn::Data::Struct(st) => impl_table_init(ident, st),
        _ => abort_call_site!("Only struct is supprted currently"),
    }
}

fn impl_table_init(ident: &syn::Ident, st: &syn::DataStruct) -> TokenStream2 {
    let mut table_names = Vec::new();
    let mut entity_names = Vec::new();
    for field in &st.fields {
        let ident = field
            .ident
            .as_ref()
            .expect_or_abort("Tuple struct case is not supported");
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
    }
}
