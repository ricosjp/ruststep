use super::{namespace::*, scope::*, type_ref::*, *};
use crate::ast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    /// Name of entity in snake_case
    pub name: String,
    pub attributes: Vec<EntityAttribute>,

    /// List of constraints corresponding to `SUBTYPE_CONSTRAINTS`
    /// and `SUPERTYPE OF` declaration in EXPRESS schema
    pub constraints: Vec<Vec<TypeRef>>,

    /// List of types to be inherited by this entity
    ///
    /// When this entity is `sub` defined like:
    ///
    /// ```text
    /// ENTITY sub SUBTYPE OF (base);
    /// END_ENTITY;
    /// ```
    ///
    /// then this `supertypes` is `[base]`.
    ///
    pub supertypes: Vec<TypeRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityAttribute {
    pub name: String,
    pub ty: TypeRef,
    pub optional: bool,
}

impl Legalize for EntityAttribute {
    type Input = ast::EntityAttribute;

    fn legalize(
        ns: &Namespace,
        ss: &Constraints,
        scope: &Scope,
        attr: &Self::Input,
    ) -> Result<Self, SemanticError> {
        let ty = TypeRef::legalize(ns, ss, scope, &attr.ty)?;
        let name = match &attr.name {
            ast::AttributeDecl::Reference(name) => name.clone(),
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
    type Input = ast::Entity;

    fn legalize(
        ns: &Namespace,
        ss: &Constraints,
        scope: &Scope,
        entity: &ast::Entity,
    ) -> Result<Self, SemanticError> {
        let name = entity.name.clone();
        let attributes = entity
            .attributes
            .iter()
            .map(|attr| EntityAttribute::legalize(ns, ss, scope, attr))
            .collect::<Result<Vec<_>, _>>()?;

        let supertypes = if let Some(supertypes) = &entity.subtype_of {
            supertypes
                .entity_references
                .iter()
                .map(|sup| TypeRef::from_path(ns, ss, &ns.resolve(scope, sup)?.0))
                .collect::<Result<Vec<TypeRef>, _>>()?
        } else {
            Vec::new()
        };

        let path = Path::entity(scope, &entity.name);
        let constraints = if let Some(instantiables) = ss.instantiables.get(&path) {
            instantiables
                .iter()
                .map(|pce| {
                    pce.iter()
                        .map(|path| TypeRef::from_path(ns, ss, path))
                        .collect()
                })
                .collect::<Result<Vec<Vec<TypeRef>>, SemanticError>>()?
        } else {
            Vec::new()
        };

        Ok(Entity {
            name,
            attributes,
            constraints,
            supertypes,
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
        let ss = Constraints::new(&ns, &example).unwrap();
        dbg!(&ns);
        let entity = &example.schemas[0].entities[0];
        let scope = Scope::root().pushed(ScopeType::Schema, &example.schemas[0].name);
        let entity = Entity::legalize(&ns, &ss, &scope, entity).unwrap();
        dbg!(&entity);
    }
}
