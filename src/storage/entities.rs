// THANKS TO: https://skypjack.github.io/2019-05-06-ecs-baf-part-3/

use std::num::NonZeroU32;
use std::slice;

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct EntityId {
    index: NonZeroU32,
    version: u32,
}

impl EntityId {
    #[cfg(test)]
    fn new(index: u32, version: u32) -> Option<Self> {
        Some(Self {
            index: NonZeroU32::new(index)?,
            version,
        })
    }

    #[inline]
    pub(crate) fn index(&self) -> usize {
        u32::from(self.index) as usize
    }
}

impl std::fmt::Debug for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}v{}", self.index, self.version)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum EntityError {
    #[error("no more entities available")]
    OutOfEntities,

    #[error("entity is dead")]
    DeadEntity,
}

#[derive(Debug, Clone, Copy)]
struct EntityEntry {
    state: EntryState,
    version: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum EntryState {
    Dead(NonZeroU32),
    Alive,
}

impl EntityEntry {
    fn as_id(&self, index: NonZeroU32) -> EntityId {
        EntityId {
            index,
            version: self.version,
        }
    }
}

impl EntryState {
    fn next_free(self) -> Option<NonZeroU32> {
        match self {
            Self::Dead(index) => Some(index),
            Self::Alive => None,
        }
    }
}

#[derive(Debug)]
pub(crate) struct EntityStorage {
    entries: Vec<EntityEntry>,
    next_free: NonZeroU32,
    num_free: usize,
}

impl Default for EntityStorage {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl EntityStorage {
    pub fn new() -> Self {
        Self {
            entries: vec![EntityEntry {
                // We need one dummy entity so that no real entity has an index of zero.

                // This isn't free, but it can't be marked as occupied, and it
                // won't be accessible by the free list, so it's fine.
                state: EntryState::Dead(NonZeroU32::new(1).unwrap()),
                version: 0,
            }],
            // it doesn't matter what this is
            next_free: NonZeroU32::new(1).unwrap(),
            num_free: 0,
        }
    }

    /// Allocate a new entity.
    pub fn alloc(&mut self) -> Result<EntityId, EntityError> {
        if self.num_free > 0 {
            let entry = &mut self.entries[u32::from(self.next_free) as usize];
            let index = self.next_free;

            // Pop from the implicit linked list.
            self.next_free = entry.state.next_free().unwrap();
            self.num_free -= 1;
            entry.state = EntryState::Alive;

            Ok(entry.as_id(index))
        } else {
            // The storage length will never be greater than `u32::MAX`, so it's fine to
            // truncate it.
            let index = self.entries.len() as u32;

            if index == u32::MAX {
                return Err(EntityError::OutOfEntities);
            }

            let entity = EntityEntry {
                state: EntryState::Alive,
                version: 0,
            };
            self.entries.push(entity);

            Ok(entity.as_id(NonZeroU32::new(index).unwrap()))
        }
    }

    /// Deallocate an entity.
    pub fn dealloc(&mut self, entity: EntityId) -> Result<(), EntityError> {
        if !self.is_alive(entity) {
            return Err(EntityError::DeadEntity);
        }

        let entry = &mut self.entries[entity.index()];

        // Increment the version.
        // Version will not be greater than `u32::MAX` - 1, so it won't overflow.
        entry.version += 1;

        // Recycle this index if possible by adding it to the implicit linked list.
        // The entity can't be reused if its new version is `u32::MAX`, because its
        // version wouldn't be incrementable when it was despawned.

        if entry.version < u32::MAX {
            entry.state = EntryState::Dead(self.next_free);
            self.next_free = entity.index;
            self.num_free += 1;
        }

        Ok(())
    }

    /// Check if an entity is alive (and present in this storage).
    #[inline]
    pub fn is_alive(&self, entity: EntityId) -> bool {
        let Some(stored) = self.entries.get(entity.index()) else {
            return false;
        };

        // An entity is dead if the version is not the most recent,
        // or the entity is in the free list.

        // Normally, checking that the version is the same would be enough,
        // but an entity that is not from this storage could have the same version as an
        // entity in the free list, so we check anyway.

        stored.state == EntryState::Alive && stored.version == entity.version
    }

    /// Iterate over all alive entities.
    #[inline]
    pub fn iter(&self) -> EntityIter {
        EntityIter {
            iter: self.entries.iter(),
            index: 0,
        }
    }
}

pub struct EntityIter<'a> {
    iter: slice::Iter<'a, EntityEntry>,
    index: u32,
}

impl Iterator for EntityIter<'_> {
    type Item = EntityId;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let next = self.iter.next()?;
            let index = self.index;

            // If this wraps, this is the last element, so this
            // is fine.
            self.index = self.index.wrapping_add(1);

            if let EntryState::Alive = next.state {
                let index = NonZeroU32::new(index).unwrap();
                return Some(next.as_id(index));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entity_id_size() {
        assert_eq!(std::mem::size_of::<EntityId>(), 8);
    }

    #[test]
    fn alloc() {
        let mut storage = EntityStorage::new();

        let a = storage.alloc().unwrap();
        let b = storage.alloc().unwrap();

        assert_eq!(a, EntityId::new(1, 0).unwrap());
        assert_eq!(b, EntityId::new(2, 0).unwrap());

        assert!(storage.is_alive(a));
        assert!(storage.is_alive(b));
    }

    #[test]
    fn dealloc() {
        let mut storage = EntityStorage::new();

        let a = storage.alloc().unwrap();
        let b = storage.alloc().unwrap();

        storage.dealloc(a).unwrap();
        storage.dealloc(b).unwrap();

        assert!(!storage.is_alive(a));
        assert!(!storage.is_alive(b));
    }

    #[test]
    fn realloc() {
        let mut storage = EntityStorage::new();

        let a = storage.alloc().unwrap();
        let b = storage.alloc().unwrap();

        storage.dealloc(a).unwrap();
        storage.dealloc(b).unwrap();

        let b_v2 = storage.alloc().unwrap();
        let a_v2 = storage.alloc().unwrap();

        let c = storage.alloc().unwrap();

        assert_eq!(a_v2, EntityId::new(1, 1).unwrap());
        assert_eq!(b_v2, EntityId::new(2, 1).unwrap());
        assert_eq!(c, EntityId::new(3, 0).unwrap());
    }

    #[test]
    fn random_entities() {
        let mut storage = EntityStorage::new();

        let a = storage.alloc().unwrap();
        let b = storage.alloc().unwrap();

        assert_eq!(a, EntityId::new(1, 0).unwrap());
        assert_eq!(b, EntityId::new(2, 0).unwrap());

        assert!(storage.is_alive(a));
        assert!(storage.is_alive(b));
        assert_eq!(storage.iter().count(), 2);

        storage.dealloc(a).unwrap();
        assert!(!storage.is_alive(a));
        assert!(storage.is_alive(b));
        assert_eq!(storage.iter().count(), 1);

        storage.dealloc(b).unwrap();
        assert!(!storage.is_alive(a));
        assert!(!storage.is_alive(b));
        assert_eq!(storage.iter().count(), 0);
    }
}
