use super::{entity::*, expression::*, identifier::*, stmt::*, subsuper::*, types::*, util::*};

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

/// 199 declaration = [entity_decl] | [function_decl] | [procedure_decl] | [subtype_constraint_decl] | [type_decl] .
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

/// 271 procedure_decl = [procedure_head] [algorithm_head] { [stmt] } END_PROCEDURE `;` .
pub fn procedure_decl(input: &str) -> ParseResult<()> {
    todo!()
}

/// 272 procedure_head = PROCEDURE [procedure_id] \[ `(` \[ VAR \] [formal_parameter] { `;` \[ VAR \] formal_parameter } `)` \] `;` .
pub fn procedure_head(input: &str) -> ParseResult<()> {
    todo!()
}

/// 220 function_decl = [function_head] [algorithm_head] [stmt] { [stmt] } END_FUNCTION `;` .
pub fn function_decl(input: &str) -> ParseResult<()> {
    todo!()
}

/// 221 function_head = FUNCTION [function_id] \[ `(` [formal_parameter] { `;` [formal_parameter] } `)` \] `:` [parameter_type] `;` .
pub fn function_head(input: &str) -> ParseResult<()> {
    todo!()
}

/// 218 formal_parameter = [parameter_id] { `,` [parameter_id] } `:` [parameter_type] .
pub fn formal_parameter(input: &str) -> ParseResult<()> {
    todo!()
}

/// 195 constant_decl = CONSTANT [constant_body] { [constant_body] } END_CONSTANT `;` .
pub fn constant_decl(input: &str) -> ParseResult<()> {
    todo!()
}

/// 194 constant_body = [constant_id] `:` [instantiable_type] `:=` [expression] `;` .
pub fn constant_body(input: &str) -> ParseResult<()> {
    todo!()
}

/// 240 instantiable_type = [concrete_types] | [entity_ref] .
pub fn instantiable_type(input: &str) -> ParseResult<()> {
    todo!()
}

/// 291 rule_decl = [rule_head] [algorithm_head] { [stmt] } [where_clause] END_RULE `;` .
pub fn rule_decl(input: &str) -> ParseResult<()> {
    todo!()
}

/// 292 rule_head = RULE [rule_id] FOR `(` [entity_ref] { `,` [entity_ref] } `)` `;` .
pub fn rule_head(input: &str) -> ParseResult<()> {
    todo!()
}

/// 173 algorithm_head = { [declaration] } \[ [constant_decl] \] \[ [local_decl] \] .
pub fn algorithm_head(input: &str) -> ParseResult<()> {
    todo!()
}

/// 252 local_decl = LOCAL [local_variable] { [local_variable] } END_LOCAL `;` .
pub fn local_decl(input: &str) -> ParseResult<()> {
    todo!()
}

/// 253 local_variable = [variable_id] { `,` [variable_id] } `:` [parameter_type] \[ `:=` [expression] \] `;` .
pub fn local_variable(input: &str) -> ParseResult<()> {
    todo!()
}

/// 242 interface_specification = [reference_clause] | [use_clause] .
pub fn interface_specification(input: &str) -> ParseResult<()> {
    todo!()
}

/// 281 reference_clause = REFERENCE FROM [schema_ref] \[ `(` [resource_or_rename] { `,` [resource_or_rename] } `)` \] `;` .
pub fn reference_clause(input: &str) -> ParseResult<()> {
    todo!()
}

/// 288 resource_or_rename = [resource_ref] \[ AS [rename_id] \] .
pub fn resource_or_rename(input: &str) -> ParseResult<()> {
    todo!()
}

/// 284 rename_id = [constant_id] | [entity_id] | [function_id] | [procedure_id] | [type_id] .
pub fn rename_id(input: &str) -> ParseResult<()> {
    todo!()
}

/// 289 resource_ref = [constant_ref] | [entity_ref] | [function_ref] | [procedure_ref] | [type_ref] .
pub fn resource_ref(input: &str) -> ParseResult<()> {
    todo!()
}

/// 336 use_clause = USE FROM [schema_ref] \[ `(` [named_type_or_rename] { `,` [named_type_or_rename] } `)` \] `;` .
pub fn use_clause(input: &str) -> ParseResult<()> {
    todo!()
}

/// 259 named_type_or_rename = [named_types] \[ AS ( [entity_id] | [type_id] ) \] .
pub fn named_type_or_rename(input: &str) -> ParseResult<()> {
    todo!()
}

/// 295 schema_body = { [interface_specification] } \[ [constant_decl] \] { [declaration] | [rule_decl] } .
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
