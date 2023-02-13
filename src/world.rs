use crate::erased_storages::storage_map::StorageResult;
use crate::erased_storages::AllStorages;
use crate::prelude::Query;

pub struct World {
    pub(crate) all_storages: AllStorages,
}

impl World {
    #[inline]
    pub fn borrow<Q: Query>(&self) -> StorageResult<Q::Output<'_>> {
        Q::borrow(self)
    }
}
