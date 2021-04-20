use super::{
    super::{combinator::*, identifier::*, literal::*},
    aggregate_initializer::*,
    simple::*,
};
use crate::ast::expression::*;

/// 269 primary = [literal] | ( [qualifiable_factor] { [qualifier] } ) .
pub fn primary(input: &str) -> ParseResult<Expression> {
    alt((
        literal.map(|literal| Expression::Literal(literal)),
        tuple((qualifiable_factor, many0(qualifier)))
            .map(|(factor, qualifiers)| Expression::QualifiableFactor { factor, qualifiers }),
    ))
    .parse(input)
}

/// 274 qualifiable_factor = [attribute_ref] | [constant_factor] | [function_call] | [general_ref] | [population] .
pub fn qualifiable_factor(input: &str) -> ParseResult<QualifiableFactor> {
    alt((
        function_call,
        alt((attribute_ref, general_ref, population)).map(QualifiableFactor::Reference),
        constant_factor,
    ))
    .parse(input)
}

/// 999 function_call = ( [built_in_function] | [function_ref] ) [actual_parameter_list] .
///
/// Different from official definition
///
/// ```text
/// 219 function_call = ( built_in_function | function_ref ) [ actual_parameter_list ] .
/// ```
///
/// This function always requires `actual_parameter_list` in order to identify from
/// other reference types.
pub fn function_call(input: &str) -> ParseResult<QualifiableFactor> {
    let function_name = alt((
        built_in_function.map(|f| FunctionCallName::BuiltInFunction(f)),
        function_ref.map(|f| FunctionCallName::Reference(f)),
    ));
    tuple((function_name, actual_parameter_list))
        .map(|(name, args)| QualifiableFactor::FunctionCall { name, args })
        .parse(input)
}

/// 167 actual_parameter_list = `(` [parameter] { `,` [parameter] } `)` .
pub fn actual_parameter_list(input: &str) -> ParseResult<Vec<Expression>> {
    tuple((
        char('('),
        opt(comma_separated(parameter)).map(|opt| opt.unwrap_or_default()),
        char(')'),
    ))
    .map(|(_open, parameters, _close)| parameters)
    .parse(input)
}

/// 264 parameter = [expression] .
pub fn parameter(input: &str) -> ParseResult<Expression> {
    expression(input)
}

/// 187 built_in_function = ABS
///                       | ACOS
///                       | ASIN
///                       | ATAN
///                       | BLENGTH
///                       | COS
///                       | EXISTS
///                       | EXP
///                       | FORMAT
///                       | HIBOUND
///                       | HIINDEX
///                       | LENGTH
///                       | LOBOUND
///                       | LOINDEX
///                       | LOG
///                       | LOG2
///                       | LOG10
///                       | NVL
///                       | ODD
///                       | ROLESOF
///                       | SIN
///                       | SIZEOF
///                       | SQRT
///                       | TAN
///                       | TYPEOF
///                       | USEDIN
///                       | VALUE
///                       | VALUE_IN
///                       | VALUE_UNIQUE .
pub fn built_in_function(input: &str) -> ParseResult<BuiltInFunction> {
    // alt impl is up to 11-element tuple. In reverse order to match longer case first.
    alt((
        alt((
            value(BuiltInFunction::FORMAT, tag("FORMAT")),
            value(BuiltInFunction::EXP, tag("EXP")),
            value(BuiltInFunction::EXISTS, tag("EXISTS")),
            value(BuiltInFunction::COS, tag("COS")),
            value(BuiltInFunction::BLENGTH, tag("BLENGTH")),
            value(BuiltInFunction::ATAN, tag("ATAN")),
            value(BuiltInFunction::ASIN, tag("ASIN")),
            value(BuiltInFunction::ACOS, tag("ACOS")),
            value(BuiltInFunction::ABS, tag("ABS")),
        )),
        alt((
            value(BuiltInFunction::NVL, tag("NVL")),
            value(BuiltInFunction::LOINDEX, tag("LOINDEX")),
            value(BuiltInFunction::LOG2, tag("LOG2")),
            value(BuiltInFunction::LOG10, tag("LOG10")),
            value(BuiltInFunction::LOG, tag("LOG")), // must be after `LOG2` and `LOG10`
            value(BuiltInFunction::LOBOUND, tag("LOBOUND")),
            value(BuiltInFunction::LENGTH, tag("LENGTH")),
            value(BuiltInFunction::HIINDEX, tag("HIINDEX")),
            value(BuiltInFunction::HIBOUND, tag("HIBOUND")),
        )),
        alt((
            value(BuiltInFunction::VALUE_UNIQUE, tag("VALUE_UNIQUE")),
            value(BuiltInFunction::VALUE_IN, tag("VALUE_IN")),
            value(BuiltInFunction::VALUE, tag("VALUE")), // must be after `VALUE_IN` and `VALUE_UNIQUE`
            value(BuiltInFunction::USEDIN, tag("USEDIN")),
            value(BuiltInFunction::TYPEOF, tag("TYPEOF")),
            value(BuiltInFunction::TAN, tag("TAN")),
            value(BuiltInFunction::SQRT, tag("SQRT")),
            value(BuiltInFunction::SIZEOF, tag("SIZEOF")),
            value(BuiltInFunction::SIN, tag("SIN")),
            value(BuiltInFunction::ROLESOF, tag("ROLESOF")),
            value(BuiltInFunction::ODD, tag("ODD")),
        )),
    ))
    .parse(input)
}

/// 267 population = entity_ref .
pub fn population(input: &str) -> ParseResult<String> {
    entity_ref(input)
}

/// 196 constant_factor = [built_in_constant] | [constant_ref] .
pub fn constant_factor(input: &str) -> ParseResult<QualifiableFactor> {
    alt((
        built_in_constant.map(QualifiableFactor::BuiltInConstant),
        constant_ref.map(QualifiableFactor::Reference),
    ))
    .parse(input)
}

/// 276 qualifier = [attribute_qualifier] | [group_qualifier] | [index_qualifier] .
pub fn qualifier(input: &str) -> ParseResult<Qualifier> {
    alt((
        attribute_qualifier.map(Qualifier::Attribute),
        group_qualifier.map(Qualifier::Group),
        index_qualifier,
    ))
    .parse(input)
}

/// 179 attribute_qualifier = `.` [attribute_ref] .
pub fn attribute_qualifier(input: &str) -> ParseResult<String> {
    tuple((char('.'), attribute_ref))
        .map(|(_dot, id)| id)
        .parse(input)
}

/// 232 group_qualifier = `\` [entity_ref] .
pub fn group_qualifier(input: &str) -> ParseResult<String> {
    tuple((char('\\'), entity_ref))
        .map(|(_dot, id)| id)
        .parse(input)
}

/// 239 index_qualifier = `[` [index_1] [ `:` [index_2] ] `]` .
pub fn index_qualifier(input: &str) -> ParseResult<Qualifier> {
    tuple((
        char('['),
        index_1,
        opt(tuple((char(':'), index_2))),
        char(']'),
    ))
    .map(|(_open, index, opt, _close)| {
        if let Some((_colon, end)) = opt {
            Qualifier::Range { begin: index, end }
        } else {
            Qualifier::Index(index)
        }
    })
    .parse(input)
}

/// 236 index = [numeric_expression] .
pub fn index(input: &str) -> ParseResult<Expression> {
    numeric_expression(input)
}

/// 237 index_1 = [index] .
pub fn index_1(input: &str) -> ParseResult<Expression> {
    index(input)
}

/// 238 index_2 = [index] .
pub fn index_2(input: &str) -> ParseResult<Expression> {
    index(input)
}

/// 186 built_in_constant = `CONST_E` | `PI` | `SELF` | `?` .
pub fn built_in_constant(input: &str) -> ParseResult<BuiltInConstant> {
    alt((
        value(BuiltInConstant::Napier, tag("CONST_E")),
        value(BuiltInConstant::Pi, tag("PI")),
        value(BuiltInConstant::Self_, tag("SELF")),
        value(BuiltInConstant::Indeterminate, char('?')),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::{Expression, FunctionCallName, QualifiableFactor, Qualifier};
    use nom::Finish;

    #[test]
    fn no_qualifier() {
        let (res, (q, _remarks)) = super::primary("x").finish().unwrap();
        assert_eq!(res, "");
        if let Expression::QualifiableFactor { factor, qualifiers } = q {
            match factor {
                QualifiableFactor::Reference(name) => {
                    assert_eq!(name, "x");
                }
                _ => panic!("Must be reference"),
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
        if let Expression::QualifiableFactor { factor, qualifiers } = q {
            match factor {
                QualifiableFactor::Reference(name) => {
                    assert_eq!(name, "x");
                }
                _ => panic!("Must be reference"),
            }
            assert_eq!(qualifiers.len(), 2);
            assert_eq!(qualifiers[0], Qualifier::Group("group".to_string()));
            assert_eq!(qualifiers[1], Qualifier::Attribute("attr".to_string()));
        } else {
            panic!("Must be factor")
        }
    }

    #[test]
    fn function_call() {
        let (res, (q, _remarks)) = super::primary("f(x)").finish().unwrap();
        assert_eq!(res, "");
        if let Expression::QualifiableFactor { factor, qualifiers } = q {
            match factor {
                QualifiableFactor::FunctionCall { name, args } => {
                    assert_eq!(name, FunctionCallName::Reference("f".to_string()));
                    assert_eq!(args.len(), 1);
                }
                _ => panic!("Must be reference"),
            }
            assert_eq!(qualifiers.len(), 0);
        } else {
            panic!("Must be factor")
        }
    }

    #[test]
    fn function_call_builtin() {
        let (res, (q, _remarks)) = super::primary("VALUE_UNIQUE(x)").finish().unwrap();
        assert_eq!(res, "");
        dbg!(q);
    }

    #[test]
    fn index() {
        let (res, (q, _remarks)) = super::primary("x[2 * 2]").finish().unwrap();
        assert_eq!(res, "");
        if let Expression::QualifiableFactor { factor, qualifiers } = q {
            match factor {
                QualifiableFactor::Reference(name) => {
                    assert_eq!(name, "x");
                }
                _ => panic!("Must be reference"),
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
        if let Expression::QualifiableFactor { factor, qualifiers } = q {
            match factor {
                QualifiableFactor::Reference(name) => {
                    assert_eq!(name, "x");
                }
                _ => panic!("Must be reference"),
            }
            assert_eq!(qualifiers.len(), 1);
            assert!(matches!(qualifiers[0], Qualifier::Range { .. }));
        } else {
            panic!("Must be factor")
        }
    }

    #[test]
    fn indeterminate() {
        let (res, (q, _remarks)) = super::primary("x[1:?]").finish().unwrap();
        assert_eq!(res, "");
        if let Expression::QualifiableFactor { factor, qualifiers } = q {
            match factor {
                QualifiableFactor::Reference(name) => {
                    assert_eq!(name, "x");
                }
                _ => panic!("Must be reference"),
            }
            assert_eq!(qualifiers.len(), 1);
            match &qualifiers[0] {
                Qualifier::Range { begin: _, end } => {
                    use super::*;
                    assert_eq!(
                        end,
                        &Expression::QualifiableFactor {
                            factor: QualifiableFactor::BuiltInConstant(
                                BuiltInConstant::Indeterminate
                            ),
                            qualifiers: Vec::new()
                        }
                    );
                }
                _ => panic!("Must be range"),
            }
        } else {
            panic!("Must be factor")
        }
    }

    #[test]
    fn call_attr() {
        let (residual, (expr, _remarks)) = super::primary("f(a, b).attr").finish().unwrap();
        dbg!(expr);
        assert_eq!(residual, "");
    }
}
