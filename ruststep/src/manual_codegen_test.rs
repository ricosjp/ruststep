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

use crate::tables::*;

#[derive(Debug)]
pub struct Ap000 {
    a: Table<AEntry>,
    b: Table<BEntry>,
    c: Table<CEntry>,
}

impl<'tables> EntryTable<'tables, AEntry> for Ap000 {
    fn get_entry(&self, id: &Id<AEntry>) -> &AEntry {
        self.a.get(id).unwrap()
    }
    fn entry_iter(&'tables self) -> Box<dyn Iterator<Item = &AEntry> + 'tables> {
        Box::new(self.a.iter().map(|(_id, entry)| entry))
    }
}

impl<'tables> EntryTable<'tables, BEntry> for Ap000 {
    fn get_entry(&self, id: &Id<BEntry>) -> &BEntry {
        self.b.get(id).unwrap()
    }
    fn entry_iter(&'tables self) -> Box<dyn Iterator<Item = &BEntry> + 'tables> {
        Box::new(self.b.iter().map(|(_id, entry)| entry))
    }
}

impl<'tables> EntryTable<'tables, CEntry> for Ap000 {
    fn get_entry(&self, id: &Id<CEntry>) -> &CEntry {
        self.c.get(id).unwrap()
    }
    fn entry_iter(&'tables self) -> Box<dyn Iterator<Item = &CEntry> + 'tables> {
        Box::new(self.c.iter().map(|(_id, entry)| entry))
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
pub struct ARef<'tables> {
    pub x: &'tables u64,
    pub y: &'tables u64,
}

impl<'tables> Entity<'tables> for A {
    type Schema = Ap000;
    type Entry = AEntry;
    type Ref = ARef<'tables>;
}

impl<'tables> TableEntry<'tables> for AEntry {
    type Schema = Ap000;
    type Ref = ARef<'tables>;

    fn as_ref(&'tables self, _schema: &'tables Self::Schema) -> Self::Ref {
        let AEntry { x, y } = self;
        ARef { x, y }
    }
}

impl<'tables> EntityRef for ARef<'tables> {
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
pub struct BEntry {
    z: u64,
    w: Id<AEntry>,
}

/// Element-wise immutable reference to [B]
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct BRef<'tables> {
    pub z: &'tables u64,
    pub w: ARef<'tables>,
}

impl<'tables> Entity<'tables> for B {
    type Schema = Ap000;
    type Entry = BEntry;
    type Ref = BRef<'tables>;
}

impl<'tables> TableEntry<'tables> for BEntry {
    type Schema = Ap000;
    type Ref = BRef<'tables>;

    fn as_ref(&'tables self, schema: &'tables Self::Schema) -> Self::Ref {
        let BEntry { z, w } = self;
        BRef {
            z,
            w: schema.a.get(w).unwrap().as_ref(schema),
        }
    }
}

impl<'tables> EntityRef for BRef<'tables> {
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
pub struct CEntry {
    p: Id<AEntry>,
    q: Id<BEntry>,
}

/// Element-wise immutable reference to [C]
#[derive(Debug, Clone, PartialEq, Hash)]
pub struct CRef<'tables> {
    pub p: ARef<'tables>,
    pub q: BRef<'tables>,
}

impl<'tables> Entity<'tables> for C {
    type Schema = Ap000;
    type Entry = CEntry;
    type Ref = CRef<'tables>;
}

impl<'tables> TableEntry<'tables> for CEntry {
    type Schema = Ap000;
    type Ref = CRef<'tables>;

    fn as_ref(&'tables self, schema: &'tables Self::Schema) -> Self::Ref {
        let CEntry { p, q } = self;
        CRef {
            p: schema.a.get(p).unwrap().as_ref(schema),
            q: schema.b.get(q).unwrap().as_ref(schema),
        }
    }
}

impl<'tables> EntityRef for CRef<'tables> {
    type Entity = C;
    fn to_instance(&self) -> C {
        C {
            p: self.p.to_instance(),
            q: self.q.to_instance(),
        }
    }
}
