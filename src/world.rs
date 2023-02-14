use crate::erased_storages::AllStorages;
use crate::query::{Query, QueryResult};
use crate::storage::entities::{EntityError, EntityId};
use crate::storage::unique::{Unique, UniqueStorage};

#[derive(Default)]
pub struct World {
    pub(crate) all_storages: AllStorages,
}

impl World {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn spawn(&mut self) -> Result<EntityId, EntityError> {
        self.all_storages.entities.alloc()
    }

    #[inline]
    pub fn insert_unique<T: Unique>(&mut self, unique: T) {
        self.all_storages.uniques.insert(UniqueStorage(unique));
    }

    #[inline]
    pub fn borrow<Q: Query>(&self) -> QueryResult<Q::Output<'_>> {
        Q::borrow(self)
    }
}
