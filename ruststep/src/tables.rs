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

pub trait TableEntry<'rf>: Sized {
    type Schema: EntryTable<'rf, Self>;
    type Ref: 'rf + EntityRef;

    fn as_ref<'schema: 'rf, 'entity: 'rf>(
        &'entity self,
        schema: &'schema Self::Schema,
    ) -> Self::Ref;
}

pub trait EntityRef {
    type Entity;
    fn to_instance(&self) -> Self::Entity;
}

pub trait EntryTable<'rf, E: TableEntry<'rf>> {
    fn get_entry(&self, id: &Id<E>) -> &E;
    fn entries<'schema>(&'schema self) -> Box<dyn Iterator<Item = &E> + 'schema>;
}
