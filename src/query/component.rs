use std::cell::{Ref, RefMut};
use std::marker::PhantomData;

use super::Query;
use crate::erased_storages::storage_map::StorageResult;
use crate::prelude::World;
use crate::storage::component::{Component, ComponentStorage};

pub struct Comp<C: Component>(PhantomData<C>);
pub struct CompView<'a, C: Component>(Ref<'a, ComponentStorage<C>>);

impl<C: Component> Query for Comp<C> {
    type Output<'a> = CompView<'a, C>;

    fn borrow(world: &World) -> StorageResult<Self::Output<'_>> {
        let storage = world.all_storages.components.borrow_ref_or_insert()?;
        Ok(CompView(storage))
    }
}

pub struct CompMut<C: Component>(PhantomData<C>);
pub struct CompViewMut<'a, C: Component>(RefMut<'a, ComponentStorage<C>>);

impl<C: Component> Query for CompMut<C> {
    type Output<'a> = CompViewMut<'a, C>;

    fn borrow(world: &World) -> StorageResult<Self::Output<'_>> {
        let storage = world.all_storages.components.borrow_mut_or_insert()?;
        Ok(CompViewMut(storage))
    }
}
