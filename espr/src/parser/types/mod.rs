mod concrete;
mod enumeration;
mod generalized;
mod select;
mod simple;

pub use concrete::*;
pub use enumeration::*;
pub use generalized::*;
pub use select::*;
pub use simple::*;

use super::{domain::*, identifier::*, util::*};
use derive_more::From;

/// `EXTENSIBLE` and `GENERIC_ENTITY` keywords for [select_type] and [enumeration_type]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Extensiblity {
    /// No `EXTENSIBLE`
    None,
    /// `EXTENSIBLE`
    Extensible,
    /// `EXTENSIBLE GENERIC_ENTITY`, which is allowed only for `SELECT`
    GenericEntity,
}

/// Output of [type_decl]
#[derive(Debug, Clone, PartialEq)]
pub struct TypeDecl {
    type_id: String,
    underlying_type: UnderlyingType,
    where_clause: Option<WhereClause>,
}

/// Output of [constructed_types]
#[derive(Debug, Clone, PartialEq, Eq, From)]
pub enum ConstructedType {
    Enumeration(EnumerationType),
    Select(SelectType),
}

/// 198 constructed_types = [enumeration_type] | [select_type] .
pub fn constructed_types(input: &str) -> ParseResult<ConstructedType> {
    alt((
        enumeration_type.map(|e| ConstructedType::Enumeration(e)),
        select_type.map(|s| ConstructedType::Select(s)),
    ))
    .parse(input)
}

/// Output of [underlying_type]
#[derive(Debug, Clone, PartialEq, From)]
pub enum UnderlyingType {
    Constructed(ConstructedType),
    Concrete(ConcreteType),
}

/// 332 underlying_type = [concrete_types] | [constructed_types] .
pub fn underlying_type(input: &str) -> ParseResult<UnderlyingType> {
    alt((
        concrete_types.map(|ty| UnderlyingType::Concrete(ty)),
        constructed_types.map(|ty| UnderlyingType::Constructed(ty)),
    ))
    .parse(input)
}

/// 327 type_decl = TYPE [type_id] `=` [underlying_type] `;` \[ [where_clause] \] END_TYPE `;` .
pub fn type_decl(input: &str) -> ParseResult<TypeDecl> {
    tuple((
        tag("TYPE"),
        type_id,
        char('='),
        underlying_type,
        char(';'),
        opt(where_clause),
        tag("END_TYPE"),
        char(';'),
    ))
    .map(
        |(
            _start,
            type_id,
            _equal,
            underlying_type,
            _semicoron1,
            where_clause,
            _end,
            _semicoron2,
        )| {
            TypeDecl {
                type_id,
                underlying_type,
                where_clause,
            }
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
            super::TypeDecl {
                type_id: "my_type".to_string(),
                underlying_type: super::UnderlyingType::Concrete(super::ConcreteType::Simple(
                    super::SimpleType::String_ { width_spec: None }
                )),
                where_clause: None,
            }
        );
    }

    #[test]
    fn type_decl_where() {
        let (residual, (ty, _remark)) = super::type_decl(
            r#"
            TYPE dimension_count = INTEGER;
            WHERE
              wr1: SELF > 0;
            END_TYPE;
            "#
            .trim(), // from AP201
        )
        .finish()
        .unwrap();
        assert_eq!(residual, "");
        dbg!(ty);
    }
}
