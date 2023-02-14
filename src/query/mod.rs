use crate::{prelude::World, world::WorldData};

pub mod component;
pub mod unique;

pub trait Query<D: WorldData> {
    type Output<'a>;
    fn borrow(world: &World<D>) -> QueryResult<Self::Output<'_>>;
}

#[derive(thiserror::Error, Debug)]
pub enum QueryError {
    #[error("storage is missing")]
    StorageMissing,

    #[error("{0}")]
    BorrowError(#[from] std::cell::BorrowError),

    #[error("{0}")]
    BorrowMutError(#[from] std::cell::BorrowMutError),

    #[error("entity is dead")]
    EntityDead,

    #[error("entity not found in storage")]
    EntityMissing,
}

pub type QueryResult<T> = Result<T, QueryError>;
