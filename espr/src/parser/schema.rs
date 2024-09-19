use super::{
    combinator::*, entity::*, expression::*, identifier::*, stmt::*, subsuper::*, types::*,
};
use crate::ast::*;

/// 296 schema_decl = SCHEMA [schema_id] \[ schema_version_id \] `;` [schema_body] END_SCHEMA `;` .
pub fn schema_decl(input: &str) -> ParseResult<Schema> {
    // FIXME schema_version_id
    let schema_head =
        tuple((tag("SCHEMA "), schema_id, char(';'))).map(|(_start, id, _semicolon)| id);
    tuple((schema_head, schema_body, tag("END_SCHEMA"), char(';')))
        .map(|(name, (interfaces, constants, decls), _end, _semicolon)| {
            let mut entities = Vec::new();
            let mut types = Vec::new();
            let mut functions = Vec::new();
            let mut procedures = Vec::new();
            let mut rules = Vec::new();
            let mut subtype_constraints = Vec::new();

            for decl in decls {
                match decl {
                    Declaration::Entity(e) => entities.push(e),
                    Declaration::Type(ty) => types.push(ty),
                    Declaration::Function(f) => functions.push(f),
                    Declaration::Procedure(p) => procedures.push(p),
                    Declaration::Rule(r) => rules.push(r),
                    Declaration::SubTypeConstraint(sub) => subtype_constraints.push(sub),
                }
            }

            Schema {
                name,
                entities,
                types,
                functions,
                procedures,
                rules,
                constants,
                interfaces,
                subtype_constraints,
            }
        })
        .parse(input)
}

/// 295 schema_body = { [interface_specification] } \[ [constant_decl] \] { [declaration] | [rule_decl] } .
pub fn schema_body(
    input: &str,
) -> ParseResult<(Vec<InterfaceSpec>, Vec<Constant>, Vec<Declaration>)> {
    tuple((
        many0(interface_specification),
        opt(constant_decl).map(|opt| opt.unwrap_or_default()),
        many0(alt((declaration, rule_decl.map(Declaration::Rule)))),
    ))
    .parse(input)
}

/// 199 declaration = [entity_decl] | [function_decl] | [procedure_decl] | [subtype_constraint_decl] | [type_decl] .
pub fn declaration(input: &str) -> ParseResult<Declaration> {
    alt((
        entity_decl.map(Declaration::Entity),
        type_decl.map(Declaration::Type),
        function_decl.map(Declaration::Function),
        procedure_decl.map(Declaration::Procedure),
        subtype_constraint_decl.map(Declaration::SubTypeConstraint),
    ))
    .parse(input)
}

/// 271 procedure_decl = [procedure_head] [algorithm_head] { [stmt] } END_PROCEDURE `;` .
pub fn procedure_decl(input: &str) -> ParseResult<Procedure> {
    tuple((
        procedure_head,
        algorithm_head,
        many0(stmt),
        tag("END_PROCEDURE"),
        char(';'),
    ))
    .map(
        |(
            (name, parameters),
            (declarations, constants, variables),
            statements,
            _end,
            _semicolon,
        )| Procedure {
            name,
            parameters,
            declarations,
            constants,
            variables,
            statements,
        },
    )
    .parse(input)
}

/// 272 procedure_head = PROCEDURE [procedure_id]
///                    \[ `(`
///                      \[ VAR \] [formal_parameter] { `;`
///                      \[ VAR \] [formal_parameter]
///                    }
///                    `)` \] `;` .
pub fn procedure_head(input: &str) -> ParseResult<(String, Vec<FormalParameter>)> {
    let param = tuple((opt(tag("VAR")), formal_parameter)).map(|(var, mut params)| {
        for param in &mut params {
            param.is_variable = var.is_some();
        }
        params
    });
    tuple((
        tag("PROCEDURE"),
        procedure_id,
        opt(
            tuple((char('('), semicolon_separated(param), char(')'))).map(
                |(_open, params, _close)| {
                    params.into_iter().flat_map(|ps| ps.into_iter()).collect()
                },
            ),
        )
        .map(|opt| opt.unwrap_or_default()),
        char(';'),
    ))
    .map(|(_procedure, name, parameters, _semicolon)| (name, parameters))
    .parse(input)
}

/// 220 function_decl = [function_head] [algorithm_head] [stmt] { [stmt] } END_FUNCTION `;` .
pub fn function_decl(input: &str) -> ParseResult<Function> {
    tuple((
        function_head,
        algorithm_head,
        many1(stmt),
        tag("END_FUNCTION"),
        char(';'),
    ))
    .map(
        |(
            (name, parameters, return_type),
            (declarations, constants, variables),
            statements,
            _end,
            _semicolon,
        )| Function {
            name,
            parameters,
            declarations,
            constants,
            variables,
            statements,
            return_type,
        },
    )
    .parse(input)
}

/// 221 function_head = FUNCTION [function_id]
///                   \[ `(` [formal_parameter] { `;` [formal_parameter] } `)` \]
///                   `:` [parameter_type] `;` .
pub fn function_head(input: &str) -> ParseResult<(String, Vec<FormalParameter>, Type)> {
    tuple((
        tag("FUNCTION"),
        function_id,
        opt(
            tuple((char('('), semicolon_separated(formal_parameter), char(')'))).map(
                |(_open, params, _close)| {
                    params.into_iter().flat_map(|ps| ps.into_iter()).collect()
                },
            ),
        )
        .map(|opt| opt.unwrap_or_default()),
        char(':'),
        parameter_type,
        char(';'),
    ))
    .map(|(_function, name, parameters, _comma, ty, _semicolon)| (name, parameters, ty))
    .parse(input)
}

/// 218 formal_parameter = [parameter_id] { `,` [parameter_id] } `:` [parameter_type] .
pub fn formal_parameter(input: &str) -> ParseResult<Vec<FormalParameter>> {
    tuple((comma_separated(parameter_id), char(':'), parameter_type))
        .map(|(names, _comma, ty)| {
            names
                .into_iter()
                .map(|name| FormalParameter {
                    name,
                    ty: ty.clone(),
                    is_variable: false,
                })
                .collect()
        })
        .parse(input)
}

/// 195 constant_decl = CONSTANT [constant_body] { [constant_body] } END_CONSTANT `;` .
pub fn constant_decl(input: &str) -> ParseResult<Vec<Constant>> {
    tuple((
        tag("CONSTANT"),
        many1(constant_body),
        tag("END_CONSTANT"),
        char(';'),
    ))
    .map(|(_constant, consts, _end, _semicolon)| consts)
    .parse(input)
}

/// 194 constant_body = [constant_id] `:` [instantiable_type] `:=` [expression] `;` .
pub fn constant_body(input: &str) -> ParseResult<Constant> {
    tuple((
        constant_id,
        char(':'),
        instantiable_type,
        tag(":="),
        expression,
        char(';'),
    ))
    .map(|(name, _colon, ty, _def, expr, _semicolon)| Constant { name, ty, expr })
    .parse(input)
}

/// 291 rule_decl = [rule_head] [algorithm_head] { [stmt] } [where_clause] END_RULE `;` .
pub fn rule_decl(input: &str) -> ParseResult<Rule> {
    tuple((
        rule_head,
        algorithm_head,
        many0(stmt),
        where_clause,
        tag("END_RULE"),
        char(';'),
    ))
    .map(
        |(
            (name, references),
            (declarations, constants, variables),
            statements,
            where_clause,
            _end,
            _semicolon,
        )| Rule {
            name,
            references,
            declarations,
            constants,
            variables,
            statements,
            where_clause,
        },
    )
    .parse(input)
}

/// 292 rule_head = RULE [rule_id] FOR `(` [entity_ref] { `,` [entity_ref] } `)` `;` .
pub fn rule_head(input: &str) -> ParseResult<(String, Vec<String>)> {
    tuple((
        tag("RULE"),
        rule_id,
        tag("FOR"),
        char('('),
        comma_separated(entity_ref),
        char(')'),
        char(';'),
    ))
    .map(|(_rule, name, _for, _open, references, _close, _semicolon)| (name, references))
    .parse(input)
}

/// 173 algorithm_head = { [declaration] } \[ [constant_decl] \] \[ [local_decl] \] .
pub fn algorithm_head(
    input: &str,
) -> ParseResult<(Vec<Declaration>, Vec<Constant>, Vec<LocalVariable>)> {
    tuple((
        many0(declaration),
        opt(constant_decl).map(|opt| opt.unwrap_or_default()),
        opt(local_decl).map(|opt| opt.unwrap_or_default()),
    ))
    .parse(input)
}

/// 252 local_decl = LOCAL [local_variable] { [local_variable] } END_LOCAL `;` .
pub fn local_decl(input: &str) -> ParseResult<Vec<LocalVariable>> {
    tuple((
        tag("LOCAL"),
        many1(local_variable),
        tag("END_LOCAL"),
        char(';'),
    ))
    .map(|(_local, vars, _end, _semicolon)| {
        vars.into_iter().flat_map(|var| var.into_iter()).collect()
    })
    .parse(input)
}

/// 253 local_variable = [variable_id] { `,` [variable_id] } `:` [parameter_type] \[ `:=` [expression] \] `;` .
pub fn local_variable(input: &str) -> ParseResult<Vec<LocalVariable>> {
    tuple((
        comma_separated(variable_id),
        char(':'),
        parameter_type,
        opt(tuple((tag(":="), expression)).map(|(_def, expr)| expr)),
        char(';'),
    ))
    .map(|(names, _comma, ty, expr, _semicolon)| {
        names
            .into_iter()
            .map(|name| LocalVariable {
                name,
                ty: ty.clone(),
                expr: expr.clone(),
            })
            .collect()
    })
    .parse(input)
}

/// 242 interface_specification = [reference_clause] | [use_clause] .
pub fn interface_specification(input: &str) -> ParseResult<InterfaceSpec> {
    alt((reference_clause, use_clause)).parse(input)
}

/// 281 reference_clause = REFERENCE FROM [schema_ref] \[ `(` [resource_or_rename] { `,` [resource_or_rename] } `)` \] `;` .
pub fn reference_clause(input: &str) -> ParseResult<InterfaceSpec> {
    tuple((
        tag("REFERENCE"),
        tag("FROM"),
        schema_ref,
        opt(
            tuple((char('('), comma_separated(resource_or_rename), char(')')))
                .map(|(_open, ty, _close)| ty),
        )
        .map(|opt| opt.unwrap_or_default()),
        char(';'),
    ))
    .map(|(_use, _from, name, resources, _semicolon)| InterfaceSpec::Reference { name, resources })
    .parse(input)
}

/// 288 resource_or_rename = [resource_ref] \[ AS [rename_id] \] .
pub fn resource_or_rename(input: &str) -> ParseResult<(String, Option<String>)> {
    tuple((
        resource_ref,
        opt(tuple((tag("AS"), rename_id)).map(|(_as, rename)| rename)),
    ))
    .parse(input)
}

/// 336 use_clause = USE FROM [schema_ref]
///                \[ `(` [named_type_or_rename] { `,` [named_type_or_rename] } `)`
///                \] `;` .
pub fn use_clause(input: &str) -> ParseResult<InterfaceSpec> {
    tuple((
        tag("USE"),
        tag("FROM"),
        schema_ref,
        opt(
            tuple((char('('), comma_separated(named_type_or_rename), char(')')))
                .map(|(_open, ty, _close)| ty),
        )
        .map(|opt| opt.unwrap_or_default()),
        char(';'),
    ))
    .map(|(_use, _from, name, types, _semicolon)| InterfaceSpec::Use { name, types })
    .parse(input)
}

/// 259 named_type_or_rename = [named_types] \[ AS ( [entity_id] | [type_id] ) \] .
pub fn named_type_or_rename(input: &str) -> ParseResult<(String, Option<String>)> {
    tuple((
        named_types,
        opt(tuple((tag("AS"), alt((entity_id, type_id)))).map(|(_as, id)| id)),
    ))
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

    #[test]
    fn constant() {
        let exp_str = r#"
        CONSTANT
            dummy_gri : geometric_representation_item := representation_item('') || geometric_representation_item();
            dummy_tri : topological_representation_item := representation_item('') || topological_representation_item();
        END_CONSTANT;
        "#
        .trim();
        let (residual, (local, _remark)) = super::constant_decl(exp_str).finish().unwrap();
        dbg!(&local);
        assert_eq!(residual, "");
    }

    #[test]
    fn local1() {
        // From ISO-10303-11 p.72
        let exp_str = r#"
        LOCAL
            r_result : REAL := 0.0;
            i_result : INTEGER;
        END_LOCAL;
        "#
        .trim();
        let (residual, (local, _remark)) = super::local_decl(exp_str).finish().unwrap();
        dbg!(&local);
        assert_eq!(residual, "");
    }

    #[test]
    fn local2() {
        // Part of rule1
        let exp_str = r#"
        LOCAL
            first_oct ,
            seventh_oct : SET OF point := []; -- empty set of point (see 12.9)
        END_LOCAL;
        "#
        .trim();
        let (residual, (local, _remark)) = super::local_decl(exp_str).finish().unwrap();
        dbg!(&local);
        assert_eq!(residual, "");
    }

    #[test]
    fn rule1() {
        // From ISO-10303-11 p.73
        let exp_str = r#"
        RULE point_match FOR (point);
        LOCAL
            first_oct ,
            seventh_oct : SET OF point := []; -- empty set of point (see 12.9), change `POINT` -> `point`
        END_LOCAL;
            first_oct := QUERY(temp <* point | (temp.x > 0) AND
                (temp.y > 0) AND
                (temp.z > 0) );
            seventh_oct := QUERY(temp <* point | (temp.x < 0) AND
                (temp.y < 0) AND
                (temp.z < 0) );
        WHERE
            SIZEOF(first_oct) = SIZEOF(seventh_oct);
        END_RULE;
        "#
        .trim();
        let (residual, (rule, _remark)) = super::rule_decl(exp_str).finish().unwrap();
        dbg!(&rule);
        assert_eq!(residual, "");
    }

    #[test]
    fn rule2() {
        // From ISO-10303-11 p.73
        let exp_str = r#"
        RULE vu FOR (b);
            ENTITY temp;
                a1 : c;
                a2 : d;
            END_ENTITY;
        LOCAL
            s : SET OF temp := [];
        END_LOCAL;
        REPEAT i := 1 TO SIZEOF(b);
            s := s + temp(b[i].a1, b[i].a2);
        END_REPEAT;
        WHERE
            wr1 : VALUE_UNIQUE(s);
        END_RULE;
        "#
        .trim();
        let (residual, (rule, _remark)) = super::rule_decl(exp_str).finish().unwrap();
        dbg!(&rule);
        assert_eq!(residual, "");
    }

    #[test]
    fn formal_parameter1() {
        let (residual, (p, _remark)) = super::formal_parameter("input : AGGREGATE:intype OF REAL")
            .finish()
            .unwrap();
        dbg!(&p);
        assert_eq!(residual, "");
    }

    #[test]
    fn formal_parameter2() {
        let (residual, (p, _remark)) = super::formal_parameter("a,b: GENERIC:intype")
            .finish()
            .unwrap();
        dbg!(&p);
        assert_eq!(residual, "");
    }

    #[test]
    fn function_aggregate() {
        // From ISO-10303-11 p.66-67
        let exp_str = r#"
        FUNCTION scale(input : AGGREGATE:intype OF REAL; scalar: REAL): AGGREGATE:intype OF REAL;
            LOCAL
                result : AGGREGATE:intype OF REAL := input;
            END_LOCAL;
            IF SIZEOF(['BAG','SET'] * TYPEOF(input)) > 0 THEN
                REPEAT i := LOINDEX(input) TO HIINDEX(input);
                    result := result - input[i];              -- remove the original
                    result := result + scalar*input[i];       -- insert the scaled
                END_REPEAT;
            ELSE
                REPEAT i := LOINDEX(input) TO HIINDEX(input);
                    result[i] := scalar*input[i];
                END_REPEAT;
            END_IF;
            RETURN(result);
        END_FUNCTION;
        "#
        .trim();
        let (residual, (rule, _remark)) = super::function_decl(exp_str).finish().unwrap();
        dbg!(&rule);
        assert_eq!(residual, "");
    }

    #[test]
    fn function_generic() {
        // From ISO-10303-11 p.67-68
        let exp_str = r#"
        FUNCTION add(a,b: GENERIC:intype): GENERIC:intype;
            LOCAL
                nr : NUMBER; -- integer or real
                vr : vector;
            END_LOCAL;
            IF ('NUMBER' IN TYPEOF(a)) AND ('NUMBER' IN TYPEOF(b)) THEN
                nr := a+b;
                RETURN(nr);
            ELSE
                IF ('THIS_SCHEMA.VECTOR' IN TYPEOF(a)) AND ('THIS_SCHEMA.VECTOR' IN TYPEOF(b)) THEN
                    vr := vector(a.i + b.i,
                    a.j + b.j,
                    a.k + b.k);
                    RETURN(vr);
                END_IF;
            END_IF;
            RETURN (?); --"add" if we receive input that is invalid, return a no-value
        END_FUNCTION;
        "#
        .trim();
        let (residual, (rule, _remark)) = super::function_decl(exp_str).finish().unwrap();
        dbg!(&rule);
        assert_eq!(residual, "");
    }

    #[test]
    fn function_generic_entity() {
        // From ISO-10303-11 p.68
        let exp_str = r#"
        FUNCTION check_relating (type1 : instance_of_type_1;
                                 type2 : instance_of_type_2;
                                 sample : GENERIC_ENTITY): BOOLEAN;
            RETURN ((type1 IN USEDIN(sample, '')) AND (type2 IN USEDIN(sample, '')));
        END_FUNCTION;
        "#
        .trim();
        let (residual, (rule, _remark)) = super::function_decl(exp_str).finish().unwrap();
        dbg!(&rule);
        assert_eq!(residual, "");
    }

    #[test]
    fn function_ap201_bag_to_set() {
        // From AP201
        let exp_str = r#"
        FUNCTION bag_to_set (the_bag : BAG OF GENERIC : intype) : SET OF GENERIC : intype;
            LOCAL
                the_set: SET OF GENERIC : intype := [];
                i      : INTEGER;
            END_LOCAL;
            IF SIZEOF (the_bag) > 0 THEN
                REPEAT i := 1 TO HIINDEX (the_bag);
                    the_set := the_set + the_bag [i];
                END_REPEAT;
            END_IF;
            RETURN (the_set);
        END_FUNCTION;
        "#
        .trim();
        let (residual, (rule, _remark)) = super::function_decl(exp_str).finish().unwrap();
        dbg!(&rule);
        assert_eq!(residual, "");
    }

    #[test]
    fn function_ap201_valid_calendar_date() {
        // From AP201
        let exp_str = r#"
        FUNCTION valid_calendar_date (date : calendar_date) : LOGICAL;
            IF NOT ({1 <= date.day_component <= 31}) THEN
                RETURN(FALSE);
            END_IF;
            CASE date.month_component OF
                4         : RETURN({ 1<= date.day_component <= 30});
                6         : RETURN({ 1<= date.day_component <= 30});
                9         : RETURN({ 1<= date.day_component <= 30});
                11        : RETURN({ 1<= date.day_component <= 30});
                2         : 
                    BEGIN
                        IF (leap_year(date.year_component)) THEN
                            RETURN({ 1<= date.day_component <= 29});
                        ELSE
                            RETURN({ 1<= date.day_component <= 28});
                        END_IF;
                    END;
                OTHERWISE : RETURN(TRUE);
            END_CASE;
        END_FUNCTION;
        "#
        .trim();
        let (residual, (rule, _remark)) = super::function_decl(exp_str).finish().unwrap();
        dbg!(&rule);
        assert_eq!(residual, "");
    }
}
