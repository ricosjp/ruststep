use super::super::util::*;

/// 283 rel_op_extended = rel_op | `IN` | `LIKE` .
pub fn rel_op_extended(input: &str) -> ParseResult<Relation> {
    alt((
        rel_op,
        alt((
            value(Relation::In, tag("IN")),
            value(Relation::Like, tag("LIKE")),
        )),
    ))
    .parse(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Relation {
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
pub fn rel_op(input: &str) -> ParseResult<Relation> {
    alt((
        value(Relation::Equal, tag("=")),
        value(Relation::NotEqual, tag("<>")),
        value(Relation::LT, tag("<")),
        value(Relation::GT, tag(">")),
        value(Relation::LEQ, tag("<=")),
        value(Relation::GEQ, tag(">=")),
        value(Relation::InstanceEqual, tag(":=:")),
        value(Relation::InstanceNotEqual, tag(":<>:")),
    ))
    .parse(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unary {
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `NOT`
    Not,
}

/// 331 unary_op = `+` | `-` | `NOT` .
pub fn unary_op(input: &str) -> ParseResult<Unary> {
    alt((
        value(Unary::Plus, tag("+")),
        value(Unary::Minus, tag("-")),
        value(Unary::Not, tag("NOT")),
    ))
    .parse(input)
}

/// 257 multiplication_like_op = `*` | `/` | `DIV` | `MOD` | `AND` | `||` .
pub fn multiplication_like_op(input: &str) -> ParseResult<Binary> {
    alt((
        value(Binary::Mul, tag("*")),
        value(Binary::RealDiv, tag("/")),
        value(Binary::IntegerDiv, tag("DIV")),
        value(Binary::Mod, tag("MOD")),
        value(Binary::And, tag("AND")),
        value(Binary::ComplexEntityInstanceConstruction, tag("||")),
    ))
    .parse(input)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Binary {
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
pub fn add_like_op(input: &str) -> ParseResult<Binary> {
    use Binary::*;
    alt((
        value(Add, tag("+")),
        value(Sub, tag("-")),
        value(Or, tag("OR")),
        value(Xor, tag("XOR")),
    ))
    .parse(input)
}

/// 999 power_op = `**`
///
/// Additional trivial rule for managing operators uniformly
pub fn power_op(input: &str) -> ParseResult<Binary> {
    value(Binary::Power, tag("**")).parse(input)
}
