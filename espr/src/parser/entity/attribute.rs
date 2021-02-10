use crate::parser::{expression::*, identifier::*, types::*, combinator::*};

#[derive(Debug, Clone, PartialEq)]
pub enum AttributeDecl {
    Reference(String),
    Qualified {
        /// Like `\point`
        group: Option<String>,
        /// Like `.x`
        attribute: Option<String>,
        /// For [redeclared_attribute]
        rename: Option<String>,
    },
}

// for easy testing
impl<'a> PartialEq<&'a str> for AttributeDecl {
    fn eq(&self, other: &&'a str) -> bool {
        match self {
            AttributeDecl::Reference(name) => other.eq(name),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct EntityAttribute {
    pub name: AttributeDecl,
    pub ty: ParameterType,
    pub optional: bool,
}

/// 177 attribute_decl = [attribute_id] | [redeclared_attribute] .
pub fn attribute_decl(input: &str) -> ParseResult<AttributeDecl> {
    alt((
        attribute_id.map(|id| AttributeDecl::Reference(id)),
        redeclared_attribute,
    ))
    .parse(input)
}

/// 280 referenced_attribute = [attribute_ref] | [qualified_attribute] .
pub fn referenced_attribute(input: &str) -> ParseResult<AttributeDecl> {
    alt((
        attribute_ref.map(|r| AttributeDecl::Reference(r)),
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
