mod enumerate;
mod select;
mod simple;

pub use enumerate::*;
pub use select::*;
pub use simple::*;

use super::{basis::*, util::*};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Extensiblity {
    None,
    Extensible,
    GenericEntity,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    type_id: String,
    underlying_type: String,
}

/// 332 underlying_type = concrete_types | constructed_types .
/// 193 concrete_types = aggregation_types | simple_types | type_ref.
/// 198 constructed_types = enumeration_type | select_type .
pub fn underlying_type(input: &str) -> ParseResult<String> {
    // FIXME
    remarked(simple_id).parse(input)
}

/// 327 type_decl = TYPE type_id `=` underlying_type `;` [ where_clause ] END_TYPE `;` .
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
                underlying_type: "STRING".to_string()
            }
        );
    }
}
