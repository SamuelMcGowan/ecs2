mod erased_storages;
mod query;
mod sparse;
mod storage;
mod world;

pub mod prelude {
    pub use crate::query::component::{Comp, CompMut, CompView, CompViewMut};
    pub use crate::query::Query;
    pub use crate::storage::component::Component;
    pub use crate::storage::entities::{EntityError, EntityId, EntityIter};
    pub use crate::storage::unique::Unique;
    pub use crate::world::World;
}
