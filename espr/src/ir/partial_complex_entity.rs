//! Partial complex entities described in ISO-10303-11 Annex B

/// Partial complex entity data type, e.g. `A & B & C` in ISO document
///
/// Each component `A` will be represented by an index.
/// `&` operation corresponds to `BitAnd`
///
/// ```
/// # use espr::ir::*;
/// let a = PartialComplexEntity::new(&[1]);
/// let b = PartialComplexEntity::new(&[3]);
/// let c = PartialComplexEntity::new(&[2]);
/// assert_eq!(a & b & c, PartialComplexEntity::new(&[1, 2, 3]));
/// ```
#[derive(Debug, PartialEq, Eq)]
pub struct PartialComplexEntity {
    /// Sorted and non-duplicated indices
    indices: Vec<usize>,
}

impl Into<PartialComplexEntity> for Vec<usize> {
    fn into(self) -> PartialComplexEntity {
        let mut pce = PartialComplexEntity { indices: self };
        pce.indices.sort();
        pce
    }
}

impl PartialComplexEntity {
    pub fn new(indices: &[usize]) -> Self {
        indices.to_vec().into()
    }
}

impl std::ops::BitAnd for PartialComplexEntity {
    type Output = Self;
    fn bitand(mut self, mut rhs: Self) -> Self {
        self.indices.append(&mut rhs.indices);
        self.indices.sort();
        self
    }
}

/// Complex entity, a list of partial complex entity.
#[derive(Debug, PartialEq, Eq)]
pub struct ComplexEntity {
    parts: Vec<PartialComplexEntity>,
}
