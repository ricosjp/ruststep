use super::*;

/// Parsed result of EXPRESS's SCHEMA
///
/// Example
/// --------
///
/// ```
/// use exp2rs::parser;
/// use nom::Finish;
///
/// let exp_str = r#"
/// SCHEMA my_first_schema;
///   ENTITY first;
///     m_ref : second;
///     fattr : STRING;
///   END_ENTITY;
///
///   ENTITY second;
///     sattr : STRING;
///   END_ENTITY;
/// END_SCHEMA;
/// "#.trim();
///
/// let (residual, schema) = parser::schema(exp_str).finish().unwrap();
/// assert_eq!(schema.name, "my_first_schema");
/// assert_eq!(schema.entities.len(), 2);
/// assert_eq!(
///     schema.entities[0],
///     parser::entity(r#"
///       ENTITY first;
///         m_ref : second;
///         fattr : STRING;
///       END_ENTITY;
///     "#.trim()).finish().unwrap().1
/// );
/// assert_eq!(
///     schema.entities[1],
///     parser::entity(r#"
///       ENTITY second;
///         sattr : STRING;
///       END_ENTITY;
///     "#.trim()).finish().unwrap().1
/// );
/// assert_eq!(residual, "");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Schema {
    pub name: String,
    pub entities: Vec<Entity>,
}

fn schema_decl(input: &str) -> IResult<&str, String> {
    tuple((tag("SCHEMA"), multispace1, simple_id, multispace0, tag(";")))
        .map(|(_, _, id, _, _)| id)
        .parse(input)
}

fn schema_end(input: &str) -> IResult<&str, ()> {
    tuple((tag("END_SCHEMA"), multispace0, tag(";")))
        .map(|_| ())
        .parse(input)
}

/// 9.3 Schema
///
/// ```text
/// 296 schema_decl = SCHEMA schema_id [ schema_version_id ] `;` schema_body END_SCHEMA `;` .
/// 298 schema_version_id = string_literal .
/// 295 schema_body = { interface_specification } [ constant_decl ] { declaration | rule_decl } .
/// 242 interface_specification = reference_clause | use_clause .
/// 199 declaration = entity_decl | function_decl | procedure_decl | subtype_constraint_decl | type_decl .
/// ```
pub fn schema(input: &str) -> IResult<&str, Schema> {
    tuple((
        schema_decl,
        multispace0,
        separated_list0(multispace0, entity_decl),
        multispace0,
        schema_end,
    ))
    .map(|(name, _, entities, _, _)| Schema { name, entities })
    .parse(input)
}
