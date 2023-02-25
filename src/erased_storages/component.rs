use std::any::Any;

use crate::storage::component::{Component, ComponentStorage};
use crate::storage::entities::EntityId;

use super::storage_map::ErasableStorage;

trait ErasedComponentStorageTrait: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn remove_entity(&mut self, entity: EntityId);
}

impl<C: Component> ErasedComponentStorageTrait for ComponentStorage<C> {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn remove_entity(&mut self, entity: EntityId) {
        self.remove(entity);
    }
}

pub(crate) struct ErasedComponentStorage(Box<dyn ErasedComponentStorageTrait>);

impl ErasedComponentStorage {
    pub fn new<C: Component>(storage: ComponentStorage<C>) -> Self {
        Self(Box::new(storage))
    }

    pub fn downcast_ref<S: Any>(&self) -> Option<&S> {
        (*self.0).as_any().downcast_ref()
    }

    pub fn downcast_mut<S: Any>(&mut self) -> Option<&mut S> {
        (*self.0).as_any_mut().downcast_mut()
    }

    pub fn remove_entity(&mut self, entity: EntityId) {
        (*self.0).remove_entity(entity);
    }
}

impl<C: Component> ErasableStorage for ComponentStorage<C> {
    type ErasedStorage = ErasedComponentStorage;

    #[inline]
    fn erase(self) -> Self::ErasedStorage {
        ErasedComponentStorage::new(self)
    }

    #[inline]
    fn downcast_ref(erased: &Self::ErasedStorage) -> Option<&Self> {
        erased.downcast_ref()
    }

    #[inline]
    fn downcast_mut(erased: &mut Self::ErasedStorage) -> Option<&mut Self> {
        erased.downcast_mut()
    }
}
