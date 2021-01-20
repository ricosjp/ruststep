use super::{
    super::{basis::*, util::*},
    *,
};

/// Output of [primary]
#[derive(Debug, Clone, PartialEq, From)]
pub enum Primary {
    Literal(Literal),
    Factor {
        factor: QualifiableFactor,
        qualifiers: Vec<Qualifier>,
    },
}

/// 269 primary = literal | ( qualifiable_factor { qualifier } ) .
pub fn primary(input: &str) -> ParseResult<Primary> {
    alt((
        literal.map(|literal| Primary::Literal(literal)),
        tuple((qualifiable_factor, spaced_many0(qualifier)))
            .map(|(factor, qualifiers)| Primary::Factor { factor, qualifiers }),
    ))
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum QualifiableFactor {
    /// `attribute_ref` or `general_ref` or `population`
    Reference(String),
}

/// 274 qualifiable_factor = attribute_ref | constant_factor | function_call | general_ref | population .
pub fn qualifiable_factor(input: &str) -> ParseResult<QualifiableFactor> {
    // FIXME support constant_factor
    // FIXME support function_call
    remarked(simple_id)
        .map(|id| QualifiableFactor::Reference(id))
        .parse(input)
}

/// Output of [qualifier]
///
/// `SELF\point.x`
#[derive(Debug, Clone, PartialEq)]
pub enum Qualifier {
    /// Like `.x`
    Attribute(String),
    /// Like `\point`
    Group(String),
    /// Like `[1]`
    Index(SimpleExpression),
    /// Like `[1:3]`
    Range {
        begin: SimpleExpression,
        end: SimpleExpression,
    },
}

/// 276 qualifier = attribute_qualifier | group_qualifier | index_qualifier .
pub fn qualifier(input: &str) -> ParseResult<Qualifier> {
    alt((attribute_qualifier, group_qualifier, index_qualifier)).parse(input)
}

/// 179 attribute_qualifier = `.` attribute_ref .
pub fn attribute_qualifier(input: &str) -> ParseResult<Qualifier> {
    tuple((char('.'), remarked(simple_id)))
        .map(|(_dot, id)| Qualifier::Attribute(id))
        .parse(input)
}

/// 232 group_qualifier = `\` entity_ref .
pub fn group_qualifier(input: &str) -> ParseResult<Qualifier> {
    tuple((char('\\'), remarked(simple_id)))
        .map(|(_dot, id)| Qualifier::Group(id))
        .parse(input)
}

/// 239 index_qualifier = `[` index_1 [ `:` index_2 ] `]` .
pub fn index_qualifier(input: &str) -> ParseResult<Qualifier> {
    tuple((
        char('['),
        simple_expression,
        opt(tuple((char(':'), simple_expression))),
        char(']'),
    ))
    .map(|(_open, index, opt, _close)| {
        if let Some((_coron, end)) = opt {
            Qualifier::Range { begin: index, end }
        } else {
            Qualifier::Index(index)
        }
    })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::{Primary, QualifiableFactor, Qualifier};
    use nom::Finish;

    #[test]
    fn no_qualifier() {
        let (res, (q, _remarks)) = super::primary("x").finish().unwrap();
        assert_eq!(res, "");
        if let Primary::Factor { factor, qualifiers } = q {
            match factor {
                QualifiableFactor::Reference(name) => {
                    assert_eq!(name, "x");
                }
            }
            assert_eq!(qualifiers.len(), 0);
        } else {
            panic!("Must be factor")
        }
    }

    #[test]
    fn simple() {
        let (res, (q, _remarks)) = super::primary(r"x\group.attr").finish().unwrap();
        assert_eq!(res, "");
        if let Primary::Factor { factor, qualifiers } = q {
            match factor {
                QualifiableFactor::Reference(name) => {
                    assert_eq!(name, "x");
                }
            }
            assert_eq!(qualifiers.len(), 2);
            assert_eq!(qualifiers[0], Qualifier::Group("group".to_string()));
            assert_eq!(qualifiers[1], Qualifier::Attribute("attr".to_string()));
        } else {
            panic!("Must be factor")
        }
    }

    #[test]
    fn index() {
        let (res, (q, _remarks)) = super::primary("x[2 * 2]").finish().unwrap();
        assert_eq!(res, "");
        if let Primary::Factor { factor, qualifiers } = q {
            match factor {
                QualifiableFactor::Reference(name) => {
                    assert_eq!(name, "x");
                }
            }
            assert_eq!(qualifiers.len(), 1);
            assert!(matches!(qualifiers[0], Qualifier::Index(_)));
        } else {
            panic!("Must be factor")
        }
    }

    #[test]
    fn range() {
        let (res, (q, _remarks)) = super::primary("x[1:3]").finish().unwrap();
        assert_eq!(res, "");
        if let Primary::Factor { factor, qualifiers } = q {
            match factor {
                QualifiableFactor::Reference(name) => {
                    assert_eq!(name, "x");
                }
            }
            assert_eq!(qualifiers.len(), 1);
            assert!(matches!(qualifiers[0], Qualifier::Range { .. }));
        } else {
            panic!("Must be factor")
        }
    }
}
