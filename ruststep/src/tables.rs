//! Traits for espr-generated structures

use std::collections::HashMap;

/// Trait for resolving a reference through entity id
pub trait Holder: Clone + 'static {
    type Owned;
    type Table;
    fn into_owned(self, table: &Self::Table) -> Result<Self::Owned, crate::error::Error>;
    fn name() -> &'static str;
    fn attr_len() -> usize;
}

/// Trait for tables which pulls an entity (`T`) from an entity id (`u64`)
pub trait EntityTable<T: Holder<Table = Self>> {
    /// Get owned entity from table
    fn get_owned(&self, entity_id: u64) -> Result<T::Owned, crate::error::Error>;

    /// Get owned entities as an iterator
    fn owned_iter<'table>(
        &'table self,
    ) -> Box<dyn Iterator<Item = Result<T::Owned, crate::error::Error>> + 'table>;
}

pub fn get_owned<T, Table>(
    table: &Table,
    map: &HashMap<u64, T>,
    entity_id: u64,
) -> Result<T::Owned, crate::error::Error>
where
    T: Holder<Table = Table>,
    Table: EntityTable<T>,
{
    match map.get(&entity_id) {
        Some(holder) => holder.clone().into_owned(table),
        None => Err(crate::error::Error::UnknownEntity(entity_id)),
    }
}

pub fn owned_iter<'table, T, Table>(
    table: &'table Table,
    map: &'table HashMap<u64, T>,
) -> Box<dyn Iterator<Item = Result<T::Owned, crate::error::Error>> + 'table>
where
    T: Holder<Table = Table>,
    Table: EntityTable<T>,
{
    Box::new(
        map.values()
            .cloned()
            .map(move |value| value.into_owned(table)),
    )
}
