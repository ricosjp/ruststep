use super::{
    super::{identifier::*, literal::*, util::*},
    aggregate_initializer::*,
    simple::*,
};

/// 269 primary = [literal] | ( [qualifiable_factor] { [qualifier] } ) .
pub fn primary(input: &str) -> ParseResult<Expression> {
    alt((
        literal.map(|literal| Expression::Literal(literal)),
        tuple((qualifiable_factor, spaced_many0(qualifier)))
            .map(|(factor, qualifiers)| Expression::QualifiableFactor { factor, qualifiers }),
    ))
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum QualifiableFactor {
    /// [attribute_ref], [general_ref], [population], or [constant_ref]
    Reference(String),
    /// [built_in_constant]
    BuiltInConstant(BuiltInConstant),
    /// [function_call]
    FunctionCall {
        name: Function,
        args: Vec<Expression>,
    },
}

/// 274 qualifiable_factor = [attribute_ref] | [constant_factor] | [function_call] | [general_ref] | [population] .
pub fn qualifiable_factor(input: &str) -> ParseResult<QualifiableFactor> {
    alt((
        alt((attribute_ref, general_ref, population)).map(|id| QualifiableFactor::Reference(id)),
        constant_factor,
        function_call,
    ))
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Function {
    BuiltInFunction(BuiltInFunction),
    Reference(String),
}

/// 219 function_call = ( [built_in_function] | [function_ref] ) \[ [actual_parameter_list] \] .
pub fn function_call(input: &str) -> ParseResult<QualifiableFactor> {
    let function_name = alt((
        built_in_function.map(|f| Function::BuiltInFunction(f)),
        function_ref.map(|f| Function::Reference(f)),
    ));
    tuple((function_name, opt(actual_parameter_list)))
        .map(|(name, args)| QualifiableFactor::FunctionCall {
            name,
            args: args.unwrap_or(Vec::new()),
        })
        .parse(input)
}

/// 167 actual_parameter_list = `(` [parameter] { `,` [parameter] } `)` .
pub fn actual_parameter_list(input: &str) -> ParseResult<Vec<Expression>> {
    tuple((char('('), comma_separated(parameter), char(')')))
        .map(|(_open, parameters, _close)| parameters)
        .parse(input)
}

/// 264 parameter = [expression] .
pub fn parameter(input: &str) -> ParseResult<Expression> {
    expression(input)
}

#[allow(non_camel_case_types)] // to use original identifiers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltInFunction {
    ABS,
    ACOS,
    ASIN,
    ATAN,
    BLENGTH,
    COS,
    EXISTS,
    EXP,
    FORMAT,
    HIBOUND,
    HIINDEX,
    LENGTH,
    LOBOUND,
    LOINDEX,
    LOG,
    LOG2,
    LOG10,
    NVL,
    ODD,
    ROLESOF,
    SIN,
    SIZEOF,
    SQRT,
    TAN,
    TYPEOF,
    USEDIN,
    VALUE,
    VALUE_IN,
    VALUE_UNIQUE,
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
    // alt impl is up to 9-element tuple
    alt((
        alt((
            value(BuiltInFunction::ABS, tag("ABS")),
            value(BuiltInFunction::ACOS, tag("ACOS")),
            value(BuiltInFunction::ASIN, tag("ASIN")),
            value(BuiltInFunction::ATAN, tag("ATAN")),
            value(BuiltInFunction::BLENGTH, tag("BLENGTH")),
            value(BuiltInFunction::COS, tag("COS")),
            value(BuiltInFunction::EXISTS, tag("EXISTS")),
            value(BuiltInFunction::EXP, tag("EXP")),
            value(BuiltInFunction::FORMAT, tag("FORMAT")),
        )),
        alt((
            value(BuiltInFunction::HIBOUND, tag("HIBOUND")),
            value(BuiltInFunction::HIINDEX, tag("HIINDEX")),
            value(BuiltInFunction::LENGTH, tag("LENGTH")),
            value(BuiltInFunction::LOBOUND, tag("LOBOUND")),
            value(BuiltInFunction::LOINDEX, tag("LOINDEX")),
            value(BuiltInFunction::LOG, tag("LOG")),
            value(BuiltInFunction::LOG2, tag("LOG2")),
            value(BuiltInFunction::LOG10, tag("LOG10")),
            value(BuiltInFunction::NVL, tag("NVL")),
        )),
        alt((
            value(BuiltInFunction::ODD, tag("ODD")),
            value(BuiltInFunction::ROLESOF, tag("ROLESOF")),
            value(BuiltInFunction::SIN, tag("SIN")),
            value(BuiltInFunction::SIZEOF, tag("SIZEOF")),
            value(BuiltInFunction::SQRT, tag("SQRT")),
            value(BuiltInFunction::TAN, tag("TAN")),
            value(BuiltInFunction::TYPEOF, tag("TYPEOF")),
            value(BuiltInFunction::USEDIN, tag("USEDIN")),
            value(BuiltInFunction::VALUE, tag("VALUE")),
        )),
        value(BuiltInFunction::VALUE_IN, tag("VALUE_IN")),
        value(BuiltInFunction::VALUE_UNIQUE, tag("VALUE_UNIQUE")),
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
        built_in_constant.map(|c| QualifiableFactor::BuiltInConstant(c)),
        constant_ref.map(|name| QualifiableFactor::Reference(name)),
    ))
    .parse(input)
}

/// Output of [qualifier]
#[derive(Debug, Clone, PartialEq)]
pub enum Qualifier {
    /// Like `.x`
    Attribute(String),
    /// Like `\point`
    Group(String),
    /// Like `[1]`
    Index(Expression),
    /// Like `[1:3]`
    Range { begin: Expression, end: Expression },
}

/// 276 qualifier = [attribute_qualifier] | [group_qualifier] | [index_qualifier] .
pub fn qualifier(input: &str) -> ParseResult<Qualifier> {
    alt((attribute_qualifier, group_qualifier, index_qualifier)).parse(input)
}

/// 179 attribute_qualifier = `.` [attribute_ref] .
pub fn attribute_qualifier(input: &str) -> ParseResult<Qualifier> {
    tuple((char('.'), attribute_ref))
        .map(|(_dot, id)| Qualifier::Attribute(id))
        .parse(input)
}

/// 232 group_qualifier = `\` [entity_ref] .
pub fn group_qualifier(input: &str) -> ParseResult<Qualifier> {
    tuple((char('\\'), entity_ref))
        .map(|(_dot, id)| Qualifier::Group(id))
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
        if let Some((_coron, end)) = opt {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltInConstant {
    /// `CONST_E`, Napier's constant `e = 2.71828 …`
    NAPIER,
    /// The ratio of a circle's circumference to its diameter, `π = 3.14159 …`
    PI,
    /// `SELF` is not a constant, but behaves as one in every context in which it can appear.
    SELF,
    /// The indeterminate symbol `?` stands for an ambiguous value.
    /// It is compatible with all data types.
    INDETERMINATE,
}

/// 186 built_in_constant = `CONST_E` | `PI` | `SELF` | `?` .
pub fn built_in_constant(input: &str) -> ParseResult<BuiltInConstant> {
    alt((
        value(BuiltInConstant::NAPIER, tag("CONST_E")),
        value(BuiltInConstant::PI, tag("PI")),
        value(BuiltInConstant::SELF, tag("SELF")),
        value(BuiltInConstant::INDETERMINATE, char('?')),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::{Expression, QualifiableFactor, Qualifier};
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
                                BuiltInConstant::INDETERMINATE
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
}
