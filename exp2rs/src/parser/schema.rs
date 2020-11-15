use super::*;

/// Parsed result of EXPRESS's SCHEMA
#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub name: String,
    pub entities: Vec<Entity>,
}

pub fn schema_decl(input: &str) -> IResult<&str, String> {
    tuple((tag("SCHEMA"), multispace1, simple_id, multispace0, tag(";")))
        .map(|(_, _, id, _, _)| id)
        .parse(input)
}

/// 295 schema_body = { interface_specification } [ constant_decl ] { declaration | rule_decl } .
pub fn schema_body(input: &str) -> IResult<&str, Vec<Entity>> {
    // FIXME constant_decl
    separated_list0(multispace0, entity_decl).parse(input)
}

/// 296 schema_decl = SCHEMA schema_id [ schema_version_id ] `;` schema_body END_SCHEMA `;` .
pub fn schema(input: &str) -> IResult<&str, Schema> {
    // FIXME schema_version_id
    tuple((
        schema_decl,
        multispace0,
        schema_body,
        multispace0,
        tag("END_SCHEMA"),
        multispace0,
        tag(";"),
    ))
    .map(|(name, _, entities, _, _, _, _)| Schema { name, entities })
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

        let (residual, schema) = super::schema(exp_str).finish().unwrap();
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
        );
        assert_eq!(residual, "");
    }
}
