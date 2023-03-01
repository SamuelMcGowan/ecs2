use super::array::SparseArray;

#[derive(Default, Debug, Clone)]
struct DenseEntry<T> {
    sparse_index: usize,
    element: T,
}

/// A sparse set. This container allows its contents to be densely packed while allowing sparsely
/// populated indices.
///
/// It's implemented as a sparse array of indices mapping to a dense array of the actual elements.
/// The sparse array is paginated so that the memory usage is acceptable.
#[derive(Debug, Clone)]
pub(crate) struct SparseSet<T> {
    sparse: SparseArray<usize>,
    dense: Vec<DenseEntry<T>>,
}

impl<T> Default for SparseSet<T> {
    fn default() -> Self {
        Self {
            sparse: SparseArray::default(),
            dense: vec![],
        }
    }
}

impl<T> SparseSet<T> {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Get a reference to an element.
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        let dense_index = *self.sparse.get(index)?;
        Some(&self.dense[dense_index].element)
    }

    /// Get a mutable reference to an element.
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let dense_index = *self.sparse.get(index)?;
        Some(&mut self.dense[dense_index].element)
    }

    /// Insert a new element.
    ///
    /// Returns the previous element at this index if there was one.
    #[inline]
    pub fn insert(&mut self, index: usize, element: T) -> Option<T> {
        let entry = DenseEntry {
            element,
            sparse_index: index,
        };

        match self.sparse.get(index) {
            // Replace an existing entry.
            Some(&dense_index) => {
                let prev = std::mem::replace(&mut self.dense[dense_index], entry);
                Some(prev.element)
            }

            // Add a new entry.
            None => {
                self.dense.push(entry);
                self.sparse.insert(index, self.dense.len() - 1);
                None
            }
        }
    }

    /// Remove an element.
    #[inline]
    pub fn remove(&mut self, index: usize) -> Option<T> {
        let dense_index = self.sparse.remove(index)?;

        // Swap-remove the entry from the dense array.
        let removed = self.dense.swap_remove(dense_index);

        // If another entry was moved in to replace the removed one, update its
        // sparse array entry.
        if dense_index < self.dense.len() {
            let sparse_swapped_index = self.dense[dense_index].sparse_index;
            self.sparse.insert(sparse_swapped_index, index);
        }

        Some(removed.element)
    }

    /// Iterate over the elements in this set.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.dense.iter().map(|dense_entry| &dense_entry.element)
    }

    /// Iterate mutably over the elements in this set.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.dense
            .iter_mut()
            .map(|dense_entry| &mut dense_entry.element)
    }

    #[inline]
    pub fn iter_with_indices(&self) -> impl Iterator<Item = (usize, &T)> {
        self.dense
            .iter()
            .map(|dense_entry| (dense_entry.sparse_index, &dense_entry.element))
    }

    #[inline]
    pub fn iter_mut_with_indices(&mut self) -> impl Iterator<Item = (usize, &mut T)> {
        self.dense
            .iter_mut()
            .map(|dense_entry| (dense_entry.sparse_index, &mut dense_entry.element))
    }

    #[inline]
    pub fn contains(&self, index: usize) -> bool {
        self.sparse.get(index).is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_elements() {
        let mut set = SparseSet::default();

        set.insert(0, 12);
        set.insert(1, 16);

        assert_eq!(set.get(0), Some(&12));
    }

    #[test]
    fn remove_elements() {
        let mut set = SparseSet::default();

        set.insert(0, 12);
        set.insert(1, 16);

        assert_eq!(set.remove(0), Some(12));
        assert_eq!(set.remove(0), None);

        assert_eq!(set.get(1), Some(&16));

        assert_eq!(set.remove(1), Some(16));
        assert_eq!(set.remove(1), None);
    }
}
