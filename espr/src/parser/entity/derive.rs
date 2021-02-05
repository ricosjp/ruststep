use super::attribute::*;
use crate::parser::{expression::*, types::*, util::*};

#[derive(Debug, Clone, PartialEq)]
pub struct DeriveClause {
    pub attributes: Vec<DerivedAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DerivedAttribute {
    pub attr: AttributeDecl,
    pub ty: ParameterType,
    pub expr: Expression,
}

/// 201 derive_clause = DERIVE [derived_attr] { [derived_attr] } .
pub fn derive_clause(input: &str) -> ParseResult<DeriveClause> {
    tuple((tag("DERIVE"), space_separated(derived_attr)))
        .map(|(_derive, attributes)| DeriveClause { attributes })
        .parse(input)
}

/// 200 derived_attr = [attribute_decl] `:` [parameter_type] `:=` [expression] `;` .
pub fn derived_attr(input: &str) -> ParseResult<DerivedAttribute> {
    tuple((
        attribute_decl,
        char(':'),
        parameter_type,
        tag(":="),
        expression,
        char(';'),
    ))
    .map(|(attr, _colon, ty, _equal, expr, _semicolon)| DerivedAttribute { attr, ty, expr })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn derive_clause() {
        let (residual, (c, _remarks)) = super::derive_clause(
            r#"
            DERIVE
              SELF\named_unit.dimensions : dimensional_exponents := dimensions_for_si_unit(SELF.name);
            "#
            .trim(),
        )
        .finish()
        .unwrap();
        assert_eq!(residual, "");
        assert_eq!(c.attributes.len(), 1);
    }
}
