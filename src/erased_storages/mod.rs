mod components;
mod uniques;
use self::components::ErasedComponentStorage;
use self::storage_map::StorageMap;
use self::uniques::ErasedUniqueStorage;
use crate::storage::entities::EntityStorage;

mod storage_map;

#[derive(Default)]
pub(crate) struct AllStorages {
    pub(crate) entities: EntityStorage,
    pub(crate) components: StorageMap<ErasedComponentStorage>,
    pub(crate) uniques: StorageMap<ErasedUniqueStorage>,
}
