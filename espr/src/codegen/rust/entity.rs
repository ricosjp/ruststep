use crate::ir::*;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let field_name = format_ident!("{}", self.name);
        let name = format_ident!("{}", self.name.to_pascal_case());

        let mut attr_name = Vec::new();
        let mut attr_type = Vec::new();
        let mut holder_attr_type = Vec::new();
        let mut holder_attr_expr = Vec::new();

        for EntityAttribute { name, ty, optional } in &self.attributes {
            let name = format_ident!("{}", name);
            attr_name.push(name.clone());
            if *optional {
                attr_type.push(quote! { Option<#ty> });
                holder_attr_expr.push(quote! { #name });
                if ty.is_simple() {
                    holder_attr_type.push(quote! { Option<#ty> });
                } else {
                    holder_attr_type.push(quote! { Option<PlaceHolder<#ty>> });
                }
            } else {
                attr_type.push(quote! { #ty });
                if ty.is_simple() {
                    holder_attr_type.push(quote! { #ty });
                    holder_attr_expr.push(quote! { #name });
                } else {
                    holder_attr_type.push(quote! { PlaceHolder<#ty> });
                    holder_attr_expr.push(quote! { #name.into_owned(tables)? });
                }
            }
        }

        for ty in &self.supertypes {
            let (attr, ty) = match ty {
                TypeRef::Named { name, .. } | TypeRef::Entity { name, .. } => {
                    (format_ident!("{}", name), ty)
                }
                _ => unreachable!(),
            };

            attr_name.push(attr.clone());
            attr_type.push(ty.to_token_stream());

            if ty.is_simple() {
                holder_attr_type.push(quote! { #ty });
                holder_attr_expr.push(quote! { #attr });
            } else {
                holder_attr_type.push(quote! { PlaceHolder<#ty> });
                holder_attr_expr.push(quote! { #attr.into_owned(tables)? });
            }

            if let TypeRef::Entity {
                name: supertype_name,
                is_supertype,
                ..
            } = ty
            {
                if *is_supertype {
                    let name = if self.subtypes.is_empty() {
                        format_ident!("{}", self.name.to_pascal_case())
                    } else {
                        format_ident!("{}Any", self.name.to_pascal_case())
                    };
                    let any_enum = format_ident!("{}Any", supertype_name.to_pascal_case());
                    tokens.append_all(quote! {
                        impl Into<#any_enum> for #name {
                            fn into(self) -> #any_enum {
                                #any_enum::#name(Box::new(self))
                            }
                        }
                    });
                }
            }
        }

        assert_eq!(attr_name.len(), attr_type.len());
        assert_eq!(attr_name.len(), holder_attr_type.len());
        assert_eq!(attr_name.len(), holder_attr_expr.len());

        tokens.append_all(quote! {
            #[derive(Debug, Clone, PartialEq, ::derive_new::new, ::ruststep_derive::Holder)]
            #[holder(table = Tables, field = #field_name)]
            pub struct #name {
                #(
                pub #attr_name : #attr_type,
                )*
            }
        });

        if !self.subtypes.is_empty() {
            let subtypes = &self.subtypes;
            let enum_name = format_ident!("{}Any", name);
            tokens.append_all(quote! {
                #[derive(Debug, Clone, PartialEq)]
                pub enum #enum_name {
                    #(#subtypes(Box<#subtypes>)),*
                }
            }); // tokens.append_all
        }
    }
}
