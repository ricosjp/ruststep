use super::{namespace::*, scope::*, type_ref::*, *};
use crate::ast;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Entity {
    /// Name of entity in snake_case
    pub name: String,
    pub attributes: Vec<EntityAttribute>,
    pub subtypes: Option<Vec<TypeRef>>,
    pub supertypes: Option<Vec<TypeRef>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EntityAttribute {
    pub name: String,
    pub ty: TypeRef,
    pub optional: bool,
}

impl Legalize for EntityAttribute {
    type Input = ast::EntityAttribute;

    fn legalize(ns: &Namespace, scope: &Scope, attr: &Self::Input) -> Result<Self, SemanticError> {
        let ty = TypeRef::legalize(ns, scope, &attr.ty)?;
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

/// Convert `SUPERTYPE OF (type_name)` to `[type_name]`, `SUPERTYPE OF ONE_OF (t1, t2)` to `[t1, t2]`
fn flatten_super_type_expression(expr: &ast::entity::SuperTypeExpression) -> Vec<String> {
    let mut names = Vec::new();
    match expr {
        ast::entity::SuperTypeExpression::Reference(name) => names.push(name.clone()),
        // ignore the differences between `ONE_OF`, `ANDOR`, and `AND`
        ast::entity::SuperTypeExpression::OneOf { exprs }
        | ast::entity::SuperTypeExpression::AndOr { factors: exprs }
        | ast::entity::SuperTypeExpression::And { terms: exprs } => {
            for expr in exprs {
                let mut sub_names = flatten_super_type_expression(expr);
                names.append(&mut sub_names);
            }
        }
    }
    names
}

impl Legalize for Entity {
    type Input = ast::Entity;

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

        let mut supertypes = Vec::new();
        for c in &entity.constraint {
            use ast::entity::Constraint;
            match c {
                Constraint::SuperTypeRule(rule_expr)
                | Constraint::AbstractSuperType(Some(rule_expr)) => {
                    let names = flatten_super_type_expression(rule_expr);
                    for name in names {
                        supertypes.push(ns.lookup_type(scope, &name)?);
                    }
                }
                _ => continue,
            }
        }
        let supertypes = if supertypes.is_empty() {
            None
        } else {
            Some(supertypes)
        };

        let name = entity.name.clone();
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
        dbg!(&ns);
        let entity = &example.schemas[0].entities[0];
        let scope = Scope::root().pushed(ScopeType::Schema, &example.schemas[0].name);
        let entity = Entity::legalize(&ns, &scope, entity).unwrap();
        dbg!(&entity);
    }
}
