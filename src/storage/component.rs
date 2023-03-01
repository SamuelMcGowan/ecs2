use crate::sparse::SparseSet;

use super::entities::EntityId;

pub trait Component: 'static {}

pub(crate) struct ComponentStorage<C: Component>(SparseSet<C>);

impl<C: Component> Default for ComponentStorage<C> {
    #[inline]
    fn default() -> Self {
        ComponentStorage(SparseSet::default())
    }
}

impl<C: Component> ComponentStorage<C> {
    #[inline]
    pub fn get(&self, id: EntityId) -> Option<&C> {
        self.0.get(id.index())
    }

    #[inline]
    pub fn get_mut(&mut self, id: EntityId) -> Option<&mut C> {
        self.0.get_mut(id.index())
    }

    #[inline]
    pub fn insert(&mut self, entity: EntityId, component: C) -> Option<C> {
        self.0.insert(entity.index(), component)
    }

    #[inline]
    pub fn remove(&mut self, entity: EntityId) -> Option<C> {
        self.0.remove(entity.index())
    }

    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &C> {
        self.0.iter()
    }

    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut C> {
        self.0.iter_mut()
    }

    #[inline]
    pub fn iter_with_indices(&self) -> impl Iterator<Item = (usize, &C)> {
        self.0.iter_with_indices()
    }

    #[inline]
    pub fn iter_mut_with_indices(&mut self) -> impl Iterator<Item = (usize, &mut C)> {
        self.0.iter_mut_with_indices()
    }

    #[inline]
    pub fn contains(&self, entity: EntityId) -> bool {
        self.0.contains(entity.index())
    }
}
