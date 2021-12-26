use crate::ir::*;

use check_keyword::CheckKeyword;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

impl ToTokens for EntityAttribute {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let EntityAttribute { name, ty, optional } = self;

        let attr_name = format_ident!("{}", name.to_safe());
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

// Additional functions to use in codegen/rust for ir::Entity.
impl Entity {
    fn name_ident(&self) -> syn::Ident {
        format_ident!("{}", self.name.to_pascal_case())
    }

    fn any_ident(&self) -> syn::Ident {
        // `Any` indentifier must be appears if the entity is supertype
        assert!(!self.subtypes.is_empty());
        format_ident!("{}Any", self.name.to_pascal_case())
    }

    /// Field identifier
    fn field_ident(&self) -> syn::Ident {
        format_ident!("{}", self.name.to_safe())
    }

    /// Generate declaration of `XxxAny` enum
    fn generate_any_enum(&self, tokens: &mut TokenStream) {
        let any = self.any_ident();

        let mut fields = vec![format_ident!("{}", self.name.to_safe())];
        let mut variants = vec![format_ident!("{}", self.name.to_pascal_case())];
        let mut subtypes = vec![format_ident!("{}", self.name.to_pascal_case())];

        for ty in &self.subtypes {
            match &ty {
                TypeRef::Entity {
                    name, is_supertype, ..
                } => {
                    fields.push(format_ident!("{}", name.to_safe()));
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

    /// Generate `impl Into<SelfAny> for SubType` for self and all subtypes
    fn generate_into_any(&self, tokens: &mut TokenStream) {
        let any = self.any_ident();
        let name = self.name_ident();

        // `Self` to `SelfAny`
        tokens.append_all(quote! {
            impl Into<#any> for #name {
                fn into(self) -> #any {
                    #any::#name(Box::new(self))
                }
            }
        });

        for ty in &self.subtypes {
            if let TypeRef::Entity { name, .. } = ty {
                let name = format_ident!("{}", name.to_pascal_case());
                tokens.append_all(quote! {
                    impl Into<#any> for #name {
                        fn into(self) -> #any {
                            #any::#name(Box::new(self.into()))
                        }
                    }
                });
            }
        }
    }

    fn generate_asref_from_any(&self, tokens: &mut TokenStream) {
        let any = self.any_ident();
        let name = self.name_ident();

        let subtypes = self
            .subtypes
            .iter()
            .map(|ty| match &ty {
                TypeRef::Entity { name, .. } => {
                    format_ident!("{}", name.to_pascal_case())
                }
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();
        let subtypes_token = quote! { #(#any::#subtypes (x) => AsRef::<#name>::as_ref(x.as_ref()).as_ref(),)* };

        let supertypes = self
            .supertypes
            .iter()
            .map(|ty| match &ty {
                TypeRef::Entity { name, .. } => {
                    format_ident!("{}", name.to_pascal_case())
                }
                _ => unreachable!(),
            })
            .collect::<Vec<_>>();

        tokens.append_all(quote! {
            impl AsRef<#name> for #any {
                fn as_ref(&self) -> &#name {
                    match self {
                        #any::#name (x) => x.as_ref(),
                        #(#any::#subtypes (x) => (**x).as_ref(),)*
                    }
                }
            }
            #(impl AsRef<#supertypes> for #any {
                fn as_ref(&self) -> &#supertypes {
                    match self {
                        #any::#name (x) => x.as_ref(),
                        #subtypes_token
                    }
                }
            })*
        });
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
                        format_ident!("{}", name.to_safe()),
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

        // Generate `Any` enum if this entity is a supertype of other entities
        if !self.subtypes.is_empty() {
            self.generate_any_enum(tokens);
            // Generate `impl Into<XxxAny> for Yyy` for self and all subtypes
            self.generate_into_any(tokens);
            self.generate_asref_from_any(tokens);
        }
    }
}
