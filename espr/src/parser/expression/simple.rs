use super::{
    super::{identifier::*, literal::*, combinator::*},
    aggregate_initializer::*,
    operator::*,
    primary::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Unary {
        op: UnaryOperator,
        arg: Box<Expression>,
    },
    Binary {
        op: BinaryOperator,
        arg1: Box<Expression>,
        arg2: Box<Expression>,
    },
    Relation {
        op: RelationOperator,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Literal(Literal),
    QualifiableFactor {
        factor: QualifiableFactor,
        qualifiers: Vec<Qualifier>,
    },
    EntityConstructor {
        name: String,
        values: Vec<Expression>,
    },
    Interval {
        op_low: IntervalOperator,
        op_high: IntervalOperator,
        high: Box<Expression>,
        low: Box<Expression>,
        item: Box<Expression>,
    },
    EnumerationReference {
        ty: Option<String>,
        enum_ref: String,
    },
    AggregateInitializer {
        elements: Vec<Element>,
    },
    Query {
        variable: String,
        source: Box<Expression>,
        expr: Box<Expression>,
    },
}

macro_rules! impl_relation_op_expression {
    ($f:ident, $op:path) => {
        pub fn $f(self, other: Self) -> Self {
            Expression::Relation {
                op: $op,
                lhs: Box::new(self),
                rhs: Box::new(other),
            }
        }
    };
}

macro_rules! impl_binary_op_expression {
    ($f:ident, $op:path) => {
        pub fn $f(self, other: Self) -> Self {
            Expression::Binary {
                op: $op,
                arg1: Box::new(self),
                arg2: Box::new(other),
            }
        }
    };
}

impl Expression {
    /// `SELF` constant
    pub fn self_() -> Self {
        Self::self_qualified(Vec::new())
    }

    /// `SELF` constant with qualifiers
    pub fn self_qualified(qualifiers: Vec<Qualifier>) -> Self {
        Expression::QualifiableFactor {
            factor: QualifiableFactor::BuiltInConstant(BuiltInConstant::SELF),
            qualifiers,
        }
    }

    /// `?` constant
    pub fn indeterminate() -> Self {
        Expression::QualifiableFactor {
            factor: QualifiableFactor::BuiltInConstant(BuiltInConstant::INDETERMINATE),
            qualifiers: Vec::new(),
        }
    }

    /// Real value literal
    pub fn real(value: f64) -> Self {
        Expression::Literal(Literal::Real(value))
    }

    impl_relation_op_expression!(leq, RelationOperator::LEQ);
    impl_relation_op_expression!(geq, RelationOperator::GEQ);
    impl_relation_op_expression!(lt, RelationOperator::LT);
    impl_relation_op_expression!(gt, RelationOperator::GT);
    impl_relation_op_expression!(eq, RelationOperator::Equal);
    impl_relation_op_expression!(neq, RelationOperator::NotEqual);
    impl_relation_op_expression!(in_, RelationOperator::In);
    impl_relation_op_expression!(like, RelationOperator::Like);

    impl_binary_op_expression!(and, BinaryOperator::And);
    impl_binary_op_expression!(or, BinaryOperator::Or);
    impl_binary_op_expression!(xor, BinaryOperator::Xor);
}

macro_rules! impl_binary_op_expression {
    ($f:ident, $ops:path, $op:path) => {
        impl $ops for Expression {
            type Output = Self;
            fn $f(self, other: Expression) -> Self {
                Expression::Binary {
                    op: $op,
                    arg1: Box::new(self),
                    arg2: Box::new(other),
                }
            }
        }
    };
}

impl_binary_op_expression!(add, std::ops::Add, BinaryOperator::Add);
impl_binary_op_expression!(sub, std::ops::Sub, BinaryOperator::Sub);
impl_binary_op_expression!(mul, std::ops::Mul, BinaryOperator::Mul);

/// Create left-joined tree `1.0 + 2.0 - 3.0` into `(- (+ 1.0 2.0) 3.0)`
fn create_tree(mut head: Expression, tails: Vec<(BinaryOperator, Expression)>) -> Expression {
    for (op, expr) in tails {
        head = Expression::Binary {
            op,
            arg1: Box::new(head),
            arg2: Box::new(expr),
        }
    }
    head
}

/// 305 simple_expression = [term] { [add_like_op] [term] } .
pub fn simple_expression(input: &str) -> ParseResult<Expression> {
    tuple((term, spaced_many0(tuple((add_like_op, term)))))
        .map(|(head, tails)| create_tree(head, tails))
        .parse(input)
}

/// 325 term = [factor] { [multiplication_like_op] [factor] } .
pub fn term(input: &str) -> ParseResult<Expression> {
    tuple((
        factor,
        spaced_many0(tuple((multiplication_like_op, factor))),
    ))
    .map(|(head, tails)| create_tree(head, tails))
    .parse(input)
}

/// 217 factor = [simple_factor] \[ `**` [simple_factor] \] .
pub fn factor(input: &str) -> ParseResult<Expression> {
    tuple((simple_factor, opt(tuple((power_op, simple_factor)))))
        .map(|(arg1, opt)| {
            if let Some((op, arg2)) = opt {
                Expression::Binary {
                    op,
                    arg1: Box::new(arg1),
                    arg2: Box::new(arg2),
                }
            } else {
                arg1
            }
        })
        .parse(input)
}

/// 306 simple_factor = [aggregate_initializer]
///                   | [entity_constructor]
///                   | [enumeration_reference]
///                   | [interval]
///                   | [query_expression]
///                   | ( \[ [unary_op] \] ( `(` [expression] `)` | [primary] ) ) .
pub fn simple_factor(input: &str) -> ParseResult<Expression> {
    let paren_expr = tuple((char('('), expression, char(')'))).map(|(_open, e, _close)| e);
    // ( \[ unary_op \] ( `(` expression `)` | primary ) )
    let unary = tuple((opt(unary_op), alt((paren_expr, primary)))).map(|(opt, expr)| {
        if let Some(op) = opt {
            Expression::Unary {
                op,
                arg: Box::new(expr),
            }
        } else {
            expr
        }
    });
    alt((
        unary,
        aggregate_initializer,
        entity_constructor,
        interval,
        query_expression,
        enumeration_reference, // must be after `unary` case
    ))
    .parse(input)
}

/// 216 expression = [simple_expression] \[ [rel_op_extended] [simple_expression] \] .
pub fn expression(input: &str) -> ParseResult<Expression> {
    tuple((
        simple_expression,
        opt(tuple((rel_op_extended, simple_expression))),
    ))
    .map(|(lhs, opt)| {
        if let Some((op, rhs)) = opt {
            Expression::Relation {
                op,
                lhs: Box::new(lhs),
                rhs: Box::new(rhs),
            }
        } else {
            lhs
        }
    })
    .parse(input)
}

/// 212 enumeration_reference = \[ [type_ref] ’.’ \] [enumeration_ref] .
pub fn enumeration_reference(input: &str) -> ParseResult<Expression> {
    tuple((opt(tuple((type_ref, char('.')))), enumeration_ref))
        .map(|(opt, enum_ref)| Expression::EnumerationReference {
            ty: opt.map(|(ty, _comma)| ty),
            enum_ref,
        })
        .parse(input)
}

/// 243 interval = `{` [interval_low] [interval_op] [interval_item] [interval_op] [interval_high] `}` .
pub fn interval(input: &str) -> ParseResult<Expression> {
    tuple((
        char('{'),
        interval_low,
        interval_op,
        interval_item,
        interval_op,
        interval_high,
        char('}'),
    ))
    .map(
        |(_open, low, op_low, item, op_high, high, _close)| Expression::Interval {
            op_low,
            op_high,
            low: Box::new(low),
            item: Box::new(item),
            high: Box::new(high),
        },
    )
    .parse(input)
}

/// 244 interval_high = [simple_expression] .
pub fn interval_high(input: &str) -> ParseResult<Expression> {
    simple_expression(input)
}

/// 245 interval_item = [simple_expression] .
pub fn interval_item(input: &str) -> ParseResult<Expression> {
    simple_expression(input)
}

/// 246 interval_low = [simple_expression] .
pub fn interval_low(input: &str) -> ParseResult<Expression> {
    simple_expression(input)
}

/// 170 aggregate_source = [simple_expression] .
pub fn aggregate_source(input: &str) -> ParseResult<Expression> {
    simple_expression(input)
}

/// 254 logical_expression = [expression] .
pub fn logical_expression(input: &str) -> ParseResult<Expression> {
    expression(input)
}

/// 277 query_expression = QUERY `(` variable_id `<*` aggregate_source `|` logical_expression `)` .
pub fn query_expression(input: &str) -> ParseResult<Expression> {
    tuple((
        tag("QUERY"),
        char('('),
        variable_id,
        tag("<*"),
        aggregate_source,
        char('|'),
        logical_expression,
        char(')'),
    ))
    .map(
        |(_start, _open, variable, _star, source, _bar, expr, _close)| Expression::Query {
            variable,
            source: Box::new(source),
            expr: Box::new(expr),
        },
    )
    .parse(input)
}

/// 205 entity_constructor = entity_ref ’(’ [ [expression] { ’,’ [expression] } ] ’)’ .
pub fn entity_constructor(input: &str) -> ParseResult<Expression> {
    tuple((
        entity_ref,
        char('('),
        opt(comma_separated(expression)),
        char(')'),
    ))
    .map(
        |(name, _open, values, _close)| Expression::EntityConstructor {
            name,
            values: values.unwrap_or(Vec::new()),
        },
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Finish;

    #[test]
    fn unary() {
        let (res, (expr, _remarks)) = super::expression("-1.0").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(
            expr,
            Expression::Unary {
                op: UnaryOperator::Minus,
                arg: Box::new(Expression::Literal(Literal::Real(1.0))),
            }
        );
    }

    #[test]
    fn interval() {
        let (res, (expr, _remarks)) = super::interval("{1 <= date.day_component <= 31}")
            .finish()
            .unwrap();
        assert_eq!(res, "");
        dbg!(expr);
    }

    #[test]
    fn binary() {
        let (res, (expr, _remarks)) = super::expression("1.0 + 2.0").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(
            expr,
            Expression::Binary {
                op: BinaryOperator::Add,
                arg1: Box::new(Expression::Literal(Literal::Real(1.0))),
                arg2: Box::new(Expression::Literal(Literal::Real(2.0))),
            }
        );
    }

    #[test]
    fn binary3() {
        let (res, (expr, _remarks)) = super::expression("1.0 + 2.0 - 3.0").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(
            expr,
            Expression::Binary {
                op: BinaryOperator::Sub,
                arg1: Box::new(Expression::Binary {
                    op: BinaryOperator::Add,
                    arg1: Box::new(Expression::Literal(Literal::Real(1.0))),
                    arg2: Box::new(Expression::Literal(Literal::Real(2.0))),
                }),
                arg2: Box::new(Expression::Literal(Literal::Real(3.0)))
            }
        );
    }

    #[test]
    fn relation() {
        let (res, (expr, _remarks)) = super::expression("1 <= 2").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(expr, Expression::real(1.0).leq(Expression::real(2.0)));
    }

    #[test]
    fn relation_self() {
        let (res, (expr, _remarks)) = super::expression("1 <= SELF").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(expr, Expression::real(1.0).leq(Expression::self_()));
    }

    #[test]
    fn binary_relation() {
        let (res, (expr, _remarks)) = super::expression("(1 <= SELF) AND (SELF <= 12)")
            .finish()
            .unwrap();
        assert_eq!(res, "");
        assert_eq!(
            expr,
            Expression::real(1.0)
                .leq(Expression::self_())
                .and(Expression::self_().leq(Expression::real(12.0)))
        );
    }

    #[test]
    fn literal() {
        let (res, (expr, _remarks)) = super::expression("1.0").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(expr, Expression::real(1.0));
    }

    #[test]
    fn qualifiable_factor() {
        let (res, (expr, _remarks)) = super::expression(r"x\group.attr").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(
            expr,
            Expression::QualifiableFactor {
                factor: QualifiableFactor::Reference("x".to_string()),
                qualifiers: vec![
                    Qualifier::Group("group".to_string()),
                    Qualifier::Attribute("attr".to_string())
                ]
            }
        );
    }

    #[test]
    fn entity_constructor() {
        let (residual, (ctor, _remarks)) = super::entity_constructor("point(0.0, 0.0, 0.0)")
            .finish()
            .unwrap();
        assert_eq!(residual, "");
        if let super::Expression::EntityConstructor { name, values } = ctor {
            assert_eq!(name, "point");
            assert_eq!(values.len(), 3);
        } else {
            panic!("Must be entity constructor")
        }
    }

    #[test]
    fn call_attr() {
        let (residual, (expr, _remarks)) = super::expression("f(a, b).attr").finish().unwrap();
        dbg!(expr);
        assert_eq!(residual, "");
    }
}
