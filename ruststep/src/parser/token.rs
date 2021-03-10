//! Parser for tokens defined in the table 2 of ISO-10303-21

use super::{basic::*, combinator::*};
use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0, none_of, satisfy},
    combinator::opt,
    multi::{many0, many1},
    number::complete::double,
    sequence::tuple,
    Parser,
};

/// sign = `+` | `-` .
pub fn sign(input: &str) -> ParseResult<char> {
    alt((char('+'), char('-'))).parse(input)
}

/// integer = \[ [sign] \] [digit] { [digit] } .
pub fn integer(input: &str) -> ParseResult<i64> {
    tuple((opt(sign), multispace0, digit1))
        .map(|(sign, _space, numbers)| {
            let num: i64 = numbers.parse().expect("Failed to parse into integer");
            match sign {
                Some('-') => -num,
                _ => num,
            }
        })
        .parse(input)
}

/// real = \[ [sign] \] [digit] { [digit] } `.` { [digit] } \[ `E` \[ [sign] \] [digit] { [digit] } \] .
pub fn real(input: &str) -> ParseResult<f64> {
    tuple((opt(sign), multispace0, double))
        .map(|(sign, _space, number)| match sign {
            Some('-') => -number,
            _ => number,
        })
        .parse(input)
}

/// string = `'` { [special] | [digit] | [space] | [lower] | [upper] | high_codepoint | [apostrophe] [apostrophe] | [reverse_solidus] [reverse_solidus] | control_directive } `'` .
pub fn string(input: &str) -> ParseResult<String> {
    tuple((char('\''), many0(none_of("'")), char('\'')))
        .map(|(_start, s, _end)| s.iter().collect())
        .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct URI(pub String);

/// resource = `<` UNIVERSAL_RESOURCE_IDENTIFIER `>` .
///
/// Parse as string, without validating as URI
pub fn resource(input: &str) -> ParseResult<URI> {
    tuple((char('<'), many0(none_of(">")), char('>')))
        .map(|(_start, s, _end)| URI(s.iter().collect()))
        .parse(input)
}

/// enumeration = `.` [upper] { [upper] | [digit] } `.` .
pub fn enumeration(input: &str) -> ParseResult<String> {
    tuple((char('.'), standard_keyword, char('.')))
        .map(|(_head, name, _tail)| name)
        .parse(input)
}

/// Left hand side value
#[derive(Debug, Clone, PartialEq)]
pub enum LValue {
    /// Like `#11`
    Entity(u64),
    /// Like `@11`
    Value(u64),
}

/// Right hand side value
#[derive(Debug, Clone, PartialEq)]
pub enum RValue {
    /// Like `#11`
    Entity(u64),
    /// Like `@11`
    Value(u64),
    /// Like `#CONST_ENTITY`
    ConstantEntity(String),
    /// Like `@CONST_VALUE`
    ConstantValue(String),
}

// Root error for u64 overflow
//
// FIXME Though it works, should we use `VerboseErrorKind::Context` for this usage?
fn u64_overflow(input: &str) -> nom::Err<nom::error::VerboseError<&str>> {
    nom::Err::Failure(nom::error::VerboseError {
        errors: vec![(input, nom::error::VerboseErrorKind::Context("u64-overflow"))],
    })
}

/// entity_instance_name = `#` ( [digit] ) { [digit] } .
///
/// As discussed in ISO-10303-21 6.4.4.3 Entity instance names,
///
/// > NOTE 2 Leading zeros in entity instance names are ignored so "#001" is the same identifier as "#1".
///
/// leading zeros are ignored, and convert into `u64` type.
///
/// Error
/// -------
/// - FIXME: If the input cannot be represented by `u64`, i.e. larger than [std::u64::MAX]
///
pub fn entity_instance_name(input: &str) -> ParseResult<u64> {
    let (input, name) = tuple((char('#'), digit1))
        .map(|(_sharp, name): (_, &str)| name.parse())
        .parse(input)?;
    if let Ok(name) = name {
        Ok((input, name))
    } else {
        Err(u64_overflow(input))
    }
}

/// value_instance_name = `@` ( [digit] ) { [digit] } .
///
/// Leading zeros are ignored like as [entity_instance_name].
///
/// Error
/// -------
/// - FIXME: If the input cannot be represented by `u64`, i.e. larger than [std::u64::MAX]
///
pub fn value_instance_name(input: &str) -> ParseResult<u64> {
    let (input, name) = tuple((char('@'), digit1))
        .map(|(_sharp, name): (_, &str)| name.parse())
        .parse(input)?;
    if let Ok(name) = name {
        Ok((input, name))
    } else {
        Err(u64_overflow(input))
    }
}

/// constant_entity_name = `#` ( [upper] ) { [upper] | [digit] } .
pub fn constant_entity_name(input: &str) -> ParseResult<String> {
    tuple((char('#'), standard_keyword))
        .map(|(_sharp, name)| name)
        .parse(input)
}

/// constant_value_name = `@` ( [upper] ) { [upper] | [digit] } .
pub fn constant_value_name(input: &str) -> ParseResult<String> {
    tuple((char('@'), standard_keyword))
        .map(|(_sharp, name)| name)
        .parse(input)
}

/// lhs_occurrence_name = ( [entity_instance_name] | [value_instance_name] ) .
pub fn lhs_occurrence_name(input: &str) -> ParseResult<LValue> {
    alt((
        entity_instance_name.map(LValue::Entity),
        value_instance_name.map(LValue::Value),
    ))
    .parse(input)
}

/// rhs_occurrence_name = ( [entity_instance_name] | [value_instance_name] | [constant_entity_name] | [constant_value_name]) .
pub fn rhs_occurrence_name(input: &str) -> ParseResult<RValue> {
    alt((
        entity_instance_name.map(RValue::Entity),
        value_instance_name.map(RValue::Value),
        constant_entity_name.map(RValue::ConstantEntity),
        constant_value_name.map(RValue::ConstantValue),
    ))
    .parse(input)
}

/// anchor_name = `<` URI_FRAGMENT_IDENTIFIER `>` .
///
/// Parse as string, without validating as URI fragment identifier
pub fn anchor_name(input: &str) -> ParseResult<String> {
    tuple((char('<'), many0(none_of(">")), char('>')))
        .map(|(_start, s, _end)| s.iter().collect())
        .parse(input)
}

/// keyword = [user_defined_keyword] | [standard_keyword] .
pub fn keyword(input: &str) -> ParseResult<String> {
    alt((user_defined_keyword, standard_keyword)).parse(input)
}

/// standard_keyword = [upper] { [upper] | [digit] } .
pub fn standard_keyword(input: &str) -> ParseResult<String> {
    tuple((upper, many0(alt((upper, digit)))))
        .map(|(first, tail)| {
            let head = &[first];
            head.iter().chain(tail.iter()).collect()
        })
        .parse(input)
}

/// user_defined_keyword = `!` [upper] { [upper] | [digit] } .
pub fn user_defined_keyword(input: &str) -> ParseResult<String> {
    tuple((char('!'), standard_keyword))
        .map(|(_e, name)| name)
        .parse(input)
}

/// tag_name = ( [upper] | [lower] ) { [upper] | [lower] | [digit] } .
pub fn tag_name(input: &str) -> ParseResult<String> {
    tuple((alt((upper, lower)), many0(alt((upper, lower, digit)))))
        .map(|(first, tail)| {
            let head = &[first];
            head.iter().chain(tail.iter()).collect()
        })
        .parse(input)
}

/// signature_content = BASE64 .
pub fn signature_content(input: &str) -> ParseResult<String> {
    let base_char = satisfy(|c| matches!(c, '0'..='9' | 'a'..='z' | 'A'..='Z' | '+' | '/' | '='));
    many1(base_char)
        .map(|chars| chars.iter().collect())
        .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn string() {
        let (res, s) = super::string("'vim'").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(s, "vim");
    }

    #[test]
    fn u64_overflow() {
        let (res, s) = super::entity_instance_name("#18446744073709551615" /* u64::MAX */)
            .finish()
            .unwrap();
        assert_eq!(res, "");
        assert_eq!(s, std::u64::MAX);

        let (res, s) = super::value_instance_name("@18446744073709551615" /* u64::MAX */)
            .finish()
            .unwrap();
        assert_eq!(res, "");
        assert_eq!(s, std::u64::MAX);

        assert!(
            super::entity_instance_name("#18446744073709551616" /* u64::MAX + 1 */)
                .finish()
                .is_err()
        );
        assert!(
            super::value_instance_name("@18446744073709551616" /* u64::MAX + 1 */)
                .finish()
                .is_err()
        );
    }
}
