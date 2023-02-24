use crate::erased_storages::AllStorages;
use crate::prelude::{Component, EntityId};
use crate::query::QueryResult;
use crate::storage::component::ComponentStorage;
use std::cell::RefMut;

pub struct EntityMut<'a> {
    pub(crate) all_storages: &'a mut AllStorages,
    pub(crate) entity: EntityId,
}

impl<'a> EntityMut<'a> {
    pub fn insert<C: Component>(self, component: C) -> QueryResult<Self> {
        let mut components: RefMut<ComponentStorage<C>> =
            self.all_storages.components.borrow_mut_or_insert().unwrap();
        components.insert(self.entity, component);
        drop(components);
        Ok(self)
    }

    pub fn remove<C: Component>(self) -> QueryResult<Self> {
        let mut components: RefMut<ComponentStorage<C>> =
            self.all_storages.components.borrow_mut()?;
        let _ = components.remove(self.entity);
        drop(components);
        Ok(self)
    }

    #[inline]
    pub fn id(&self) -> EntityId {
        self.entity
    }
}
