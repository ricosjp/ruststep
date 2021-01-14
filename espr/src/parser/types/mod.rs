mod enumeration;
mod select;
mod simple;

pub use enumeration::*;
pub use select::*;
pub use simple::*;

use super::{basis::*, util::*};
use derive_more::From;

/// `EXTENSIBLE` keyword for `SELECT` and `ENUMERATION` declarations
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Extensiblity {
    /// No `EXTENSIBLE`
    None,
    /// `EXTENSIBLE`
    Extensible,
    /// `EXTENSIBLE GENERIC_ENTITY`, which is allowed only for `SELECT`
    GenericEntity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    type_id: String,
    underlying_type: UnderlyingType,
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum ConstructedType {
    Enumeration(EnumerationType),
    Select(SelectType),
}

/// 198 constructed_types = enumeration_type | select_type .
pub fn constructed_types(input: &str) -> ParseResult<ConstructedType> {
    alt((
        enumeration_type.map(|e| ConstructedType::Enumeration(e)),
        select_type.map(|s| ConstructedType::Select(s)),
    ))
    .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum ConcreteType {
    Simple(SimpleType),
    AggrecationType,
    Ref(String),
}

/// 193 concrete_types = aggregation_types | simple_types | type_ref.
pub fn concrete_types(input: &str) -> ParseResult<ConcreteType> {
    // FIXME aggregation_types
    alt((
        remarked(simple_id).map(|s| ConcreteType::Ref(s)),
        simple_types.map(|ty| ConcreteType::Simple(ty)),
    ))
    .parse(input)
}

#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum UnderlyingType {
    Constructed(ConstructedType),
    Concrete(ConcreteType),
}

/// 332 underlying_type = concrete_types | constructed_types .
pub fn underlying_type(input: &str) -> ParseResult<UnderlyingType> {
    alt((
        concrete_types.map(|ty| UnderlyingType::Concrete(ty)),
        constructed_types.map(|ty| UnderlyingType::Constructed(ty)),
    ))
    .parse(input)
}

/// 327 type_decl = TYPE type_id `=` underlying_type `;` \[ where_clause \] END_TYPE `;` .
pub fn type_decl(input: &str) -> ParseResult<Type> {
    tuple((
        tag("TYPE"),
        remarked(simple_id), // type_id
        char('='),
        underlying_type,
        char(';'),
        tag("END_TYPE"),
        char(';'),
    ))
    .map(
        |(_start, type_id, _equal, underlying_type, _semicoron1, _end, _semicoron2)| Type {
            type_id,
            underlying_type,
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn type_decl() {
        let (residual, (ty, _remark)) = super::type_decl("TYPE my_type = STRING; END_TYPE;")
            .finish()
            .unwrap();
        assert_eq!(residual, "");
        assert_eq!(
            ty,
            super::Type {
                type_id: "my_type".to_string(),
                underlying_type: super::UnderlyingType::Concrete(super::ConcreteType::Simple(
                    super::SimpleType::String_ { width_spec: None }
                ))
            }
        );
    }
}
