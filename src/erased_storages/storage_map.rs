use std::any::{Any, TypeId};
use std::cell::{Ref, RefCell, RefMut};
use std::slice::{Iter, IterMut};

use elsa::FrozenMap;

use crate::query::{QueryError, QueryResult};

pub(crate) trait ErasableStorage: Any + Sized {
    type ErasedStorage;

    fn erase(self) -> Self::ErasedStorage;

    fn downcast_ref(erased: &Self::ErasedStorage) -> Option<&Self>;
    fn downcast_mut(erased: &mut Self::ErasedStorage) -> Option<&mut Self>;
}

pub(crate) struct StorageMap<ErasedStorage> {
    storages: FrozenMap<TypeId, Box<RefCell<ErasedStorage>>>,
}

impl<ErasedStorage> Default for StorageMap<ErasedStorage> {
    fn default() -> Self {
        Self {
            storages: FrozenMap::new(),
        }
    }
}

impl<ErasedStorage> StorageMap<ErasedStorage> {
    pub fn insert<S: ErasableStorage<ErasedStorage=ErasedStorage>>(&self, storage: S) {
        let type_id = TypeId::of::<S>();
        self.storages
            .insert(type_id, Box::new(RefCell::new(storage.erase())));
    }

    pub fn borrow_ref<S: ErasableStorage<ErasedStorage=ErasedStorage>>(
        &self,
    ) -> QueryResult<Ref<S>> {
        let erased_storage = self.get::<S>()?;
        borrow_ref(erased_storage)
    }

    pub fn borrow_mut<S: ErasableStorage<ErasedStorage=ErasedStorage>>(
        &self,
    ) -> QueryResult<RefMut<S>> {
        let erased_storage = self.get::<S>()?;
        borrow_mut(erased_storage)
    }

    pub fn borrow_ref_or_insert<S: ErasableStorage<ErasedStorage=ErasedStorage> + Default>(
        &self,
    ) -> QueryResult<Ref<S>> {
        let erased_storage = self.get_or_insert::<S>();
        borrow_ref(erased_storage)
    }

    pub fn borrow_mut_or_insert<S: ErasableStorage<ErasedStorage=ErasedStorage> + Default>(
        &self,
    ) -> QueryResult<RefMut<S>> {
        let erased_storage = self.get_or_insert::<S>();
        borrow_mut(erased_storage)
    }

    #[inline]
    fn get<S: ErasableStorage<ErasedStorage=ErasedStorage>>(
        &self,
    ) -> QueryResult<&RefCell<ErasedStorage>> {
        let type_id = TypeId::of::<S>();
        self.storages
            .get(&type_id)
            .ok_or(QueryError::StorageMissing)
    }

    #[inline]
    fn get_or_insert<S: ErasableStorage<ErasedStorage=ErasedStorage> + Default>(
        &self,
    ) -> &RefCell<ErasedStorage> {
        let type_id = TypeId::of::<S>();
        self.storages.get(&type_id).unwrap_or_else(|| {
            self.storages
                .insert(type_id, Box::new(RefCell::new(S::default().erase())))
        })
    }
}

pub(crate) struct ErasedStorageIter<'a, ErasedStorage>(Iter<'a, RefCell<ErasedStorage>>);

impl<'a, ErasedStorage> Iterator for ErasedStorageIter<'a, ErasedStorage> {
    type Item = QueryResult<Ref<'a, ErasedStorage>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|erased_storage| erased_storage.try_borrow().map_err(From::from))
    }
}

pub(crate) struct ErasedStorageIterMut<'a, ErasedStorage>(IterMut<'a, RefCell<ErasedStorage>>);

impl<'a, ErasedStorage> Iterator for ErasedStorageIterMut<'a, ErasedStorage> {
    type Item = QueryResult<RefMut<'a, ErasedStorage>>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|erased_storage| erased_storage.try_borrow_mut().map_err(From::from))
    }
}

#[inline]
fn borrow_ref<S: ErasableStorage>(
    erased_storage: &RefCell<S::ErasedStorage>,
) -> QueryResult<Ref<S>> {
    let erased_storage_ref = erased_storage.try_borrow()?;
    let storage = Ref::map(erased_storage_ref, |erased| {
        S::downcast_ref(erased).unwrap()
    });
    Ok(storage)
}

#[inline]
fn borrow_mut<S: ErasableStorage>(
    erased_storage: &RefCell<S::ErasedStorage>,
) -> QueryResult<RefMut<S>> {
    let erased_storage_mut = erased_storage.try_borrow_mut()?;
    let storage = RefMut::map(erased_storage_mut, |erased| {
        S::downcast_mut(erased).unwrap()
    });
    Ok(storage)
}
