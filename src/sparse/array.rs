const PAGE_SIZE: usize = 4096;

type Page<T> = [Option<T>];

#[derive(Debug, Clone)]
pub(super) struct SparseArray<T> {
    pages: Vec<Option<Box<Page<T>>>>,
}

impl<T> Default for SparseArray<T> {
    fn default() -> Self {
        Self { pages: vec![] }
    }
}

impl<T> SparseArray<T> {
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        let (page_index, offset) = page_index(index);

        let page = self.pages.get(page_index)?.as_ref()?;
        page[offset].as_ref()
    }

    #[inline]
    pub fn insert(&mut self, index: usize, element: T) -> Option<T> {
        let (page_index, offset) = page_index(index);

        if page_index >= self.pages.len() {
            self.create_page(page_index);
        }

        let page = self.pages[page_index].as_deref_mut().unwrap();
        page[offset].replace(element)
    }

    #[inline]
    pub fn remove(&mut self, index: usize) -> Option<T> {
        let (page_index, offset) = page_index(index);
        let page = self.pages.get_mut(page_index)?.as_deref_mut()?;
        page[offset].take()
    }

    /// Panics if the page already exists.
    #[inline]
    fn create_page(&mut self, page_index: usize) {
        debug_assert!(page_index >= self.pages.len());

        let new_len = page_index + 1;
        let none_pages = new_len - self.pages.len() - 1;

        self.pages.reserve(none_pages);
        self.pages
            .extend(std::iter::repeat_with(|| None).take(none_pages));

        let page = std::iter::repeat_with(|| None)
            .take(PAGE_SIZE)
            .collect::<Box<_>>();

        self.pages.push(Some(page));
    }
}

fn page_index(index: usize) -> (usize, usize) {
    (index / PAGE_SIZE, index % PAGE_SIZE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_elements() {
        let mut arr = SparseArray::default();

        arr.insert(0, 12);
        assert_eq!(arr.get(0), Some(&12));

        // skip over one page
        arr.insert(PAGE_SIZE * 2, 16);
        assert_eq!(arr.get(PAGE_SIZE * 2), Some(&16));

        assert_eq!(arr.pages.len(), 3);

        // next page
        arr.insert(PAGE_SIZE * 3, 20);
        assert_eq!(arr.get(PAGE_SIZE * 3), Some(&20));

        assert_eq!(arr.pages.len(), 4);
    }

    #[test]
    fn remove_elements() {
        let mut arr = SparseArray::default();

        arr.insert(0, 12);
        assert_eq!(arr.get(0), Some(&12));

        arr.insert(PAGE_SIZE * 2, 16);
        assert_eq!(arr.get(PAGE_SIZE * 2), Some(&16));

        assert_eq!(arr.remove(0), Some(12));
        assert_eq!(arr.remove(0), None);

        assert_eq!(arr.remove(PAGE_SIZE * 2), Some(16));
        assert_eq!(arr.remove(PAGE_SIZE * 2), None);
    }
}
