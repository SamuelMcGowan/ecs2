mod erased_storages;
mod sparse;
mod storage;
mod world;

pub mod prelude {
    pub use crate::storage::components::Component;
    pub use crate::storage::entities::{EntityError, EntityId, EntityIter};
    pub use crate::storage::unique::Unique;
    pub use crate::world::World;
}
