//! Syntatic analysis of EXPRESS language standardized as [ISO-10303-11](https://www.iso.org/standard/38047.html)
//!
//! This module is based on [nom](https://github.com/Geal/nom) parser combinater.

mod entity;
mod expression;
mod literal;
mod schema;
mod simple_data_type;

pub use entity::*;
pub use expression::*;
pub use literal::*;
pub use schema::*;
pub use simple_data_type::*;

use nom::{
    branch::*, bytes::complete::*, character::complete::*, multi::*, sequence::*, IResult, Parser,
};

/// 128 letter = `a` | `b` | `c` | `d` | `e` | `f` | `g` | `h` | `i` | `j` | `k` | `l` |`m` | `n` | `o` | `p` | `q` | `r` | `s` | `t` | `u` | `v` | `w` | `x` |`y` | `z` .
pub fn letter(input: &str) -> IResult<&str, char> {
    satisfy(|c| matches!(c, 'a'..='z')).parse(input)
}

/// 124 digit = `0` | `1` | `2` | `3` | `4` | `5` | `6` | `7` | `8` | `9` .
pub fn digit(input: &str) -> IResult<&str, char> {
    satisfy(|c| matches!(c, '0'..='9')).parse(input)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IdKind {
    Attribute,
    Constant,
    Entity,
    Enumeration,
    Function,
    Parameter,
    Procedure,
    Rename,
    Rule,
    RuleLabel,
    Schema,
    SubtypeConstraint,
    Type,
    TypeLabel,
    Variable,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RefKind {
    Attribute,
    Constant,
    Entity,
    Enumeration,
    Function,
    Parameter,
    Procedure,
    Rule,
    RuleLabel,
    Schema,
    SubtypeConstraint,
    Type,
    TypeLabel,
    Variable,
}

/// 143 simple_id = letter { letter | digit | `_` } .
///
/// This parser identifies following **ALL** identifier types and references:
///
/// - Identifier types
///   - 178 attribute_id = simple_id .
///   - 197 constant_id = simple_id .
///   - 208 entity_id = simple_id .
///   - 210 enumeration_id = simple_id .
///   - 222 function_id = simple_id .
///   - 265 parameter_id = simple_id .
///   - 273 procedure_id = simple_id .
///   - 284 rename_id = constant_id | entity_id | function_id | procedure_id | type_id .
///   - 293 rule_id = simple_id .
///   - 294 rule_label_id = simple_id .
///   - 297 schema_id = simple_id .
///   - 298 schema_version_id = string_literal .
///   - 317 subtype_constraint_id = simple_id .
///   - 328 type_id = simple_id .
///   - 330 type_label_id = simple_id .
///   - 337 variable_id = simple_id .
/// - Reference types
///   - 150 attribute_ref = attribute_id .
///   - 151 constant_ref = constant_id .
///   - 152 entity_ref = entity_id .
///   - 153 enumeration_ref = enumeration_id .
///   - 154 function_ref = function_id .
///   - 155 parameter_ref = parameter_id .
///   - 156 procedure_ref = procedure_id .
///   - 157 rule_label_ref = rule_label_id .
///   - 158 rule_ref = rule_id .
///   - 159 schema_ref = schema_id .
///   - 160 subtype_constraint_ref = subtype_constraint_id .
///   - 161 type_label_ref = type_label_id .
///   - 162 type_ref = type_id .
///   - 163 variable_ref = variable_id .
pub fn simple_id(input: &str) -> IResult<&str, String> {
    tuple((letter, many0(alt((letter, digit, char('_'))))))
        .map(|(head, tail)| format!("{}{}", head, tail.into_iter().collect::<String>()))
        .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn letter() {
        let (residual, l) = super::letter("h").finish().unwrap();
        assert_eq!(l, 'h');
        assert_eq!(residual, "");

        let (residual, l) = super::letter("abc").finish().unwrap();
        assert_eq!(l, 'a');
        assert_eq!(residual, "bc");

        // Capital is not allowed
        assert!(super::letter("H").finish().is_err());
        // Number is not allowed
        assert!(super::letter("2").finish().is_err());
    }

    #[test]
    fn digit() {
        let (residual, l) = super::digit("123").finish().unwrap();
        assert_eq!(l, '1');
        assert_eq!(residual, "23");

        // Alphabets are not allowed
        assert!(super::digit("h").finish().is_err());
    }

    #[test]
    fn simple_id_valid() {
        let (residual, id) = super::simple_id("h").finish().unwrap();
        assert_eq!(id, "h");
        assert_eq!(residual, "");

        let (residual, id) = super::simple_id("homhom").finish().unwrap();
        assert_eq!(id, "homhom");
        assert_eq!(residual, "");

        let (residual, id) = super::simple_id("ho_mhom").finish().unwrap();
        assert_eq!(id, "ho_mhom");
        assert_eq!(residual, "");

        let (residual, id) = super::simple_id("h10o_1mh2om").finish().unwrap();
        assert_eq!(id, "h10o_1mh2om");
        assert_eq!(residual, "");
    }

    #[test]
    fn simple_id_invalid() {
        // Capital is not allowed
        assert!(super::simple_id("HomHom").finish().is_err());
        // `_` cannot use as first
        assert!(super::simple_id("_homhom").finish().is_err());
        // digit cannot use as first
        assert!(super::simple_id("1homhom").finish().is_err());
        // Empty is invlaid
        assert!(super::simple_id("").finish().is_err());
    }
}
