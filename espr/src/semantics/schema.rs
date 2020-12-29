use super::{entity::*, namespace::*, scope::*, *};
use crate::parser;
use proc_macro2::TokenStream;
use quote::*;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schema {
    pub name: String,
    pub entities: Vec<Entity>,
}

impl Legalize for Schema {
    type Input = parser::schema::Schema;
    fn legalize(
        ns: &Namespace,
        scope: &Scope,
        schema: &Self::Input,
    ) -> Result<Self, SemanticError> {
        let name = schema.name.clone();
        let entities = schema
            .entities
            .iter()
            .map(|entity| Entity::legalize(ns, &scope.pushed(ScopeType::Schema, &name), entity))
            .collect::<Result<Vec<Entity>, SemanticError>>()?;
        Ok(Schema { name, entities })
    }
}

impl ToTokens for Schema {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = format_ident!("{}", self.name);
        let entities = &self.entities;
        tokens.append_all(quote! {
            mod #name {
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
