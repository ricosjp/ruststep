use super::super::util::*;

/// 283 rel_op_extended = rel_op | `IN` | `LIKE` .
pub fn rel_op_extended(input: &str) -> ParseResult<RelOp> {
    use RelOp::*;
    alt((
        rel_op,
        alt((value(In, tag("IN")), value(Like, tag("LIKE")))),
    ))
    .parse(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /* extended */
    /// `IN`
    In,
    /// `LIKE`
    Like,
}

/// 282 rel_op = `<` | `>` | `<=` | `>=` | `<>` | `=` | `:<>:` | `:=:` .
pub fn rel_op(input: &str) -> ParseResult<RelOp> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `NOT`
    Not,
}

/// 331 unary_op = `+` | `-` | `NOT` .
pub fn unary_op(input: &str) -> ParseResult<UnaryOp> {
    use UnaryOp::*;
    alt((
        value(Plus, tag("+")),
        value(Minus, tag("-")),
        value(Not, tag("NOT")),
    ))
    .parse(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// 257 multiplication_like_op = `*` | `/` | `DIV` | `MOD` | `AND` | `||` .
pub fn multiplication_like_op(input: &str) -> ParseResult<MultiplicationLikeOp> {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// 168 add_like_op = `+` | `-` | `OR` | `XOR` .
pub fn add_like_op(input: &str) -> ParseResult<AddLikeOp> {
    use AddLikeOp::*;
    alt((
        value(Add, tag("+")),
        value(Sub, tag("-")),
        value(Or, tag("OR")),
        value(Xor, tag("XOR")),
    ))
    .parse(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerOp {
    /// `**`
    Power,
}

/// 999 power_op = `**`
///
/// Additional trivial rule for managing operators uniformly
pub fn power_op(input: &str) -> ParseResult<PowerOp> {
    value(PowerOp::Power, tag("**")).parse(input)
}
