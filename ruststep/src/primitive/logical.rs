use serde::Deserialize;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Parameter;

    #[test]
    fn deserialize_logical() {
        let p = Parameter::Enumeration("TRUE".to_string());
        let logical = Logical::deserialize(&p).unwrap();
        assert_eq!(logical, Logical::True);

        let p = Parameter::Enumeration("FALSE".to_string());
        let logical = Logical::deserialize(&p).unwrap();
        assert_eq!(logical, Logical::False);

        let p = Parameter::Enumeration("UNKNOWN".to_string());
        let logical = Logical::deserialize(&p).unwrap();
        assert_eq!(logical, Logical::Unknown);
    }
}
