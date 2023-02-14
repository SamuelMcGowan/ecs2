pub mod query;
pub mod storage;
pub mod world;

mod sparse;
mod erased_storages;

pub mod prelude {
    pub use crate::query::component::{Comp, CompMut, CompBorrow, CompBorrowMut};
    pub use crate::query::Query;
    pub use crate::storage::component::Component;
    pub use crate::storage::entities::EntityId;
    pub use crate::storage::unique::Unique;
    pub use crate::world::World;
}
