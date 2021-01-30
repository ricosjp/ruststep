use super::super::util::*;

/// Output of [width_spec]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WidthSpec {
    pub width: usize,
    pub fixed: bool,
}

/// Output of [simple_types]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SimpleType {
    /// 8.1.1 Number data type
    Number,
    /// 8.1.2 Real data type
    Real,
    /// 8.1.3 Integer data type
    Integer,
    /// 8.1.4 Logical data type
    Logical,
    /// 8.1.5 Boolen data type
    Boolen,
    /// 8.1.6 String data type
    String_ { width_spec: Option<WidthSpec> },
    /// 8.1.7 Binary data type
    Binary { width_spec: Option<WidthSpec> },
}

/// 307 simple_types = [binary_type] | [boolean_type] | [integer_type] | [logical_type] | [number_type] | [real_type] | [string_type] .
pub fn simple_types(input: &str) -> ParseResult<SimpleType> {
    alt((
        number_type,
        real_type,
        integer_type,
        logical_type,
        boolen_type,
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
pub fn boolen_type(input: &str) -> ParseResult<SimpleType> {
    value(SimpleType::Boolen, tag("BOOLEN")).parse(input)
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
    use super::{SimpleType, WidthSpec};
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
