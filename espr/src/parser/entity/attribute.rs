use crate::{
    ast::entity::*,
    parser::{combinator::*, expression::*, identifier::*},
};

/// 177 attribute_decl = [attribute_id] | [redeclared_attribute] .
pub fn attribute_decl(input: &str) -> ParseResult<AttributeDecl> {
    alt((
        attribute_id.map(AttributeDecl::Reference),
        redeclared_attribute,
    ))
    .parse(input)
}

/// 280 referenced_attribute = [attribute_ref] | [qualified_attribute] .
pub fn referenced_attribute(input: &str) -> ParseResult<AttributeDecl> {
    alt((
        attribute_ref.map(AttributeDecl::Reference),
        qualified_attribute,
    ))
    .parse(input)
}

/// 275 qualified_attribute = SELF [group_qualifier] [attribute_qualifier] .
pub fn qualified_attribute(input: &str) -> ParseResult<AttributeDecl> {
    tuple((tag("SELF"), opt(group_qualifier), opt(attribute_qualifier)))
        .map(|(_self, group, attribute)| AttributeDecl::Qualified {
            group,
            attribute,
            rename: None,
        })
        .parse(input)
}

/// 279 redeclared_attribute = [qualified_attribute] \[ RENAMED [attribute_id] \] .
pub fn redeclared_attribute(input: &str) -> ParseResult<AttributeDecl> {
    tuple((
        qualified_attribute,
        opt(tuple((tag("RENAMED"), attribute_id))),
    ))
    .map(|(attr, opt)| match attr {
        AttributeDecl::Qualified {
            group,
            attribute,
            rename: _,
        } => AttributeDecl::Qualified {
            group,
            attribute,
            rename: opt.map(|(_renamed, name)| name),
        },
        _ => unreachable!(),
    })
    .parse(input)
}
