use nom::{
    branch::alt, bytes::complete::*, character::complete::*, combinator::*, sequence::*, IResult,
    Parser,
};

/// Parsed result of `width_spec`
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WidthSpec {
    pub width: usize,
    pub fixed: bool,
}

/// 8.1 Simple data types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SimpleDataType {
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

fn number(input: &str) -> IResult<&str, SimpleDataType> {
    value(SimpleDataType::Number, tag("NUMBER")).parse(input)
}

fn real(input: &str) -> IResult<&str, SimpleDataType> {
    value(SimpleDataType::Real, tag("REAL")).parse(input)
}

fn integer(input: &str) -> IResult<&str, SimpleDataType> {
    value(SimpleDataType::Integer, tag("INTEGER")).parse(input)
}

fn logical(input: &str) -> IResult<&str, SimpleDataType> {
    value(SimpleDataType::Logical, tag("LOGICAL")).parse(input)
}

fn boolen(input: &str) -> IResult<&str, SimpleDataType> {
    value(SimpleDataType::Boolen, tag("BOOLEN")).parse(input)
}

/// `width_spec` in String data type (8.1.6) and Binary data type (8.1.7)
///
/// ```text
/// 341 width_spec = ’(’ width ’)’ [ FIXED ] .
/// 340 width = numeric_expression .
/// ```
pub fn width_spec(input: &str) -> IResult<&str, WidthSpec> {
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

/// 8.1.6 String data type
///
/// ```text
/// 311 string_type = STRING [ width_spec ] .
/// ```
pub fn string(input: &str) -> IResult<&str, SimpleDataType> {
    tuple((tag("STRING"), multispace0, opt(width_spec)))
        .map(|(_, _, width_spec)| SimpleDataType::String_ { width_spec })
        .parse(input)
}

/// 8.1.7 Binary data type
///
/// ```text
/// 181 binary_type = BINARY [ width_spec ] .
/// ```
pub fn binary(input: &str) -> IResult<&str, SimpleDataType> {
    tuple((tag("BINARY"), multispace0, opt(width_spec)))
        .map(|(_, _, width_spec)| SimpleDataType::Binary { width_spec })
        .parse(input)
}

/// 8.1 Simple data type
pub fn simple_data_type(input: &str) -> IResult<&str, SimpleDataType> {
    alt((number, real, integer, logical, boolen, string, binary)).parse(input)
}

#[cfg(test)]
mod tests {
    use super::{SimpleDataType, WidthSpec};
    use nom::Finish;

    #[test]
    fn string() {
        let (res, string) = super::string("STRING").finish().unwrap();
        assert_eq!(string, SimpleDataType::String_ { width_spec: None });
        assert_eq!(res, "");

        let (res, string) = super::string("STRING (10)").finish().unwrap();
        assert_eq!(
            string,
            SimpleDataType::String_ {
                width_spec: Some(WidthSpec {
                    width: 10,
                    fixed: false,
                })
            }
        );
        assert_eq!(res, "");

        let (res, string) = super::string("STRING (10) FIXED").finish().unwrap();
        assert_eq!(
            string,
            SimpleDataType::String_ {
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
        let (res, binary) = super::binary("BINARY").finish().unwrap();
        assert_eq!(binary, SimpleDataType::Binary { width_spec: None });
        assert_eq!(res, "");

        let (res, binary) = super::binary("BINARY (10)").finish().unwrap();
        assert_eq!(
            binary,
            SimpleDataType::Binary {
                width_spec: Some(WidthSpec {
                    width: 10,
                    fixed: false
                })
            }
        );
        assert_eq!(res, "");

        let (res, binary) = super::binary("BINARY (10) FIXED").finish().unwrap();
        assert_eq!(
            binary,
            SimpleDataType::Binary {
                width_spec: Some(WidthSpec {
                    width: 10,
                    fixed: true
                })
            }
        );
        assert_eq!(res, "");
    }
}
