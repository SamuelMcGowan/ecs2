use std::cell::RefCell;

use crate::erased_storages::AllStorages;
use crate::query::{Query, QueryResult};
use crate::storage::entities::{EntityError, EntityId};
use crate::storage::unique::{Unique, UniqueStorage};
use crate::system::System;

pub trait WorldData: Default + 'static {}

impl WorldData for () {}

#[derive(Default)]
pub struct World<D: WorldData = ()> {
    pub(crate) all_storages: AllStorages,
    pub data: RefCell<D>,
}

impl<Data: WorldData> World<Data> {
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

    pub fn borrow<'a, Q: Query<'a, Data>>(&'a self) -> QueryResult<Q> {
        Q::borrow(self)
    }

    #[inline]
    pub fn run<'a, S: System<'a, Data, Input, Output>, Input, Output>(
        &'a self,
        system: S,
    ) -> QueryResult<Output> {
        system.run(self)
    }
}
