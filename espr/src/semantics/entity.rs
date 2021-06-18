use super::{namespace::*, scope::*, type_ref::*, *};
use crate::ast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    /// Name of entity in snake_case
    pub name: String,
    pub attributes: Vec<EntityAttribute>,
    pub subtypes: Option<Vec<TypeRef>>,
    /// FIXME This assumes that `SUPERTYPE` declaration exists for all supertypes.
    pub has_supertype_decl: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityAttribute {
    pub name: String,
    pub ty: TypeRef,
    pub optional: bool,
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
        let name = entity.name.clone();
        Ok(Entity {
            name,
            attributes,
            subtypes,
            has_supertype_decl: entity.has_supertype_decl(),
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
