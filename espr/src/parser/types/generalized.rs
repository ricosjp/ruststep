use super::{
    super::{combinator::*, identifier::*},
    *,
};

/// 258 named_types = [entity_ref] | [type_ref] .
pub fn named_types(input: &str) -> ParseResult<String> {
    alt((entity_ref, type_ref)).parse(input)
}

/// 266 parameter_type = [generalized_types] | [named_types] | [simple_types] .
pub fn parameter_type(input: &str) -> ParseResult<ParameterType> {
    alt((
        generalized_types,
        simple_types.map(ParameterType::Simple),
        named_types.map(ParameterType::Named),
    ))
    .parse(input)
}

/// 223 generalized_types = [aggregate_type] | [general_aggregation_types] | [generic_entity_type] | [generic_type] .
pub fn generalized_types(input: &str) -> ParseResult<ParameterType> {
    alt((
        aggregate_type,
        general_aggregation_types,
        generic_entity_type,
        generic_type,
    ))
    .parse(input)
}

/// 171 aggregate_type = AGGREGATE \[ `:` [type_label] \] OF [parameter_type] .
pub fn aggregate_type(input: &str) -> ParseResult<ParameterType> {
    tuple((
        tag("AGGREGATE"),
        opt(tuple((char(':'), type_label))),
        tag("OF"),
        parameter_type,
    ))
    .map(
        |(_aggregate, opt_type_label, _of, base)| ParameterType::Aggregate {
            base: Box::new(base),
            label: opt_type_label.map(|(_colon, label)| label),
        },
    )
    .parse(input)
}

/// 230 generic_entity_type = GENERIC_ENTITY \[ `:` [type_label] \] .
pub fn generic_entity_type(input: &str) -> ParseResult<ParameterType> {
    tuple((tag("GENERIC_ENTITY"), opt(tuple((char(':'), type_label)))))
        .map(|(_generic, opt)| ParameterType::GenericEntity(opt.map(|(_colon, label)| label)))
        .parse(input)
}

/// 231 generic_type = GENERIC \[ `:` [type_label] \] .
pub fn generic_type(input: &str) -> ParseResult<ParameterType> {
    tuple((tag("GENERIC"), opt(tuple((char(':'), type_label)))))
        .map(|(_generic, opt)| ParameterType::Generic(opt.map(|(_colon, label)| label)))
        .parse(input)
}

/// 329 type_label = [type_label_id] | [type_label_ref] .
pub fn type_label(input: &str) -> ParseResult<String> {
    alt((type_label_id, type_label_ref)).parse(input)
}

/// 224 general_aggregation_types = [general_array_type] | [general_bag_type] | [general_list_type] | [general_set_type] .
pub fn general_aggregation_types(input: &str) -> ParseResult<ParameterType> {
    alt((
        general_array_type,
        general_bag_type,
        general_list_type,
        general_set_type,
    ))
    .parse(input)
}

/// 225 general_array_type = ARRAY \[ [bound_spec] \] OF \[ OPTIONAL \] \[ UNIQUE \] [parameter_type] .
pub fn general_array_type(input: &str) -> ParseResult<ParameterType> {
    tuple((
        tag("ARRAY"),
        opt(bound_spec),
        tag("OF"),
        opt(tag("OPTIONAL")),
        opt(tag("UNIQUE")),
        parameter_type,
    ))
    .map(
        |(_bag, bound, _of, optional, unique, base)| ParameterType::Array {
            base: Box::new(base),
            bound,
            unique: unique.is_some(),
            optional: optional.is_some(),
        },
    )
    .parse(input)
}

/// 226 general_bag_type = BAG \[ [bound_spec] \] OF [parameter_type] .
pub fn general_bag_type(input: &str) -> ParseResult<ParameterType> {
    tuple((tag("BAG"), opt(bound_spec), tag("OF"), parameter_type))
        .map(|(_bag, bound, _of, base)| ParameterType::Bag {
            base: Box::new(base),
            bound,
        })
        .parse(input)
}

/// 227 general_list_type = LIST \[ [bound_spec] \] OF \[ UNIQUE \] [parameter_type] .
pub fn general_list_type(input: &str) -> ParseResult<ParameterType> {
    tuple((
        tag("LIST"),
        opt(bound_spec),
        tag("OF"),
        opt(tag("UNIQUE")),
        parameter_type,
    ))
    .map(|(_list, bound, _of, unique, base)| ParameterType::List {
        base: Box::new(base),
        unique: unique.is_some(),
        bound,
    })
    .parse(input)
}

/// 229 general_set_type = SET \[ [bound_spec] \] OF [parameter_type] .
pub fn general_set_type(input: &str) -> ParseResult<ParameterType> {
    tuple((tag("SET"), opt(bound_spec), tag("OF"), parameter_type))
        .map(|(_set, bound, _of, base)| ParameterType::Set {
            base: Box::new(base),
            bound,
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::expression::Expression;
    use nom::Finish;

    #[test]
    fn simple() {
        let (res, (set, _remarks)) = super::parameter_type("STRING").finish().unwrap();
        dbg!(&set);
        assert_eq!(res, "");
        assert_eq!(
            set,
            ParameterType::Simple(SimpleType::String_ { width_spec: None }),
        )
    }

    #[test]
    fn named() {
        let (res, (set, _remarks)) = super::parameter_type("vim").finish().unwrap();
        dbg!(&set);
        assert_eq!(res, "");
        assert_eq!(set, ParameterType::Named("vim".to_string()),)
    }

    #[test]
    fn set() {
        let (res, (set, _remarks)) = super::parameter_type("SET [1:?] OF curve")
            .finish()
            .unwrap();
        dbg!(&set);
        assert_eq!(res, "");
        assert_eq!(
            set,
            ParameterType::Set {
                base: Box::new(ParameterType::Named("curve".to_string())),
                bound: Some(Bound {
                    upper: Expression::indeterminate(),
                    lower: Expression::real(1.0),
                })
            }
        )
    }
}
