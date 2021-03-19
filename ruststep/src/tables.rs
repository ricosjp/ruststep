use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt,
    hash::{Hash, Hasher},
};

pub type Table<T> = HashMap<Id<T>, T>;

#[derive(Deserialize)]
pub struct Id<T: 'static> {
    id: usize,
    phantom: std::marker::PhantomData<&'static T>,
}

impl<T: 'static> fmt::Debug for Id<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

impl<T: 'static> PartialEq for Id<T> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<T: 'static> Eq for Id<T> {}

impl<T: 'static> Hash for Id<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

pub trait Entity<'tables> {
    type Schema: EntryTable<'tables, Self::Entry>;
    type Entry: TableEntry<'tables, Schema = Self::Schema>;
    type Ref: 'tables + EntityRef<Entity = Self>;
}

/// Trait for table entry struct. See module level document for detail.
pub trait TableEntry<'tables>: 'tables + Sized {
    type Schema: EntryTable<'tables, Self>;
    type Ref: 'tables + EntityRef;
    fn as_ref(&'tables self, schema: &'tables Self::Schema) -> Self::Ref;
}

pub trait EntityRef {
    type Entity;
    fn to_instance(&self) -> Self::Entity;
}

pub trait EntryTable<'tables, E: TableEntry<'tables, Schema = Self>> {
    fn get_entry(&self, id: &Id<E>) -> &E;
    fn entry_iter(&'tables self) -> Box<dyn Iterator<Item = &E> + 'tables>;

    fn get_ref(&'tables self, id: &Id<E>) -> E::Ref {
        self.get_entry(id).as_ref(self)
    }

    fn iter(&'tables self) -> Box<dyn Iterator<Item = E::Ref> + 'tables> {
        Box::new(self.entry_iter().map(move |entry| entry.as_ref(self)))
    }
}
