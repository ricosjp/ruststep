use crate::parser::value::RValue;
use serde::Deserialize;
use std::{
    collections::HashMap,
    fmt,
    hash::{Hash, Hasher},
};

pub type Table<T> = HashMap<Id<T>, T>;

#[derive(Deserialize)]
#[serde(transparent)]
pub struct Id<T: 'static> {
    id: u64,
    #[serde(skip)]
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

/// Owned value or reference to entity/value
///
/// ```
/// use serde::Deserialize;
/// use nom::Finish;
/// use ruststep::{parser::{value::RValue, exchange}, tables::PlaceHolder};
///
/// #[derive(Debug, Deserialize)]
/// struct A {
///     x: f64,
///     y: f64,
/// }
///
/// let value = RValue::Entity(11);
/// let a: PlaceHolder<A> = Deserialize::deserialize(&value).unwrap();
/// dbg!(a);
///
/// let (_, record) = exchange::simple_record("A(1.0, 2.0)").finish().unwrap();
/// let a: PlaceHolder<A> = Deserialize::deserialize(&record).unwrap();
/// dbg!(a);
/// ```
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub enum PlaceHolder<T> {
    Ref(RValue),
    Owned(T),
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

#[cfg(test)]
mod tests {
    use nom::Finish;
    use serde::Deserialize;

    struct A {}

    #[test]
    fn deserialize_id() {
        let (res, p) = crate::parser::exchange::parameter("#10").finish().unwrap();
        assert_eq!(res, "");
        let id: super::Id<A> = Deserialize::deserialize(&p).unwrap();
        assert_eq!(id.id, 10);
    }
}
