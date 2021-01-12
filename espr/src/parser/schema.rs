use super::{basis::*, entity::*, util::*};

/// Parsed result of EXPRESS's SCHEMA
#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub name: String,
    pub entities: Vec<Entity>,
}

pub fn schema_decl(input: &str) -> ParseResult<String> {
    remarked_tuple((
        remarked_tag("SCHEMA "),
        remarked(simple_id),
        remarked_char(';'),
    ))
    .remarked_map(|(_start, id, _semicoron)| id)
    .remarked_parse(input)
}

/// 295 schema_body = { interface_specification } \[ constant_decl \] { declaration | rule_decl } .
pub fn schema_body(input: &str) -> ParseResult<Vec<Entity>> {
    // FIXME constant_decl
    spaced_many0(entity_decl).remarked_parse(input)
}

/// 296 schema_decl = SCHEMA schema_id \[ schema_version_id \] `;` schema_body END_SCHEMA `;` .
pub fn schema(input: &str) -> ParseResult<Schema> {
    // FIXME schema_version_id
    remarked_tuple((
        schema_decl,
        schema_body,
        remarked_tag("END_SCHEMA"),
        remarked_char(';'),
    ))
    .remarked_map(|(name, entities, _end, _semicoron)| Schema { name, entities })
    .remarked_parse(input)
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

        let (residual, (schema, _remark)) = super::schema(exp_str).finish().unwrap();
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
