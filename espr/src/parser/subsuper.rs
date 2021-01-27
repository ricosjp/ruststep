use super::{expression::*, identifier::*, types::*, util::*};

/// 164 abstract_entity_declaration = ABSTRACT .
pub fn abstract_entity_declaration(input: &str) -> ParseResult<()> {
    todo!()
}

/// 165 abstract_supertype = ABSTRACT SUPERTYPE ’;’ .
pub fn abstract_supertype(input: &str) -> ParseResult<()> {
    todo!()
}

/// 166 abstract_supertype_declaration = ABSTRACT SUPERTYPE [ subtype_constraint ] .
pub fn abstract_supertype_declaration(input: &str) -> ParseResult<()> {
    todo!()
}

/// 312 subsuper = [ supertype_constraint ] [ subtype_declaration ] .
pub fn subsuper(input: &str) -> ParseResult<()> {
    todo!()
}

/// 318 subtype_declaration = SUBTYPE OF ’(’ entity_ref { ’,’ entity_ref } ’)’ .
pub fn subtype_declaration(input: &str) -> ParseResult<()> {
    todo!()
}

/// 313 subtype_constraint = OF ’(’ supertype_expression ’)’ .
pub fn subtype_constraint(input: &str) -> ParseResult<()> {
    todo!()
}

/// 319 supertype_constraint = abstract_entity_declaration | abstract_supertype_declaration | supertype_rule .
pub fn supertype_constraint(input: &str) -> ParseResult<()> {
    todo!()
}

/// 320 supertype_expression = supertype_factor { ANDOR supertype_factor } .
pub fn supertype_expression(input: &str) -> ParseResult<()> {
    todo!()
}

/// 321 supertype_factor = supertype_term { AND supertype_term } .
pub fn supertype_factor(input: &str) -> ParseResult<()> {
    todo!()
}

/// 323 supertype_term = entity_ref | one_of | ’(’ supertype_expression ’)’ .
pub fn supertype_term(input: &str) -> ParseResult<()> {
    todo!()
}

/// 322 supertype_rule = SUPERTYPE subtype_constraint .
pub fn supertype_rule(input: &str) -> ParseResult<()> {
    todo!()
}

/// 263 one_of = ONEOF ’(’ supertype_expression { ’,’ supertype_expression } ’)’ .
pub fn one_of(input: &str) -> ParseResult<()> {
    todo!()
}
