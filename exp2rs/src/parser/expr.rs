use nom::{branch::*, bytes::complete::*, combinator::*, IResult, Parser};

pub struct Expr {}

/// ```text
/// 216 expression = simple_expression [ rel_op_extended simple_expression ] .
/// 305 simple_expression = term { add_like_op term } .
/// 325 term = factor { multiplication_like_op factor } .
/// 217 factor = simple_factor [ ’**’ simple_factor ] .
/// 306 simple_factor = aggregate_initializer
///                   | entity_constructor
///                   | enumeration_reference
///                   | interval
///                   | query_expression
///                   | ( [ unary_op ] ( ’(’ expression ’)’ | primary ) ) .
/// 269 primary = literal | ( qualifiable_factor { qualifier } ) .
/// ```
pub fn expr(_input: &str) -> IResult<&str, Expr> {
    todo!()
}

#[derive(Debug, Clone, PartialEq)]
pub enum RelOpExtended {
    RelOp(RelOp),
    /// `IN`
    In,
    /// `LIKE`
    Like,
}

impl From<RelOp> for RelOpExtended {
    fn from(op: RelOp) -> Self {
        RelOpExtended::RelOp(op)
    }
}

/// Relation operator extened with `IN` and `LIKE`
///
/// ```text
/// 283 rel_op_extended = rel_op | IN | LIKE .
/// ```
pub fn rel_op_extended(input: &str) -> IResult<&str, RelOpExtended> {
    use RelOpExtended::*;
    alt((
        rel_op.map(|op| RelOp(op)),
        alt((value(In, tag("IN")), value(Like, tag("LIKE")))),
    ))
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum RelOp {
    /// `=`
    Equal,
    /// `<>`
    NotEqual,
    /// `<`
    LT,
    /// `>`
    GT,
    /// `<=`
    LEQ,
    /// `>=`
    GEQ,
    /// `:=:`
    InstanceEqual,
    /// `:<>:`
    InstanceNotEqual,
}

/// Relation operator
///
/// ```text
/// 282 rel_op = ’<’ | ’>’ | ’<=’ | ’>=’ | ’<>’ | ’=’ | ’:<>:’ | ’:=:’ .
/// ```
pub fn rel_op(input: &str) -> IResult<&str, RelOp> {
    use RelOp::*;
    alt((
        value(Equal, tag("=")),
        value(NotEqual, tag("<>")),
        value(LT, tag("<")),
        value(GT, tag(">")),
        value(LEQ, tag("<=")),
        value(GEQ, tag(">=")),
        value(InstanceEqual, tag(":=:")),
        value(InstanceNotEqual, tag(":<>:")),
    ))
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `NOT`
    Not,
}

/// Unary operator
///
/// ```text
/// 331 unary_op = ’+’ | ’-’ | NOT .
/// ```
pub fn unary_op(input: &str) -> IResult<&str, UnaryOp> {
    use UnaryOp::*;
    alt((
        value(Plus, tag("+")),
        value(Minus, tag("-")),
        value(Not, tag("NOT")),
    ))
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum MultiplicationLikeOp {
    /// `*`
    Mul,
    /// `/`
    RealDiv,
    /// `DIV`
    IntegerDiv,
    /// `MOD`
    Mod,
    /// `AND`
    And,
    /// `||`, Complex entity instance construction operator (12.10)
    ComplexEntityInstanceConstruction,
}

/// Operator which behaves like `*`, i.e. `*`, `/`, `DIV`, `MOD`, `AND`, and `||`
///
/// ```text
/// 257 multiplication_like_op = ’*’ | ’/’ | DIV | MOD | AND | ’||’ .
/// ```
pub fn multiplication_like_op(input: &str) -> IResult<&str, MultiplicationLikeOp> {
    use MultiplicationLikeOp::*;
    alt((
        value(Mul, tag("*")),
        value(RealDiv, tag("/")),
        value(IntegerDiv, tag("DIV")),
        value(Mod, tag("MOD")),
        value(And, tag("AND")),
        value(ComplexEntityInstanceConstruction, tag("||")),
    ))
    .parse(input)
}

#[derive(Debug, Clone, PartialEq)]
pub enum AddLikeOp {
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `OR`
    Or,
    /// `XOR`
    Xor,
}

/// Operator which behaves like `+`, i.e. `+`, `-`, `OR`, and `XOR`
///
/// ```text
/// 168 add_like_op = ’+’ | ’-’ | OR | XOR .
/// ```
pub fn add_like_op(input: &str) -> IResult<&str, AddLikeOp> {
    use AddLikeOp::*;
    alt((
        value(Add, tag("+")),
        value(Sub, tag("-")),
        value(Or, tag("OR")),
        value(Xor, tag("XOR")),
    ))
    .parse(input)
}
