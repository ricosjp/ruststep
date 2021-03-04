//! Experimental schema definitions corresponding following EXPRESS Schema
//!
//! ```
//! ENTITY a;
//!   x: INTEGER;
//!   y: INTEGER;
//! END_ENTITY;
//!
//! ENTITY b;
//!   z: INTEGER;
//!   w: a;
//! END_ENTITY;
//! ```

use std::marker::PhantomData;

pub trait Table {
    type Entity: Sized;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Index<Entity> {
    index: usize,
    marker: PhantomData<Entity>,
}

impl<Entity> Index<Entity> {
    pub fn get<'table>(&self, table: &'table impl Table<Entity = Entity>) -> &'table Entity {
        todo!()
    }
}

/* ENTITY a */

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct A {
    x: u64,
    y: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableA {
    x: Vec<u64>,
    y: Vec<u64>,
}

impl Table for TableA {
    type Entity = A;
}
