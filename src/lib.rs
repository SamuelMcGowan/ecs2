pub mod query;
pub mod storage;
pub mod world;

mod erased_storages;
mod sparse;

pub mod prelude {
    pub use crate::query::component::{BorrowComp, BorrowCompMut, QueryComp, QueryCompMut};
    pub use crate::query::unique::{BorrowUnique, BorrowUniqueMut, QueryUnique, QueryUniqueMut};
    pub use crate::query::Query;
    pub use crate::storage::component::Component;
    pub use crate::storage::entities::EntityId;
    pub use crate::storage::unique::Unique;
    pub use crate::world::World;
}
