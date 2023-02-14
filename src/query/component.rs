use std::cell::{Ref, RefMut};
use std::marker::PhantomData;

use crate::prelude::World;
use crate::query::{QueryError, QueryResult};
use crate::storage::component::{Component, ComponentStorage};
use crate::storage::entities::{EntityId, EntityStorage};

use super::Query;

pub struct Comp<C: Component>(PhantomData<C>);

pub struct CompBorrow<'a, C: Component> {
    storage: Ref<'a, ComponentStorage<C>>,
    entities: &'a EntityStorage,
}

impl<C: Component> Query for Comp<C> {
    type Output<'a> = CompBorrow<'a, C>;

    fn borrow(world: &World) -> QueryResult<Self::Output<'_>> {
        let storage = world.all_storages.components.borrow_ref_or_insert()?;
        let entities = &world.all_storages.entities;
        Ok(CompBorrow { storage, entities })
    }
}

pub struct CompMut<C: Component>(PhantomData<C>);

pub struct CompBorrowMut<'a, C: Component> {
    storage: RefMut<'a, ComponentStorage<C>>,
    entities: &'a EntityStorage,
}

impl<C: Component> Query for CompMut<C> {
    type Output<'a> = CompBorrowMut<'a, C>;

    fn borrow(world: &World) -> QueryResult<Self::Output<'_>> {
        let storage = world.all_storages.components.borrow_mut_or_insert()?;
        let entities = &world.all_storages.entities;
        Ok(CompBorrowMut { storage, entities })
    }
}

impl<C: Component> CompBorrow<'_, C> {
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

impl<C: Component> CompBorrowMut<'_, C> {
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
