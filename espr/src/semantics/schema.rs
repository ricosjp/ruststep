use super::{entity::*, namespace::*, scope::*, type_decl::*, *};
use crate::ast;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schema {
    pub name: String,
    pub entities: Vec<Entity>,
    pub types: Vec<TypeDecl>,
}

impl Legalize for Schema {
    type Input = ast::schema::Schema;
    fn legalize(
        ns: &Namespace,
        scope: &Scope,
        schema: &Self::Input,
    ) -> Result<Self, SemanticError> {
        let name = schema.name.clone();
        let here = scope.pushed(ScopeType::Schema, &name);
        let entities = schema
            .entities
            .iter()
            .map(|entity| Entity::legalize(ns, &here, entity))
            .collect::<Result<Vec<Entity>, _>>()?;
        let types = schema
            .types
            .iter()
            .map(|entity| TypeDecl::legalize(ns, &here, entity))
            .collect::<Result<Vec<TypeDecl>, _>>()?;
        Ok(Schema {
            name,
            entities,
            types,
        })
    }
}

impl ToTokens for Schema {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name);
        let types = &self.types;
        let entities = &self.entities;
        tokens.append_all(quote! {
            pub mod #name {
                use crate::primitive::*;
                #(#types)*
                #(#entities)*
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
        let schema = &example.schemas[0];
        let scope = Scope::root();
        let schema = Schema::legalize(&ns, &scope, schema).unwrap();
        dbg!(&schema);
    }
}
