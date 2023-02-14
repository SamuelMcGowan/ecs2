use std::any::Any;

use crate::storage::unique::{Unique, UniqueStorage};

use super::storage_map::ErasableStorage;

pub(crate) struct ErasedUniqueStorage(Box<dyn Any>);

impl ErasedUniqueStorage {
    pub fn new<T: Unique>(storage: UniqueStorage<T>) -> Self {
        Self(Box::new(storage))
    }

    pub fn downcast_ref<S: Any>(&self) -> Option<&S> {
        (*self.0).downcast_ref()
    }

    pub fn downcast_mut<S: Any>(&mut self) -> Option<&mut S> {
        (*self.0).downcast_mut()
    }
}

impl<C: Unique> ErasableStorage for UniqueStorage<C> {
    type ErasedStorage = ErasedUniqueStorage;

    #[inline]
    fn erase(self) -> Self::ErasedStorage {
        ErasedUniqueStorage::new(self)
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
