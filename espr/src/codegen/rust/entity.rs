use crate::semantics::*;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name.to_pascal_case());
        let holder_name = format_ident!("{}Holder", self.name.to_pascal_case());

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

        if let Some(subtypes) = &self.subtypes {
            for ty in subtypes {
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
                    has_supertype_decl,
                    ..
                } = ty
                {
                    if *has_supertype_decl {
                        let any_trait = format_ident!("{}Any", supertype_name.to_pascal_case());
                        tokens.append_all(quote! {
                            impl #any_trait for #name {}
                        });
                    }
                }
            }
        }

        assert_eq!(attr_name.len(), attr_type.len());
        assert_eq!(attr_name.len(), holder_attr_type.len());
        assert_eq!(attr_name.len(), holder_attr_expr.len());

        let name_str = self.name.to_uppercase();
        let attr_len = attr_name.len();

        tokens.append_all(quote! {
            #[derive(Debug, Clone, derive_new::new)]
            pub struct #name {
                #(
                pub #attr_name : #attr_type,
                )*
            }

            #[derive(Clone, Debug)]
            struct #holder_name {
                #(
                #attr_name : #holder_attr_type,
                )*
            }

            impl Holder for #holder_name {
                type Table = Tables;
                type Owned = #name;
                fn into_owned(self, _tables: &Self::Table) -> Result<Self::Owned> {
                    todo!()
                }
                fn name() -> &'static str {
                    #name_str
                }
                fn attr_len() -> usize {
                    #attr_len
                }
            }
        });

        if self.has_supertype_decl {
            let trait_name = format_ident!("{}Any", name);
            tokens.append_all(quote! {
                pub trait #trait_name:
                    ::std::any::Any
                  + ::std::fmt::Debug
                  + dyn_clone::DynClone
                {}
                dyn_clone::clone_trait_object!(#trait_name);
            });
            tokens.append_all(quote! {
                impl #trait_name for #name {}
            });
        }
    }
}
