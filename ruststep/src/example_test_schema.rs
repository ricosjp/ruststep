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

#[derive(Debug)]
pub struct Ap000 {
    a: Table<A>,
    b: Table<BEntry>,
    c: Table<CEntry>,
}

impl Ap000 {
    pub fn a_iter(&self) -> impl Iterator<Item = &A> {
        self.a.iter().map(|(_id, a)| a)
    }

    fn b_ref<'s, 'e: 's>(&'s self, entry: &'e BEntry) -> BRef<'s> {
        let BEntry { z, w } = entry;
        BRef {
            z,
            w: self.a.get(w).unwrap(),
        }
    }

    pub fn b_iter(&self) -> impl Iterator<Item = BRef> {
        self.b.iter().map(move |(_id, entry)| self.b_ref(entry))
    }

    fn c_ref<'s, 'e: 's>(&'s self, entry: &'e CEntry) -> CRef<'s> {
        let CEntry { p, q } = entry;
        CRef {
            p: self.a.get(p).unwrap(),
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
    pub w: &'schema A,
}

impl<'schema> From<&'schema B> for BRef<'schema> {
    fn from(b: &'schema B) -> Self {
        let B { z, w } = b;
        BRef { z, w }
    }
}

impl<'schema> BRef<'schema> {
    pub fn to_owned(&self) -> B {
        B {
            z: self.z.clone(),
            w: self.w.clone(),
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
    pub p: &'schema A,
    pub q: BRef<'schema>,
}

impl<'schema> From<&'schema C> for CRef<'schema> {
    fn from(c: &'schema C) -> Self {
        let C { p, q } = c;
        CRef { p, q: q.into() }
    }
}

impl<'schema> CRef<'schema> {
    pub fn to_owned(&self) -> C {
        C {
            p: self.p.clone(),
            q: self.q.to_owned(),
        }
    }
}
