use std::{
    collections::HashMap,
    fmt,
    hash::{Hash, Hasher},
};

pub type Table<T> = HashMap<Id<T>, T>;

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

pub trait Entity<'rf> {
    type Schema: EntryTable<'rf, Self::Entry>;
    type Entry: TableEntry<'rf, Schema = Self::Schema>;
    type Ref: 'rf + EntityRef<Entity = Self>;
}

/// Trait for table entry struct. See module level document for detail.
pub trait TableEntry<'rf>: Sized {
    type Schema: EntryTable<'rf, Self>;
    type Ref: 'rf + EntityRef;
    fn as_ref(&'rf self, schema: &'rf Self::Schema) -> Self::Ref;
}

pub trait EntityRef {
    type Entity;
    fn to_instance(&self) -> Self::Entity;
}

pub trait EntryTable<'rf, E: TableEntry<'rf, Schema = Self>> {
    fn get_entry(&self, id: &Id<E>) -> &E;

    fn get_ref(&'rf self, id: &Id<E>) -> E::Ref {
        self.get_entry(id).as_ref(self)
    }
}
