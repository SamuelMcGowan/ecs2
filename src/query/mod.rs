pub mod component;

use crate::erased_storages::storage_map::StorageResult;
use crate::prelude::World;

pub trait Query {
    type Output<'a>;
    fn borrow(world: &World) -> StorageResult<Self::Output<'_>>;
}
