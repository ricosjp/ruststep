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
/// assert_eq!((a.clone() & b.clone()) & c.clone(), a.clone() & (b.clone() & c.clone()));
/// assert_eq!(a & b & c, PartialComplexEntity::new(&[1, 2, 3]));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
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

/// Complex entity, a list of partial complex entity.
#[derive(Debug, PartialEq, Eq)]
pub struct ComplexEntity {
    parts: Vec<PartialComplexEntity>,
}
