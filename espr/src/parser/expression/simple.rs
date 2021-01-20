use super::{super::util::*, operator::*, primary::*};

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleExpression {
    /// End node of expression tree
    Primary(Primary),
    Unary {
        op: UnaryOperator,
        arg: Box<SimpleExpression>,
    },
    Binary {
        op: BinaryOperator,
        arg1: Box<SimpleExpression>,
        arg2: Box<SimpleExpression>,
    },
}

/// 305 simple_expression = term { add_like_op term } .
pub fn simple_expression(input: &str) -> ParseResult<SimpleExpression> {
    todo!()
}

/// 325 term = factor { multiplication_like_op factor } .
pub fn term(input: &str) -> ParseResult<SimpleExpression> {
    todo!()
}

/// 217 factor = simple_factor \[ `**` simple_factor \] .
pub fn factor(input: &str) -> ParseResult<SimpleExpression> {
    todo!()
}

/// 306 simple_factor = aggregate_initializer
///                   | entity_constructor
///                   | enumeration_reference
///                   | interval
///                   | query_expression
///                   | ( \[ unary_op \] ( `(` expression `)` | primary ) ) .
pub fn simple_factor(input: &str) -> ParseResult<SimpleExpression> {
    todo!()
}
