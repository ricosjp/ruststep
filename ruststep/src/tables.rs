use std::collections::HashMap;

pub trait Holder {
    type Owned;
    type Table;
    fn into_owned(self, table: &Self::Table) -> Result<Self::Owned, crate::error::Error>;
}

pub trait EntityTable<T> {
    fn get_entity(&self, entity_id: u64) -> Result<&T, crate::error::Error>;
}

impl<T> EntityTable<T> for HashMap<u64, T> {
    fn get_entity(&self, id: u64) -> Result<&T, crate::error::Error> {
        Ok(self
            .get(&id)
            .ok_or_else(|| crate::error::Error::UnknownEntity(id))?)
    }
}
