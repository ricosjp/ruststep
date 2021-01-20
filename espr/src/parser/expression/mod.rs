use super::{literal::*, util::*};
use derive_more::From;

mod operator;
mod qualifier;

pub use operator::*;
pub use qualifier::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Relation(Relation),
    ExprTree(ExprTree),
}

/// Relationship between two expressions, parsed by [expression]
#[derive(Debug, Clone, PartialEq)]
pub struct Relation {
    pub op: RelationOperator,
    pub lhs: ExprTree,
    pub rhs: ExprTree,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExprTree {
    /// End node of expression tree
    Primary(Primary),
    Unary {
        op: UnaryOperator,
        arg: Box<ExprTree>,
    },
    Binary {
        op: BinaryOperator,
        arg1: Box<ExprTree>,
        arg2: Box<ExprTree>,
    },
}

/// 216 expression = simple_expression \[ rel_op_extended simple_expression \] .
pub fn expression(input: &str) -> ParseResult<Expression> {
    todo!()
}

/// 305 simple_expression = term { add_like_op term } .
pub fn simple_expression(input: &str) -> ParseResult<ExprTree> {
    todo!()
}

/// 325 term = factor { multiplication_like_op factor } .
pub fn term(input: &str) -> ParseResult<ExprTree> {
    todo!()
}

/// 217 factor = simple_factor \[ `**` simple_factor \] .
pub fn factor(input: &str) -> ParseResult<ExprTree> {
    todo!()
}

/// 306 simple_factor = aggregate_initializer
///                   | entity_constructor
///                   | enumeration_reference
///                   | interval
///                   | query_expression
///                   | ( \[ unary_op \] ( `(` expression `)` | primary ) ) .
pub fn simple_factor(input: &str) -> ParseResult<ExprTree> {
    todo!()
}
