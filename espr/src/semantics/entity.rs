use super::{namespace::*, scope::*, type_ref::*, *};
use crate::ast;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    /// Name of entity in PascalCase
    name: String,
    attributes: Vec<EntityAttribute>,
    subtypes: Option<Vec<TypeRef>>,
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
            name: entity.name.to_pascal_case(),
            attributes,
            subtypes,
        })
    }
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name);
        let holder_name = format_ident!("{}Holder", self.name);

        let mut attr_name = Vec::new();
        let mut attr_type = Vec::new();
        let mut holder_attr_type = Vec::new();

        for EntityAttribute { name, ty, optional } in &self.attributes {
            attr_name.push(format_ident!("{}", name));
            attr_type.push(if *optional {
                quote! { Option<#ty> }
            } else {
                quote! { #ty }
            });
            match ty {
                TypeRef::SimpleType(..) => holder_attr_type.push(if *optional {
                    quote! { Option<#ty> }
                } else {
                    quote! { #ty }
                }),
                _ => holder_attr_type.push(if *optional {
                    quote! { Option<PlaceHolder<#ty>> }
                } else {
                    quote! { PlaceHolder<#ty> }
                }),
            }
        }

        if let Some(subtypes) = &self.subtypes {
            for ty in subtypes {
                let (attr, ty) = match ty {
                    TypeRef::Named { name, .. } => (format_ident!("{}", name), ty),
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
                match ty {
                    TypeRef::SimpleType(..) => holder_attr_type.push(quote! { #ty }),
                    _ => holder_attr_type.push(quote! { PlaceHolder<#ty> }),
                }
            }
        }

        tokens.append_all(quote! {
            #[derive(Clone, Debug, PartialEq, derive_new::new)]
            pub struct #name {
                #(
                pub #attr_name : #attr_type,
                )*
            }

            #[derive(Clone, Debug, PartialEq)]
            pub struct #holder_name {
                #(
                pub #attr_name : #holder_attr_type,
                )*
            }
        });
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
