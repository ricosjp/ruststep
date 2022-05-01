use crate::ast::*;

#[cfg(doc)]
use crate::parser;

/// Primitive value type in STEP data
///
/// Inline struct or list can be nested, i.e. `Parameter` can be a tree.
///
/// ```
/// use nom::Finish;
/// use ruststep::{parser::exchange, ast::{Parameter, Record}};
///
/// let (residual, p) = exchange::parameter("B((1.0, A((2.0, 3.0))))")
///     .finish()
///     .unwrap();
/// assert_eq!(residual, "");
///
/// // A((2.0, 3.0))
/// let a = Parameter::Typed(Record {
///     name: "A".to_string(),
///     parameter: Box::new(vec![Parameter::real(2.0), Parameter::real(3.0)].into()),
/// });
///
/// // B((1.0, a))
/// let b = Parameter::Typed(Record {
///     name: "B".to_string(),
///     parameter: Box::new(vec![Parameter::real(1.0), a].into()),
/// });
///
/// assert_eq!(p, b);
/// ```
///
/// FromIterator
/// -------------
/// Create a list as `Parameter::List` from `Iterator<Item=Parameter>` or `Iterator<Item=&Parameter>`.
///
/// ```
/// use ruststep::ast::Parameter;
///
/// let p: Parameter = [Parameter::real(1.0), Parameter::real(2.0)]
///     .iter()
///     .collect();
/// assert!(matches!(p, Parameter::List(_)));
/// ```
///
#[derive(Debug, Clone, PartialEq, derive_more::From)]
pub enum Parameter {
    /// Corresponding to `TYPED_PARAMETER` in WSN:
    ///
    /// ```text
    /// TYPED_PARAMETER = KEYWORD "(" PARAMETER ")" .
    /// ```
    ///
    /// and [parser::exchange::typed_parameter].
    /// It takes only one `PARAMETER` different from [Record],
    /// which takes many `PARAMETER`s.
    ///
    /// ```text
    /// SIMPLE_RECORD = KEYWORD "(" [ PARAMETER_LIST ] ")" .
    /// ```
    ///
    /// ```
    /// # use nom::Finish;
    /// # use ruststep::{parser, ast::Parameter};
    /// let (residual, p) = parser::exchange::parameter("FILE_NAME('ruststep')").finish().unwrap();
    /// assert!(matches!(p, Parameter::Typed(_)));
    /// ```
    Typed(Record),

    /// Signed integer
    ///
    /// ```
    /// # use nom::Finish;
    /// # use ruststep::{parser, ast::Parameter};
    /// let (residual, p) = parser::exchange::parameter("10").finish().unwrap();
    /// assert_eq!(p, Parameter::Integer(10));
    /// # assert_eq!(residual, "");
    /// let (residual, p) = parser::exchange::parameter("-10").finish().unwrap();
    /// assert_eq!(p, Parameter::Integer(-10));
    /// # assert_eq!(residual, "");
    /// ```
    #[from]
    Integer(i64),

    /// Real number
    ///
    /// ```
    /// # use nom::Finish;
    /// # use ruststep::{parser, ast::Parameter};
    /// let (residual, p) = parser::exchange::parameter("1.0").finish().unwrap();
    /// assert_eq!(p, Parameter::Real(1.0));
    /// # assert_eq!(residual, "");
    /// ```
    #[from]
    Real(f64),

    /// string literal
    ///
    /// ```
    /// # use nom::Finish;
    /// # use ruststep::{parser, ast::Parameter};
    /// let (residual, p) = parser::exchange::parameter("'EXAMPLE STRING'").finish().unwrap();
    /// assert_eq!(p, Parameter::String("EXAMPLE STRING".to_string()));
    /// # assert_eq!(residual, "");
    /// ```
    #[from]
    String(String),

    /// Enumeration defined in EXPRESS schema, like `.TRUE.`
    ///
    /// ```
    /// # use nom::Finish;
    /// # use ruststep::{parser, ast::Parameter};
    /// let (residual, p) = parser::exchange::parameter(".TRUE.").finish().unwrap();
    /// assert_eq!(p, Parameter::Enumeration("TRUE".to_string()));
    /// # assert_eq!(residual, "");
    /// ```
    Enumeration(String),

    /// List of parameters. This can be non-uniform.
    ///
    /// ```
    /// # use nom::Finish;
    /// # use ruststep::{parser, ast::Parameter};
    /// let (residual, p) = parser::exchange::parameter("(1.0, 2, 'STRING')").finish().unwrap();
    /// assert_eq!(p, Parameter::List(vec![
    ///   Parameter::Real(1.0),
    ///   Parameter::Integer(2),
    ///   Parameter::String("STRING".to_string()),
    /// ]));
    /// # assert_eq!(residual, "");
    /// ```
    #[from]
    List(Vec<Parameter>),

    /// A reference to entity or value
    #[from]
    RValue(RValue),

    /// The special token dollar sign (`$`) is used to represent
    /// an object whose value is not provided in the exchange structure.
    NotProvided,
    /// Omitted parameter denoted by `*`
    Omitted,
}

impl Parameter {
    pub fn integer(i: i64) -> Self {
        Parameter::Integer(i)
    }

    pub fn real(x: f64) -> Self {
        Parameter::Real(x)
    }

    pub fn string(s: &str) -> Self {
        Parameter::String(s.to_string())
    }
}

impl std::iter::FromIterator<Parameter> for Parameter {
    fn from_iter<Iter: IntoIterator<Item = Parameter>>(iter: Iter) -> Self {
        Parameter::List(iter.into_iter().collect())
    }
}

impl<'a> std::iter::FromIterator<&'a Parameter> for Parameter {
    fn from_iter<Iter: IntoIterator<Item = &'a Parameter>>(iter: Iter) -> Self {
        iter.into_iter().cloned().collect()
    }
}
