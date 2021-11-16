//! Traits for espr-generated structures

use serde::de;
use std::collections::HashMap;

use crate::{
    ast::{DataSection, Record},
    error::*,
};

/// Trait for resolving a reference through entity id
pub trait Holder: Clone + 'static {
    type Owned;
    type Table;
    fn into_owned(self, table: &Self::Table) -> Result<Self::Owned>;
    fn name() -> &'static str;
    fn attr_len() -> usize;
}

pub trait WithVisitor {
    type Visitor: for<'de> de::Visitor<'de, Value = Self>;
    fn visitor_new() -> Self::Visitor;
}

/// Trait for tables which pulls an entity (`T`) from an entity id (`u64`)
pub trait EntityTable<T: Holder<Table = Self>> {
    /// Get owned entity from table
    fn get_owned(&self, entity_id: u64) -> Result<T::Owned>;

    /// Get owned entities as an iterator
    fn owned_iter<'table>(&'table self) -> Box<dyn Iterator<Item = Result<T::Owned>> + 'table>;
}

/// Create Table from [DataSection]
pub trait TableInit: Default {
    fn append_data_section(&mut self, section: &DataSection) -> Result<()>;

    fn from_data_section(section: &DataSection) -> Result<Self> {
        let mut table = Self::default();
        table.append_data_section(section)?;
        Ok(table)
    }

    fn from_data_sections(sections: &[DataSection]) -> Result<Self> {
        let mut table = Self::default();
        for section in sections {
            table.append_data_section(section)?;
        }
        Ok(table)
    }
}

pub fn get_owned<T, Table>(table: &Table, map: &HashMap<u64, T>, entity_id: u64) -> Result<T::Owned>
where
    T: Holder<Table = Table>,
    Table: EntityTable<T>,
{
    match map.get(&entity_id) {
        Some(holder) => holder.clone().into_owned(table),
        None => Err(Error::UnknownEntity(entity_id)),
    }
}

pub fn owned_iter<'table, T, Table>(
    table: &'table Table,
    map: &'table HashMap<u64, T>,
) -> Box<dyn Iterator<Item = Result<T::Owned>> + 'table>
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

/// Helper function to implement TableInit trait
pub fn insert_record<'de, T: de::Deserialize<'de>>(
    table: &mut HashMap<u64, T>,
    id: u64,
    record: &Record,
) -> crate::error::Result<()> {
    if let Some(_) = table.insert(id, de::Deserialize::deserialize(record)?) {
        Err(Error::DuplicatedEntity(id))
    } else {
        Ok(())
    }
}
