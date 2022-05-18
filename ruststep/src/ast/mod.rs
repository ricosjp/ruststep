//! Abstract syntax tree for exchange structure
//!
//! This module contains implementation of [serde::Serialize] and [serde::Deserialize]
//! for AST structs.
//!
//! Deserialize
//! ------------
//!
//! [Implementing a Deserializer](https://serde.rs/impl-deserializer.html) page of [serde manual](https://serde.rs/) says
//! > The deserializer is responsible for mapping the input data
//! > into [Serde's data model](https://serde.rs/data-model.html) by invoking exactly one of the methods
//! > on the Visitor that it receives.
//!
//! [serde::de::Deserializer] trait is implemented for [Parameter],
//! [Record], and [SubSuperRecord].
//! Be sure that this mapping is not only for espr-generated structs.
//! This can be used with other Rust structs using `serde_derive::Deserialize` custom derive:
//!
//! ```text
//! ┌────────────────────┐
//! │ Exchange Structure │
//! └─┬──────────────────┘
//!   │ Deserialier trait  ◄── Implemented here
//! ┌─▼────────────────┐
//! │ serde data model │
//! └─┬────┬───────────┘
//!   │    │ ruststep_derive::Deserialize
//!   │ ┌──▼─────────────────────────┐
//!   │ │ espr-generated Rust struct │
//!   │ └────────────────────────────┘
//!   │ serde_derive::Deserialize
//! ┌─▼─────────────────┐
//! │ Other Rust struct │
//! └───────────────────┘
//! ```

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
/// FromStr
/// --------
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
///         parameter: vec![Parameter::Integer(1), Parameter::Integer(2)].into(),
///     }
/// )
/// ```
///
/// Deserialize as a map
/// ---------------------
///
/// [serde::Deserializer] implementation for [Record] provides
/// a mapping it into "map" in serde data model.
/// The keyword is mapped into the key and the parameters are its value:
///
/// ```
/// use std::{str::FromStr, collections::HashMap};
/// use ruststep::ast::*;
/// use serde::Deserialize;
///
/// let p = Record::from_str("DATA_KEYWORD(1, 2)").unwrap();
///
/// // Map can be deserialize as a hashmap
/// assert_eq!(
///     HashMap::<String, Vec<i32>>::deserialize(&p).unwrap(),
///     maplit::hashmap! {
///         "DATA_KEYWORD".to_string() => vec![1, 2]
///     }
/// );
///
/// // Map in serde can be interpreted as Rust field
/// #[derive(Debug, Clone, PartialEq, Deserialize)]
/// struct X {
///     #[serde(rename = "DATA_KEYWORD")]
///     a: Vec<i32>,
/// }
/// assert_eq!(
///     X::deserialize(&p).unwrap(),
///     X { a: vec![1, 2] }
/// );
/// ```
///
/// Mapping to simple instance
/// ---------------------------
///
/// It is deserialized as a "struct" only when the hint function
/// [serde::Deserializer::deserialize_struct] is called
/// and the struct name matches to its keyword appears in the exchange structure.
/// See [the manual of container attribute in serde](https://serde.rs/container-attrs.html)
/// for detail.
///
/// ```
/// use std::{str::FromStr, collections::HashMap};
/// use ruststep::ast::*;
/// use serde::Deserialize;
///
/// let p = Record::from_str("DATA_KEYWORD(1, 2)").unwrap();
///
/// #[derive(Debug, Clone, PartialEq, Deserialize)]
/// #[serde(rename = "DATA_KEYWORD")] // keyword matches
/// struct A {
///     x: i32,
///     y: i32,
/// }
/// assert_eq!(
///     A::deserialize(&p).unwrap(),
///     A { x: 1, y: 2 }
/// );
///
/// #[derive(Debug, Clone, PartialEq, Deserialize)]
/// #[serde(rename = "ANOTHER_KEYWORD")] // keyword does not match
/// struct B {
///     x: i32,
///     y: i32,
/// }
/// assert!(B::deserialize(&p).is_err());
/// ```
///
/// Internal mapping to complex entity instance
/// --------------------------------------------
///
/// Complex entity in EXPRESS language is
/// a set of two or more primitive component called partial complex entity.
///
/// ```text
/// ENTITY person;
///   name: STRING;
/// END_ENTITY;
///
/// ENTITY employee SUBTYPE OF (person);
///   pay: INTEGER;
/// END_ENTITY;
///
/// ENTITY student SUBTYPE OF (person);
///   school_name: STRING;
/// END_ENTITY;
/// ```
///
/// In this EXPRESS schema, a complex entity of `person` can have three components
/// representing `person`, `employee`, and `student`.
/// There are two way of mapping it from an exchange structure.
/// The internal mapping looks like usual case:
///
/// ```text
/// #1 = EMPLOYEE('Hitori Goto', 10);
/// #2 = STUDENT('Ikuno Kita', 'Shuka');
/// ```
///
/// `#1` has two parameters while `employee` definition has a field `pay`.
/// These parameters are consumed by supertype `person` first,
/// and then subtype `employee` consumes:
///
/// ```text
/// #1 = EMPLOYEE('Hitori Goto', 10);
///               ▲              ▲
///               │              └─ map to pay in employee
///               └─ map to name in person
/// ```
///
/// Internal mapping cannot handle the case where
/// both `employee` and `student` components co-exist.
/// This case will be handled by external mapping using [SubSuperRecord].
///
/// The detail of internal mapping is defined in 12.2.5.2 "Internal mapping"
/// of [ISO-10303-21](https://www.iso.org/standard/63141.html).
///
/// In terms of serde data model, [Record] is not self-describing
/// when using internal mapping rule.
/// Structs using internal mapping should implement [serde::Deserialize]
/// as described in subtype-supertype constraint in EXPRESS schema.
///
#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub name: String,
    pub parameter: Parameter,
}
derive_ast_from_str!(Record, parser::exchange::simple_record);

/// A set of [Record] mapping to complex entity instance,
/// e.g. `(A(1) B(2.0) C("3"))`
///
/// FromStr
/// --------
///
/// ```
/// use ruststep::ast::*;
/// use std::str::FromStr;
///
/// let record = SubSuperRecord::from_str("(A(1, 2) B(3, 4))").unwrap();
/// assert_eq!(
///     record,
///     SubSuperRecord(vec![
///         Record {
///             name: "A".to_string(),
///             parameter: vec![
///                 Parameter::Integer(1),
///                 Parameter::Integer(2)
///             ].into(),
///         },
///         Record {
///             name: "B".to_string(),
///             parameter: vec![
///                 Parameter::Integer(3),
///                 Parameter::Integer(4)
///             ].into(),
///         }
///     ])
/// )
/// ```
///
/// Deserialize as a map
/// ---------------------
///
/// Similar to [Record], [SubSuperRecord] can be deserialized as a "map":
///
/// ```
/// use std::{str::FromStr, collections::HashMap};
/// use ruststep::ast::*;
/// use serde::Deserialize;
///
/// let p = SubSuperRecord::from_str("(A(1, 2) B(3, 4))").unwrap();
///
/// // Map can be deserialize as a hashmap
/// assert_eq!(
///     HashMap::<String, Vec<i32>>::deserialize(&p).unwrap(),
///     maplit::hashmap! {
///         "A".to_string() => vec![1, 2],
///         "B".to_string() => vec![3, 4],
///     }
/// );
///
/// // Map in serde can be interpreted as Rust field
/// #[derive(Debug, Clone, PartialEq, Deserialize)]
/// struct X {
///     #[serde(rename = "A")]
///     a: Vec<i32>,
///     #[serde(rename = "B")]
///     b: Vec<i32>,
/// }
/// assert_eq!(
///     X::deserialize(&p).unwrap(),
///     X {
///         a: vec![1, 2],
///         b: vec![3, 4]
///     }
/// );
/// ```
///
/// External mapping to complex entity instance
/// --------------------------------------------
///
/// As discussed in [Record] for internal mapping,
///
/// ```text
/// ENTITY person;
///   name: STRING;
/// END_ENTITY;
///
/// ENTITY employee SUBTYPE OF (person);
///   pay: INTEGER;
/// END_ENTITY;
///
/// ENTITY student SUBTYPE OF (person);
///   school_name: STRING;
/// END_ENTITY;
/// ```
///
/// The instance of `person` may have three partial complex entity.
/// External mapping has different looks in exchange structure:
///
/// ```text
/// #3 = (PERSON('Hitori Goto') EMPLOYEE(10));
/// #4 = (PERSON('Ikuno Kita') STUDENT('Shuka));
/// #5 = (PERSON('Nizika Iziti') EMPLOYEE(15) STUDENT('Simokitazawa'))
/// ```
///
/// Each components enclosed by `()` corresponds to a partial entity instance,
/// which consists of fields declared in its entity definition.
///
/// ```text
/// #5 = (PERSON('Nizika Iziti') EMPLOYEE(15) STUDENT('Simokitazawa'))
///              ▲                        ▲           ▲
///              │                        │           └─ school_name in student
///              │                        └─ pay in employee
///              └─ name in person
/// ```
///
/// The detail of external mapping is defined in 12.2.5.3 "External mapping"
/// of [ISO-10303-21](https://www.iso.org/standard/63141.html).
///
/// [SubSuperRecord] is not self-describing because
/// EXPRESS does not defines memory layout of complex entities.
///
#[derive(Debug, Clone, PartialEq)]
pub struct SubSuperRecord(pub Vec<Record>);
derive_ast_from_str!(SubSuperRecord, parser::exchange::subsuper_record);

impl IntoIterator for SubSuperRecord {
    type Item = Record;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl<'a> IntoIterator for &'a SubSuperRecord {
    type Item = &'a Record;
    type IntoIter = std::slice::Iter<'a, Record>;
    fn into_iter(self) -> Self::IntoIter {
        self.0.iter()
    }
}

impl FromIterator<Record> for SubSuperRecord {
    fn from_iter<I: IntoIterator<Item = Record>>(iter: I) -> Self {
        Self(iter.into_iter().collect())
    }
}

impl<'a> FromIterator<&'a Record> for SubSuperRecord {
    fn from_iter<I: IntoIterator<Item = &'a Record>>(iter: I) -> Self {
        Self(iter.into_iter().cloned().collect())
    }
}

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
/// let a = Parameter::Typed {
///     keyword: "A".to_string(),
///     parameter: Box::new(vec![Parameter::real(2.0), Parameter::real(3.0)].into()),
/// };
///
/// // B((1.0, a))
/// let b = Parameter::Typed {
///     keyword: "B".to_string(),
///     parameter: Box::new(vec![Parameter::real(1.0), a].into()),
/// };
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
/// Deserialize
/// ------------
///
/// | Parameter   | serde data model |
/// |:------------|:-----------------|
/// | Integer     | i64              |
/// | Real        | f64              |
/// | String      | string           |
/// | List        | seq              |
/// | NotProvided | option (always none)|
/// | Omitted     | option (always none)|
/// | Enumeration | unit_variant (through [serde::de::value::StringDeserializer])|
/// | Typed       | map (through [de::RecordDeserializer])|
/// | Ref         | newtype_variant  |
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
    /// FromStr
    /// --------
    /// ```
    /// use std::str::FromStr;
    /// use ruststep::ast::Parameter;
    ///
    /// let p = Parameter::from_str("FILE_NAME('ruststep')").unwrap();
    /// assert!(matches!(p, Parameter::Typed { .. }));
    /// ```
    ///
    /// Deserialize
    /// ------------
    /// ```
    /// use std::{str::FromStr, collections::HashMap};
    /// use ruststep::ast::*;
    /// use serde::Deserialize;
    ///
    /// // Regarded as a map `{ "A": [1, 2] }` in serde data model
    /// let p = Parameter::from_str("A((1, 2))").unwrap();
    ///
    /// // Map can be deserialize as a hashmap
    /// assert_eq!(
    ///     HashMap::<String, Vec<i32>>::deserialize(&p).unwrap(),
    ///     maplit::hashmap! {
    ///         "A".to_string() => vec![1, 2]
    ///     }
    /// );
    ///
    /// // Map in serde can be interpreted as Rust field
    /// #[derive(Debug, Clone, PartialEq, Deserialize)]
    /// struct X {
    ///     #[serde(rename = "A")]
    ///     a: Vec<i32>,
    /// }
    /// assert_eq!(X::deserialize(&p).unwrap(), X { a: vec![1, 2] });
    /// ```
    ///
    /// Different from [Record], deserializing into a struct is not supported:
    ///
    /// ```
    /// use std::{str::FromStr, collections::HashMap};
    /// use ruststep::ast::*;
    /// use serde::Deserialize;
    ///
    /// let p = Parameter::from_str("A(1)").unwrap();
    ///
    /// #[derive(Debug, Clone, PartialEq, Deserialize)]
    /// struct A {
    ///     x: i32,
    /// }
    /// assert!(A::deserialize(&p).is_err());
    /// ```
    ///
    Typed {
        keyword: String,
        parameter: Box<Parameter>,
    },

    /// Signed integer
    ///
    /// FromStr
    /// --------
    /// ```
    /// use std::str::FromStr;
    /// use ruststep::ast::Parameter;
    ///
    /// let p = Parameter::from_str("10").unwrap();
    /// assert_eq!(p, Parameter::Integer(10));
    ///
    /// let p = Parameter::from_str("-10").unwrap();
    /// assert_eq!(p, Parameter::Integer(-10));
    /// ```
    ///
    /// Deserialize
    /// ------------
    /// ```
    /// use ruststep::ast::*;
    /// use serde::Deserialize;
    ///
    /// let p = Parameter::Integer(2);
    /// let a = i64::deserialize(&p).unwrap();
    /// assert_eq!(a, 2);
    ///
    /// // can be deserialized as unsigned
    /// let a = u64::deserialize(&p).unwrap();
    /// assert_eq!(a, 2);
    ///
    /// // cannot be deserialized negative integer into unsigned
    /// let p = Parameter::Integer(-2);
    /// let a = i64::deserialize(&p).unwrap();
    /// assert_eq!(a, -2);
    /// assert!(u64::deserialize(&p).is_err());
    /// ```
    #[from]
    Integer(i64),

    /// Real number
    ///
    /// FromStr
    /// --------
    /// ```
    /// use std::str::FromStr;
    /// use ruststep::ast::Parameter;
    ///
    /// let p = Parameter::from_str("1.0").unwrap();
    /// assert_eq!(p, Parameter::Real(1.0));
    /// ```
    #[from]
    Real(f64),

    /// string literal
    ///
    /// FromStr
    /// --------
    /// ```
    /// use std::str::FromStr;
    /// use ruststep::ast::Parameter;
    ///
    /// let p = Parameter::from_str("'EXAMPLE STRING'").unwrap();
    /// assert_eq!(p, Parameter::String("EXAMPLE STRING".to_string()));
    /// ```
    #[from]
    String(String),

    /// Enumeration defined in EXPRESS schema, like `.TRUE.`
    ///
    /// FromStr
    /// --------
    /// ```
    /// # use std::str::FromStr;
    /// # use ruststep::ast::Parameter;
    /// let p = Parameter::from_str(".TRUE.").unwrap();
    /// assert_eq!(p, Parameter::Enumeration("TRUE".to_string()));
    /// ```
    ///
    /// Deserialize
    /// ------------
    /// ```
    /// use ruststep::ast::*;
    /// use serde::Deserialize;
    /// use std::str::FromStr;
    ///
    /// let p = Parameter::from_str(".A.").unwrap();
    ///
    /// #[derive(Debug, PartialEq, Deserialize)]
    /// enum E {
    ///   A,
    ///   B
    /// }
    /// assert_eq!(E::deserialize(&p).unwrap(), E::A);
    /// ```
    ///
    Enumeration(String),

    /// List of parameters. This can be non-uniform.
    ///
    /// FromStr
    /// --------
    /// ```
    /// use std::str::FromStr;
    /// use ruststep::ast::Parameter;
    ///
    /// let p = Parameter::from_str("(1.0, 2, 'STRING')").unwrap();
    /// assert_eq!(p, Parameter::List(vec![
    ///   Parameter::Real(1.0),
    ///   Parameter::Integer(2),
    ///   Parameter::String("STRING".to_string()),
    /// ]));
    /// ```
    ///
    /// Deserialize
    /// ------------
    /// ```
    /// use std::str::FromStr;
    /// use ruststep::ast::*;
    /// use serde::Deserialize;
    ///
    /// let p = Parameter::from_str("(1, 2, 3)").unwrap();
    ///
    /// // As Vec<i32>
    /// let a = Vec::<i32>::deserialize(&p).unwrap();
    /// assert_eq!(a, vec![1, 2, 3]);
    ///
    /// // As user-defined struct
    /// #[derive(Debug, Clone, PartialEq, Deserialize)]
    /// struct A {
    ///     x: i32,
    ///     y: i32,
    ///     z: i32,
    /// }
    /// let a = A::deserialize(&p).unwrap();
    /// assert_eq!(a, A { x: 1, y: 2, z: 3 });
    /// ```
    #[from]
    List(Vec<Parameter>),

    /// A reference to entity or value
    ///
    /// Deserialize
    /// ------------
    /// ```
    /// use ruststep::ast::*;
    /// use serde::Deserialize;
    /// use std::str::FromStr;
    ///
    /// let p = Parameter::from_str("#12").unwrap();
    ///
    /// #[derive(Debug, PartialEq, Deserialize)]
    /// enum Id {
    ///   #[serde(rename = "Entity")] // "Entity" is keyword for entity reference
    ///   E(usize),
    ///   #[serde(rename = "Value")]  // "Value" is keyword for value reference
    ///   V(usize),
    /// }
    /// assert_eq!(Id::deserialize(&p).unwrap(), Id::E(12));
    /// ```
    #[from]
    Ref(Name),

    /// The special token dollar sign (`$`) is used to represent
    /// an object whose value is not provided in the exchange structure.
    ///
    /// Deserialize
    /// -----------
    /// ```
    /// use ruststep::ast::*;
    /// use serde::Deserialize;
    ///
    /// let p = Parameter::NotProvided;
    /// assert_eq!(Option::<i64>::deserialize(&p).unwrap(), None);
    /// ```
    NotProvided,

    /// Omitted parameter denoted by `*`
    ///
    /// Deserialize
    /// ------------
    /// ```
    /// use ruststep::ast::*;
    /// use serde::Deserialize;
    ///
    /// let p = Parameter::Omitted;
    /// assert_eq!(Option::<i64>::deserialize(&p).unwrap(), None);
    /// ```
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
