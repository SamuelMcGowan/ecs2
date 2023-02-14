use crate::erased_storages::storage_map::StorageResult;
use crate::erased_storages::AllStorages;
use crate::prelude::Query;
use crate::storage::entities::{EntityError, EntityId};

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
    pub fn borrow<Q: Query>(&self) -> StorageResult<Q::Output<'_>> {
        Q::borrow(self)
    }
}
