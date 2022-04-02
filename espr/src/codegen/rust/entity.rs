use crate::ir::*;

use check_keyword::CheckKeyword;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;
use syn::parse_quote;

// Each component of Rust struct corresponding to ENTITY in EXPRESS
struct Field {
    name: syn::Ident,
    ty: syn::Type,
    attributes: Vec<syn::Attribute>,
}

/// Check if the field should use place holder
///
/// A field will not use place holder iff its type is one of:
///
/// - a simple type
/// - an enumeration
/// - a set or list whose base type use place holder
///
fn use_place_holder(ty: &TypeRef) -> bool {
    match ty {
        TypeRef::SimpleType(..) => false,
        TypeRef::Named { is_enumerate, .. } => !*is_enumerate,
        TypeRef::Set { base, .. } | TypeRef::List { base, .. } => use_place_holder(base),
        _ => true,
    }
}

impl From<EntityAttribute> for Field {
    fn from(attr: EntityAttribute) -> Self {
        let EntityAttribute { name, ty, optional } = attr;

        let name = format_ident!("{}", name.to_safe());
        let attributes = if use_place_holder(&ty) {
            vec![parse_quote! { #[holder(use_place_holder)] }]
        } else {
            Vec::new()
        };
        let ty = if optional {
            parse_quote! { Option<#ty> }
        } else {
            parse_quote! { #ty }
        };

        Field {
            name,
            ty,
            attributes,
        }
    }
}

impl ToTokens for Field {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Field {
            name,
            ty,
            attributes,
        } = self;

        tokens.append_all(quote! {
            #( #attributes )*
            pub #name : #ty
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
        assert!(!self.constraints.is_empty());
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
        let mut constraints = vec![format_ident!("{}", self.name.to_pascal_case())];

        for ty in &self.constraints {
            match ty {
                TypeRef::Entity {
                    name, is_supertype, ..
                } => {
                    fields.push(format_ident!("{}", name.to_safe()));
                    variants.push(format_ident!("{}", name.to_pascal_case()));
                    if *is_supertype {
                        constraints.push(format_ident!("{}Any", name.to_pascal_case()));
                    } else {
                        constraints.push(format_ident!("{}", name.to_pascal_case()));
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
                #variants(Box<#constraints>)
                ),*
            }
        }); // tokens.append_all
    }

    /// Generate `impl Into<SelfAny> for SubType` for self and all constraints
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

        for ty in &self.constraints {
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

    /// Generate `impl AsRef<Self> for SelfAny` and `impl AsRef<Super> for SelfAny`
    fn generate_asref_from_any(&self, tokens: &mut TokenStream) {
        let any = self.any_ident();
        let name = self.name_ident();

        let constraints = self
            .constraints
            .iter()
            .map(|ty| match ty {
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
                        #(#any::#constraints (x) => (**x).as_ref(),)*
                    }
                }
            }
        });

        for ty in &self.supertypes {
            let supertype = match ty {
                TypeRef::Entity { name, .. } => {
                    format_ident!("{}", name.to_pascal_case())
                }
                _ => unreachable!(),
            };

            tokens.append_all(quote! {
                impl AsRef<#supertype> for #any {
                    fn as_ref(&self) -> &#supertype {
                        match self {
                            #any::#name (x) => AsRef::<#name>::as_ref(x).as_ref(),
                            #(#any::#constraints (x) => AsRef::<#name>::as_ref(x.as_ref()).as_ref(),)*
                        }
                    }
                }
            });
        }
    }

    fn supertype_fields(&self) -> Vec<Field> {
        self.supertypes
            .iter()
            .map(|ty| {
                let mut attributes = Vec::new();
                attributes.push(parse_quote! { #[as_ref] });
                attributes.push(parse_quote! { #[as_mut] });

                if self.supertypes.len() == 1 {
                    attributes.push(parse_quote! { #[deref] });
                    attributes.push(parse_quote! { #[deref_mut] });
                }
                attributes.push(parse_quote! { #[holder(use_place_holder)] });
                let (name, ty) = match ty {
                    TypeRef::Named { name, .. } | TypeRef::Entity { name, .. } => {
                        let ty = format_ident!("{}", name.to_pascal_case());
                        (format_ident!("{}", name.to_safe()), parse_quote! { #ty })
                    }
                    _ => unreachable!(),
                };

                Field {
                    name,
                    ty,
                    attributes,
                }
            })
            .collect()
    }

    fn derives(&self) -> Vec<syn::Path> {
        let mut derives = vec![
            syn::parse_str("Debug").unwrap(),
            syn::parse_str("Clone").unwrap(),
            syn::parse_str("PartialEq").unwrap(),
            syn::parse_str("::derive_new::new").unwrap(),
            syn::parse_str("Holder").unwrap(),
        ];
        if !self.supertypes.is_empty() {
            derives.push(syn::parse_str("AsRef").unwrap());
            derives.push(syn::parse_str("AsMut").unwrap());
        }
        if self.supertypes.len() == 1 {
            derives.push(syn::parse_str("Deref").unwrap());
            derives.push(syn::parse_str("DerefMut").unwrap());
        }
        derives
    }
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = self.name_ident();
        let field_name = self.field_ident();

        // Each component of struct is called "field" in Rust,
        // and "attribute" refers other items
        //
        // https://doc.rust-lang.org/std/keyword.struct.html
        let fields = self
            .attributes
            .iter()
            .map(|attr| Field::from(attr.clone()))
            .collect::<Vec<Field>>();
        let supertype_fields = self.supertype_fields();

        let derive = self.derives();

        tokens.append_all(quote! {
            #( #[derive(#derive)] )*
            #[holder(table = Tables)]
            #[holder(field = #field_name)]
            #[holder(generate_deserialize)]
            pub struct #name {
                #(#supertype_fields,)*
                #(#fields,)*
            }
        });

        // Generate `Any` enum if this entity is a supertype of other entities
        if !self.constraints.is_empty() {
            self.generate_any_enum(tokens);
            // Generate `impl Into<XxxAny> for Yyy` for self and all constraints
            self.generate_into_any(tokens);
            self.generate_asref_from_any(tokens);
        }
    }
}
