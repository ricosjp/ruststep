use serde::{Deserialize, Serialize};
use std::ops::*;

/// `LOGICAL` type
///
/// ```
/// use ruststep::primitive::Logical;
///
/// // Default
/// assert_eq!(Logical::default(), Logical::Unknown);
///
/// // From<bool>
/// assert_eq!(Logical::True, true.into());
/// assert_eq!(Logical::False, false.into());
///
/// // From<Option<bool>>
/// assert_eq!(Logical::True, Some(true).into());
/// assert_eq!(Logical::False, Some(false).into());
/// assert_eq!(Logical::Unknown, None.into());
///
/// // Not
/// assert_eq!(Logical::True, !Logical::False);
/// assert_eq!(Logical::False, !Logical::True);
/// assert_eq!(Logical::Unknown, !Logical::Unknown);
///
/// // BitAnd
/// assert_eq!(Logical::True & Logical::True, Logical::True);
/// assert_eq!(Logical::True & Logical::Unknown, Logical::Unknown);
/// assert_eq!(Logical::True & Logical::False, Logical::False);
/// assert_eq!(Logical::False & Logical::True, Logical::False);
/// assert_eq!(Logical::False & Logical::Unknown, Logical::False);
/// assert_eq!(Logical::False & Logical::False, Logical::False);
/// assert_eq!(Logical::Unknown & Logical::True, Logical::Unknown);
/// assert_eq!(Logical::Unknown & Logical::Unknown, Logical::Unknown);
/// assert_eq!(Logical::Unknown & Logical::False, Logical::False);
///
/// // BitOr
/// assert_eq!(Logical::True | Logical::True, Logical::True);
/// assert_eq!(Logical::True | Logical::Unknown, Logical::True);
/// assert_eq!(Logical::True | Logical::False, Logical::True);
/// assert_eq!(Logical::False | Logical::True, Logical::True);
/// assert_eq!(Logical::False | Logical::Unknown, Logical::Unknown);
/// assert_eq!(Logical::False | Logical::False, Logical::False);
/// assert_eq!(Logical::Unknown | Logical::True, Logical::True);
/// assert_eq!(Logical::Unknown | Logical::Unknown, Logical::Unknown);
/// assert_eq!(Logical::Unknown | Logical::False, Logical::Unknown);
///
/// // BitXor
/// assert_eq!(Logical::True ^ Logical::True, Logical::False);
/// assert_eq!(Logical::True ^ Logical::Unknown, Logical::Unknown);
/// assert_eq!(Logical::True ^ Logical::False, Logical::True);
/// assert_eq!(Logical::False ^ Logical::True, Logical::True);
/// assert_eq!(Logical::False ^ Logical::Unknown, Logical::Unknown);
/// assert_eq!(Logical::False ^ Logical::False, Logical::False);
/// assert_eq!(Logical::Unknown ^ Logical::True, Logical::Unknown);
/// assert_eq!(Logical::Unknown ^ Logical::Unknown, Logical::Unknown);
/// assert_eq!(Logical::Unknown ^ Logical::False, Logical::Unknown);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize)]
pub enum Logical {
    False,
    Unknown,
    True,
}

impl Default for Logical {
    fn default() -> Logical {
        Logical::Unknown
    }
}

impl std::fmt::Display for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Logical::True => f.pad("true"),
            Logical::Unknown => f.pad("unknown"),
            Logical::False => f.pad("false"),
        }
    }
}

impl From<bool> for Logical {
    fn from(b: bool) -> Logical {
        match b {
            true => Logical::True,
            false => Logical::False,
        }
    }
}

impl From<Option<bool>> for Logical {
    fn from(option: Option<bool>) -> Logical {
        match option {
            Some(b) => b.into(),
            None => Logical::Unknown,
        }
    }
}

impl From<Logical> for Option<bool> {
    fn from(l: Logical) -> Option<bool> {
        match l {
            Logical::Unknown => None,
            _ => Some(l == Logical::True),
        }
    }
}

impl BitAnd for Logical {
    type Output = Self;
    fn bitand(self, other: Self) -> Self {
        match (self, other) {
            (Logical::False, _) => Logical::False,
            (_, Logical::False) => Logical::False,
            (Logical::Unknown, _) => Logical::Unknown,
            (_, Logical::Unknown) => Logical::Unknown,
            (Logical::True, Logical::True) => Logical::True,
        }
    }
}

impl BitOr for Logical {
    type Output = Self;
    fn bitor(self, other: Self) -> Self {
        match (self, other) {
            (Logical::True, _) => Logical::True,
            (_, Logical::True) => Logical::True,
            (Logical::Unknown, _) => Logical::Unknown,
            (_, Logical::Unknown) => Logical::Unknown,
            (Logical::False, Logical::False) => Logical::False,
        }
    }
}

impl BitXor for Logical {
    type Output = Logical;
    fn bitxor(self, other: Self) -> Logical {
        match (self, other) {
            (Logical::Unknown, _) => Logical::Unknown,
            (_, Logical::Unknown) => Logical::Unknown,
            (_, _) => (self != other).into(),
        }
    }
}

impl Not for Logical {
    type Output = Logical;
    fn not(self) -> Logical {
        match self {
            Logical::True => Logical::False,
            Logical::Unknown => Logical::Unknown,
            Logical::False => Logical::True,
        }
    }
}

enum SubLogical {
    T,
    True,
    F,
    False,
    U,
    Unknown,
}

struct SubVisitor;

const VARIANTS: &[&str] = &["T", "True", "F", "False", "U", "Unknown"];

impl<'de> serde::de::Visitor<'de> for SubVisitor {
    type Value = SubLogical;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "Logical")
    }
    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "T" => Ok(SubLogical::T),
            "True" => Ok(SubLogical::True),
            "F" => Ok(SubLogical::F),
            "False" => Ok(SubLogical::False),
            "U" => Ok(SubLogical::U),
            "Unknown" => Ok(SubLogical::Unknown),
            _ => Err(E::unknown_field(v, VARIANTS)),
        }
    }
    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            b"T" => Ok(SubLogical::T),
            b"True" => Ok(SubLogical::True),
            b"F" => Ok(SubLogical::F),
            b"False" => Ok(SubLogical::False),
            b"U" => Ok(SubLogical::U),
            b"Unknown" => Ok(SubLogical::Unknown),
            _ => Err(E::unknown_field(&String::from_utf8_lossy(v), VARIANTS)),
        }
    }
}

impl From<SubLogical> for Logical {
    fn from(value: SubLogical) -> Self {
        match value {
            SubLogical::T => Self::True,
            SubLogical::True => Self::True,
            SubLogical::F => Self::False,
            SubLogical::False => Self::False,
            SubLogical::U => Self::Unknown,
            SubLogical::Unknown => Self::Unknown,
        }
    }
}

impl<'de> Deserialize<'de> for SubLogical {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(SubVisitor)
    }
}

#[derive(Clone, Debug)]
struct Visitor;

impl<'de> serde::de::Visitor<'de> for Visitor {
    type Value = Logical;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "enum Logical")
    }
    fn visit_enum<A>(self, data: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::EnumAccess<'de>,
    {
        data.variant::<SubLogical>().map(|(x, _)| x.into())
    }
}

impl<'de> Deserialize<'de> for Logical {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_enum("SubLogical", VARIANTS, Visitor)
    }
}
