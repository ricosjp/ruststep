use derive_more::From;

#[cfg(doc)]
use crate::parser::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
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
    Literal(Literal),
    QualifiableFactor {
        factor: QualifiableFactor,
        qualifiers: Vec<Qualifier>,
    },
    EntityConstructor {
        name: String,
        values: Vec<Expression>,
    },
    Interval {
        op_low: IntervalOperator,
        op_high: IntervalOperator,
        high: Box<Expression>,
        low: Box<Expression>,
        item: Box<Expression>,
    },
    EnumerationReference {
        ty: Option<String>,
        enum_ref: String,
    },
    AggregateInitializer {
        elements: Vec<Element>,
    },
    Query {
        variable: String,
        source: Box<Expression>,
        expr: Box<Expression>,
    },
}

macro_rules! impl_relation_op_expression {
    ($f:ident, $op:path) => {
        pub fn $f(self, other: Self) -> Self {
            Expression::Relation {
                op: $op,
                lhs: Box::new(self),
                rhs: Box::new(other),
            }
        }
    };
}

macro_rules! impl_binary_op_expression {
    ($f:ident, $op:path) => {
        pub fn $f(self, other: Self) -> Self {
            Expression::Binary {
                op: $op,
                arg1: Box::new(self),
                arg2: Box::new(other),
            }
        }
    };
}

impl Expression {
    /// `SELF` constant
    pub fn self_() -> Self {
        Self::self_qualified(Vec::new())
    }

    /// `SELF` constant with qualifiers
    pub fn self_qualified(qualifiers: Vec<Qualifier>) -> Self {
        Expression::QualifiableFactor {
            factor: QualifiableFactor::BuiltInConstant(BuiltInConstant::SELF),
            qualifiers,
        }
    }

    /// `?` constant
    pub fn indeterminate() -> Self {
        Expression::QualifiableFactor {
            factor: QualifiableFactor::BuiltInConstant(BuiltInConstant::INDETERMINATE),
            qualifiers: Vec::new(),
        }
    }

    /// Real value literal
    pub fn real(value: f64) -> Self {
        Expression::Literal(Literal::Real(value))
    }

    impl_relation_op_expression!(leq, RelationOperator::LEQ);
    impl_relation_op_expression!(geq, RelationOperator::GEQ);
    impl_relation_op_expression!(lt, RelationOperator::LT);
    impl_relation_op_expression!(gt, RelationOperator::GT);
    impl_relation_op_expression!(eq, RelationOperator::Equal);
    impl_relation_op_expression!(neq, RelationOperator::NotEqual);
    impl_relation_op_expression!(in_, RelationOperator::In);
    impl_relation_op_expression!(like, RelationOperator::Like);

    impl_binary_op_expression!(and, BinaryOperator::And);
    impl_binary_op_expression!(or, BinaryOperator::Or);
    impl_binary_op_expression!(xor, BinaryOperator::Xor);
}

macro_rules! impl_binary_op_expression {
    ($f:ident, $ops:path, $op:path) => {
        impl $ops for Expression {
            type Output = Self;
            fn $f(self, other: Expression) -> Self {
                Expression::Binary {
                    op: $op,
                    arg1: Box::new(self),
                    arg2: Box::new(other),
                }
            }
        }
    };
}

impl_binary_op_expression!(add, std::ops::Add, BinaryOperator::Add);
impl_binary_op_expression!(sub, std::ops::Sub, BinaryOperator::Sub);
impl_binary_op_expression!(mul, std::ops::Mul, BinaryOperator::Mul);

#[derive(Debug, Clone, PartialEq)]
pub enum QualifiableFactor {
    /// [attribute_ref], [general_ref], [population], or [constant_ref]
    Reference(String),
    /// [built_in_constant]
    BuiltInConstant(BuiltInConstant),
    /// [function_call]
    FunctionCall {
        name: FunctionCallName,
        args: Vec<Expression>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FunctionCallName {
    BuiltInFunction(BuiltInFunction),
    Reference(String),
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)] // to use original identifiers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BuiltInFunction {
    ABS,
    ACOS,
    ASIN,
    ATAN,
    BLENGTH,
    COS,
    EXISTS,
    EXP,
    FORMAT,
    HIBOUND,
    HIINDEX,
    LENGTH,
    LOBOUND,
    LOINDEX,
    LOG,
    LOG2,
    LOG10,
    NVL,
    ODD,
    ROLESOF,
    SIN,
    SIZEOF,
    SQRT,
    TAN,
    TYPEOF,
    USEDIN,
    VALUE,
    VALUE_IN,
    VALUE_UNIQUE,
}

/// Output of [qualifier]
#[derive(Debug, Clone, PartialEq)]
pub enum Qualifier {
    /// Like `.x`
    Attribute(String),
    /// Like `\point`
    Group(String),
    /// Like `[1]`
    Index(Expression),
    /// Like `[1:3]`
    Range { begin: Expression, end: Expression },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltInConstant {
    /// `CONST_E`, Napier's constant `e = 2.71828 …`
    NAPIER,
    /// The ratio of a circle's circumference to its diameter, `π = 3.14159 …`
    PI,
    /// `SELF` is not a constant, but behaves as one in every context in which it can appear.
    SELF,
    /// The indeterminate symbol `?` stands for an ambiguous value.
    /// It is compatible with all data types.
    INDETERMINATE,
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IntervalOperator {
    /// `<`
    LessThan,
    /// `<=`
    LessThanEqual,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    pub expr: Expression,
    pub repetition: Option<Expression>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Logical {
    False,
    True,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, From)]
pub enum Literal {
    Real(f64),
    String(String),
    Logial(Logical),
}
