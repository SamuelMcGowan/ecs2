mod components;
mod uniques;
use self::components::ErasedComponentStorage;
use self::storage_set::StorageSet;
use self::uniques::ErasedUniqueStorage;
use crate::storage::entities::EntityStorage;

mod storage_set;

#[derive(Default)]
pub(crate) struct AllStorages {
    pub(crate) entities: EntityStorage,
    pub(crate) components: StorageSet<ErasedComponentStorage>,
    pub(crate) uniques: StorageSet<ErasedUniqueStorage>,
}
