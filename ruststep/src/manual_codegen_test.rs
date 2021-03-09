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

impl<'rf> EntryTable<'rf, AEntry> for Ap000 {
    fn get_entry(&self, id: &Id<AEntry>) -> &AEntry {
        self.a.get(id).unwrap()
    }
    fn entries<'schema>(&'schema self) -> Box<dyn Iterator<Item = &AEntry> + 'schema> {
        Box::new(self.a.iter().map(|(_id, entry)| entry))
    }
}

impl<'rf> EntryTable<'rf, BEntry> for Ap000 {
    fn get_entry(&self, id: &Id<BEntry>) -> &BEntry {
        self.b.get(id).unwrap()
    }
    fn entries<'schema>(&'schema self) -> Box<dyn Iterator<Item = &BEntry> + 'schema> {
        Box::new(self.b.iter().map(|(_id, entry)| entry))
    }
}

impl<'rf> EntryTable<'rf, CEntry> for Ap000 {
    fn get_entry(&self, id: &Id<CEntry>) -> &CEntry {
        self.c.get(id).unwrap()
    }
    fn entries<'schema>(&'schema self) -> Box<dyn Iterator<Item = &CEntry> + 'schema> {
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
pub struct ARef<'schema> {
    pub x: &'schema u64,
    pub y: &'schema u64,
}

impl<'rf> Entity<'rf> for A {
    type Schema = Ap000;
    type Entry = AEntry;
    type Ref = ARef<'rf>;
}

impl<'rf> TableEntry<'rf> for AEntry {
    type Schema = Ap000;
    type Ref = ARef<'rf>;

    fn as_ref<'schema: 'rf, 'entry: 'rf>(
        &'entry self,
        _schema: &'schema Self::Schema,
    ) -> Self::Ref {
        let AEntry { x, y } = self;
        ARef { x, y }
    }
}

impl<'schema> EntityRef for ARef<'schema> {
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
pub struct BRef<'schema> {
    pub z: &'schema u64,
    pub w: ARef<'schema>,
}

impl<'rf> Entity<'rf> for B {
    type Schema = Ap000;
    type Entry = BEntry;
    type Ref = BRef<'rf>;
}

impl<'rf> TableEntry<'rf> for BEntry {
    type Schema = Ap000;
    type Ref = BRef<'rf>;

    fn as_ref<'schema: 'rf, 'entity: 'rf>(
        &'entity self,
        schema: &'schema Self::Schema,
    ) -> Self::Ref {
        let BEntry { z, w } = self;
        BRef {
            z,
            w: schema.a.get(w).unwrap().as_ref(schema),
        }
    }
}

impl<'schema> EntityRef for BRef<'schema> {
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
pub struct CRef<'schema> {
    pub p: ARef<'schema>,
    pub q: BRef<'schema>,
}

impl<'rf> Entity<'rf> for C {
    type Schema = Ap000;
    type Entry = CEntry;
    type Ref = CRef<'rf>;
}

impl<'rf> TableEntry<'rf> for CEntry {
    type Schema = Ap000;
    type Ref = CRef<'rf>;

    fn as_ref<'schema: 'rf, 'entity: 'rf>(
        &'entity self,
        schema: &'schema Self::Schema,
    ) -> Self::Ref {
        let CEntry { p, q } = self;
        CRef {
            p: schema.a.get(p).unwrap().as_ref(schema),
            q: schema.b.get(q).unwrap().as_ref(schema),
        }
    }
}

impl<'schema> EntityRef for CRef<'schema> {
    type Entity = C;
    fn to_instance(&self) -> C {
        C {
            p: self.p.to_instance(),
            q: self.q.to_instance(),
        }
    }
}
