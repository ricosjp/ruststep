use super::{
    super::{combinator::*, expression::*, identifier::*},
    *,
};
use crate::ast::expression::*;

/// 193 concrete_types = [aggregation_types] | [simple_types] | [type_ref].
pub fn concrete_types(input: &str) -> ParseResult<ConcreteType> {
    alt((
        aggregation_types,
        simple_types.map(|ty| ConcreteType::Simple(ty)),
        type_ref.map(|s| ConcreteType::Reference(s)),
    ))
    .parse(input)
}

/// 172 aggregation_types = [array_type] | [bag_type] | [list_type] | [set_type] .
pub fn aggregation_types(input: &str) -> ParseResult<ConcreteType> {
    alt((array_type, bag_type, list_type, set_type)).parse(input)
}

/// 175 array_type = ARRAY [bound_spec] OF \[ OPTIONAL \] \[ UNIQUE \] [instantiable_type] .
pub fn array_type(input: &str) -> ParseResult<ConcreteType> {
    tuple((
        tag("ARRAY"),
        bound_spec,
        tag("OF"),
        opt(tag("OPTIONAL")),
        opt(tag("UNIQUE")),
        instantiable_type,
    ))
    .map(
        |(_set, bound, _of, optional, unique, base)| ConcreteType::Array {
            bound,
            unique: unique.is_some(),
            optional: optional.is_some(),
            base: Box::new(base),
        },
    )
    .parse(input)
}

/// 180 bag_type = BAG \[ [bound_spec] \] OF [instantiable_type] .
pub fn bag_type(input: &str) -> ParseResult<ConcreteType> {
    tuple((tag("BAG"), opt(bound_spec), tag("OF"), instantiable_type))
        .map(|(_set, bound, _of, base)| ConcreteType::Bag {
            bound,
            base: Box::new(base),
        })
        .parse(input)
}

/// 250 list_type = LIST \[ [bound_spec] \] OF \[ UNIQUE \] [instantiable_type] .
pub fn list_type(input: &str) -> ParseResult<ConcreteType> {
    tuple((
        tag("LIST"),
        opt(bound_spec),
        tag("OF"),
        opt(tag("UNIQUE")),
        instantiable_type,
    ))
    .map(|(_set, bound, _of, unique, base)| ConcreteType::List {
        bound,
        unique: unique.is_some(),
        base: Box::new(base),
    })
    .parse(input)
}

/// 303 set_type = SET \[ [bound_spec] \] OF [instantiable_type] .
pub fn set_type(input: &str) -> ParseResult<ConcreteType> {
    tuple((tag("SET"), opt(bound_spec), tag("OF"), instantiable_type))
        .map(|(_set, bound, _of, base)| ConcreteType::Set {
            bound,
            base: Box::new(base),
        })
        .parse(input)
}

/// 183 bound_1 = [numeric_expression] .
pub fn bound_1(input: &str) -> ParseResult<Expression> {
    numeric_expression(input)
}

/// 184 bound_2 = [numeric_expression] .
pub fn bound_2(input: &str) -> ParseResult<Expression> {
    numeric_expression(input)
}

/// 185 bound_spec = `[` [bound_1] `:` [bound_2] `]` .
pub fn bound_spec(input: &str) -> ParseResult<Bound> {
    tuple((char('['), bound_1, char(':'), bound_2, char(']')))
        .map(|(_open, lower, _comma, upper, _close)| Bound { lower, upper })
        .parse(input)
}

/// 240 instantiable_type = [concrete_types] | [entity_ref] .
pub fn instantiable_type(input: &str) -> ParseResult<ConcreteType> {
    alt((
        concrete_types,
        entity_ref.map(|r| ConcreteType::Reference(r)),
    ))
    .parse(input)
}
