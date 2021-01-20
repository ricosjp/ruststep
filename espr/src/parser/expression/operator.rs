use super::super::util::*;

/// Relation operators parsed by [rel_op] and [rel_op_extended]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RelationOperator {
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

    /* extended */
    /// `IN`
    In,
    /// `LIKE`
    Like,
}

/// 282 rel_op = `<` | `>` | `<=` | `>=` | `<>` | `=` | `:<>:` | `:=:` .
pub fn rel_op(input: &str) -> ParseResult<RelationOperator> {
    alt((
        value(RelationOperator::Equal, tag("=")),
        value(RelationOperator::NotEqual, tag("<>")),
        value(RelationOperator::LT, tag("<")),
        value(RelationOperator::GT, tag(">")),
        value(RelationOperator::LEQ, tag("<=")),
        value(RelationOperator::GEQ, tag(">=")),
        value(RelationOperator::InstanceEqual, tag(":=:")),
        value(RelationOperator::InstanceNotEqual, tag(":<>:")),
    ))
    .parse(input)
}

/// 283 rel_op_extended = [rel_op] | `IN` | `LIKE` .
pub fn rel_op_extended(input: &str) -> ParseResult<RelationOperator> {
    alt((
        rel_op,
        alt((
            value(RelationOperator::In, tag("IN")),
            value(RelationOperator::Like, tag("LIKE")),
        )),
    ))
    .parse(input)
}

/// Unary operators parsed by [unary_op]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `NOT`
    Not,
}

/// 331 unary_op = `+` | `-` | `NOT` .
pub fn unary_op(input: &str) -> ParseResult<UnaryOperator> {
    alt((
        value(UnaryOperator::Plus, tag("+")),
        value(UnaryOperator::Minus, tag("-")),
        value(UnaryOperator::Not, tag("NOT")),
    ))
    .parse(input)
}

/// Binary operators parsed by [add_like_op], [multiplication_like_op], and [power_op]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    /* Mul-like */
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

    /* Add-like */
    /// `+`
    Add,
    /// `-`
    Sub,
    /// `OR`
    Or,
    /// `XOR`
    Xor,

    /* power */
    /// `**`
    Power,
}

/// 168 add_like_op = `+` | `-` | `OR` | `XOR` .
pub fn add_like_op(input: &str) -> ParseResult<BinaryOperator> {
    use BinaryOperator::*;
    alt((
        value(Add, tag("+")),
        value(Sub, tag("-")),
        value(Or, tag("OR")),
        value(Xor, tag("XOR")),
    ))
    .parse(input)
}

/// 257 multiplication_like_op = `*` | `/` | `DIV` | `MOD` | `AND` | `||` .
pub fn multiplication_like_op(input: &str) -> ParseResult<BinaryOperator> {
    alt((
        value(BinaryOperator::Mul, tag("*")),
        value(BinaryOperator::RealDiv, tag("/")),
        value(BinaryOperator::IntegerDiv, tag("DIV")),
        value(BinaryOperator::Mod, tag("MOD")),
        value(BinaryOperator::And, tag("AND")),
        value(BinaryOperator::ComplexEntityInstanceConstruction, tag("||")),
    ))
    .parse(input)
}

/// 999 power_op = `**`
///
/// Additional trivial rule for managing operators uniformly
pub fn power_op(input: &str) -> ParseResult<BinaryOperator> {
    value(BinaryOperator::Power, tag("**")).parse(input)
}
