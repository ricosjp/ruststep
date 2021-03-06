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
    b: Table<BEntry>,
}

impl Ap000 {
    pub fn a_iter(&self) -> impl Iterator<Item = &A> {
        self.a.iter().map(|(_id, a)| a)
    }

    pub fn b_iter(&self) -> impl Iterator<Item = BRef> {
        self.b.iter().map(move |(_id, BEntry { z, w })| BRef {
            z,
            w: self.a.get(w).unwrap(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct A {
    pub x: u64,
    pub y: u64,
}

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
