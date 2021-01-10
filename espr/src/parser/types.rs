use super::{basis::*, remark::*, util::*};
use nom::{
    branch::alt, bytes::complete::*, character::complete::*, combinator::*, sequence::*, IResult,
    Parser,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Type {
    type_id: String,
    underlying_type: String,
}

/// 332 underlying_type = concrete_types | constructed_types .
/// 193 concrete_types = aggregation_types | simple_types | type_ref.
/// 198 constructed_types = enumeration_type | select_type .
/// 213 enumeration_type = [ EXTENSIBLE ] ENUMERATION [ ( OF enumeration_items ) | enumeration_extension ] .
/// 302 select_type = [ EXTENSIBLE [ GENERIC_ENTITY ] ] SELECT [ select_list | select_extension ] .
/// 301 select_list = `(` named_types { `,` named_types } `)` .
/// 300 select_extension = BASED_ON type_ref [ WITH select_list ] .
/// 258 named_types = entity_ref | type_ref .
pub fn underlying_type(input: &str) -> ParseResult<String> {
    // FIXME
    remarked(simple_id)(input)
}

/// 327 type_decl = TYPE type_id `=` underlying_type `;` [ where_clause ] END_TYPE `;` .
pub fn type_decl(input: &str) -> ParseResult<Type> {
    tuple((
        tag("TYPE"),
        spaces_or_remarks,
        simple_id, // type_id
        spaces_or_remarks,
        char('='),
        spaces_or_remarks,
        underlying_type,
        spaces_or_remarks,
        char(';'),
        spaces_or_remarks,
        tag("END_TYPE"),
        spaces_or_remarks,
        char(';'),
    ))
    .map(
        |(
            _start,
            mut remarks,
            type_id,
            mut r1,
            _equal,
            mut r2,
            (underlying_type, mut r3),
            mut r4,
            _semicoron1,
            mut r5,
            _end,
            mut r6,
            _semicoron2,
        )| {
            remarks.append(&mut r1);
            remarks.append(&mut r2);
            remarks.append(&mut r3);
            remarks.append(&mut r4);
            remarks.append(&mut r5);
            remarks.append(&mut r6);
            (
                Type {
                    type_id,
                    underlying_type,
                },
                remarks,
            )
        },
    )
    .parse(input)
}

/// Parsed result of `width_spec`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WidthSpec {
    pub width: usize,
    pub fixed: bool,
}

/// 8.1 Simple data types
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

/// 307 simple_types = binary_type | boolean_type | integer_type | logical_type | number_type | real_type | string_type .
pub fn simple_types(input: &str) -> IResult<&str, SimpleType> {
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
pub fn number_type(input: &str) -> IResult<&str, SimpleType> {
    value(SimpleType::Number, tag("NUMBER")).parse(input)
}

/// 278 real_type = REAL \[ `(` precision_spec `)` \] .
///
/// 268 precision_spec = numeric_expression .
pub fn real_type(input: &str) -> IResult<&str, SimpleType> {
    // FIXME precision_spec is not supported
    value(SimpleType::Real, tag("REAL")).parse(input)
}

/// 241 integer_type = INTEGER .
pub fn integer_type(input: &str) -> IResult<&str, SimpleType> {
    value(SimpleType::Integer, tag("INTEGER")).parse(input)
}

/// 256 logical_type = LOGICAL .
pub fn logical_type(input: &str) -> IResult<&str, SimpleType> {
    value(SimpleType::Logical, tag("LOGICAL")).parse(input)
}

/// 182 boolean_type = BOOLEAN .
pub fn boolen_type(input: &str) -> IResult<&str, SimpleType> {
    value(SimpleType::Boolen, tag("BOOLEN")).parse(input)
}

/// 311 string_type = STRING \[ width_spec \] .
pub fn string_type(input: &str) -> IResult<&str, SimpleType> {
    tuple((tag("STRING"), multispace0, opt(width_spec)))
        .map(|(_, _, width_spec)| SimpleType::String_ { width_spec })
        .parse(input)
}

/// 181 binary_type = BINARY \[ width_spec \] .
pub fn binary_type(input: &str) -> IResult<&str, SimpleType> {
    tuple((tag("BINARY"), multispace0, opt(width_spec)))
        .map(|(_, _, width_spec)| SimpleType::Binary { width_spec })
        .parse(input)
}

/// 341 width_spec = `(` width `)` \[ FIXED \] .
pub fn width_spec(input: &str) -> IResult<&str, WidthSpec> {
    // FIXME Should use `numeric_expression` parser
    tuple((
        delimited(char('('), is_not(")"), char(')')),
        multispace0,
        opt(tag("FIXED")),
    ))
    .map(|(width, _, fixed): (&str, _, _)| {
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
        let (res, string) = super::string_type("STRING").finish().unwrap();
        assert_eq!(string, SimpleType::String_ { width_spec: None });
        assert_eq!(res, "");

        let (res, string) = super::string_type("STRING (10)").finish().unwrap();
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

        let (res, string) = super::string_type("STRING (10) FIXED").finish().unwrap();
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
        let (res, binary) = super::binary_type("BINARY").finish().unwrap();
        assert_eq!(binary, SimpleType::Binary { width_spec: None });
        assert_eq!(res, "");

        let (res, binary) = super::binary_type("BINARY (10)").finish().unwrap();
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

        let (res, binary) = super::binary_type("BINARY (10) FIXED").finish().unwrap();
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
