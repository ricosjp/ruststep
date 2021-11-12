use proc_macro2::TokenStream as TokenStream2;
use proc_macro_error::abort_call_site;
use quote::quote;

use crate::ruststep_crate;

pub fn derive_table_init(ast: &syn::DeriveInput) -> TokenStream2 {
    let ident = &ast.ident;
    match &ast.data {
        syn::Data::Struct(st) => impl_table_init(ident, st),
        _ => abort_call_site!("Only struct is supprted currently"),
    }
}

fn impl_table_init(ident: &syn::Ident, _st: &syn::DataStruct) -> TokenStream2 {
    let ruststep = ruststep_crate();
    quote! {
        #[automatically_derived]
        impl #ruststep::tables::TableInit for #ident {
            use #ruststep::error::*;
            fn append_data_section(&mut self, data_sec: &DataSection) -> Result<()> {
                for entity in &data_sec.entities {
                    match entity {
                        EntityInstance::Simple { id, record } => match record.name.as_str() {
                            "A" => insert_record(&mut self.a, *id, record)?,
                            "B" => insert_record(&mut self.b, *id, record)?,
                            _ => {
                                return Err(Error::UnknownEntityName {
                                    entity_name: record.name.clone(),
                                    schema: "test_holder".to_string(),
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
