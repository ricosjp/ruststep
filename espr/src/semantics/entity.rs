use super::{namespace::*, scope::*, *};
use crate::parser;
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    name: String,
    attributes: Vec<EntityAttribute>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityAttribute {
    name: String,
    ty: TypeRef,
    optional: bool,
}

impl Legalize for EntityAttribute {
    type Input = parser::EntityAttribute;

    fn legalize(ns: &Namespace, scope: &Scope, attr: &Self::Input) -> Result<Self, SemanticError> {
        use crate::ast::types::ParameterType::*;
        let ty = match &attr.ty {
            Named(name) => ns.lookup_type(scope, name)?,
            Simple(ty) => namespace::TypeRef::SimpleType(*ty),
            _ => unimplemented!(),
        };
        let name = match &attr.name {
            parser::AttributeDecl::Reference(name) => name.clone(),
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
    type Input = parser::Entity;

    fn legalize(
        ns: &Namespace,
        scope: &Scope,
        entity: &parser::Entity,
    ) -> Result<Self, SemanticError> {
        let attributes = entity
            .attributes
            .iter()
            .map(|attr| EntityAttribute::legalize(ns, scope, attr))
            .collect::<Result<Vec<_>, _>>()?;
        Ok(Entity {
            name: entity.name.clone(),
            attributes,
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
            #[derive(Clone, Debug, PartialEq)]
            pub struct #name {
                #(
                #attr_name : #attr_type,
                )*
            }

            impl #name {
                pub fn new(#(#attr_name : #attr_type),*) -> Self {
                    Self { #(#attr_name),* }
                }
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
