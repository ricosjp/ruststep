use nom::IResult;

pub struct Expr {}

/// ```text
/// 216 expression = simple_expression [ rel_op_extended simple_expression ] .
/// 283 rel_op_extended = rel_op | IN | LIKE .
/// 282 rel_op = ’<’ | ’>’ | ’<=’ | ’>=’ | ’<>’ | ’=’ | ’:<>:’ | ’:=:’ .
/// 305 simple_expression = term { add_like_op term } .
/// 325 term = factor { multiplication_like_op factor } .
/// 217 factor = simple_factor [ ’**’ simple_factor ] .
/// 306 simple_factor = aggregate_initializer | entity_constructor |enumeration_reference | interval | query_expression |( [ unary_op ] ( ’(’ expression ’)’ | primary ) ) .
/// 331 unary_op = ’+’ | ’-’ | NOT .
/// 269 primary = literal | ( qualifiable_factor { qualifier } ) .
/// 257 multiplication_like_op = ’*’ | ’/’ | DIV | MOD | AND | ’||’ .
/// 168 add_like_op = ’+’ | ’-’ | OR | XOR .
/// ```
pub fn expr(input: &str) -> IResult<&str, Expr> {
    todo!()
}
