use super::{
    super::{basis::*, util::*},
    operator::*,
    primary::*,
};

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// End node of expression tree
    Primary(Primary),
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
    EntityConstructor {
        name: String,
        values: Vec<Expression>,
    },
}

/// 305 simple_expression = term { add_like_op term } .
pub fn simple_expression(input: &str) -> ParseResult<Expression> {
    todo!()
}

/// 325 term = factor { multiplication_like_op factor } .
pub fn term(input: &str) -> ParseResult<Expression> {
    todo!()
}

/// 217 factor = simple_factor \[ `**` simple_factor \] .
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

/// 306 simple_factor = aggregate_initializer
///                   | entity_constructor
///                   | enumeration_reference
///                   | interval
///                   | query_expression
///                   | ( \[ unary_op \] ( `(` expression `)` | primary ) ) .
pub fn simple_factor(input: &str) -> ParseResult<Expression> {
    let paren_expr = tuple((char('('), expression, char(')'))).map(|(_open, e, _close)| e);
    // ( \[ unary_op \] ( `(` expression `)` | primary ) )
    let unary = tuple((
        opt(unary_op),
        alt((paren_expr, primary.map(|p| Expression::Primary(p)))),
    ))
    .map(|(opt, expr)| {
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
        aggregate_initializer,
        entity_constructor,
        enumeration_reference,
        interval,
        query_expression,
        unary,
    ))
    .parse(input)
}

/// 216 expression = simple_expression \[ rel_op_extended simple_expression \] .
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

/// 169 aggregate_initializer = ’[’ [ element { ’,’ element } ] ’]’ .
pub fn aggregate_initializer(input: &str) -> ParseResult<Expression> {
    todo!()
}

/// 203 element = expression [ ’:’ repetition ] .
pub fn element(input: &str) -> ParseResult<Expression> {
    todo!()
}

/// 287 repetition = numeric_expression .
pub fn repetition(input: &str) -> ParseResult<Expression> {
    todo!()
}

/// 212 enumeration_reference = [ type_ref ’.’ ] enumeration_ref .
pub fn enumeration_reference(input: &str) -> ParseResult<Expression> {
    todo!()
}

/// 243 interval = `{` interval_low interval_op interval_item interval_op interval_high `}` .
/// 244 interval_high = simple_expression .
/// 245 interval_item = simple_expression .
/// 246 interval_low = simple_expression .
pub fn interval(input: &str) -> ParseResult<Expression> {
    todo!()
}

/// 277 query_expression = QUERY `(` variable_id `<*` aggregate_source `|` logical_expression `)` .
pub fn query_expression(input: &str) -> ParseResult<Expression> {
    todo!()
}

/// 205 entity_constructor = entity_ref ’(’ [ expression { ’,’ expression } ] ’)’ .
pub fn entity_constructor(input: &str) -> ParseResult<Expression> {
    tuple((
        remarked(simple_id),
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
    use nom::Finish;

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
}
