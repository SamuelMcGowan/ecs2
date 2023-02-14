use std::cell::{Ref, RefMut};
use std::marker::PhantomData;

use crate::prelude::World;
use crate::query::{Query, QueryResult};
use crate::storage::unique::{Unique, UniqueStorage};

pub struct QueryUnique<T: Unique>(PhantomData<T>);

pub struct BorrowUnique<'a, T: Unique> {
    storage: Ref<'a, UniqueStorage<T>>,
}

impl<T: Unique> Query for QueryUnique<T> {
    type Output<'a> = BorrowUnique<'a, T>;

    fn borrow(world: &World) -> QueryResult<Self::Output<'_>> {
        let storage = world.all_storages.uniques.borrow_ref()?;
        Ok(BorrowUnique { storage })
    }
}

pub struct QueryUniqueMut<T: Unique>(PhantomData<T>);

pub struct BorrowUniqueMut<'a, T: Unique> {
    storage: RefMut<'a, UniqueStorage<T>>,
}

impl<T: Unique> Query for QueryUniqueMut<T> {
    type Output<'a> = BorrowUniqueMut<'a, T>;

    fn borrow(world: &World) -> QueryResult<Self::Output<'_>> {
        let storage = world.all_storages.uniques.borrow_mut()?;
        Ok(BorrowUniqueMut { storage })
    }
}

impl<T: Unique> BorrowUnique<'_, T> {
    pub fn get(&self) -> &T {
        &self.storage.0
    }
}

impl<T: Unique> BorrowUniqueMut<'_, T> {
    pub fn get(&self) -> &T {
        &self.storage.0
    }

    pub fn get_mut(&mut self) -> &mut T {
        &mut self.storage.0
    }
}
