use super::{namespace::*, scope::*, type_ref::*, *};
use crate::ast;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    name: String,
    attributes: Vec<EntityAttribute>,
    subtypes: Option<Vec<TypeRef>>,

    // FIXME This assumes that `SUPERTYPE` declaration exists for all supertypes.
    has_supertype_decl: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityAttribute {
    name: String,
    ty: TypeRef,
    optional: bool,
}

impl Legalize for EntityAttribute {
    type Input = ast::entity::EntityAttribute;

    fn legalize(ns: &Namespace, scope: &Scope, attr: &Self::Input) -> Result<Self, SemanticError> {
        let ty = TypeRef::legalize(ns, scope, &attr.ty)?;
        let name = match &attr.name {
            ast::entity::AttributeDecl::Reference(name) => name.clone(),
            _ => unimplemented!(),
        };
        Ok(EntityAttribute {
            name,
            ty,
            optional: attr.optional,
        })
    }
}

impl Legalize for Entity {
    type Input = ast::entity::Entity;

    fn legalize(
        ns: &Namespace,
        scope: &Scope,
        entity: &Self::Input,
    ) -> Result<Self, SemanticError> {
        let attributes = entity
            .attributes
            .iter()
            .map(|attr| EntityAttribute::legalize(ns, scope, attr))
            .collect::<Result<Vec<_>, _>>()?;
        let subtypes = entity
            .subtype
            .as_ref()
            .map(|subtype| {
                subtype
                    .entity_references
                    .iter()
                    .map(|name| ns.lookup_type(scope, &name))
                    .collect::<Result<Vec<_>, _>>()
            })
            .transpose()?;
        Ok(Entity {
            name: entity.name.clone(),
            attributes,
            subtypes,
            has_supertype_decl: entity.has_supertype_decl(),
        })
    }
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // EXPRESS identifier should be snake_case, but Rust struct should be PascalCase.
        let name = format_ident!("{}", self.name.to_pascal_case());

        let mut attr_name: Vec<_> = self
            .attributes
            .iter()
            .map(|EntityAttribute { name, .. }| format_ident!("{}", name))
            .collect();
        let mut attr_type: Vec<_> = self
            .attributes
            .iter()
            .map(|EntityAttribute { ty, optional, .. }| {
                if *optional {
                    quote! { Option<#ty> }
                } else {
                    quote! { #ty }
                }
            })
            .collect();

        if let Some(subtypes) = &self.subtypes {
            for ty in subtypes {
                let (attr, ty) = match ty {
                    TypeRef::Named { name, .. } | TypeRef::Entity { name, .. } => {
                        (format_ident!("{}", name), ty)
                    }
                    _ => unreachable!(),
                };

                // impl Deref for single subtype case
                if subtypes.len() == 1 {
                    tokens.append_all(quote! {
                        impl ::std::ops::Deref for #name {
                            type Target = #ty;
                            fn deref(&self) -> &Self::Target {
                                &self.#attr
                            }
                        }
                    });
                }

                attr_name.push(attr);
                attr_type.push(ty.to_token_stream());

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
        tokens.append_all(quote! {
            #[derive(Debug, derive_new::new)]
            pub struct #name {
                #(
                pub #attr_name : #attr_type,
                )*
            }
        });

        if self.has_supertype_decl {
            let trait_name = format_ident!("{}Any", name);
            tokens.append_all(quote! {
                pub trait #trait_name : ::std::any::Any + ::std::fmt::Debug {}
            });
            tokens.append_all(quote! {
                impl #trait_name for #name {}
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legalize() {
        let example = SyntaxTree::example();
        let ns = Namespace::new(&example).unwrap();
        dbg!(&ns);
        let entity = &example.schemas[0].entities[0];
        let scope = Scope::root().pushed(ScopeType::Schema, &example.schemas[0].name);
        let entity = Entity::legalize(&ns, &scope, entity).unwrap();
        dbg!(&entity);
    }
}
