use super::util::*;

mod operator;
mod primary;
mod simple;

pub use operator::*;
pub use primary::*;
pub use simple::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Relation(Relation),
    SimpleExpression(SimpleExpression),
}

/// Relationship between two expressions, parsed by [expression]
#[derive(Debug, Clone, PartialEq)]
pub struct Relation {
    pub op: RelationOperator,
    pub lhs: SimpleExpression,
    pub rhs: SimpleExpression,
}

/// 216 expression = simple_expression \[ rel_op_extended simple_expression \] .
pub fn expression(input: &str) -> ParseResult<Expression> {
    todo!()
}
