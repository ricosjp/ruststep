//! Partial complex entities described in ISO-10303-11 Annex B

use itertools::Itertools;

/// Partial complex entity data type, e.g. `A & B & C` in ISO document
///
/// Each component `A` will be represented by an index.
/// `&` operation corresponds to `BitAnd`
///
/// `A & A == A`
///
/// ```
/// # use espr::ir::*;
/// let a = PartialComplexEntity::new(&[1]);
/// assert_eq!(a.clone() & a.clone(), a);
/// ```
///
/// `A & B == B & A`
///
/// ```
/// # use espr::ir::*;
/// let a = PartialComplexEntity::new(&[1]);
/// let b = PartialComplexEntity::new(&[2]);
/// assert_eq!(a.clone() & b.clone(), b & a);
/// ```
///
/// `A & (B & C) == (A & B) & C == A & B & C`
///
/// ```
/// # use espr::ir::*;
/// let a = PartialComplexEntity::new(&[1]);
/// let b = PartialComplexEntity::new(&[3]);
/// let c = PartialComplexEntity::new(&[2]);
/// assert_eq!(
///     (a.clone() & b.clone()) & c.clone(),
///      a.clone() & (b.clone() & c.clone())
/// );
/// assert_eq!(a & b & c, PartialComplexEntity::new(&[1, 2, 3]));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PartialComplexEntity {
    /// Sorted and non-duplicated indices
    indices: Vec<usize>,
}

impl PartialComplexEntity {
    pub fn new(indices: &[usize]) -> Self {
        PartialComplexEntity {
            indices: indices.iter().cloned().dedup().collect(),
        }
    }
}

impl From<&[usize]> for PartialComplexEntity {
    fn from(indices: &[usize]) -> PartialComplexEntity {
        PartialComplexEntity::new(indices)
    }
}

impl std::ops::BitAnd for PartialComplexEntity {
    type Output = Self;
    fn bitand(mut self, mut rhs: Self) -> Self {
        self.indices.append(&mut rhs.indices);
        self.indices.sort_unstable();
        PartialComplexEntity {
            indices: self.indices.into_iter().dedup().collect(),
        }
    }
}

/// Complex entity, a list of [PartialComplexEntity]
///
/// ```
/// # use espr::ir::*;
/// let ce = ComplexEntity::new(&[PartialComplexEntity::new(&[1]), PartialComplexEntity::new(&[2])]);
/// ```
///
/// `[A, A & B, A & C, A & B & D, B & C, D] / A = [A, A & B, A & C, A & B & D]`
///
/// ```
/// # use espr::ir::*;
/// let a = PartialComplexEntity::new(&[1]);
/// let b = PartialComplexEntity::new(&[2]);
/// let c = PartialComplexEntity::new(&[3]);
/// let d = PartialComplexEntity::new(&[4]);
///
/// let ce = ComplexEntity::new(&[
///   a.clone(),
///   a.clone() & b.clone(),
///   a.clone() & c.clone(),
///   a.clone() & b.clone() & d.clone(),
///   b.clone() & c.clone(),
///   d.clone()
/// ]);
///
/// assert_eq!(ce / a.clone(), ComplexEntity::new(&[
///   a.clone(),
///   a.clone() & b.clone(),
///   a.clone() & c.clone(),
///   a.clone() & b.clone() & d.clone(),
/// ]));
/// ```
///
#[derive(Debug, PartialEq, Eq)]
pub struct ComplexEntity {
    /// Sorted and non-duplicated list of partial complex entities
    parts: Vec<PartialComplexEntity>,
}

impl ComplexEntity {
    pub fn new(parts: &[PartialComplexEntity]) -> Self {
        parts.iter().collect()
    }
}

impl<'a> FromIterator<&'a PartialComplexEntity> for ComplexEntity {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = &'a PartialComplexEntity>,
    {
        Self {
            parts: iter.into_iter().cloned().sorted().dedup().collect(),
        }
    }
}

// [A, B] + [C, D] = [A, B, C, D]
impl std::ops::Add for ComplexEntity {
    type Output = Self;
    fn add(mut self, mut rhs: ComplexEntity) -> Self {
        self.parts.append(&mut rhs.parts);
        self.parts.sort_unstable();
        self
    }
}

// [A, B] + C = [A, B, C]
impl std::ops::Add<PartialComplexEntity> for ComplexEntity {
    type Output = Self;
    fn add(mut self, rhs: PartialComplexEntity) -> Self {
        self.parts.push(rhs);
        self.parts.sort_unstable();
        self
    }
}

// A + [B, C] = [A, B, C]
impl std::ops::Add<ComplexEntity> for PartialComplexEntity {
    type Output = ComplexEntity;
    fn add(self, rhs: ComplexEntity) -> ComplexEntity {
        rhs + self
    }
}

// [A, B] & [C, D] = [A & C, A & D, B & C, B & D]
impl std::ops::BitAnd for ComplexEntity {
    type Output = ComplexEntity;
    fn bitand(self, rhs: ComplexEntity) -> ComplexEntity {
        let mut parts = Vec::with_capacity(self.parts.len() * rhs.parts.len());
        for p in &self.parts {
            for q in &rhs.parts {
                parts.push(p.clone() & q.clone());
            }
        }
        ComplexEntity { parts }
    }
}

// [A, B] & C = [A & C, B & C]
impl std::ops::BitAnd<PartialComplexEntity> for ComplexEntity {
    type Output = ComplexEntity;
    fn bitand(self, q: PartialComplexEntity) -> ComplexEntity {
        ComplexEntity {
            parts: self
                .parts
                .into_iter()
                .map(|p| p & q.clone())
                .sorted()
                .dedup()
                .collect(),
        }
    }
}

// A & [B, C] = [A & B, A & C]
impl std::ops::BitAnd<ComplexEntity> for PartialComplexEntity {
    type Output = ComplexEntity;
    fn bitand(self, rhs: ComplexEntity) -> ComplexEntity {
        rhs & self
    }
}

// [A, A & B, A & C, A & B & D, B & C, D] / A = [A, A & B, A & C, A & B & D]
impl std::ops::Div<PartialComplexEntity> for ComplexEntity {
    type Output = ComplexEntity;
    fn div(self, rhs: PartialComplexEntity) -> ComplexEntity {
        ComplexEntity {
            parts: self
                .parts
                .into_iter()
                .filter(|part| {
                    for i in &rhs.indices {
                        if let Ok(_) = part.indices.binary_search(i) {
                            continue;
                        } else {
                            return false;
                        }
                    }
                    return true;
                })
                .collect(),
        }
    }
}
