use super::{expression::*, identifier::*, types::*, util::*};

#[derive(Debug, Clone, PartialEq)]
pub struct EntityAttribute {
    pub name: String,
    pub ty: ParameterType,
    pub optional: bool,
}

/// 177 attribute_decl = [attribute_id] | redeclared_attribute .
pub fn attribute_decl(input: &str) -> ParseResult<String> {
    // FIXME Support redeclared_attribute
    attribute_id(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReferencedAttribute {
    Reference(String),
    QualifiedAttribute(Expression),
}

/// 280 referenced_attribute = [attribute_ref] | [qualified_attribute] .
/// 279 redeclared_attribute = qualified_attribute [ RENAMED attribute_id ] .
pub fn referenced_attribute(input: &str) -> ParseResult<ReferencedAttribute> {
    alt((
        attribute_ref.map(|r| ReferencedAttribute::Reference(r)),
        qualified_attribute.map(|a| ReferencedAttribute::QualifiedAttribute(a)),
    ))
    .parse(input)
}

/// 275 qualified_attribute = SELF [group_qualifier] [attribute_qualifier] .
pub fn qualified_attribute(input: &str) -> ParseResult<Expression> {
    tuple((tag("SELF"), opt(group_qualifier), opt(attribute_qualifier)))
        .map(|(_self, group, attr)| {
            let mut qualifiers = Vec::new();
            if let Some(group) = group {
                qualifiers.push(group)
            }
            if let Some(attr) = attr {
                qualifiers.push(attr)
            }
            Expression::self_qualified(qualifiers)
        })
        .parse(input)
}
