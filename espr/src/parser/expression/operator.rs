use super::super::combinator::*;
use crate::ast::expression::*;

/// 282 rel_op = `<` | `>` | `<=` | `>=` | `<>` | `=` | `:<>:` | `:=:` .
pub fn rel_op(input: &str) -> ParseResult<RelationOperator> {
    alt((
        value(RelationOperator::InstanceEqual, tag(":=:")),
        value(RelationOperator::InstanceNotEqual, tag(":<>:")),
        value(RelationOperator::LEQ, tag("<=")),
        value(RelationOperator::GEQ, tag(">=")),
        value(RelationOperator::Equal, tag("=")),
        value(RelationOperator::NotEqual, tag("<>")),
        value(RelationOperator::LT, tag("<")),
        value(RelationOperator::GT, tag(">")),
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

/// 331 unary_op = `+` | `-` | `NOT` .
pub fn unary_op(input: &str) -> ParseResult<UnaryOperator> {
    alt((
        value(UnaryOperator::Plus, tag("+")),
        value(UnaryOperator::Minus, tag("-")),
        value(UnaryOperator::Not, tag("NOT")),
    ))
    .parse(input)
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

/// 247 interval_op = `<` | `<=` .
pub fn interval_op(input: &str) -> ParseResult<IntervalOperator> {
    alt((
        value(IntervalOperator::LessThanEqual, tag("<=")),
        value(IntervalOperator::LessThan, tag("<")),
    ))
    .parse(input)
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    #[test]
    fn rel_op() {
        let (res, (op, _remarks)) = super::rel_op("<=").finish().unwrap();
        dbg!(op);
        assert_eq!(res, "");
    }
}
