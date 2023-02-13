use std::any::{Any, TypeId};
use std::cell::{BorrowError, BorrowMutError, Ref, RefCell, RefMut};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::slice::{Iter, IterMut};

pub(crate) trait ErasableStorage: Any + Sized {
    type ErasedStorage;

    fn erase(self) -> Self::ErasedStorage;

    fn downcast_ref(erased: &Self::ErasedStorage) -> Option<&Self>;
    fn downcast_mut(erased: &mut Self::ErasedStorage) -> Option<&mut Self>;
}

pub(crate) struct StorageIdx<S: ErasableStorage> {
    idx: usize,
    phantom_data: PhantomData<S>,
}

pub(crate) struct StorageSet<ErasedStorage> {
    lookup: HashMap<TypeId, usize>,
    storages: Vec<RefCell<ErasedStorage>>,
}

impl<ErasedStorage> Default for StorageSet<ErasedStorage> {
    fn default() -> Self {
        Self {
            lookup: HashMap::new(),
            storages: vec![],
        }
    }
}

impl<ErasedStorage> StorageSet<ErasedStorage> {
    /// Panics if the new storage capacity exceeds `isize::MAX` bytes.
    fn insert<S: ErasableStorage<ErasedStorage = ErasedStorage>>(
        &mut self,
        storage: S,
    ) -> Option<usize> {
        let type_id = TypeId::of::<S>();
        match self.lookup.entry(type_id) {
            Entry::Vacant(entry) => {
                let idx = self.storages.len();
                let storage = RefCell::new(storage.erase());

                self.storages.push(storage);
                entry.insert(idx);

                Some(idx)
            }
            Entry::Occupied(_) => None,
        }
    }

    fn lookup<S: ErasableStorage<ErasedStorage = ErasedStorage>>(&self) -> Option<StorageIdx<S>> {
        let type_id = TypeId::of::<S>();
        Some(StorageIdx {
            idx: self.lookup.get(&type_id).copied()?,
            phantom_data: PhantomData,
        })
    }

    fn lookup_or_insert<S: ErasableStorage<ErasedStorage = ErasedStorage> + Default>(
        &mut self,
    ) -> StorageIdx<S> {
        let type_id = TypeId::of::<S>();

        let idx = match self.lookup.entry(type_id) {
            Entry::Vacant(entry) => {
                let idx = self.storages.len();
                let storage = RefCell::new(S::default().erase());

                self.storages.push(storage);
                entry.insert(idx);

                idx
            }
            Entry::Occupied(entry) => *entry.get(),
        };

        StorageIdx {
            idx,
            phantom_data: PhantomData,
        }
    }

    /// Panics if index is not valid for this storage.
    fn borrow_ref<S: ErasableStorage<ErasedStorage = ErasedStorage>>(
        &self,
        idx: StorageIdx<S>,
    ) -> Result<Ref<S>, BorrowError> {
        let erased_storage_ref = self.storages[idx.idx].try_borrow()?;

        let storage = Ref::map(erased_storage_ref, |erased| {
            S::downcast_ref(erased).unwrap()
        });

        Ok(storage)
    }

    /// Panics if index is not valid for this storage.
    fn borrow_mut<S: ErasableStorage<ErasedStorage = ErasedStorage>>(
        &self,
        idx: StorageIdx<S>,
    ) -> Result<RefMut<S>, BorrowMutError> {
        let erased_storage_ref = self.storages[idx.idx].try_borrow_mut()?;

        let storage = RefMut::map(erased_storage_ref, |erased| {
            S::downcast_mut(erased).unwrap()
        });

        Ok(storage)
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
