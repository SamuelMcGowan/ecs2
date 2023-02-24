use std::cell::{Ref, RefMut};

use crate::prelude::World;
use crate::query::{Query, QueryResult};
use crate::storage::unique::{Unique, UniqueStorage};
use crate::world::WorldData;

pub struct QueryUnique<'a, T: Unique> {
    storage: Ref<'a, UniqueStorage<T>>,
}

impl<'a, T: Unique, D: WorldData> Query<'a, D> for QueryUnique<'a, T> {
    #[inline]
    fn borrow(world: &'a World<D>) -> QueryResult<Self> {
        let storage = world.all_storages.uniques.borrow_ref()?;
        Ok(QueryUnique { storage })
    }
}

pub struct QueryUniqueMut<'a, T: Unique> {
    storage: RefMut<'a, UniqueStorage<T>>,
}

impl<'a, T: Unique, D: WorldData> Query<'a, D> for QueryUniqueMut<'a, T> {
    #[inline]
    fn borrow(world: &'a World<D>) -> QueryResult<Self> {
        let storage = world.all_storages.uniques.borrow_mut()?;
        Ok(QueryUniqueMut { storage })
    }
}

impl<T: Unique> QueryUnique<'_, T> {
    #[inline]
    pub fn get(&self) -> &T {
        &self.storage.0
    }
}

impl<T: Unique> QueryUniqueMut<'_, T> {
    #[inline]
    pub fn get(&self) -> &T {
        &self.storage.0
    }

    #[inline]
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.storage.0
    }
}
