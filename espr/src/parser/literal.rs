use derive_more::From;
use nom::{
    branch::*, bytes::complete::*, character::complete::*, combinator::*, number::complete::double,
    IResult, Parser,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Logical {
    False,
    True,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, From)]
pub enum Literal {
    Real(f64),
    Logial(Logical),
}

/// 251 literal = binary_literal | logical_literal | real_literal | string_literal .
///
/// Integer value, e.g. `23` will be recognized as a real number `23.0`
///
/// Example
/// --------
///
/// ```
/// use espr::parser::*;
/// use nom::Finish;
///
/// let (residual, l) = literal("23").finish().unwrap();
/// assert_eq!(l, Literal::Real(23.0));
/// assert_eq!(residual, "");
/// ```
pub fn literal(input: &str) -> IResult<&str, Literal> {
    alt((
        logical_literal.map(|val| Literal::Logial(val)),
        real_literal.map(|val| Literal::Real(val)),
        // FIXME binary_literal,
        // FIXME string_literal
    ))
    .parse(input)
}

/// 255 logical_literal = `FALSE` | `TRUE` | `UNKNOWN` .
pub fn logical_literal(input: &str) -> IResult<&str, Logical> {
    alt((
        value(Logical::True, tag("TRUE")),
        value(Logical::False, tag("FALSE")),
        value(Logical::Unknown, tag("UNKNOWN")),
    ))
    .parse(input)
}

/// 141 integer_literal = digits .
///
/// Negative integer, e.g. `-23`,
/// will be represented by the combination of `-` unary operator and integer literal `23`
///
/// Example
/// --------
///
/// ```
/// use espr::parser;
/// use nom::Finish;
///
/// let (residual, value) = parser::integer_literal("123").finish().unwrap();
/// assert_eq!(value, 123);
/// assert_eq!(residual, "");
/// ```
pub fn integer_literal(input: &str) -> IResult<&str, u64> {
    digit1.map(|d: &str| d.parse().unwrap()).parse(input)
}

/// 142 real_literal = integer_literal | ( digits `.` \[ digits \] \[ `e` \[ sign \] digits \] ) .
///
/// Example
/// --------
///
/// ```
/// use espr::parser;
/// use nom::Finish;
///
/// let (residual, value) = parser::real_literal("123").finish().unwrap();
/// assert_eq!(value, 123.0);
/// assert_eq!(residual, "");
///
/// let (residual, value) = parser::real_literal("123.456").finish().unwrap();
/// assert_eq!(value, 123.456);
/// assert_eq!(residual, "");
///
/// let (residual, value) = parser::real_literal("1.23e-5").finish().unwrap();
/// assert_eq!(value, 1.23e-5);
/// assert_eq!(residual, "");
/// ```
pub fn real_literal(input: &str) -> IResult<&str, f64> {
    double.parse(input)
}
