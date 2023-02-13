use std::any::{Any, TypeId};
use std::cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut};
use std::slice::{Iter, IterMut};

use elsa::FrozenMap;

pub(crate) trait ErasableStorage: Any + Sized {
    type ErasedStorage;

    fn erase(self) -> Self::ErasedStorage;

    fn downcast_ref(erased: &Self::ErasedStorage) -> Option<&Self>;
    fn downcast_mut(erased: &mut Self::ErasedStorage) -> Option<&mut Self>;
}

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("storage is missing")]
    StorageMissing,

    #[error("{0}")]
    BorrowError(#[from] BorrowError),

    #[error("{0}")]
    BorrowMutError(#[from] BorrowMutError),
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
    pub fn insert<S: ErasableStorage<ErasedStorage = ErasedStorage>>(&self, storage: S) {
        let type_id = TypeId::of::<S>();
        self.storages
            .insert(type_id, Box::new(RefCell::new(storage.erase())));
    }

    pub fn borrow_ref<S: ErasableStorage<ErasedStorage = ErasedStorage>>(
        &self,
    ) -> Result<Ref<S>, StorageError> {
        let erased_storage = self.get::<S>()?;
        borrow_ref(erased_storage)
    }

    pub fn borrow_mut<S: ErasableStorage<ErasedStorage = ErasedStorage>>(
        &self,
    ) -> Result<RefMut<S>, StorageError> {
        let erased_storage = self.get::<S>()?;
        borrow_mut(erased_storage)
    }

    pub fn borrow_ref_or_insert<S: ErasableStorage<ErasedStorage = ErasedStorage> + Default>(
        &self,
    ) -> Result<Ref<S>, StorageError> {
        let erased_storage = self.get_or_insert::<S>();
        borrow_ref(erased_storage)
    }

    pub fn borrow_mut_or_insert<S: ErasableStorage<ErasedStorage = ErasedStorage> + Default>(
        &self,
    ) -> Result<RefMut<S>, StorageError> {
        let erased_storage = self.get_or_insert::<S>();
        borrow_mut(erased_storage)
    }

    #[inline]
    fn get<S: ErasableStorage<ErasedStorage = ErasedStorage>>(
        &self,
    ) -> Result<&RefCell<ErasedStorage>, StorageError> {
        let type_id = TypeId::of::<S>();
        self.storages
            .get(&type_id)
            .ok_or(StorageError::StorageMissing)
    }

    #[inline]
    fn get_or_insert<S: ErasableStorage<ErasedStorage = ErasedStorage> + Default>(
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
    type Item = Result<Ref<'a, ErasedStorage>, BorrowError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|erased_storage| erased_storage.try_borrow())
    }
}

pub(crate) struct ErasedStorageIterMut<'a, ErasedStorage>(IterMut<'a, RefCell<ErasedStorage>>);

impl<'a, ErasedStorage> Iterator for ErasedStorageIterMut<'a, ErasedStorage> {
    type Item = Result<RefMut<'a, ErasedStorage>, BorrowMutError>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.0
            .next()
            .map(|erased_storage| erased_storage.try_borrow_mut())
    }
}

#[inline]
fn borrow_ref<S: ErasableStorage>(
    erased_storage: &RefCell<S::ErasedStorage>,
) -> Result<Ref<S>, StorageError> {
    let erased_storage_ref = erased_storage.try_borrow()?;
    let storage = Ref::map(erased_storage_ref, |erased| {
        S::downcast_ref(erased).unwrap()
    });
    Ok(storage)
}

#[inline]
fn borrow_mut<S: ErasableStorage>(
    erased_storage: &RefCell<S::ErasedStorage>,
) -> Result<RefMut<S>, StorageError> {
    let erased_storage_mut = erased_storage.try_borrow_mut()?;
    let storage = RefMut::map(erased_storage_mut, |erased| {
        S::downcast_mut(erased).unwrap()
    });
    Ok(storage)
}
