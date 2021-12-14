use crate::ir::*;

use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

impl ToTokens for EntityAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let EntityAttribute { name, ty, optional } = self;

        let attr_name = format_ident!("{}", name);
        let attr_type = if *optional {
            quote! { Option<#ty> }
        } else {
            quote! { #ty }
        };
        let use_place_holder = if ty.is_simple() {
            quote! {}
        } else {
            quote! { #[holder(use_place_holder)] }
        };

        tokens.append_all(quote! {
            #use_place_holder
            pub #attr_name : #attr_type
        });
    }
}

impl Entity {
    fn name_ident(&self) -> syn::Ident {
        format_ident!("{}", self.name.to_pascal_case())
    }

    fn any_ident(&self) -> syn::Ident {
        format_ident!("{}Any", self.name.to_pascal_case())
    }

    fn field_ident(&self) -> syn::Ident {
        format_ident!("{}", self.name)
    }

    fn generate_any_def(&self, tokens: &mut TokenStream) {
        if !self.subtypes.is_empty() {
            let mut fields = vec![format_ident!("{}", self.name)];
            let mut variants = vec![format_ident!("{}", self.name.to_pascal_case())];
            let mut subtypes = vec![format_ident!("{}", self.name.to_pascal_case())];

            for ty in &self.subtypes {
                match &ty {
                    TypeRef::Entity {
                        name, is_supertype, ..
                    } => {
                        fields.push(format_ident!("{}", name));
                        variants.push(format_ident!("{}", name.to_pascal_case()));
                        if *is_supertype {
                            subtypes.push(format_ident!("{}Any", name.to_pascal_case()));
                        } else {
                            subtypes.push(format_ident!("{}", name.to_pascal_case()));
                        }
                    }
                    _ => unreachable!(),
                }
            }

            let any = self.any_ident();
            tokens.append_all(quote! {
                #[derive(Debug, Clone, PartialEq, Holder)]
                #[holder(table = Tables)]
                #[holder(generate_deserialize)]
                pub enum #any {
                    #(
                    #[holder(use_place_holder)]
                    #[holder(field = #fields)]
                    #variants(Box<#subtypes>)
                    ),*
                }
            }); // tokens.append_all
        }
    }

    // derive self -> SuperTypeAny
    fn generate_into_any(&self, tokens: &mut TokenStream) {
        let name = if self.subtypes.is_empty() {
            self.name_ident()
        } else {
            self.any_ident()
        };
        for ty in &self.supertypes {
            if let TypeRef::Entity {
                name: supertype_name,
                ..
            } = ty
            {
                let variant = self.name_ident();
                let any_enum = format_ident!("{}Any", supertype_name.to_pascal_case());
                tokens.append_all(quote! {
                    impl Into<#any_enum> for #name {
                        fn into(self) -> #any_enum {
                            #any_enum::#variant(Box::new(self))
                        }
                    }
                });
            }
        }
    }

    fn supertype_attributes(&self) -> Vec<TokenStream> {
        self.supertypes
            .iter()
            .map(|ty| {
                let use_place_holder = if ty.is_simple() {
                    quote! {}
                } else {
                    quote! { #[holder(use_place_holder)] }
                };
                let (attr, ty) = match ty {
                    TypeRef::Named { name, .. } | TypeRef::Entity { name, .. } => (
                        format_ident!("{}", name),
                        format_ident!("{}", name.to_pascal_case()),
                    ),
                    _ => unreachable!(),
                };
                quote! {
                    #use_place_holder
                    pub #attr: #ty
                }
            })
            .collect()
    }

    fn generate_derive(&self) -> (TokenStream, TokenStream) {
        if self.supertypes.len() == 1 {
            (
                quote! {
                    #[derive(Debug, Clone, PartialEq, AsRef, AsMut, Deref, DerefMut, ::derive_new::new, Holder)]
                },
                quote! { #[as_ref] #[as_mut] #[deref] #[deref_mut] },
            )
        } else {
            (
                quote! {
                    #[derive(Debug, Clone, PartialEq, ::derive_new::new, Holder)]
                },
                quote! {},
            )
        }
    }
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name_ident();
        let field_name = self.field_ident();

        let attributes = &self.attributes;
        let supertype_attributes = self.supertype_attributes();
        let (derive, attr_macro) = self.generate_derive();

        tokens.append_all(quote! {
            #derive
            #[holder(table = Tables)]
            #[holder(field = #field_name)]
            #[holder(generate_deserialize)]
            pub struct #name {
                #attr_macro
                #(#supertype_attributes,)*
                #(#attributes,)*
            }
        });

        self.generate_any_def(tokens);
        self.generate_into_any(tokens);
    }
}
