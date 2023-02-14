use crate::erased_storages::storage_map::StorageResult;
use crate::prelude::World;

pub mod component;

pub trait Query {
    type Output<'a>;
    fn borrow(world: &World) -> StorageResult<Self::Output<'_>>;
}
