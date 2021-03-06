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
//! END_SCHEMA;
//! ```

use std::collections::HashMap;

type Table<T> = HashMap<usize, T>;

#[derive(Debug)]
pub struct Ap000 {
    a: Table<A>,
    b: Table<B>,
}

impl Ap000 {
    pub fn a_iter(&self) -> impl Iterator<Item = (&usize, &A)> {
        self.a.iter()
    }

    pub fn b_iter(&self) -> impl Iterator<Item = (&usize, BRef)> {
        self.b.iter().map(move |(id, B { z, w })| {
            (
                id,
                BRef {
                    z,
                    w: self.a.get(w).unwrap(),
                },
            )
        })
    }

    pub fn b_get(&self, id: usize) -> Option<BRef> {
        let B { z, w } = &self.b.get(&id)?;
        Some(BRef {
            z,
            w: self.a.get(w).unwrap(),
        })
    }

    pub fn b_get_mut(&mut self, id: usize) -> Option<BMut> {
        let B { z, w } = self.b.get_mut(&id)?;
        Some(BMut {
            z,
            w: self.a.get_mut(w).unwrap(),
        })
    }
}

#[derive(Debug, PartialEq, Hash)]
pub struct A {
    x: u64,
    y: u64,
}

#[derive(Debug, PartialEq, Hash)]
struct B {
    z: u64,
    w: usize, // ref to A
}

#[derive(Debug, PartialEq, Hash)]
pub struct BRef<'schema> {
    z: &'schema u64,
    w: &'schema A,
}

#[derive(Debug, PartialEq, Hash)]
pub struct BMut<'schema> {
    z: &'schema mut u64,
    w: &'schema mut A,
}
