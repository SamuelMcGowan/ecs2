use crate::storage::entities::EntityStorage;

use self::component::ErasedComponentStorage;
use self::storage_map::StorageMap;
use self::unique::ErasedUniqueStorage;

mod component;
mod unique;

pub(crate) mod storage_map;

#[derive(Default)]
pub(crate) struct AllStorages {
    pub(crate) entities: EntityStorage,
    pub(crate) components: StorageMap<ErasedComponentStorage>,
    pub(crate) uniques: StorageMap<ErasedUniqueStorage>,
}
