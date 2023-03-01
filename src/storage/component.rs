use crate::sparse::SparseSet;

pub trait Component: 'static {}

pub(crate) struct ComponentStorage<C: Component>(pub SparseSet<C>);

impl<C: Component> Default for ComponentStorage<C> {
    #[inline]
    fn default() -> Self {
        ComponentStorage(SparseSet::default())
    }
}
