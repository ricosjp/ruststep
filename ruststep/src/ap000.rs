//! Manually generated schema definitions corresponding following EXPRESS Schema.
//!
//! This is for testing espr code generator.
//!
//! ```text
//! SCHEMA ap000;
//!   ENTITY a;
//!     x: REAL;
//!     y: REAL;
//!   END_ENTITY;
//!
//!   ENTITY b;
//!     z: REAL;
//!     w: a;
//!   END_ENTITY;
//!
//!   ENTITY c;
//!     p: a;
//!     q: b;
//!   END_ENTITY;
//! END_SCHEMA;
//! ```
//!

use crate::{
    error::*,
    parser::{
        exchange::{DataSection, EntityInstance},
        value::PlaceHolder,
    },
    tables::*,
};
use serde::Deserialize;
use std::collections::HashMap;

/// Tables containing entities with id
///
/// Examples
/// ---------
///
/// Convert STEP data section into tables.
///
/// ```
/// use ruststep::{parser::exchange, ap000::Ap000};
/// use nom::Finish;
///
/// let (_, data_section) = exchange::data_section(
///     r#"
///     DATA;
///       #2 = A(1.0, 2.0);
///       #4 = B(2.0, A((4.0, 5.0)));
///       #5 = B(2.0, #2);
///     ENDSEC;
///     "#
///     .trim(),
/// )
/// .finish()
/// .unwrap();
///
/// // Entity reference `#2` is not resolved at this point.
/// let table = Ap000::from_section(&data_section).unwrap();
///
/// for a in table.a_iter() { // Iterate over top-level `A`s.
///   dbg!(a);                // Do not iterate over the inline struct `A((4.0, 5.0))` in `#4`
/// }
///
/// for b in table.b_iter() { // Because reference lookup is done while iteration,
///   dbg!(b);                // `b` may be `Result::Err` if reference is undefined.
/// }
///
/// for c in table.c_iter() { // No iteration occurs since `C` is not defined
///   dbg!(c);
/// }
/// ```
///
/// STEP exchange structure AST is converted into Rust structure in two steps:
///
/// 1.  Parse AST to `*Holder` private struct without resolving entity references,
///    e.g. `B(2.0, #2)` will be converted into a Rust struct
///         `BHolder { z: 2.0, a: RValue::Entity(2)}`.
///     - `Ap000::from_section` as above example
/// 2. Resolve reference `#2` to `A(1.0, 2.0)`.
///    This will be done while the iteration.
///     - `a_iter()` as above example
///
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Ap000 {
    a: HashMap<u64, AHolder>,
    b: HashMap<u64, BHolder>,
    c: HashMap<u64, CHolder>,
}

impl Ap000 {
    pub fn from_section(sec: &DataSection) -> Result<Self> {
        let mut a = HashMap::new();
        let mut b = HashMap::new();
        let mut c = HashMap::new();

        for entity in &sec.entities {
            match entity {
                EntityInstance::Simple { name, record } => match record.name.as_str() {
                    "A" => a.insert(*name, AHolder::deserialize(record)?).is_none(),
                    "B" => b.insert(*name, BHolder::deserialize(record)?).is_none(),
                    "C" => c.insert(*name, CHolder::deserialize(record)?).is_none(),
                    _ => panic!(),
                },
                EntityInstance::Complex { .. } => unimplemented!(),
            };
        }
        Ok(Ap000 { a, b, c })
    }

    pub fn a_iter<'table>(&'table self) -> impl Iterator<Item = Result<A>> + 'table {
        self.a
            .values()
            .cloned()
            .map(move |value| value.into_owned(&self))
    }

    pub fn b_iter<'table>(&'table self) -> impl Iterator<Item = Result<B>> + 'table {
        self.b
            .values()
            .cloned()
            .map(move |value| value.into_owned(&self))
    }

    pub fn c_iter<'table>(&'table self) -> impl Iterator<Item = Result<C>> + 'table {
        self.c
            .values()
            .cloned()
            .map(move |value| value.into_owned(&self))
    }
}

impl EntityTable<AHolder> for Ap000 {
    fn get_entity(&self, id: u64) -> Result<&AHolder> {
        self.a.get_entity(id)
    }
}

impl EntityTable<BHolder> for Ap000 {
    fn get_entity(&self, id: u64) -> Result<&BHolder> {
        self.b.get_entity(id)
    }
}

impl EntityTable<CHolder> for Ap000 {
    fn get_entity(&self, id: u64) -> Result<&CHolder> {
        self.c.get_entity(id)
    }
}

/// Corresponds to `ENTITY a`
#[derive(Debug, Clone, PartialEq)]
pub struct A {
    pub x: f64,
    pub y: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct AHolder {
    x: f64,
    y: f64,
}

impl Holder for AHolder {
    type Table = Ap000;
    type Owned = A;
    fn into_owned(self, _tables: &Ap000) -> Result<A> {
        let AHolder { x, y } = self;
        Ok(A { x, y })
    }
}

/// Corresponds to `ENTITY b`
#[derive(Debug, Clone, PartialEq)]
pub struct B {
    pub z: f64,
    pub a: A,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct BHolder {
    z: f64,
    a: PlaceHolder<AHolder>,
}

impl Holder for BHolder {
    type Table = Ap000;
    type Owned = B;
    fn into_owned(self, tables: &Ap000) -> Result<B> {
        let BHolder { z, a } = self;
        Ok(B {
            z,
            a: a.into_owned(tables)?,
        })
    }
}

/// Corresponds to `ENTITY c`
#[derive(Debug, Clone, PartialEq)]
pub struct C {
    pub p: A,
    pub q: B,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
struct CHolder {
    p: PlaceHolder<AHolder>,
    q: PlaceHolder<BHolder>,
}

impl Holder for CHolder {
    type Table = Ap000;
    type Owned = C;
    fn into_owned(self, tables: &Ap000) -> Result<C> {
        let CHolder { p, q } = self;
        Ok(C {
            p: p.into_owned(tables)?,
            q: q.into_owned(tables)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::{exchange, value::RValue};
    use nom::Finish;

    #[test]
    fn a_from_record() {
        let (_, record) = exchange::simple_record("A(1.0, 2.0)").finish().unwrap();
        let a = AHolder::deserialize(&record).unwrap();
        dbg!(a);
    }

    // Example Tables generated by
    //
    // ```
    // DATA;
    //   #2 = A(1.0, 2.0);
    //   #4 = B(2.0, A((4.0, 5.0)));
    //   #5 = B(2.0, #2);
    // ENDSEC;
    // ```
    fn example_table() -> Ap000 {
        let mut tables = Ap000::default();
        tables.a.insert(2, AHolder { x: 1.0, y: 2.0 });
        tables.b.insert(
            4,
            BHolder {
                z: 2.0,
                a: PlaceHolder::Owned(AHolder { x: 4.0, y: 5.0 }),
            },
        );
        tables.b.insert(
            5,
            BHolder {
                z: 2.0,
                a: PlaceHolder::Ref(RValue::Entity(2)),
            },
        );
        tables
    }

    #[test]
    fn section_to_table() {
        let (_, sec) = exchange::data_section(
            r#"
            DATA;
              #2 = A(1.0, 2.0);
              #4 = B(2.0, A((4.0, 5.0)));
              #5 = B(2.0, #2);
            ENDSEC;
            "#
            .trim(),
        )
        .finish()
        .unwrap();
        dbg!(&sec);

        let table = Ap000::from_section(&sec).unwrap();
        dbg!(&table);
        assert_eq!(table, example_table());
    }

    #[test]
    fn b_from_record() {
        let tables = example_table();

        let (_, record) = exchange::simple_record("B(1.0, A((2.0, 3.0)))")
            .finish()
            .unwrap();
        let b = BHolder::deserialize(&record).unwrap();
        dbg!(b.into_owned(&tables).unwrap());

        let (_, record) = exchange::simple_record("B(1.0, #2)").finish().unwrap();
        let b = BHolder::deserialize(&record).unwrap();
        dbg!(b.into_owned(&tables).unwrap());
    }

    #[test]
    fn c_from_record() {
        let tables = example_table();

        // All components are inline
        let (_, record) = exchange::simple_record("C(A((1.0, 2.0)), B((1.0, A((2.0, 3.0)))))")
            .finish()
            .unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables).unwrap());

        // Use B with inline A
        let (_, record) = exchange::simple_record("C(A((1.0, 2.0)), #4)")
            .finish()
            .unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables).unwrap());

        // Use B with ref A
        let (_, record) = exchange::simple_record("C(A((1.0, 2.0)), #5)")
            .finish()
            .unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables).unwrap());

        // Use both reference
        let (_, record) = exchange::simple_record("C(#2, #4)").finish().unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables).unwrap());

        // Use both reference with DAG
        let (_, record) = exchange::simple_record("C(#2, #5)").finish().unwrap();
        let c = CHolder::deserialize(&record).unwrap();
        dbg!(c.into_owned(&tables).unwrap());
    }
}
