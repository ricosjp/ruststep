use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

use crate::ir::*;

impl ToTokens for Select {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let id = format_ident!("{}", &self.id.to_pascal_case());
        let mut entries = Vec::new();
        let mut entry_types = Vec::new();
        let mut field = Vec::new();
        let mut use_place_holder = Vec::new();
        for ty in &self.types {
            match ty {
                TypeRef::Entity {
                    name, is_supertype, ..
                } => {
                    entries.push(format_ident!("{}", name.to_pascal_case()));
                    if *is_supertype {
                        entry_types.push(quote! { #ty });
                        field.push(quote! {});
                    } else {
                        entry_types.push(quote! { Box<#ty> });
                        let field_name = format_ident!("{}", name);
                        field.push(quote! { #[holder(field = #field_name)] })
                    }
                    use_place_holder.push(quote! { #[holder(use_place_holder)] });
                }
                TypeRef::Named {
                    name, is_simple, ..
                } => {
                    field.push(quote! {});
                    entries.push(format_ident!("{}", name.to_pascal_case()));
                    if *is_simple {
                        entry_types.push(quote! { #ty });
                        use_place_holder.push(quote! {});
                    } else {
                        entry_types.push(quote! { Box<#ty> });
                        use_place_holder.push(quote! { #[holder(use_place_holder)] });
                    }
                }
                _ => unimplemented!(),
            }
        }
        tokens.append_all(quote! {
            #[derive(Debug, Clone, PartialEq, Holder)]
            #[holder(table = Tables)]
            #[holder(generate_deserialize)]
            pub enum #id {
                #(
                #field
                #use_place_holder
                #entries(#entry_types)
                ),*
            }
        });
    }
}
