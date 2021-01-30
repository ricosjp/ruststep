use super::{
    super::{identifier::*, util::*},
    *,
};

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum ParameterType {
    Named(String),
    Simple(SimpleType),
}

/// 258 named_types = [entity_ref] | [type_ref] .
pub fn named_types(input: &str) -> ParseResult<String> {
    alt((entity_ref, type_ref)).parse(input)
}

/// 266 parameter_type = generalized_types | [named_types] | [simple_types] .
pub fn parameter_type(input: &str) -> ParseResult<ParameterType> {
    // FIXME generalized_types
    alt((
        named_types.map(|ty| ParameterType::Named(ty)),
        simple_types.map(|ty| ParameterType::Simple(ty)),
    ))
    .parse(input)
}

/// 171 aggregate_type = AGGREGATE \[ `:` [type_label] \] OF [parameter_type] .
pub fn aggregate_type(input: &str) -> ParseResult<()> {
    todo!()
}

/// 223 generalized_types = [aggregate_type] | [general_aggregation_types] | [generic_entity_type] | [generic_type] .
pub fn generalized_types(input: &str) -> ParseResult<()> {
    todo!()
}

/// 224 general_aggregation_types = [general_array_type] | [general_bag_type] | [general_list_type] | [general_set_type] .
pub fn general_aggregation_types(input: &str) -> ParseResult<()> {
    todo!()
}

/// 225 general_array_type = ARRAY \[ [bound_spec] \] OF \[ OPTIONAL \] \[ UNIQUE \] [parameter_type] .
pub fn general_array_type(input: &str) -> ParseResult<()> {
    todo!()
}
/// 226 general_bag_type = BAG \[ [bound_spec] \] OF [parameter_type] .
pub fn general_bag_type(input: &str) -> ParseResult<()> {
    todo!()
}
/// 227 general_list_type = LIST \[ [bound_spec] \] OF \[ UNIQUE \] [parameter_type] .
pub fn general_list_type(input: &str) -> ParseResult<()> {
    todo!()
}

/// 229 general_set_type = SET \[ [bound_spec] \] OF [parameter_type] .
pub fn general_set_type(input: &str) -> ParseResult<()> {
    todo!()
}

/// 230 generic_entity_type = GENERIC_ENTITY \[ `:` [type_label] \] .
pub fn generic_entity_type(input: &str) -> ParseResult<()> {
    todo!()
}

/// 231 generic_type = GENERIC \[ `:` [type_label] \] .
pub fn generic_type(input: &str) -> ParseResult<()> {
    todo!()
}

/// 329 type_label = [type_label_id] | [type_label_ref] .
pub fn type_label(input: &str) -> ParseResult<()> {
    todo!()
}
