use super::super::combinator::*;
use crate::ast::types::*;

/// 307 simple_types = [binary_type] | [boolean_type] | [integer_type] | [logical_type] | [number_type] | [real_type] | [string_type] .
pub fn simple_types(input: &str) -> ParseResult<SimpleType> {
    alt((
        number_type,
        real_type,
        integer_type,
        logical_type,
        boolean_type,
        string_type,
        binary_type,
    ))
    .parse(input)
}

/// 261 number_type = NUMBER .
pub fn number_type(input: &str) -> ParseResult<SimpleType> {
    value(SimpleType::Number, tag("NUMBER")).parse(input)
}

/// 278 real_type = REAL \[ `(` precision_spec `)` \] .
///
/// 268 precision_spec = numeric_expression .
pub fn real_type(input: &str) -> ParseResult<SimpleType> {
    // FIXME precision_spec is not supported
    value(SimpleType::Real, tag("REAL")).parse(input)
}

/// 241 integer_type = INTEGER .
pub fn integer_type(input: &str) -> ParseResult<SimpleType> {
    value(SimpleType::Integer, tag("INTEGER")).parse(input)
}

/// 256 logical_type = LOGICAL .
pub fn logical_type(input: &str) -> ParseResult<SimpleType> {
    value(SimpleType::Logical, tag("LOGICAL")).parse(input)
}

/// 182 boolean_type = BOOLEAN .
pub fn boolean_type(input: &str) -> ParseResult<SimpleType> {
    value(SimpleType::Boolen, tag("BOOLEAN")).parse(input)
}

/// 311 string_type = STRING \[ [width_spec] \] .
pub fn string_type(input: &str) -> ParseResult<SimpleType> {
    tuple((tag("STRING"), opt(width_spec)))
        .map(|(_, width_spec)| SimpleType::String_ { width_spec })
        .parse(input)
}

/// 181 binary_type = BINARY \[ [width_spec] \] .
pub fn binary_type(input: &str) -> ParseResult<SimpleType> {
    tuple((tag("BINARY"), opt(width_spec)))
        .map(|(_, width_spec)| SimpleType::Binary { width_spec })
        .parse(input)
}

/// 341 width_spec = `(` width `)` \[ FIXED \] .
pub fn width_spec(input: &str) -> ParseResult<WidthSpec> {
    // FIXME Should use `numeric_expression` parser
    tuple((char('('), is_not(")"), char(')'), opt(tag("FIXED"))))
        .map(|(_lparen, width, _rparen, fixed)| {
            let width = width.parse::<usize>().unwrap(); // FIXME should raise error instead of panic
            WidthSpec {
                width,
                fixed: fixed.is_some(),
            }
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::ast::types::{SimpleType, WidthSpec};
    use nom::Finish;

    #[test]
    fn string() {
        let (res, (string, _remarks)) = super::string_type("STRING").finish().unwrap();
        assert_eq!(string, SimpleType::String_ { width_spec: None });
        assert_eq!(res, "");

        let (res, (string, _remarks)) = super::string_type("STRING (10)").finish().unwrap();
        assert_eq!(
            string,
            SimpleType::String_ {
                width_spec: Some(WidthSpec {
                    width: 10,
                    fixed: false,
                })
            }
        );
        assert_eq!(res, "");

        let (res, (string, _remarks)) = super::string_type("STRING (10) FIXED").finish().unwrap();
        assert_eq!(
            string,
            SimpleType::String_ {
                width_spec: Some(WidthSpec {
                    width: 10,
                    fixed: true,
                }),
            }
        );
        assert_eq!(res, "");
    }

    #[test]
    fn binary() {
        let (res, (binary, _remarks)) = super::binary_type("BINARY").finish().unwrap();
        assert_eq!(binary, SimpleType::Binary { width_spec: None });
        assert_eq!(res, "");

        let (res, (binary, _remarks)) = super::binary_type("BINARY (10)").finish().unwrap();
        assert_eq!(
            binary,
            SimpleType::Binary {
                width_spec: Some(WidthSpec {
                    width: 10,
                    fixed: false
                })
            }
        );
        assert_eq!(res, "");

        let (res, (binary, _remarks)) = super::binary_type("BINARY (10) FIXED").finish().unwrap();
        assert_eq!(
            binary,
            SimpleType::Binary {
                width_spec: Some(WidthSpec {
                    width: 10,
                    fixed: true
                })
            }
        );
        assert_eq!(res, "");
    }
}
