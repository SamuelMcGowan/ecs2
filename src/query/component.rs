use std::cell::{Ref, RefMut};
use std::marker::PhantomData;

use crate::query::{QueryError, QueryResult};
use crate::storage::component::{Component, ComponentStorage};
use crate::storage::entities::{EntityId, EntityStorage};
use crate::world::{World, WorldData};

use super::Query;

pub struct QueryComp<C: Component>(PhantomData<C>);

pub struct BorrowComp<'a, C: Component> {
    storage: Ref<'a, ComponentStorage<C>>,
    entities: &'a EntityStorage,
}

impl<C: Component, D: WorldData> Query<D> for QueryComp<C> {
    type Output<'a> = BorrowComp<'a, C>;

    fn borrow(world: &World<D>) -> QueryResult<Self::Output<'_>> {
        let storage = world.all_storages.components.borrow_ref_or_insert()?;
        let entities = &world.all_storages.entities;
        Ok(BorrowComp { storage, entities })
    }
}

pub struct QueryCompMut<C: Component>(PhantomData<C>);

pub struct BorrowCompMut<'a, C: Component> {
    storage: RefMut<'a, ComponentStorage<C>>,
    entities: &'a EntityStorage,
}

impl<C: Component, D: WorldData> Query<D> for QueryCompMut<C> {
    type Output<'a> = BorrowCompMut<'a, C>;

    fn borrow(world: &World<D>) -> QueryResult<Self::Output<'_>> {
        let storage = world.all_storages.components.borrow_mut_or_insert()?;
        let entities = &world.all_storages.entities;
        Ok(BorrowCompMut { storage, entities })
    }
}

impl<C: Component> BorrowComp<'_, C> {
    pub fn get(&self, entity: EntityId) -> QueryResult<&C> {
        if !self.entities.is_alive(entity) {
            return Err(QueryError::EntityDead);
        }
        self.storage.get(entity).ok_or(QueryError::EntityMissing)
    }

    pub fn iter(&self) -> impl Iterator<Item = &C> {
        self.storage.iter()
    }
}

impl<C: Component> BorrowCompMut<'_, C> {
    pub fn insert(&mut self, entity: EntityId, component: C) -> QueryResult<Option<C>> {
        if !self.entities.is_alive(entity) {
            return Err(QueryError::EntityDead);
        }
        Ok(self.storage.insert(entity, component))
    }

    pub fn get(&self, entity: EntityId) -> QueryResult<&C> {
        if !self.entities.is_alive(entity) {
            return Err(QueryError::EntityDead);
        }
        self.storage.get(entity).ok_or(QueryError::EntityMissing)
    }

    pub fn get_mut(&mut self, entity: EntityId) -> QueryResult<&mut C> {
        if !self.entities.is_alive(entity) {
            return Err(QueryError::EntityDead);
        }
        self.storage
            .get_mut(entity)
            .ok_or(QueryError::EntityMissing)
    }

    pub fn iter(&self) -> impl Iterator<Item = &C> {
        self.storage.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut C> {
        self.storage.iter_mut()
    }
}
