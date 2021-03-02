use std::ops::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Logical {
    False,
    Unknown,
    True,
}

impl Default for Logical {
    /// Returns `Logical::Unknown`.
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
    /// ```
    /// use espr_runtime::Logical;
    /// assert_eq!(Option::<bool>::from(Logical::False), Some(false));
    /// assert_eq!(Option::<bool>::from(Logical::Unknown), None);
    /// assert_eq!(Option::<bool>::from(Logical::True), Some(true));
    /// ```
    fn from(l: Logical) -> Option<bool> {
        match l {
            Logical::Unknown => None,
            _ => Some(l == Logical::True),
        }
    }
}

impl BitAnd for Logical {
    type Output = Self;
    #[inline(always)]
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

impl BitAnd<&Logical> for Logical {
    type Output = Logical;
    #[inline(always)]
    fn bitand(self, other: &Logical) -> Logical {
        self & *other
    }
}

impl BitAnd<Logical> for &Logical {
    type Output = Logical;
    #[inline(always)]
    fn bitand(self, other: Logical) -> Logical {
        *self & other
    }
}

impl BitAnd for &Logical {
    type Output = Logical;
    #[inline(always)]
    fn bitand(self, other: &Logical) -> Logical {
        *self & *other
    }
}

impl BitOr for Logical {
    type Output = Self;
    #[inline(always)]
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

impl BitOr<&Logical> for Logical {
    type Output = Logical;
    #[inline(always)]
    fn bitor(self, other: &Logical) -> Logical {
        self | *other
    }
}

impl BitOr<Logical> for &Logical {
    type Output = Logical;
    #[inline(always)]
    fn bitor(self, other: Logical) -> Logical {
        *self | other
    }
}

impl BitOr for &Logical {
    type Output = Logical;
    #[inline(always)]
    fn bitor(self, other: &Logical) -> Logical {
        *self | *other
    }
}

impl BitXor for Logical {
    type Output = Logical;
    #[inline(always)]
    fn bitxor(self, other: Self) -> Logical {
        match (self, other) {
            (Logical::Unknown, _) => Logical::Unknown,
            (_, Logical::Unknown) => Logical::Unknown,
            (_, _) => (self != other).into(),
        }
    }
}

impl BitXor<&Logical> for Logical {
    type Output = Logical;
    #[inline(always)]
    fn bitxor(self, other: &Logical) -> Logical {
        self ^ *other
    }
}

impl BitXor<Logical> for &Logical {
    type Output = Logical;
    #[inline(always)]
    fn bitxor(self, other: Logical) -> Logical {
        *self ^ other
    }
}

impl BitXor for &Logical {
    type Output = Logical;
    #[inline(always)]
    fn bitxor(self, other: &Logical) -> Logical {
        *self ^ *other
    }
}

impl Not for Logical {
    type Output = Logical;
    #[inline(always)]
    fn not(self) -> Logical {
        match self {
            Logical::True => Logical::False,
            Logical::Unknown => Logical::Unknown,
            Logical::False => Logical::True,
        }
    }
}

impl Not for &Logical {
    type Output = Logical;
    #[inline(always)]
    fn not(self) -> Logical {
        !*self
    }
}

#[test]
fn and_test() {
    assert_eq!(Logical::True & Logical::True, Logical::True);
    assert_eq!(&Logical::True & Logical::Unknown, Logical::Unknown);
    assert_eq!(Logical::True & &Logical::False, Logical::False);
    assert_eq!(&Logical::Unknown & &Logical::True, Logical::Unknown);
    assert_eq!(Logical::Unknown & Logical::Unknown, Logical::Unknown);
    assert_eq!(&Logical::Unknown & Logical::False, Logical::False);
    assert_eq!(Logical::False & &Logical::True, Logical::False);
    assert_eq!(&Logical::False & &Logical::Unknown, Logical::False);
    assert_eq!(Logical::False & Logical::False, Logical::False);
}

#[test]
fn or_test() {
    assert_eq!(Logical::True | Logical::True, Logical::True);
    assert_eq!(&Logical::True | Logical::Unknown, Logical::True);
    assert_eq!(Logical::True | &Logical::False, Logical::True);
    assert_eq!(&Logical::Unknown | &Logical::True, Logical::True);
    assert_eq!(Logical::Unknown | Logical::Unknown, Logical::Unknown);
    assert_eq!(&Logical::Unknown | Logical::False, Logical::Unknown);
    assert_eq!(Logical::False | &Logical::True, Logical::True);
    assert_eq!(&Logical::False | &Logical::Unknown, Logical::Unknown);
    assert_eq!(Logical::False | Logical::False, Logical::False);
}

#[test]
fn xor_test() {
    assert_eq!(Logical::True ^ Logical::True, Logical::False);
    assert_eq!(&Logical::True ^ Logical::Unknown, Logical::Unknown);
    assert_eq!(Logical::True ^ &Logical::False, Logical::True);
    assert_eq!(&Logical::Unknown ^ &Logical::True, Logical::Unknown);
    assert_eq!(Logical::Unknown ^ Logical::Unknown, Logical::Unknown);
    assert_eq!(&Logical::Unknown ^ Logical::False, Logical::Unknown);
    assert_eq!(Logical::False ^ &Logical::True, Logical::True);
    assert_eq!(&Logical::False ^ &Logical::Unknown, Logical::Unknown);
    assert_eq!(Logical::False ^ Logical::False, Logical::False);
}

#[test]
fn not_test() {
    assert_eq!(!Logical::True, Logical::False);
    assert_eq!(!Logical::Unknown, Logical::Unknown);
    assert_eq!(!&Logical::False, Logical::True);
}
