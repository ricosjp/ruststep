use super::{super::util::*, *};

/// 169 aggregate_initializer = `[` \[ [element] { `,` [element] } \] `]` .
pub fn aggregate_initializer(input: &str) -> ParseResult<Expression> {
    tuple((
        char('['),
        opt(comma_separated(element)).map(|opt| opt.unwrap_or(Vec::new())),
        char(']'),
    ))
    .map(|(_open, elements, _close)| Expression::AggregateInitializer { elements })
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub expr: Expression,
    pub repetition: Option<Expression>,
}

/// 203 element = [expression] \[ `:` [repetition] \] .
pub fn element(input: &str) -> ParseResult<Element> {
    tuple((expression, opt(tuple((char(':'), repetition)))))
        .map(|(expr, opt)| Element {
            expr,
            repetition: opt.map(|(_comma, r)| r),
        })
        .parse(input)
}

/// 287 repetition = [numeric_expression] .
pub fn repetition(input: &str) -> ParseResult<Expression> {
    numeric_expression(input)
}

/// 262 numeric_expression = [simple_expression] .
pub fn numeric_expression(input: &str) -> ParseResult<Expression> {
    simple_expression(input)
}

#[cfg(test)]
mod tests {
    use super::super::super::literal::*;
    use super::*;
    use nom::Finish;

    #[test]
    fn aggregate_initializer_empty() {
        let (res, (expr, _remarks)) = super::expression("[]").finish().unwrap();
        dbg!(expr);
        assert_eq!(res, "");
    }

    #[test]
    fn aggregate_initializer() {
        let (res, (expr, _remarks)) = super::expression("[1, 3, 6, 9*8, -12]").finish().unwrap();
        assert_eq!(res, "");
        assert_eq!(
            expr,
            Expression::AggregateInitializer {
                elements: vec![
                    Element {
                        expr: Expression::Literal(Literal::Real(1.0)),
                        repetition: None,
                    },
                    Element {
                        expr: Expression::Literal(Literal::Real(3.0)),
                        repetition: None,
                    },
                    Element {
                        expr: Expression::Literal(Literal::Real(6.0)),
                        repetition: None,
                    },
                    Element {
                        expr: Expression::Binary {
                            op: BinaryOperator::Mul,
                            arg1: Box::new(Expression::Literal(Literal::Real(9.0))),
                            arg2: Box::new(Expression::Literal(Literal::Real(8.0)))
                        },
                        repetition: None,
                    },
                    Element {
                        expr: Expression::Unary {
                            op: UnaryOperator::Minus,
                            arg: Box::new(Expression::Literal(Literal::Real(12.0))),
                        },
                        repetition: None,
                    },
                ]
            }
        );
    }
}
