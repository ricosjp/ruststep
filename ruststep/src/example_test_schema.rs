//! Experimental schema definitions corresponding following EXPRESS Schema
//!
//! ```text
//! SCHEMA ap000;
//!   ENTITY a;
//!     x: INTEGER;
//!     y: INTEGER;
//!   END_ENTITY;
//!
//!   ENTITY b;
//!     z: INTEGER;
//!     w: a;
//!   END_ENTITY;
//!
//!   ENTITY c;
//!     p: a;
//!     q: b;
//!   END_ENTITY;
//! END_SCHEMA;
//! ```

use std::collections::HashMap;

type Table<T> = HashMap<usize, T>;

/// Similar to [ToOwned], but not assure that `Instance: Borrow<Self>`.
///
/// [ToOwned]: https://doc.rust-lang.org/std/borrow/trait.ToOwned.html
pub trait ToInstance {
    type Entity;
    fn to_instance(&self) -> Self::Entity;
}

#[derive(Debug)]
pub struct Ap000 {
    a: Table<AEntry>,
    b: Table<BEntry>,
    c: Table<CEntry>,
}

impl Ap000 {
    fn a_ref<'s, 'e: 's>(&'s self, entry: &'e AEntry) -> ARef<'s> {
        let AEntry { x, y } = entry;
        ARef { x, y }
    }

    pub fn a_iter(&self) -> impl Iterator<Item = ARef> {
        self.a.iter().map(move |(_id, a)| self.a_ref(a))
    }

    fn b_ref<'s, 'e: 's>(&'s self, entry: &'e BEntry) -> BRef<'s> {
        let BEntry { z, w } = entry;
        BRef {
            z,
            w: self.a_ref(self.a.get(w).unwrap()),
        }
    }

    pub fn b_iter(&self) -> impl Iterator<Item = BRef> {
        self.b.iter().map(move |(_id, entry)| self.b_ref(entry))
    }

    fn c_ref<'s, 'e: 's>(&'s self, entry: &'e CEntry) -> CRef<'s> {
        let CEntry { p, q } = entry;
        CRef {
            p: self.a_ref(self.a.get(p).unwrap()),
            q: self.b_ref(&self.b.get(q).unwrap()),
        }
    }

    pub fn c_iter(&self) -> impl Iterator<Item = CRef> {
        self.c.iter().map(move |(_id, entry)| self.c_ref(entry))
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct A {
    pub x: u64,
    pub y: u64,
}

// same as [A]
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct AEntry {
    pub x: u64,
    pub y: u64,
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct ARef<'schema> {
    pub x: &'schema u64,
    pub y: &'schema u64,
}

impl<'schema> ToInstance for ARef<'schema> {
    type Entity = A;
    fn to_instance(&self) -> A {
        A {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

/* ENTITY b */

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct B {
    pub z: u64,
    pub w: A,
}

#[derive(Debug, PartialEq, Hash)]
struct BEntry {
    z: u64,
    w: usize, // ref to A
}

/// Element-wise immutable reference to [B]
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct BRef<'schema> {
    pub z: &'schema u64,
    pub w: ARef<'schema>,
}

impl<'schema> ToInstance for BRef<'schema> {
    type Entity = B;
    fn to_instance(&self) -> B {
        B {
            z: self.z.clone(),
            w: self.w.to_instance(),
        }
    }
}

/* ENTITY b */

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct C {
    pub p: A,
    pub q: B,
}

#[derive(Debug, PartialEq, Hash)]
struct CEntry {
    p: usize, // ref to A
    q: usize, // ref to B
}

/// Element-wise immutable reference to [C]
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct CRef<'schema> {
    pub p: ARef<'schema>,
    pub q: BRef<'schema>,
}

impl<'schema> ToInstance for CRef<'schema> {
    type Entity = C;
    fn to_instance(&self) -> C {
        C {
            p: self.p.to_instance(),
            q: self.q.to_instance(),
        }
    }
}
