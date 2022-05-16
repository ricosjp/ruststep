//! Abstract syntax tree for exchange structure
//!
//! This module contains implementation of [serde::Serialize] and [serde::Deserialize]
//! for AST structs.
//! See [de] and [ser] module document.
//!

pub mod de;
pub mod ser;

use crate::parser;
use std::str::FromStr;

/// AST portion
pub trait AST: FromStr<Err = crate::error::Error> {
    fn parse(input: &str) -> parser::combinator::ParseResult<Self>;
}

macro_rules! derive_ast_from_str {
    ($ast:ty, $parse:path) => {
        impl std::str::FromStr for $ast {
            type Err = $crate::error::Error;
            fn from_str(input: &str) -> $crate::error::Result<Self> {
                use nom::Finish;
                let input = input.trim();
                let (residual, record) = AST::parse(input)
                    .finish()
                    .map_err(|err| $crate::error::TokenizeFailed::new(input, err))?;
                if !residual.is_empty() {
                    return Err($crate::error::Error::ExtraInputRemaining(input.to_string()));
                }
                Ok(record)
            }
        }

        impl AST for $ast {
            fn parse(input: &str) -> parser::combinator::ParseResult<Self> {
                $parse(input)
            }
        }
    };
}

/// Name of an entity instance or a value
///
/// Corresponding to [parser::token::rhs_occurrence_name] and [parser::token::lhs_occurrence_name]
#[derive(Debug, Clone, PartialEq)]
pub enum Name {
    /// Like `#11`, corresponds to [parser::token::entity_instance_name]
    Entity(u64),
    /// Like `@11`, corresponds to [parser::token::value_instance_name]
    Value(u64),
    /// Like `#CONST_ENTITY`, corresponds to [parser::token::constant_entity_name]
    ConstantEntity(String),
    /// Like `@CONST_VALUE`, corresponds to [parser::token::constant_value_name]
    ConstantValue(String),
}
derive_ast_from_str!(Name, parser::token::rhs_occurrence_name);

/// A struct typed in EXPRESS schema, e.g. `A(1.0, 2.0)`
///
/// ```
/// use ruststep::ast::{Record, Parameter};
/// use std::str::FromStr;
///
/// let record = Record::from_str("A(1, 2)").unwrap();
/// assert_eq!(
///     record,
///     Record {
///         name: "A".to_string(),
///         parameter: Box::new(vec![Parameter::Integer(1), Parameter::Integer(2)].into())
///     }
/// )
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub name: String,
    pub parameter: Box<Parameter>,
}
derive_ast_from_str!(Record, parser::exchange::simple_record);

/// A set of [Record] mapping to complex entity instance,
/// e.g. `(A(1) B(2.0) C("3"))`
#[derive(Debug, Clone, PartialEq)]
pub struct SubSuperRecord(pub Vec<Record>);
derive_ast_from_str!(SubSuperRecord, parser::exchange::subsuper_record);

/// `DATA` section in STEP file
///
/// ```
/// use ruststep::ast::DataSection;
/// use std::str::FromStr;
///
/// let input = r#"
/// DATA;
///   #1 = A(1.0, 2.0);
///   #2 = B(3.0, A((4.0, 5.0)));
///   #3 = B(6.0, #1);
/// ENDSEC;
/// "#;
/// let data_section = DataSection::from_str(input).unwrap();
/// dbg!(data_section);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DataSection {
    /// Metadata
    pub meta: Vec<Parameter>,
    /// Each lines in data section
    pub entities: Vec<EntityInstance>,
}
derive_ast_from_str!(DataSection, parser::exchange::data_section);

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
    /// # use std::str::FromStr;
    /// # use ruststep::ast::Parameter;
    /// let p = Parameter::from_str("FILE_NAME('ruststep')").unwrap();
    /// assert!(matches!(p, Parameter::Typed(_)));
    /// ```
    Typed(Record),

    /// Signed integer
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use ruststep::ast::Parameter;
    /// let p = Parameter::from_str("10").unwrap();
    /// assert_eq!(p, Parameter::Integer(10));
    /// let p = Parameter::from_str("-10").unwrap();
    /// assert_eq!(p, Parameter::Integer(-10));
    /// ```
    #[from]
    Integer(i64),

    /// Real number
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use ruststep::ast::Parameter;
    /// let p = Parameter::from_str("1.0").unwrap();
    /// assert_eq!(p, Parameter::Real(1.0));
    /// ```
    #[from]
    Real(f64),

    /// string literal
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use ruststep::ast::Parameter;
    /// let p = Parameter::from_str("'EXAMPLE STRING'").unwrap();
    /// assert_eq!(p, Parameter::String("EXAMPLE STRING".to_string()));
    /// ```
    #[from]
    String(String),

    /// Enumeration defined in EXPRESS schema, like `.TRUE.`
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use ruststep::ast::Parameter;
    /// let p = Parameter::from_str(".TRUE.").unwrap();
    /// assert_eq!(p, Parameter::Enumeration("TRUE".to_string()));
    /// ```
    Enumeration(String),

    /// List of parameters. This can be non-uniform.
    ///
    /// ```
    /// # use std::str::FromStr;
    /// # use ruststep::ast::Parameter;
    /// let p = Parameter::from_str("(1.0, 2, 'STRING')").unwrap();
    /// assert_eq!(p, Parameter::List(vec![
    ///   Parameter::Real(1.0),
    ///   Parameter::Integer(2),
    ///   Parameter::String("STRING".to_string()),
    /// ]));
    /// ```
    #[from]
    List(Vec<Parameter>),

    /// A reference to entity or value
    #[from]
    Ref(Name),

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

derive_ast_from_str!(Parameter, parser::exchange::parameter);

/// Entire exchange structure
#[derive(Debug, Clone, PartialEq)]
pub struct Exchange {
    /// `HEADER` section
    pub header: Vec<Record>,
    /// `ANCHOR` section
    pub anchor: Vec<Anchor>,
    /// `REFERENCE` section
    pub reference: Vec<ReferenceEntry>,
    /// `DATA` section
    pub data: Vec<DataSection>,
    /// `SIGNATURE` section
    pub signature: Vec<String>,
}
derive_ast_from_str!(Exchange, parser::exchange::exchange_file);

/// Each line of data section
#[derive(Debug, Clone, PartialEq)]
pub enum EntityInstance {
    Simple { id: u64, record: Record },
    Complex { id: u64, subsuper: SubSuperRecord },
}
derive_ast_from_str!(EntityInstance, parser::exchange::entity_instance);

#[derive(Debug, Clone, PartialEq)]
pub struct ReferenceEntry {
    pub name: Name,
    pub resource: URI,
}
derive_ast_from_str!(ReferenceEntry, parser::exchange::reference);

#[derive(Debug, Clone, PartialEq)]
pub struct URI(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Anchor {
    pub name: String,
    pub item: AnchorItem,
    pub tags: Vec<(String, AnchorItem)>,
}
derive_ast_from_str!(Anchor, parser::exchange::anchor);

#[derive(Debug, Clone, PartialEq)]
pub enum AnchorItem {
    Integer(i64),
    Real(f64),
    String(String),
    Enumeration(String),
    /// The special token dollar sign (`$`) is used to represent an object whose value is not provided in the exchange structure.
    NotProvided,
    /// A reference to entity or value
    Name(Name),
    /// List of other parameters
    List(Vec<AnchorItem>),
}
derive_ast_from_str!(AnchorItem, parser::exchange::anchor_item);
