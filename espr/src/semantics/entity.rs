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
        })
    }
}

impl ToTokens for Entity {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        // EXPRESS identifier should be snake_case, but Rust struct should be PascalCase.
        let name = format_ident!("{}", self.name.to_pascal_case());

        let attr_name: Vec<_> = self
            .attributes
            .iter()
            .map(|EntityAttribute { name, .. }| format_ident!("{}", name))
            .collect();
        let attr_type: Vec<_> = self
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

        tokens.append_all(quote! {
            #[derive(Clone, Debug, PartialEq, derive_new::new)]
            pub struct #name {
                #(
                #attr_name : #attr_type,
                )*
            }
        })
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
