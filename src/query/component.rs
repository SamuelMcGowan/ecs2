use std::cell::{Ref, RefMut};

use crate::query::{QueryError, QueryResult};
use crate::storage::component::{Component, ComponentStorage};
use crate::storage::entities::{EntityId, EntityStorage};
use crate::world::{World, WorldData};

use super::Query;

pub struct QueryComp<'a, C: Component> {
    storage: Ref<'a, ComponentStorage<C>>,
    entities: &'a EntityStorage,
}

impl<'a, C: Component, D: WorldData> Query<'a, D> for QueryComp<'a, C> {
    #[inline]
    fn borrow(world: &'a World<D>) -> QueryResult<Self> {
        let storage = world.all_storages.components.borrow_ref_or_insert()?;
        let entities = &world.all_storages.entities;
        Ok(QueryComp { storage, entities })
    }
}

pub struct QueryCompMut<'a, C: Component> {
    storage: RefMut<'a, ComponentStorage<C>>,
    entities: &'a EntityStorage,
}

impl<'a, C: Component, D: WorldData> Query<'a, D> for QueryCompMut<'a, C> {
    #[inline]
    fn borrow(world: &'a World<D>) -> QueryResult<Self> {
        let storage = world.all_storages.components.borrow_mut_or_insert()?;
        let entities = &world.all_storages.entities;
        Ok(QueryCompMut { storage, entities })
    }
}

impl<C: Component> QueryComp<'_, C> {
    #[inline]
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

impl<C: Component> QueryCompMut<'_, C> {
    #[inline]
    pub fn insert(&mut self, entity: EntityId, component: C) -> QueryResult<Option<C>> {
        if !self.entities.is_alive(entity) {
            return Err(QueryError::EntityDead);
        }
        Ok(self.storage.insert(entity, component))
    }

    #[inline]
    pub fn get(&self, entity: EntityId) -> QueryResult<&C> {
        if !self.entities.is_alive(entity) {
            return Err(QueryError::EntityDead);
        }
        self.storage.get(entity).ok_or(QueryError::EntityMissing)
    }

    #[inline]
    pub fn get_mut(&mut self, entity: EntityId) -> QueryResult<&mut C> {
        if !self.entities.is_alive(entity) {
            return Err(QueryError::EntityDead);
        }
        self.storage
            .get_mut(entity)
            .ok_or(QueryError::EntityMissing)
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &C> {
        self.storage.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut C> {
        self.storage.iter_mut()
    }
}
