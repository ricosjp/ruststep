use super::*;
use derive_more::From;
use nom::{branch::*, bytes::complete::*, combinator::*, IResult, Parser};

/// Unary expresion, e.g. `x` or binary expression `x + y`
///
/// Example
/// --------
///
/// ```
/// use exp2rs::parser::*;
/// use nom::Finish;
///
/// let (residual, e) = expression("1 - 2").finish().unwrap();
/// assert_eq!(residual, "");
/// ```
#[derive(Debug, Clone, PartialEq)]
pub enum Expr<Base, Op> {
    Unary(Base),
    Binary { op: Op, arg1: Base, arg2: Base },
}

impl<Base, Op> From<Base> for Expr<Base, Op> {
    fn from(base: Base) -> Self {
        Expr::Unary(base)
    }
}

pub type Expression = Expr<SimpleExpression, RelOpExtended>;
pub type SimpleExpression = Expr<Term, AddLikeOp>;
pub type Term = Expr<Factor, MultiplicationLikeOp>;
pub type Factor = Expr<SimpleFactor, PowerOp>;

fn expr<Base, Op>(
    input: &str,
    base_parse: impl Fn(&str) -> IResult<&str, Base> + Copy,
    op_parser: impl Fn(&str) -> IResult<&str, Op>,
) -> IResult<&str, Expr<Base, Op>> {
    tuple((
        base_parse,
        opt(tuple((multispace0, op_parser, multispace0, base_parse))),
    ))
    .map(|(base, opt)| {
        if let Some((_, op, _, arg2)) = opt {
            Expr::Binary {
                op,
                arg1: base,
                arg2,
            }
        } else {
            Expr::Unary(base)
        }
    })
    .parse(input)
}

/// 216 expression = simple_expression [ rel_op_extended simple_expression ] .
pub fn expression(input: &str) -> IResult<&str, Expression> {
    expr(input, simple_expression, rel_op_extended)
}

/// 305 simple_expression = term { add_like_op term } .
pub fn simple_expression(input: &str) -> IResult<&str, SimpleExpression> {
    expr(input, term, add_like_op)
}

/// 325 term = factor { multiplication_like_op factor } .
pub fn term(input: &str) -> IResult<&str, Term> {
    expr(input, factor, multiplication_like_op)
}

/// 217 factor = simple_factor [ `**` simple_factor ] .
pub fn factor(input: &str) -> IResult<&str, Factor> {
    expr(input, simple_factor, power_op)
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimpleFactor {
    Primary {
        unary_op: Option<UnaryOp>,
        primary: Primary,
    },
}

impl From<Primary> for SimpleFactor {
    fn from(primary: Primary) -> Self {
        SimpleFactor::Primary {
            unary_op: None,
            primary,
        }
    }
}

impl From<(UnaryOp, Primary)> for SimpleFactor {
    fn from((unary_op, primary): (UnaryOp, Primary)) -> Self {
        SimpleFactor::Primary {
            unary_op: Some(unary_op),
            primary,
        }
    }
}

/// 306 simple_factor = aggregate_initializer
///                   | entity_constructor
///                   | enumeration_reference
///                   | interval
///                   | query_expression
///                   | ( [ unary_op ] ( `(` expression `)` | primary ) ) .
///
/// Example
/// --------
///
/// ```
/// use exp2rs::parser::*;
/// use nom::Finish;
///
/// let (residual, p) = simple_factor("123").finish().unwrap();
/// assert_eq!(p, Primary::Literal(Literal::Real(123.0)).into());
/// assert_eq!(residual, "");
///
/// let (residual, p) = simple_factor("-123").finish().unwrap();
/// assert_eq!(p, (UnaryOp::Minus, Primary::Literal(Literal::Real(123.0))).into());
/// assert_eq!(residual, "");
/// ```
pub fn simple_factor(input: &str) -> IResult<&str, SimpleFactor> {
    // FIXME most branches are not supported
    tuple((
        opt(tuple((unary_op, multispace0)).map(|(op, _)| op)),
        primary,
    ))
    .map(|(unary_op, primary)| SimpleFactor::Primary { unary_op, primary })
    .parse(input)
}

#[derive(Debug, Clone, PartialEq, From)]
pub enum Primary {
    Literal(Literal),
}

/// 269 primary = literal | ( qualifiable_factor { qualifier } ) .
///
/// Example
/// --------
///
/// ```
/// use exp2rs::parser::*;
/// use nom::Finish;
///
/// let (residual, p) = primary("123").finish().unwrap();
/// assert_eq!(p, Literal::Real(123.0).into());
/// assert_eq!(residual, "");
/// ```
pub fn primary(input: &str) -> IResult<&str, Primary> {
    // FIXME add qualifiable_factor branch
    literal
        .map(|literal| Primary::Literal(literal))
        .parse(input)
}

#[derive(Debug, Clone, PartialEq, From)]
pub enum RelOpExtended {
    RelOp(RelOp),
    /// `IN`
    In,
    /// `LIKE`
    Like,
}

/// 283 rel_op_extended = rel_op | `IN` | `LIKE` .
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

/// 282 rel_op = `<` | `>` | `<=` | `>=` | `<>` | `=` | `:<>:` | `:=:` .
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

/// 331 unary_op = `+` | `-` | `NOT` .
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

/// 257 multiplication_like_op = `*` | `/` | `DIV` | `MOD` | `AND` | `||` .
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

/// 168 add_like_op = `+` | `-` | `OR` | `XOR` .
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

#[derive(Debug, Clone, PartialEq)]
pub enum PowerOp {
    /// `**`
    Power,
}

/// 999 power_op = `**`
///
/// Additional trivial rule for managing operators uniformly
pub fn power_op(input: &str) -> IResult<&str, PowerOp> {
    value(PowerOp::Power, tag("**")).parse(input)
}
