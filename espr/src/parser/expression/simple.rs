use super::{super::util::*, operator::*, primary::*};

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
    todo!()
}

/// 306 simple_factor = aggregate_initializer
///                   | entity_constructor
///                   | enumeration_reference
///                   | interval
///                   | query_expression
///                   | ( \[ unary_op \] ( `(` expression `)` | primary ) ) .
pub fn simple_factor(input: &str) -> ParseResult<Expression> {
    todo!()
}

/// 216 expression = simple_expression \[ rel_op_extended simple_expression \] .
pub fn expression(input: &str) -> ParseResult<Expression> {
    todo!()
}
