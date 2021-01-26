use super::{entity::*, identifier::*, types::*, util::*};

/// Parsed result of EXPRESS's SCHEMA
#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub name: String,
    pub entities: Vec<Entity>,
    pub types: Vec<TypeDecl>,
}

/// 296 schema_decl = SCHEMA [schema_id] \[ schema_version_id \] `;` [schema_body] END_SCHEMA `;` .
pub fn schema_decl(input: &str) -> ParseResult<Schema> {
    // FIXME schema_version_id
    let schema_head =
        tuple((tag("SCHEMA "), schema_id, char(';'))).map(|(_start, id, _semicoron)| id);
    tuple((schema_head, schema_body, tag("END_SCHEMA"), char(';')))
        .map(|(name, (entities, types), _end, _semicoron)| Schema {
            name,
            entities,
            types,
        })
        .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Declaration {
    Entity(Entity),
    Type(TypeDecl),
}

/// 199 declaration = [entity_decl] | function_decl | procedure_decl | subtype_constraint_decl | [type_decl] .
pub fn declaration(input: &str) -> ParseResult<Declaration> {
    // FIXME function_decl
    // FIXME procedure_decl
    // FIXME subtype_constraint_decl
    alt((
        entity_decl.map(|e| Declaration::Entity(e)),
        type_decl.map(|ty| Declaration::Type(ty)),
    ))
    .parse(input)
}

/// 295 schema_body = { interface_specification } \[ constant_decl \] { [declaration] | rule_decl } .
pub fn schema_body(input: &str) -> ParseResult<(Vec<Entity>, Vec<TypeDecl>)> {
    // FIXME interface_specification
    // FIXME constant_decl
    // FIXME rule_decl
    spaced_many0(declaration)
        .map(|decls| {
            let mut entities = Vec::new();
            let mut types = Vec::new();
            for decl in decls {
                match decl {
                    Declaration::Entity(e) => entities.push(e),
                    Declaration::Type(ty) => types.push(ty),
                }
            }
            (entities, types)
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Finish;

    #[test]
    fn schema() {
        let exp_str = r#"
        SCHEMA my_first_schema;
          ENTITY first;
            m_ref : second;
            fattr : STRING;
          END_ENTITY;

          ENTITY second;
            sattr : STRING;
          END_ENTITY;
        END_SCHEMA;
        "#
        .trim();

        let (residual, (schema, _remark)) = super::schema_decl(exp_str).finish().unwrap();
        assert_eq!(schema.name, "my_first_schema");
        assert_eq!(schema.entities.len(), 2);
        assert_eq!(
            schema.entities[0],
            entity_decl(
                r#"
                ENTITY first;
                  m_ref : second;
                  fattr : STRING;
                END_ENTITY;
                "#
                .trim()
            )
            .finish()
            .unwrap()
            .1
             .0
        );
        assert_eq!(
            schema.entities[1],
            entity_decl(
                r#"
                ENTITY second;
                  sattr : STRING;
                END_ENTITY;
                "#
                .trim()
            )
            .finish()
            .unwrap()
            .1
             .0
        );
        assert_eq!(residual, "");
    }
}
