//! Parser for tokens
//!
//! Table 2 — WSN of token definitions
//! ------------------------------------
//!
//! ```text
//! STRING            = `'` { SPECIAL | DIGIT | SPACE | LOWER | UPPER | HIGH_CODEPOINT | APOSTROPHE APOSTROPHE | REVERSE_SOLIDUS REVERSE_SOLIDUS | CONTROL_DIRECTIVE } `'` .
//! ANCHOR_NAME       = `<` URI_FRAGMENT_IDENTIFIER `>` .
//! RESOURCE          = `<` UNIVERSAL_RESOURCE_IDENTIFIER `>` .
//! HEX               = `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` |
//!                     `8` | `9` | `A` | `B` | `C` | `D` | `E` | `F` .
//! BINARY            = ```` ( `0` | `1` | `2` | `3` ) { HEX } ```` .
//! SIGNATURE_CONTENT = BASE64 .
//! ```

use super::{basic::*, combinator::*};
use nom::{
    branch::alt,
    character::complete::{char, digit1, multispace0},
    combinator::opt,
    multi::many0,
    number::complete::double,
    sequence::tuple,
    Parser,
};

/// SIGN = `+` | `-` .
pub fn sign(input: &str) -> ParseResult<char> {
    alt((char('+'), char('-'))).parse(input)
}

/// INTEGER = [ SIGN ] DIGIT { DIGIT } .
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

/// REAL = [ SIGN ] DIGIT { DIGIT } `.` { DIGIT } [ `E` [ SIGN ] DIGIT { DIGIT } ] .
pub fn real(input: &str) -> ParseResult<f64> {
    tuple((opt(sign), multispace0, double))
        .map(|(sign, _space, number)| match sign {
            Some('-') => -number,
            _ => number,
        })
        .parse(input)
}

/// ENUMERATION = `.` UPPER { UPPER | DIGIT } `.` .
pub fn enumeration(input: &str) -> ParseResult<String> {
    tuple((char('.'), standard_keyword, char('.')))
        .map(|(_head, name, _tail)| name)
        .parse(input)
}

/// ENTITY_INSTANCE_NAME = `#` ( DIGIT ) { DIGIT } .
pub fn entity_instance_name(input: &str) -> ParseResult<String> {
    tuple((char('#'), digit1))
        .map(|(_sharp, name): (_, &str)| name.to_string())
        .parse(input)
}

/// VALUE_INSTANCE_NAME = `@` ( DIGIT ) { DIGIT } .
pub fn value_instance_name(input: &str) -> ParseResult<String> {
    tuple((char('@'), digit1))
        .map(|(_sharp, name): (_, &str)| name.to_string())
        .parse(input)
}

/// CONSTANT_ENTITY_NAME = `#` ( UPPER ) { UPPER | DIGIT } .
pub fn constant_entity_name(input: &str) -> ParseResult<String> {
    tuple((char('#'), standard_keyword))
        .map(|(_sharp, name)| name)
        .parse(input)
}

/// CONSTANT_VALUE_NAME = `@` ( UPPER ) { UPPER | DIGIT } .
pub fn constant_value_name(input: &str) -> ParseResult<String> {
    tuple((char('@'), standard_keyword))
        .map(|(_sharp, name)| name)
        .parse(input)
}

/// LHS_OCCURRENCE_NAME = ( ENTITY_INSTANCE_NAME | VALUE_INSTANCE_NAME ) .
pub fn lhs_occurrence_name(input: &str) -> ParseResult<String> {
    alt((entity_instance_name, value_instance_name)).parse(input)
}

/// RHS_OCCURRENCE_NAME = ( ENTITY_INSTANCE_NAME | VALUE_INSTANCE_NAME | CONSTANT_ENTITY_NAME | CONSTANT_VALUE_NAME) .
pub fn rhs_occurrence_name(input: &str) -> ParseResult<String> {
    alt((
        entity_instance_name,
        value_instance_name,
        constant_entity_name,
        constant_value_name,
    ))
    .parse(input)
}

/// KEYWORD = USER_DEFINED_KEYWORD | STANDARD_KEYWORD .
pub fn keyword(input: &str) -> ParseResult<String> {
    alt((user_defined_keyword, standard_keyword)).parse(input)
}

/// STANDARD_KEYWORD = UPPER { UPPER | DIGIT } .
pub fn standard_keyword(input: &str) -> ParseResult<String> {
    tuple((upper, many0(alt((upper, digit)))))
        .map(|(first, tail)| {
            let head = &[first];
            head.iter().chain(tail.iter()).collect()
        })
        .parse(input)
}

/// USER_DEFINED_KEYWORD = `!` UPPER { UPPER | DIGIT } .
pub fn user_defined_keyword(input: &str) -> ParseResult<String> {
    tuple((char('!'), standard_keyword))
        .map(|(_e, name)| name)
        .parse(input)
}

/// TAG_NAME = ( UPPER | LOWER ) { UPPER | LOWER | DIGIT } .
pub fn tag_name(input: &str) -> ParseResult<String> {
    tuple((alt((upper, lower)), many0(alt((upper, lower, digit)))))
        .map(|(first, tail)| {
            let head = &[first];
            head.iter().chain(tail.iter()).collect()
        })
        .parse(input)
}
