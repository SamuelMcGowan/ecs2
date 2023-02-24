use std::cell::RefCell;

use crate::erased_storages::AllStorages;
use crate::query::{Query, QueryResult};
use crate::storage::entities::{EntityError, EntityId};
use crate::storage::unique::{Unique, UniqueStorage};

pub trait WorldData: Default + 'static {}

impl WorldData for () {}

#[derive(Default)]
pub struct World<D: WorldData = ()> {
    pub(crate) all_storages: AllStorages,
    pub data: RefCell<D>,
}

impl<D: WorldData> World<D> {
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

    pub fn borrow<'a, Q: Query<'a, D>>(&'a self) -> QueryResult<Q> {
        Q::borrow(self)
    }
}
