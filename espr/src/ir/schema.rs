use super::{entity::*, namespace::*, scope::*, type_decl::*, *};
use crate::ast;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Schema {
    pub name: String,
    pub entities: Vec<Entity>,
    pub types: Vec<TypeDecl>,
}

impl Legalize for Schema {
    type Input = ast::Schema;
    fn legalize(
        ns: &Namespace,
        ss: &SubSuperGraph,
        scope: &Scope,
        schema: &Self::Input,
    ) -> Result<Self, SemanticError> {
        let name = schema.name.clone();
        let here = scope.pushed(ScopeType::Schema, &name);
        let entities = schema
            .entities
            .iter()
            .map(|entity| Entity::legalize(ns, ss, &here, entity))
            .collect::<Result<Vec<Entity>, _>>()?;
        let types = schema
            .types
            .iter()
            .map(|entity| TypeDecl::legalize(ns, ss, &here, entity))
            .collect::<Result<Vec<TypeDecl>, _>>()?;
        Ok(Schema {
            name,
            entities,
            types,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legalize() {
        let example = SyntaxTree::example();
        let ns = Namespace::new(&example);
        let ss = SubSuperGraph::new(&ns, &example).unwrap();
        dbg!(&ns, &ss);
        let schema = &example.schemas[0];
        let scope = Scope::root();
        let schema = Schema::legalize(&ns, &ss, &scope, schema).unwrap();
        dbg!(&schema);
    }
}
