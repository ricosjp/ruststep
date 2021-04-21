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

use super::{combinator::*, entity::*, identifier::*};
use crate::ast::types::*;

/// 198 constructed_types = [enumeration_type] | [select_type] .
pub fn constructed_types(input: &str) -> ParseResult<UnderlyingType> {
    alt((
        enumeration_type,
        select_type.map(|s| UnderlyingType::Select(s)),
    ))
    .parse(input)
}

/// 332 underlying_type = [concrete_types] | [constructed_types] .
pub fn underlying_type(input: &str) -> ParseResult<UnderlyingType> {
    alt((concrete_types, constructed_types)).parse(input)
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
            _semicolon1,
            where_clause,
            _end,
            _semicolon2,
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
                underlying_type: super::UnderlyingType::Simple(super::SimpleType::String_ {
                    width_spec: None
                }),
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
