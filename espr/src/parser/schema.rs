use super::{basis::*, entity::*, remark::*, util::*};
use nom::{bytes::complete::*, character::complete::*, sequence::*, Parser};

/// Parsed result of EXPRESS's SCHEMA
#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub name: String,
    pub entities: Vec<Entity>,
}

pub fn schema_decl(input: &str) -> ParseResult<String> {
    tuple((
        tag("SCHEMA"),
        multispace1,
        spaces_or_remarks,
        simple_id,
        spaces_or_remarks,
        tag(";"),
    ))
    .map(|(_start, _space, mut remarks, id, mut r1, _semicoron)| {
        remarks.append(&mut r1);
        (id, remarks)
    })
    .parse(input)
}

/// 295 schema_body = { interface_specification } \[ constant_decl \] { declaration | rule_decl } .
pub fn schema_body(input: &str) -> ParseResult<Vec<Entity>> {
    // FIXME constant_decl
    spaced_many0(entity_decl).parse(input)
}

/// 296 schema_decl = SCHEMA schema_id \[ schema_version_id \] `;` schema_body END_SCHEMA `;` .
pub fn schema(input: &str) -> ParseResult<Schema> {
    // FIXME schema_version_id
    tuple((
        schema_decl,
        spaces_or_remarks,
        schema_body,
        spaces_or_remarks,
        tag("END_SCHEMA"),
        spaces_or_remarks,
        tag(";"),
    ))
    .map(
        |((name, mut remarks), mut r1, (entities, mut r2), mut r3, _end, mut r4, _semicoron)| {
            remarks.append(&mut r1);
            remarks.append(&mut r2);
            remarks.append(&mut r3);
            remarks.append(&mut r4);
            (Schema { name, entities }, remarks)
        },
    )
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
