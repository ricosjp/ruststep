use super::{namespace::*, scope::*, type_ref::*, *};
use crate::ast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    /// Name of entity in snake_case
    pub name: String,
    pub attributes: Vec<EntityAttribute>,
    pub subtypes: Vec<TypeRef>,
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
        ss: &SubSuperGraph,
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
        ss: &SubSuperGraph,
        scope: &Scope,
        entity: &Self::Input,
    ) -> Result<Self, SemanticError> {
        let name = entity.name.clone();
        let self_as_type_ref = TypeRef::Entity {
            name: name.clone(),
            scope: scope.clone(),
            has_supertype_decl: true,
        };

        let attributes = entity
            .attributes
            .iter()
            .map(|attr| EntityAttribute::legalize(ns, ss, scope, attr))
            .collect::<Result<Vec<_>, _>>()?;

        let mut subtypes = Vec::new();
        if let Some(st) = &entity.subtype {
            for name in &st.entity_references {
                let ty = ns.lookup_type(scope, name)?;
                subtypes.push(ty);
            }
        }

        let mut supertypes = Vec::new();
        for c in &entity.constraint {
            use ast::Constraint;
            match c {
                Constraint::SuperTypeRule(rule_expr)
                | Constraint::AbstractSuperType(Some(rule_expr)) => {
                    for name in rule_expr.as_supertype_names() {
                        supertypes.push(ns.lookup_type(scope, &name)?);
                    }
                }
                Constraint::AbstractSuperType(None) => {
                    dbg!(&self_as_type_ref);
                    if let Some(refs) = dbg!(ss.super_to_sub.get(&self_as_type_ref)) {
                        for sub in refs {
                            supertypes.push(sub.clone());
                        }
                    }
                }
                // `ABSTRACT` entity is ignored
                _ => continue,
            }
        }

        Ok(Entity {
            name,
            attributes,
            subtypes,
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
        let ns = Namespace::new(&example).unwrap();
        let ss = SubSuperGraph::new(&ns, &example).unwrap();
        dbg!(&ns);
        let entity = &example.schemas[0].entities[0];
        let scope = Scope::root().pushed(ScopeType::Schema, &example.schemas[0].name);
        let entity = Entity::legalize(&ns, &ss, &scope, entity).unwrap();
        dbg!(&entity);
    }
}
